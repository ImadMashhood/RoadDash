//! Authored road/terrain graybox and deterministic gameplay scenery.
use crate::{
    game::{Phase, Race},
    track::Track,
    world::{self, Art, Ground, Obstacle, SceneryFootprint},
};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use rr_content::track::{PropKind, Sector, Surface};
/// Tags only the existing building mass, excluding roof/windows/trees/props.
#[derive(Component)]
pub struct LegacyBuilding {
    pub id: String,
    pub station_m: f32,
}
#[derive(Component)]
pub struct RoadChunk;
#[derive(Component)]
pub struct Terrain;
#[derive(Component)]
pub struct Cow;
#[derive(Component)]
pub struct HazardZone {
    pub kind: PropKind,
    pub radius: f32,
}
#[derive(Component)]
pub struct Pedestrian;
#[derive(Component)]
pub struct PedestrianLimb {
    pub side: f32,
    pub rest: Transform,
    pub pivot: Vec3,
}
#[derive(Component)]
pub struct CrossTraffic;
#[derive(Component)]
pub struct ZoneProp(pub Sector);
#[derive(Component)]
pub struct Patrol {
    pub center: Vec3,
    pub axis: Vec3,
    pub extent: f32,
    pub speed: f32,
    pub phase: f32,
    pub one_way: bool,
}
#[derive(Resource, Default)]
pub struct TrackInspector {
    pub visible: bool,
    pub render_triangles: usize,
    pub collision_triangles: usize,
}
fn mesh(vertices: &[Vec3], indices: &[[u32; 3]]) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vertices
            .iter()
            .map(|p| [p.x * 0.1, p.z * 0.1])
            .collect::<Vec<_>>(),
    )
    .with_inserted_indices(Indices::U32(indices.iter().flatten().copied().collect()))
    .with_computed_normals()
}
pub fn ribbon(
    track: &Track,
    start: f32,
    end: f32,
    step: f32,
    extra: f32,
    offset: f32,
) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let count = ((end - start) / step).ceil() as usize;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for i in 0..=count {
        let d = start + (end - start) * i as f32 / count as f32;
        let (p, q) = track.sample(d, 0.0);
        let half = track.half_width(d) + extra;
        vertices.push(p + q * Vec3::new(-half, offset, 0.0));
        vertices.push(p + q * Vec3::new(half, offset, 0.0));
        if i < count {
            let n = i as u32 * 2;
            indices.extend([[n, n + 2, n + 1], [n + 1, n + 2, n + 3]]);
        }
    }
    (vertices, indices)
}
fn block(
    commands: &mut Commands,
    art: &Art,
    material: Handle<StandardMaterial>,
    p: Vec3,
    size: Vec3,
    q: Quat,
    solid: bool,
) -> Entity {
    let e = world::part(commands, art.cube.clone(), material, p, size, q);
    if solid {
        commands.entity(e).insert((
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5, 0.5),
            Obstacle,
            Friction::coefficient(0.7),
        ));
    }
    e
}
pub fn build(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
    art: &Art,
) {
    let asphalt = materials.add(Color::srgb(0.10, 0.115, 0.13));
    let gravel = materials.add(Color::srgb(0.42, 0.39, 0.28));
    let grass = materials.add(Color::srgb(0.23, 0.39, 0.14));
    let yellow = materials.add(Color::srgb(0.95, 0.72, 0.22));
    let bark = materials.add(Color::srgb(0.3, 0.19, 0.10));
    let leaf = materials.add(Color::srgb(0.12, 0.28, 0.12));
    let brick = materials.add(Color::srgb(0.48, 0.25, 0.17));
    let concrete = materials.add(Color::srgb(0.45, 0.48, 0.48));
    let window = materials.add(Color::srgb(0.19, 0.36, 0.43));
    let mut render_triangles = 0;
    let mut collision_triangles = 0;
    let chunks = (track.length / 80.0).ceil() as usize;
    for i in 0..chunks {
        let start = if i == 0 && !track.definition.closed {
            -45.0
        } else {
            i as f32 * track.length / chunks as f32
        };
        let end = (i + 1) as f32 * track.length / chunks as f32;
        let (v, ix) = ribbon(track, start, end, 1.25, 0.0, 0.025);
        render_triangles += ix.len();
        let (c, ci) = ribbon(track, start, end, 2.5, 0.0, 0.02);
        collision_triangles += ci.len();
        commands.spawn((
            Mesh3d(meshes.add(mesh(&v, &ix))),
            MeshMaterial3d(asphalt.clone()),
            Transform::default(),
            RigidBody::Fixed,
            Collider::trimesh(c, ci).expect("road collision"),
            Friction::coefficient(track.surface(start, 0.0).friction()),
            Ground,
            RoadChunk,
        ));
        let (v, ix) = ribbon(track, start, end, 2.5, track.definition.verge_width, -0.04);
        render_triangles += ix.len();
        commands.spawn((
            Mesh3d(meshes.add(mesh(&v, &ix))),
            MeshMaterial3d(gravel.clone()),
            Transform::default(),
            RigidBody::Fixed,
            Collider::trimesh(v, ix).expect("verge collision"),
            Ground,
            Friction::coefficient(Surface::Gravel.friction()),
        ));
    }
    // A single-valued height field surrounds the road. Render and simplified
    // collision grids sample the same height function; chunking bounds mesh size.
    let (min, max) = track.bounds;
    let spacing = 10.0;
    let tile = 200.0;
    let nx = ((max.x - min.x + 400.0) / tile).ceil() as usize;
    let nz = ((max.z - min.z + 400.0) / tile).ceil() as usize;
    for tx in 0..nx {
        for tz in 0..nz {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            let count = (tile / spacing) as usize;
            for z in 0..=count {
                for x in 0..=count {
                    let x = min.x - 200.0 + tx as f32 * tile + x as f32 * spacing;
                    let z = min.z - 200.0 + tz as f32 * tile + z as f32 * spacing;
                    vertices.push(Vec3::new(x, track.terrain_height(x, z), z));
                }
            }
            for z in 0..count {
                for x in 0..count {
                    let a = (z * (count + 1) + x) as u32;
                    let b = a + (count + 1) as u32;
                    indices.extend([[a, b, a + 1], [a + 1, b, b + 1]]);
                }
            }
            render_triangles += indices.len();
            collision_triangles += indices.len();
            commands.spawn((
                Mesh3d(meshes.add(mesh(&vertices, &indices))),
                MeshMaterial3d(grass.clone()),
                Transform::default(),
                RigidBody::Fixed,
                Collider::trimesh(vertices, indices).expect("terrain collision"),
                Ground,
                Terrain,
                Friction::coefficient(Surface::Grass.friction()),
            ));
        }
    }
    let cone = meshes.add(Cone::new(1.0, 1.0).mesh().resolution(7));
    let count = (track.length / 14.0) as usize;
    for i in 0..count {
        let d = i as f32 * track.length / count as f32;
        let (p, q) = track.sample(d, 0.0);
        let width = track.half_width(d);
        for lane in [-width + 0.15, width - 0.15] {
            block(
                commands,
                art,
                art.white.clone(),
                p - q * Vec3::X * lane + q * Vec3::Y * 0.055,
                Vec3::new(0.10, 0.015, 14.0),
                q,
                false,
            );
        }
        if i % 2 == 0 {
            block(
                commands,
                art,
                yellow.clone(),
                p + q * Vec3::Y * 0.06,
                Vec3::new(0.13, 0.02, 6.0),
                q,
                false,
            );
        }
        let mut random = (i as u32)
            .wrapping_mul(747796405)
            .wrapping_add(track.definition.scenery_seed)
            .wrapping_add(2891336453);
        random = ((random >> ((random >> 28) + 4)) ^ random).wrapping_mul(277803737);
        random = (random >> 22) ^ random;
        let sector = track.sector(d);
        if sector == Sector::Hills && track.curvature(d).abs() > 0.0025 {
            {
                let lane = 10.8 * track.curvature(d).signum();
                let (v, r) = track.sample(d, lane);
                block(
                    commands,
                    art,
                    art.metal.clone(),
                    v + Vec3::Y * 0.8,
                    Vec3::new(0.18, 0.25, 14.3),
                    r,
                    true,
                );
                block(
                    commands,
                    art,
                    art.metal.clone(),
                    v + Vec3::Y * 0.4,
                    Vec3::new(0.15, 0.8, 0.15),
                    r,
                    true,
                );
            }
        }
        let near_crossing = track.definition.props.iter().any(|prop| {
            prop.kind == PropKind::Crossing
                && crate::game::wrapped_gap(d - prop.fraction * track.length, track.length).abs()
                    < 24.0
        });

        if i % 3 == 0 {
            for side in [-1.0, 1.0] {
                let noise = (random % 1000) as f32 / 1000.0;
                match sector {
                    Sector::Farms => {
                        if random.is_multiple_of(5) {
                            // Intermittent fences leave open fields and access gaps.
                            for k in 0..4 {
                                let (v, r) = track.sample(d + k as f32 * 3.0, side * 19.0);
                                let v = v.with_y(track.terrain_height(v.x, v.z));
                                block(
                                    commands,
                                    art,
                                    bark.clone(),
                                    v + Vec3::Y * 0.65,
                                    Vec3::new(0.16, 1.3, 0.16),
                                    r,
                                    true,
                                );
                                if k < 3 {
                                    block(
                                        commands,
                                        art,
                                        bark.clone(),
                                        v + r * Vec3::new(0.0, 0.8, 1.5),
                                        Vec3::new(0.12, 0.13, 3.1),
                                        r,
                                        true,
                                    );
                                }
                            }
                        }
                        if random.is_multiple_of(7) {
                            let (v, r) = track.sample(d, side * (38.0 + noise * 24.0));
                            if track.scenery_fits(v, 12.0, 4.0) {
                                let e = block(
                                    commands,
                                    art,
                                    brick.clone(),
                                    v.with_y(track.terrain_height(v.x, v.z) + 3.0),
                                    Vec3::new(14.0, 6.0, 18.0),
                                    r,
                                    true,
                                );
                                commands.entity(e).insert((
                                    ZoneProp(sector),
                                    SceneryFootprint { radius: 12.0 },
                                    LegacyBuilding {
                                        id: format!("legacy-farm-{i}-{side}"),
                                        station_m: d,
                                    },
                                ));
                                block(
                                    commands,
                                    art,
                                    art.dark.clone(),
                                    v.with_y(track.terrain_height(v.x, v.z) + 6.2),
                                    Vec3::new(16.0, 0.6, 20.0),
                                    r,
                                    false,
                                );
                            }
                        }
                    }
                    Sector::City => {
                        for offset in [0.0, 18.0] {
                            if near_crossing {
                                continue;
                            }
                            let (v, r) = track.sample(d + offset, side * (23.0 + noise * 8.0));
                            let height = 9.0 + noise * 25.0;
                            if !track.scenery_fits(v, 10.0, 3.0) {
                                continue;
                            }
                            let floor = track.terrain_height(v.x, v.z);
                            let e = block(
                                commands,
                                art,
                                if i % 2 == 0 {
                                    concrete.clone()
                                } else {
                                    brick.clone()
                                },
                                v.with_y(floor + height * 0.5),
                                Vec3::new(13.0, height, 15.0),
                                r,
                                true,
                            );
                            commands.entity(e).insert((
                                ZoneProp(sector),
                                SceneryFootprint { radius: 10.0 },
                                LegacyBuilding {
                                    id: format!("legacy-city-{i}-{side}-{offset}"),
                                    station_m: d + offset,
                                },
                            ));
                            for level in 0..(height / 3.0) as usize {
                                for pane in [-4.0, 0.0, 4.0] {
                                    block(
                                        commands,
                                        art,
                                        window.clone(),
                                        v.with_y(floor + 2.0 + level as f32 * 3.0)
                                            + r * Vec3::X * side * 6.56
                                            + r * Vec3::Z * pane,
                                        Vec3::new(0.04, 1.3, 2.8),
                                        r,
                                        false,
                                    );
                                }
                            }
                            block(
                                commands,
                                art,
                                art.dark.clone(),
                                v.with_y(floor + 1.4) + r * Vec3::X * side * 6.58,
                                Vec3::new(0.1, 2.8, 2.0),
                                r,
                                false,
                            );
                            block(
                                commands,
                                art,
                                art.metal.clone(),
                                v.with_y(floor + height + 0.2),
                                Vec3::new(13.5, 0.4, 15.5),
                                r,
                                false,
                            );
                        }
                    }
                    Sector::Hills => {}
                }
                if i % 3 == 0 {
                    let (v, _) = track.sample(
                        d,
                        side * (if sector == Sector::City {
                            13.4
                        } else {
                            23.0 + noise * 40.0
                        }),
                    );
                    let h = if sector == Sector::City {
                        4.0 + noise * 2.0
                    } else {
                        5.0 + noise * 5.0
                    };
                    if track.scenery_fits(
                        v,
                        h * 0.4,
                        if sector == Sector::City { 1.5 } else { 5.0 },
                    ) {
                        let v = v.with_y(track.terrain_height(v.x, v.z));
                        let e = block(
                            commands,
                            art,
                            bark.clone(),
                            v + Vec3::Y * h * 0.4,
                            Vec3::new(0.5, h * 0.8, 0.5),
                            Quat::IDENTITY,
                            true,
                        );
                        commands.entity(e).insert(SceneryFootprint { radius: 0.5 });
                        let broad = noise > 0.65;
                        let e = world::part(
                            commands,
                            if broad {
                                art.sphere.clone()
                            } else {
                                cone.clone()
                            },
                            leaf.clone(),
                            v + Vec3::Y * h * 0.8,
                            if broad {
                                Vec3::splat(h * 0.4)
                            } else {
                                Vec3::new(h * 0.4, h, h * 0.4)
                            },
                            Quat::IDENTITY,
                        );
                        commands.entity(e).insert((
                            RigidBody::Fixed,
                            if broad {
                                Collider::ball(1.0)
                            } else {
                                Collider::cone(0.5, 1.0)
                            },
                            Obstacle,
                            SceneryFootprint { radius: h * 0.4 },
                        ));
                    }
                }
            }
        }
        if sector == Sector::City && !near_crossing {
            for side in [-1.0, 1.0] {
                let (v, r) = track.sample(d, side * 9.0);
                let sidewalk = block(
                    commands,
                    art,
                    concrete.clone(),
                    v + Vec3::Y * 0.08,
                    Vec3::new(3.0, 0.16, 14.2),
                    r,
                    true,
                );
                commands
                    .entity(sidewalk)
                    .insert((Ground, crate::vehicle::RideableStep));
                if i % 9 == 0 {
                    spawn_pedestrian(commands, art, v + Vec3::Y * 0.2, r, d);
                }
            }
        }
        if sector == Sector::Farms && i % 37 == 0 {
            let (v, r) = track.sample(d, if i % 2 == 0 { 14.0 } else { -14.0 });
            spawn_cow(commands, art, v.with_y(track.terrain_height(v.x, v.z)), r);
        }
    }
    for prop in &track.definition.props {
        let d = prop.fraction * track.length;
        let (p, q) = track.sample(d, prop.lane);
        match prop.kind {
            PropKind::Farm | PropKind::Building => {
                if track.scenery_fits(p, 12.0, 2.0) {
                    let h = if prop.kind == PropKind::Farm {
                        6.0
                    } else {
                        18.0
                    };
                    let e = block(
                        commands,
                        art,
                        brick.clone(),
                        p.with_y(track.terrain_height(p.x, p.z) + h * 0.5),
                        Vec3::new(14.0, h, 18.0),
                        q,
                        true,
                    );
                    commands.entity(e).insert((
                        ZoneProp(track.sector(d)),
                        SceneryFootprint { radius: 12.0 },
                        LegacyBuilding {
                            id: format!("legacy-authored-{}", prop.id.as_str()),
                            station_m: d,
                        },
                    ));
                }
            }
            PropKind::Fence | PropKind::Guardrail => {
                block(
                    commands,
                    art,
                    bark.clone(),
                    p + Vec3::Y * 0.7,
                    Vec3::new(0.2, 1.4, 8.0),
                    q,
                    true,
                );
            }
            PropKind::Pedestrian => spawn_pedestrian(commands, art, p, q, d),
            PropKind::Cow => spawn_cow(commands, art, p.with_y(track.terrain_height(p.x, p.z)), q),
            PropKind::Crossing => {
                block(
                    commands,
                    art,
                    asphalt.clone(),
                    p - Vec3::Y * 0.01,
                    Vec3::new(100.0, 0.08, 10.0),
                    q,
                    false,
                );
                for offset in [-18.0, 18.0] {
                    let sign = if offset < 0.0 { 1.0 } else { -1.0 };
                    let center = p + q * Vec3::Z * sign * 2.4;
                    let axis = q * Vec3::X * sign;
                    let root = commands
                        .spawn((
                            Transform::from_translation(center + axis * offset),
                            Visibility::default(),
                            RigidBody::KinematicPositionBased,
                            Collider::compound(vec![(
                                Vec3::Y,
                                Quat::IDENTITY,
                                Collider::cuboid(0.95, 0.9, 1.95),
                            )]),
                            Obstacle,
                            CrossTraffic,
                            ActiveCollisionTypes::all(),
                            Ccd::enabled(),
                            ActiveEvents::COLLISION_EVENTS,
                            Patrol {
                                center,
                                axis,
                                extent: 47.0,
                                speed: 9.0,
                                phase: offset / 47.0,
                                one_way: true,
                            },
                        ))
                        .id();
                    let child = world::part(
                        commands,
                        art.cube.clone(),
                        art.colors[2].clone(),
                        Vec3::Y * 0.8,
                        Vec3::new(1.9, 1.3, 3.9),
                        Quat::IDENTITY,
                    );
                    let mut parts = vec![
                        child,
                        world::part(
                            commands,
                            art.cube.clone(),
                            window.clone(),
                            Vec3::new(0.0, 1.5, -0.1),
                            Vec3::new(1.6, 0.65, 1.8),
                            Quat::IDENTITY,
                        ),
                    ];
                    for x in [-0.9, 0.9] {
                        for z in [-1.2, 1.2] {
                            parts.push(world::part(
                                commands,
                                art.wheel.clone(),
                                art.dark.clone(),
                                Vec3::new(x, 0.42, z),
                                Vec3::ONE,
                                Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                            ));
                        }
                    }
                    for x in [-0.6, 0.6] {
                        parts.push(world::part(
                            commands,
                            art.cube.clone(),
                            art.white.clone(),
                            Vec3::new(x, 0.8, 1.97),
                            Vec3::new(0.4, 0.2, 0.04),
                            Quat::IDENTITY,
                        ));
                    }
                    crate::runtime::attach_visuals(commands, root, &parts);
                }
            }
            PropKind::Cliff | PropKind::AnimalCrossing => {
                commands.spawn((
                    HazardZone {
                        kind: prop.kind,
                        radius: 25.0,
                    },
                    Transform::from_translation(p),
                ));
            }
        }
    }
    for (i, gate) in track.definition.checkpoints.iter().enumerate() {
        let (p, q) = track.sample(gate.fraction * track.length, 0.0);
        for side in [-1.0, 1.0] {
            block(
                commands,
                art,
                if i == 0 {
                    art.white.clone()
                } else {
                    yellow.clone()
                },
                p + q * Vec3::X * side * (track.half_width(gate.fraction * track.length) + 1.0)
                    + Vec3::Y * 2.0,
                Vec3::new(0.2, 4.0, 0.2),
                q,
                true,
            );
        }
        if i == 0 {
            block(
                commands,
                art,
                art.dark.clone(),
                p + Vec3::Y * 5.0,
                Vec3::new(16.0, 0.7, 0.5),
                q,
                true,
            );
        }
    }
    commands.insert_resource(TrackInspector {
        visible: false,
        render_triangles,
        collision_triangles,
    });
}
fn spawn_cow(commands: &mut Commands, art: &Art, p: Vec3, q: Quat) {
    let root = commands
        .spawn((
            Transform::from_translation(p).with_rotation(q),
            Visibility::default(),
            RigidBody::Fixed,
            Collider::compound(vec![(
                Vec3::Y * 0.9,
                Quat::IDENTITY,
                Collider::cuboid(1.1, 0.7, 0.5),
            )]),
            Cow,
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();
    let mut parts = vec![
        world::part(
            commands,
            art.cube.clone(),
            art.white.clone(),
            Vec3::Y,
            Vec3::new(2.2, 1.1, 0.95),
            Quat::IDENTITY,
        ),
        world::part(
            commands,
            art.cube.clone(),
            art.dark.clone(),
            Vec3::new(1.05, 1.2, 0.0),
            Vec3::new(0.65, 0.75, 0.65),
            Quat::IDENTITY,
        ),
    ];
    for x in [-0.7, 0.7] {
        for z in [-0.32, 0.32] {
            parts.push(world::part(
                commands,
                art.cube.clone(),
                art.dark.clone(),
                Vec3::new(x, 0.35, z),
                Vec3::new(0.18, 0.7, 0.18),
                Quat::IDENTITY,
            ));
        }
    }
    commands.entity(root).insert(crate::npc::Npc::new(
        crate::npc::NpcKind::Cow,
        Transform::from_translation(p).with_rotation(q),
    ));
    crate::runtime::attach_visuals(commands, root, &parts);
}
pub(crate) fn spawn_pedestrian(commands: &mut Commands, art: &Art, p: Vec3, q: Quat, seed: f32) {
    let root = commands
        .spawn((
            Transform::from_translation(p),
            Visibility::default(),
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y * 1.15,
                Quat::IDENTITY,
                Collider::capsule_y(0.8, 0.27),
            )]),
            Pedestrian,
            Obstacle,
            Patrol {
                center: p,
                axis: q * Vec3::Z,
                extent: 12.0,
                speed: 1.2,
                phase: seed.sin(),
                one_way: false,
            },
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();
    commands.entity(root).insert(crate::npc::Npc::new(
        crate::npc::NpcKind::Pedestrian,
        Transform::from_translation(p).with_rotation(q),
    ));
    let mut parts = Vec::new();
    for (i, (position, _, _)) in crate::human::SPECS.iter().enumerate() {
        let e = crate::human::segment(commands, art, 4, i, *position, Quat::IDENTITY);
        if let Some((side, leg, pivot)) = crate::human::limb(i) {
            commands.entity(e).insert(PedestrianLimb {
                side: side * if leg { 1.0 } else { -1.0 },
                rest: Transform::from_translation(*position),
                pivot,
            });
        }
        parts.push(e);
    }
    crate::runtime::attach_visuals(commands, root, &parts);
}
pub fn patrol(
    race: Res<Race>,
    track: Res<Track>,
    mut actors: Query<(&Patrol, &mut Transform, Option<&CrossTraffic>), Without<ColliderDisabled>>,
) {
    if race.phase != Phase::Racing {
        return;
    }
    for (path, mut t, car) in &mut actors {
        let phase = (race.elapsed * path.speed / path.extent + path.phase).rem_euclid(4.0);
        let f = if path.one_way {
            phase.rem_euclid(2.0) - 1.0
        } else if phase < 2.0 {
            phase - 1.0
        } else {
            3.0 - phase
        };
        let p = path.center + path.axis * f * path.extent;
        t.translation = p;
        if car.is_none() {
            t.translation.y = track.terrain_height(p.x, p.z) + 0.2;
        }
        let direction = if path.one_way || phase < 2.0 {
            1.0
        } else {
            -1.0
        };
        t.rotation =
            Quat::from_rotation_y((path.axis.x * direction).atan2(path.axis.z * direction));
    }
}
pub fn inspector(
    keys: Res<ButtonInput<KeyCode>>,
    track: Res<Track>,
    mut state: ResMut<TrackInspector>,
    mut gizmos: Gizmos,
) {
    if keys.just_pressed(KeyCode::F4) {
        state.visible = !state.visible;
        info!(
            "Track {}: {:.0}m, {} render triangles, {} collision triangles, {} gates",
            track.definition.id.as_str(),
            track.length,
            state.render_triangles,
            state.collision_triangles,
            track.definition.checkpoints.len()
        );
    }
    if !state.visible {
        return;
    }
    for i in (0..track.points.len() - 1).step_by(16) {
        let p = track.points[i];
        let d = track.distances[i];
        let (_, q) = track.sample(d, 0.0);
        gizmos.line(
            p + Vec3::Y * 0.1,
            track.points[(i + 16).min(track.points.len() - 1)] + Vec3::Y * 0.1,
            Color::srgb(1.0, 0.6, 0.1),
        );
        gizmos.line(p, p + q * Vec3::Y * 4.0, Color::srgb(0.2, 1.0, 0.2));
        gizmos.line(
            p - q * Vec3::X * track.half_width(d),
            p + q * Vec3::X * track.half_width(d),
            Color::srgb(0.3, 0.6, 1.0),
        );
    }
    let (min, max) = track.bounds;
    gizmos.cuboid(
        Transform::from_translation((min + max) * 0.5).with_scale((max - min).max(Vec3::ONE)),
        Color::srgb(0.6, 0.4, 1.0),
    );
    for gate in &track.definition.checkpoints {
        let (p, q) = track.sample(gate.fraction * track.length, 0.0);
        gizmos.line(
            p - q * Vec3::X * 12.0 + Vec3::Y * 2.0,
            p + q * Vec3::X * 12.0 + Vec3::Y * 2.0,
            Color::srgb(1.0, 0.2, 0.2),
        );
    }
    for spawn in &track.definition.spawns {
        gizmos.sphere(
            Isometry3d::from_translation(track.sample(spawn.distance, spawn.lane).0 + Vec3::Y),
            0.5,
            Color::srgb(0.2, 1.0, 0.8),
        );
    }
}

pub fn animate_civilians(
    time: Res<Time>,
    race: Res<Race>,
    visuals: Query<&crate::runtime::VisualRoot>,
    npcs: Query<&crate::npc::Npc>,
    mut limbs: Query<(&PedestrianLimb, &ChildOf, &mut Transform)>,
) {
    for (limb, parent, mut t) in &mut limbs {
        if let Ok(visual) = visuals.get(parent.parent())
            && let Ok(npc) = npcs.get(visual.owner)
        {
            let turn = Quat::from_rotation_x(if race.phase == Phase::Racing && npc.active() {
                (time.elapsed_secs() * 9.0).sin() * limb.side * 0.45
            } else {
                0.0
            });
            t.translation = limb.pivot + turn * (limb.rest.translation - limb.pivot);
            t.rotation = turn * limb.rest.rotation;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn road_meshes_have_upward_winding_and_collision_uses_fewer_triangles() {
        let t = Track::from_json(include_str!("../content/tracks/valley.json")).unwrap();
        for fraction in [0.0, 0.27, 0.45, 0.60, 0.97] {
            let d = fraction * t.length;
            let (v, ix) = ribbon(&t, d, d + 40.0, 1.25, 0.0, 0.025);
            let (_, ci) = ribbon(&t, d, d + 40.0, 2.5, 0.0, 0.02);
            assert!(ci.len() < ix.len());
            for tri in ix {
                let normal = (v[tri[1] as usize] - v[tri[0] as usize])
                    .cross(v[tri[2] as usize] - v[tri[0] as usize]);
                assert!(normal.y > 0.0);
            }
        }
    }
    #[test]
    fn scenic_collision_supports_the_route_and_populates_each_gameplay_zone() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 3,
        })
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f64(1.0 / 60.0),
        ))
        .insert_resource(Track::from_json(include_str!("../content/tracks/valley.json")).unwrap())
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(
            Startup,
            (world::setup_world, world::validate_scenery).chain(),
        );
        for _ in 0..4 {
            app.update();
        }
        let world = app.world_mut();
        let mut contexts = world.query::<(
            &RapierContextSimulation,
            &RapierContextColliders,
            &RapierContextJoints,
            &RapierRigidBodySet,
        )>();
        let (sim, colliders, joints, bodies) = contexts.single(world).unwrap();
        let context = RapierContext {
            simulation: sim,
            colliders,
            rigidbody_set: bodies,
            joints,
        };
        let track = world.resource::<Track>();
        for i in 0..80 {
            for lane in [-5.0, 0.0, 5.0] {
                let p = track.sample(i as f32 * track.length / 80.0, lane).0;
                let hit = context.cast_ray(
                    p + Vec3::Y * 3.0,
                    -Vec3::Y,
                    6.0,
                    true,
                    QueryFilter::default(),
                );
                assert!(hit.is_some(), "road hole at {p:?}");
                let (entity, toi) = hit.unwrap();
                assert!(
                    world.get::<RoadChunk>(entity).is_some(),
                    "terrain occludes authored road at {p:?}"
                );
                assert!(
                    (3.0 - toi).abs() < 0.4,
                    "road/terrain mismatch at {p:?}: {toi}"
                );
            }
        }
        assert!(world.query::<&Cow>().iter(world).count() >= 2);
        assert!(world.query::<&Pedestrian>().iter(world).count() > 4);
        assert!(world.query::<&CrossTraffic>().iter(world).count() >= 2);
        let zones: Vec<_> = world
            .query::<&ZoneProp>()
            .iter(world)
            .map(|p| p.0)
            .collect();
        assert!(zones.contains(&Sector::City) && zones.contains(&Sector::Farms));
    }
}
