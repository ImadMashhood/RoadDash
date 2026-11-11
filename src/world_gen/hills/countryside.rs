//! Bounded countryside dressing using the existing original prop catalog.
use super::*;
use crate::world::{Ground, Obstacle};
use bevy::{camera::visibility::VisibilityRange, light::NotShadowCaster};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct HillsTree;
#[derive(Component)]
pub struct HillsGrass;
#[derive(Component)]
pub struct HillsFarm;
#[derive(Component)]
struct FarmField;
#[derive(Component)]
struct FarmYard;
#[derive(Clone, Debug)]
struct FarmProp {
    name: &'static str,
    position: Vec3,
    rotation: Quat,
    size: Vec3,
}
// Reuse the same authored GLB models and their materials as Metropolis, rather
// than substituting oversized primitive hay or empty barn-shaped boxes.
fn farm_prop_plan(track: &Track, p: Vec3, q: Quat) -> Vec<FarmProp> {
    let mut out: Vec<FarmProp> = vec![];
    for (name, x, z, size) in [
        ("WINDVN2", -29., -33., Vec3::new(2.2, 5.108, 1.23)),
        ("BRNGRN2", -24., 36., Vec3::new(5.08, 3.79, 7.4)),
        ("WGNBRRL2", 7., 29., Vec3::new(2.8, 1.07, 1.68)),
        ("TRUCK2", 7., -25., Vec3::new(2.56, 2.62, 6.61)),
        ("HAYDOUB2", -26., 26., Vec3::new(2.5, 0.66, 1.31)),
        ("HAYTRIP2", -22., 26., Vec3::new(2.5, 0.66, 1.31)),
        ("HAYDOUB2", -18., 26., Vec3::new(2.5, 0.66, 1.31)),
        ("HAYTRIP2", -26., 29., Vec3::new(2.5, 0.66, 1.31)),
        ("HAYDOUB2", -22., 29., Vec3::new(2.5, 0.66, 1.31)),
        ("HAYTRIP2", -18., 29., Vec3::new(2.5, 0.66, 1.31)),
        ("TRSHBNB2", 7., 9., Vec3::new(0.67, 0.89, 0.67)),
        ("TRSHBNB2", 7., 11., Vec3::new(0.67, 0.89, 0.67)),
        ("TRSHBNB2", 7., 13., Vec3::new(0.67, 0.89, 0.67)),
        ("WGNBRRL2", 26., -19., Vec3::new(2.8, 1.07, 1.68)),
    ] {
        let radius = size.x.hypot(size.z) * 0.5;
        'candidate: for (dx, dz) in [
            (0., 0.),
            (0., 3.),
            (0., -3.),
            (0., 6.),
            (0., -6.),
            (-4., 0.),
            (4., 0.),
            (-4., 6.),
            (4., 6.),
            (-4., -6.),
            (4., -6.),
        ] {
            let local = Vec3::new(x + dx, 0., z + dz);
            // Keep new equipment out of the main barn, cow pen and silos.
            if ((local.x - 16.).abs() < 5.5 + radius && local.z.abs() < 8. + radius)
                || (local.x > -27. - radius
                    && local.x < -6. + radius
                    && local.z.abs() < 23. + radius)
                || [25., 30.]
                    .iter()
                    .any(|sx| Vec2::new(local.x - sx, local.z - 20.).length() < radius + 2.5)
            {
                continue;
            }
            let mut c = p + q * local;
            if super::bridge::reserved(track, c, radius)
                || (super::mounted::reserved(track, c, radius)
                    || super::wildlife::reserved(track, c, radius))
                || (0..2).any(|route| {
                    let (station, _, d) = track.project_route(route, c, 0.);
                    d < track.half_width(station) + radius + 5.
                })
                || out.iter().any(|other| {
                    c.xz().distance(other.position.xz())
                        < radius + other.size.x.hypot(other.size.z) * 0.5 + 0.4
                })
            {
                continue;
            }
            let height = |point: Vec3| super::geometry::ground_height(track, point.x, point.z);
            let rotation = if name == "BRNGRN2" || name == "WINDVN2" {
                // Buildings and windmill towers remain upright.
                q
            } else {
                // Equipment and stacked hay rest on the local slope, as their
                // wheels/base would, rather than floating at its highest corner.
                let right = q * Vec3::X;
                let forward = q * Vec3::Z;
                let dx = size.x.max(1.) * 0.5;
                let dz = size.z.max(1.) * 0.5;
                let right =
                    right.with_y((height(c + right * dx) - height(c - right * dx)) / (2. * dx));
                let forward = forward
                    .with_y((height(c + forward * dz) - height(c - forward * dz)) / (2. * dz));
                let normal = forward.cross(right).normalize();
                if normal.y < 0.8 {
                    continue;
                }
                let forward = forward.normalize();
                let right = normal.cross(forward).normalize();
                Quat::from_mat3(&Mat3::from_cols(right, normal, right.cross(normal)))
            };
            let mut offsets = Vec::new();
            for a in [-1., 1.] {
                for b in [-1., 1.] {
                    let offset = rotation * Vec3::new(a * size.x * 0.5, 0., b * size.z * 0.5);
                    let point = c + offset;
                    offsets.push(height(point) - offset.y);
                }
            }
            let low = offsets.iter().copied().fold(f32::INFINITY, f32::min);
            let high = offsets.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            if high - low > 0.7 {
                continue 'candidate;
            }
            c.y = high - 0.04;
            out.push(FarmProp {
                name,
                position: c,
                rotation,
                size,
            });
            break;
        }
    }
    out
}
fn reuse_metropolis_farm_materials(commands: &mut Commands) {
    commands.queue(|world: &mut World| {
        let Some(mut images) = world.get_resource_mut::<Assets<Image>>() else {
            return;
        };
        let texture =
            crate::world_gen::metropolis::geometry::landscape::ground_texture(&mut images);
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let grain = materials.add(
            crate::world_gen::metropolis::geometry::landscape::ground_material(
                Color::srgb(0.57, 0.52, 0.28),
                &texture,
            ),
        );
        let soil = materials.add(
            crate::world_gen::metropolis::geometry::landscape::ground_material(
                Color::srgb(0.40, 0.36, 0.25),
                &texture,
            ),
        );
        for mut material in world
            .query_filtered::<&mut MeshMaterial3d<StandardMaterial>, With<FarmField>>()
            .iter_mut(world)
        {
            material.0 = grain.clone();
        }
        for mut material in world
            .query_filtered::<&mut MeshMaterial3d<StandardMaterial>, With<FarmYard>>()
            .iter_mut(world)
        {
            material.0 = soil.clone();
        }
    });
}
// The whole farm (including both crop plots), not just the barn, must sit on
// gently rolling land. Keep every parcel outside the road-cut transition.
const FARM_HALF: Vec2 = Vec2::new(36., 96.);
const FARM_MAX_GRADE: f32 = 0.34;
const FARM_MAX_RELIEF: f32 = 30.;
#[derive(Clone, Copy, Debug)]
struct FarmGrade {
    maximum: f32,
    relief: f32,
    barn_relief: f32,
}
fn parcel_grade(p: Vec3, q: Quat, height: impl Fn(Vec3) -> f32) -> FarmGrade {
    let mut low = f32::INFINITY;
    let mut high = f32::NEG_INFINITY;
    let mut maximum = 0f32;
    for ix in -2..=2 {
        for iz in -4..=4 {
            let c = p + q * Vec3::new(ix as f32 * 18., 0., iz as f32 * 24.);
            let h = height(c);
            low = low.min(h);
            high = high.max(h);
            let dx = (height(c + Vec3::X * 4.) - height(c - Vec3::X * 4.)) / 8.;
            let dz = (height(c + Vec3::Z * 4.) - height(c - Vec3::Z * 4.)) / 8.;
            maximum = maximum.max(dx.hypot(dz));
        }
    }
    let mut barn_low = f32::INFINITY;
    let mut barn_high = f32::NEG_INFINITY;
    for x in [10.5, 21.5] {
        for z in [-8., 8.] {
            let h = height(p + q * Vec3::new(x, 0., z));
            barn_low = barn_low.min(h);
            barn_high = barn_high.max(h);
        }
    }
    FarmGrade {
        maximum,
        relief: high - low,
        barn_relief: barn_high - barn_low,
    }
}
fn parcel_road_clear(track: &Track, p: Vec3, q: Quat) -> bool {
    // A 12m grid plus its half-cell diagonal margin covers the full footprint,
    // including its interior when another hairpin approaches the parcel.
    for ix in -3..=3 {
        for iz in -8..=8 {
            let v = p + q * Vec3::new(ix as f32 * 12., 0., iz as f32 * 12.);
            if (0..2).any(|route| track.project_route(route, v, 0.).2 < 69.) {
                return false;
            }
        }
    }
    true
}
// Each farm is a supported hill parcel; the opening city has no farm lots.
pub(super) fn farm_sites(track: &Track) -> Vec<(Vec3, Quat)> {
    let mut sites: Vec<(Vec3, Quat)> = Vec::new();
    for (start, end) in [
        (1650, 2150),
        (2250, 2750),
        (2850, 3300),
        (3450, 3950),
        (4020, 4400),
        (5020, 5330),
    ] {
        let mut candidates = Vec::new();
        for station in (start..end).step_by(30) {
            for lane in [
                -260., -230., -200., -170., -140., 140., 170., 200., 230., 260.,
            ] {
                let (mut p, road) = track.sample_route(0, station as f32, lane);
                p.y = track.hills_remote_height(p.x, p.z);
                let forward = (road * Vec3::Z).with_y(0.).normalize();
                let heading = forward.x.atan2(forward.z);
                for turn in [
                    0.,
                    std::f32::consts::FRAC_PI_4,
                    std::f32::consts::FRAC_PI_2,
                    3. * std::f32::consts::FRAC_PI_4,
                ] {
                    let q = Quat::from_rotation_y(heading + turn);
                    // Cheap continuous landform shortlist avoids thousands of full
                    // road projections/resident-mesh queries during world startup.
                    let grade = parcel_grade(p, q, |v| track.hills_remote_height(v.x, v.z));
                    if grade.maximum > FARM_MAX_GRADE
                        || grade.relief > FARM_MAX_RELIEF
                        || grade.barn_relief > 2.8
                    {
                        continue;
                    }
                    let score = grade.maximum * 100.
                        + grade.relief
                        + grade.barn_relief * 3.
                        + lane.abs() * 0.015;
                    candidates.push((score, p, q));
                }
            }
        }
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut chosen = None;
        for (_, mut p, q) in candidates {
            let (station, _, _) = track.project_route(0, p, 0.);
            if !(1500. ..5500.).contains(&station)
                || super::bridge::reserved(track, p, FARM_HALF.length() + 30.)
                || super::mounted::reserved(track, p, FARM_HALF.length())
                || sites
                    .iter()
                    .any(|(other, _)| p.xz().distance(other.xz()) < 215.)
                || !parcel_road_clear(track, p, q)
            {
                continue;
            }
            let grade = parcel_grade(p, q, |v| super::geometry::ground_height(track, v.x, v.z));
            if grade.maximum > FARM_MAX_GRADE
                || grade.relief > FARM_MAX_RELIEF
                || grade.barn_relief > 2.8
            {
                continue;
            }
            p.y = super::geometry::ground_height(track, p.x, p.z);
            chosen = Some((p, q));
            break;
        }
        sites.push(chosen.expect("each hill region needs a gentle whole farm parcel"));
    }
    sites
}
fn noise(n: u32) -> f32 {
    let n = n.wrapping_mul(747796405).wrapping_add(2891336453);
    let n = ((n >> ((n >> 28) + 4)) ^ n).wrapping_mul(277803737);
    ((n >> 22) ^ n) as f32 / u32::MAX as f32
}
pub(super) fn field_clear(p: Vec3, farms: &[(Vec3, Quat)]) -> bool {
    farms.iter().all(|(center, q)| {
        let local = q.inverse() * (p - *center);
        local.x.abs() > 35. || local.z.abs() > 95.
    })
}
fn grass_positions(
    track: &Track,
    start: f32,
    end: f32,
    farms: &[(Vec3, Quat)],
) -> Vec<(Vec3, f32)> {
    let mut out = vec![];
    for i in 0..((end - start) / 1.6) as u32 {
        let s = start + i as f32 * 1.6;
        for side in [-1., 1.] {
            for row in 0..24_u32 {
                let seed = (s as u32)
                    .wrapping_mul(97)
                    .wrapping_add(row * 31)
                    .wrapping_add(u32::from(side > 0.) * 17);
                if noise(seed) < 0.18 + 0.16 * (s * 0.023 + row as f32 * 0.7).sin() {
                    continue;
                }
                let lane = side
                    * (if s >= 8000. { 100. } else { 18. }
                        + row as f32 * 4.0
                        + noise(seed + 1) * 3.);
                let Some((p, _)) =
                    super::props::site(track, 0, s + noise(seed + 2) * 1.5, lane, 1.1)
                else {
                    continue;
                };
                if super::bridge::reserved(track, p, 1.1)
                    || (super::mounted::reserved(track, p, 1.1)
                        || super::wildlife::reserved(track, p, 1.1))
                    || !field_clear(p, farms)
                {
                    continue;
                }
                out.push((p, noise(seed + 3)));
            }
        }
    }
    out
}
fn farm_details(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    track: &Track,
    p: Vec3,
    q: Quat,
    index: usize,
) {
    let ground = |local: Vec3| {
        let world = p + q * local;
        Vec3::new(
            local.x,
            super::geometry::ground_height(track, world.x, world.z) - p.y,
            local.z,
        )
    };
    let mut m = super::props::Model::default();
    let mut solids = vec![];
    for x in [25., 30.] {
        let c = ground(Vec3::new(x, 0., 20.));
        m.rod(c - Vec3::Y * 0.5, c + Vec3::Y * 9., 2.1, [0.50, 0.54, 0.52]);
        m.ellipsoid(
            c + Vec3::Y * 9.,
            Vec3::new(2.15, 1.5, 2.15),
            [0.32, 0.37, 0.36],
            0.,
        );
        for h in 1..9 {
            m.rod(
                c + Vec3::Y * h as f32,
                c + Vec3::Y * (h as f32 + 0.06),
                2.13,
                [0.31, 0.35, 0.34],
            );
        }
        solids.push((
            c + Vec3::Y * 4.7,
            Quat::IDENTITY,
            Collider::cylinder(4.7, 2.15),
        ));
    }
    commands.spawn((
        Name::new("Hills farm corrugated silos"),
        Mesh3d(meshes.add(m.mesh())),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(p).with_rotation(q),
        RigidBody::Fixed,
        Collider::compound(solids),
        Obstacle,
        HillsSolid,
    ));
    // A worked soil yard connects the barn, silos, wagon and equipment. It
    // follows the same resident triangles, without adding a collision shelf.
    let mut yard_v = vec![];
    let mut yard_ix = vec![];
    for x in 0..17 {
        for z in 0..34 {
            let corners = [(0., 0.), (0., 2.), (2., 2.), (2., 0.)].map(|(dx, dz)| {
                p + q * ground(Vec3::new(
                    -1. + x as f32 * 2. + dx,
                    0.,
                    -34. + z as f32 * 2. + dz,
                ))
            });
            if corners.iter().any(|point| {
                (0..2).any(|route| {
                    let (station, _, distance) = track.project_route(route, *point, 0.);
                    distance < track.half_width(station) + 5.
                }) || super::bridge::reserved(track, *point, 1.)
            }) {
                continue;
            }
            let k = yard_v.len() as u32;
            yard_v.extend(corners.map(|point| point - p + Vec3::Y * 0.04));
            yard_ix.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
        }
    }
    let mut yard = crate::environment::mesh(&yard_v, &yard_ix);
    yard.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        yard_v
            .iter()
            .map(|point| [(point.x + p.x) * 0.25, (point.z + p.z) * 0.25])
            .collect::<Vec<_>>(),
    );
    commands.spawn((
        FarmYard,
        Name::new("Hills Metropolis soil farmyard"),
        Mesh3d(meshes.add(yard)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(p),
    ));
    // Terrain-following planted strips span the slopes beside each farmstead.
    let mut v = vec![];
    let mut ix = vec![];
    let mut colors = vec![];
    for side in [-1., 1.] {
        for row in 0..24 {
            for segment in 0..22 {
                let x = -29. + row as f32 * 2.4;
                let z = side * 65. - 26. + segment as f32 * 2.4;
                let locals = [
                    Vec3::new(x, 0., z),
                    Vec3::new(x, 0., z + 2.4),
                    Vec3::new(x + 2.4, 0., z + 2.4),
                    Vec3::new(x + 2.4, 0., z),
                ];
                let points = locals.map(|a| p + q * ground(a));
                if points.iter().any(|a| {
                    super::bridge::reserved(track, *a, 1.)
                        || (0..2).any(|route| {
                            let (s, _, d) = track.project_route(route, *a, 0.);
                            d < track.half_width(s) + 5.
                        })
                }) {
                    continue;
                }
                let k = v.len() as u32;
                v.extend(points.map(|a| a - p + Vec3::Y * 0.035));
                ix.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
                let shade = 0.83 + noise((row * 29 + segment * 7 + index * 97) as u32) * 0.17;
                let color = if row % 3 == 0 {
                    [shade * 0.71, shade * 0.68, shade * 0.58, 1.]
                } else {
                    [shade, shade, shade, 1.]
                };
                colors.extend([color; 4]);
                if row % 3 != 0 {
                    let c = (points[0] + points[2]) * 0.5 - p;
                    for offset in [-0.65, 0., 0.65] {
                        let c = c + q * Vec3::new(offset, 0., offset * 0.5);
                        let k = v.len() as u32;
                        v.extend([
                            c - q * Vec3::X * 0.16,
                            c + q * Vec3::X * 0.16,
                            c + Vec3::Y * (0.65 + shade * 0.2),
                        ]);
                        ix.push([k, k + 1, k + 2]);
                        colors.extend([color; 3]);
                    }
                }
            }
        }
    }
    let mut mesh = crate::environment::mesh(&v, &ix);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        v.iter()
            .map(|point| [(point.x + p.x) * 0.25, (point.z + p.z) * 0.25])
            .collect::<Vec<_>>(),
    );
    commands.spawn((
        FarmField,
        Name::new("Hills contoured Metropolis grain fields"),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(p),
    ));
}
#[allow(clippy::too_many_arguments)]
pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
    census: &mut Census,
) {
    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.95,
        ..default()
    });
    let mut farms = vec![];
    for (index, (p, q)) in farm_sites(track).into_iter().enumerate() {
        assert!(
            !(super::mounted::reserved(track, p, 35.) || super::wildlife::reserved(track, p, 35.)),
            "farm must clear mounted patrol"
        );
        // Only the barn needs a small foundation; fields and stock follow the hillside.
        let mut barn_pos = p + q * Vec3::new(16., 0., 0.);
        let mut high = super::geometry::ground_height(track, barn_pos.x, barn_pos.z);
        let mut low = high;
        for x in [-5.5, 5.5] {
            for z in [-8., 8.] {
                let v = barn_pos + q * Vec3::new(x, 0., z);
                let y = super::geometry::ground_height(track, v.x, v.z);
                high = high.max(y);
                low = low.min(y);
            }
        }
        barn_pos.y = high + 0.08;
        let depth = high - low + 0.5;
        let pad = crate::world::part(
            commands,
            art.cube.clone(),
            materials.add(Color::srgb(0.39, 0.36, 0.28)),
            barn_pos - Vec3::Y * depth / 2.,
            Vec3::new(11., depth, 16.),
            q,
        );
        commands.entity(pad).insert((
            RigidBody::Fixed,
            Collider::cuboid(0.5, 0.5, 0.5),
            Ground,
            HillsSolid,
            HillsFarm,
        ));
        let barn = commands
            .spawn((
                Transform::from_translation(barn_pos).with_rotation(q),
                Visibility::default(),
                RigidBody::Fixed,
                Collider::compound(vec![(
                    Vec3::Y * 3.79,
                    Quat::IDENTITY,
                    Collider::cuboid(5.08, 3.79, 7.4),
                )]),
                Obstacle,
                HillsSolid,
            ))
            .id();
        if let Some(props) = &art.props {
            let visual = props.fitted(
                commands,
                props.named("BRNGRN2"),
                Vec3::ZERO,
                Quat::IDENTITY,
                Vec3::new(10.16, 7.58, 14.8),
                "hills-farm",
            );
            commands.entity(barn).add_child(visual);
        } else {
            let mut model = super::props::Model::default();
            model.block(Vec3::Y * 3.5, Vec3::new(10., 7., 14.8), [0.42, 0.25, 0.16]);
            model.block(Vec3::Y * 7.2, Vec3::new(11., 0.4, 16.), [0.18, 0.20, 0.15]);
            let e = commands
                .spawn((
                    Mesh3d(meshes.add(model.mesh())),
                    MeshMaterial3d(white.clone()),
                    Transform::default(),
                ))
                .id();
            commands.entity(barn).add_child(e);
        }
        // Explicit pen support and full rail contacts; leave the entrance open.
        for z in [-22., 22.] {
            for x in [-24., -16., -8.] {
                let mut foot = p + q * Vec3::new(x, 0., z);
                foot.y = super::geometry::ground_height(track, foot.x, foot.z);
                let mut end = foot + q * Vec3::X * 8.;
                end.y = super::geometry::ground_height(track, end.x, end.z);
                let end = q.inverse() * (end - foot);
                let mut m = super::props::Model::default();
                let mut rails = vec![];
                for base in [Vec3::ZERO, end] {
                    m.rod(
                        base - Vec3::Y * 0.15,
                        base + Vec3::Y * 1.3,
                        0.09,
                        [0.43, 0.30, 0.16],
                    );
                    rails.push((
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        Collider::capsule(base, base + Vec3::Y * 1.3, 0.09),
                    ));
                }
                for y in [0.55, 1.05] {
                    m.rod(Vec3::Y * y, end + Vec3::Y * y, 0.07, [0.50, 0.36, 0.19]);
                    rails.push((
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        Collider::capsule(Vec3::Y * y, end + Vec3::Y * y, 0.07),
                    ));
                }
                commands.spawn((
                    Mesh3d(meshes.add(m.mesh())),
                    MeshMaterial3d(white.clone()),
                    Transform::from_translation(foot).with_rotation(q),
                    RigidBody::Fixed,
                    Collider::compound(rails),
                    Obstacle,
                    HillsSolid,
                ));
            }
        }
        for z in [-12., 0., 12.] {
            let mut cow = p + q * Vec3::new(-14., 0., z);
            cow.y = super::geometry::ground_height(track, cow.x, cow.z);
            crate::environment::spawn_cow(commands, art, cow, q);
            census.cows += 1;
            census.farm_cows += 1;
        }
        if let Some(props) = &art.props {
            for prop in farm_prop_plan(track, p, q) {
                let root = commands
                    .spawn((
                        Name::new(format!("Hills reused Metropolis farm {}", prop.name)),
                        Transform::from_translation(prop.position).with_rotation(prop.rotation),
                        Visibility::default(),
                        HillsSolid,
                        Obstacle,
                        Collider::compound(vec![(
                            Vec3::Y * prop.size.y * 0.5,
                            Quat::IDENTITY,
                            Collider::cuboid(
                                prop.size.x * 0.5,
                                prop.size.y * 0.5,
                                prop.size.z * 0.5,
                            ),
                        )]),
                    ))
                    .id();
                let visual = props.fitted(
                    commands,
                    props.named(prop.name),
                    Vec3::ZERO,
                    Quat::IDENTITY,
                    prop.size,
                    "farm",
                );
                crate::runtime::attach_visuals(commands, root, &[visual]);
            }
        }
        farm_details(commands, meshes, &white, track, p, q, index);
        farms.push((p, q));
        census.farms += 1;
    }
    reuse_metropolis_farm_materials(commands);
    // Forest groups follow mountain switchbacks, with isolated outskirts trees.
    for (start, end, spacing) in [(1500, 5350, 18), (8000, 8450, 65)] {
        for s in (start..end).step_by(spacing) {
            for side in [-1., 1.] {
                for row in 0..3_u32 {
                    let seed = s as u32 + row * 773 + u32::from(side > 0.) * 19;
                    let lane = side
                        * (if s >= 8000 { 125. } else { 30. }
                            + row as f32 * 24.
                            + noise(seed) * 15.);
                    let Some((p, q)) = super::props::site(track, 0, s as f32, lane, 5.) else {
                        continue;
                    };
                    if super::bridge::reserved(track, p, 5.)
                        || (super::mounted::reserved(track, p, 5.)
                            || super::wildlife::reserved(track, p, 5.))
                        || !field_clear(p, &farms)
                    {
                        continue;
                    }
                    let height = 6. + noise(seed + 7) * 4.;
                    let root = commands
                        .spawn((
                            Transform::from_translation(p).with_rotation(
                                q * Quat::from_rotation_y(noise(seed + 13) * std::f32::consts::TAU),
                            ),
                            Visibility::default(),
                            RigidBody::Fixed,
                            Collider::compound(vec![(
                                Vec3::Y * height / 2.,
                                Quat::IDENTITY,
                                Collider::cylinder(height / 2., 0.28),
                            )]),
                            Obstacle,
                            HillsSolid,
                            HillsTree,
                        ))
                        .id();
                    if let Some(props) = &art.props {
                        let conifers = ["TREE4A2", "TREE4B2", "TREE5A2", "TREEF2", "TREEU12"];
                        let broadleaves = ["TREEH2", "TREOAKA2", "TREOAKB2", "TREEK2", "TREEL2"];
                        let variant = (seed as usize / 7) % conifers.len();
                        let name = if row == 0 && s < 5200 {
                            conifers[variant]
                        } else {
                            broadleaves[variant]
                        };
                        let visual = props.fitted(
                            commands,
                            props.named(name),
                            Vec3::ZERO,
                            Quat::IDENTITY,
                            Vec3::new(
                                if row == 0 { 4. } else { 6. },
                                height,
                                if row == 0 { 4. } else { 6. },
                            ),
                            "hills-tree",
                        );
                        commands.entity(root).add_child(visual);
                    } else {
                        let trunk = crate::world::part(
                            commands,
                            art.cube.clone(),
                            materials.add(Color::srgb(0.28, 0.19, 0.10)),
                            Vec3::Y * height / 2.,
                            Vec3::new(0.5, height, 0.5),
                            Quat::IDENTITY,
                        );
                        let crown = crate::world::part(
                            commands,
                            art.sphere.clone(),
                            materials.add(Color::srgb(0.20, 0.32, 0.12)),
                            Vec3::Y * height * 0.7,
                            Vec3::new(2.5, height * 0.4, 2.5),
                            Quat::IDENTITY,
                        );
                        commands.entity(root).add_children(&[trunk, crown]);
                    }
                    census.trees += 1;
                }
            }
        }
    }
    let grass_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        cull_mode: None,
        perceptual_roughness: 1.,
        ..default()
    });
    for (start, end) in [(1500_f32, 5350_f32), (8000_f32, 8450_f32)] {
        let mut a = start;
        while a < end {
            let b = (a + 64.).min(end);
            let placements = grass_positions(track, a, b, &farms);
            if placements.is_empty() {
                a = b;
                continue;
            }
            let origin = track.sample(a, 0.).0;
            let mut vertices = vec![];
            let mut indices = vec![];
            let mut colors = vec![];
            for (p, seed) in placements {
                for blade in 0..3 {
                    let angle = seed * std::f32::consts::TAU + blade as f32 * 2.1;
                    let side = Vec3::new(angle.cos(), 0., angle.sin()) * 0.10;
                    let bend = Vec3::new(angle.sin(), 0., angle.cos()) * 0.16;
                    let h = 0.32 + seed * 0.34;
                    let k = vertices.len() as u32;
                    vertices.extend([
                        p - side - origin,
                        p + side - origin,
                        p + Vec3::Y * h + bend - origin,
                    ]);
                    indices.push([k, k + 1, k + 2]);
                    let c = [
                        0.24 + seed * 0.13,
                        0.34 + seed * 0.12,
                        0.10 + seed * 0.08,
                        1.,
                    ];
                    colors.extend([c; 3]);
                }
                census.grass_tufts += 1;
            }
            let mut mesh = crate::environment::mesh(&vertices, &indices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(grass_material.clone()),
                Transform::from_translation(origin),
                HillsGrass,
                NotShadowCaster,
                VisibilityRange {
                    start_margin: 0.0..0.0,
                    end_margin: 90.0..125.0,
                    use_aabb: false,
                },
            ));
            census.grass_batches += 1;
            a = b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reused_metropolis_farm_props_are_bounded_supported_and_road_clear() {
        let track = Track::hills().unwrap();
        let catalog = rr_content::props::Catalog::from_json(crate::props::JSON).unwrap();
        let mut count = 0;
        for (p, q) in farm_sites(&track) {
            let props = farm_prop_plan(&track, p, q);
            assert!(
                props.len() >= 6,
                "farm needs a developed yard: {}",
                props.len()
            );
            count += props.len();
            for (index, prop) in props.iter().enumerate() {
                assert!(catalog.records.iter().any(|asset| asset.name == prop.name));
                let radius = prop.size.x.hypot(prop.size.z) * 0.5;
                assert!(!super::super::bridge::reserved(
                    &track,
                    prop.position,
                    radius
                ));
                for route in 0..2 {
                    let (station, _, d) = track.project_route(route, prop.position, 0.);
                    assert!(d > track.half_width(station) + radius + 5.);
                }
                for a in [-1., 1.] {
                    for b in [-1., 1.] {
                        let corner = prop.position
                            + prop.rotation
                                * Vec3::new(a * prop.size.x * 0.5, 0., b * prop.size.z * 0.5);
                        let support =
                            super::super::geometry::ground_height(&track, corner.x, corner.z);
                        assert!((corner.y - support).abs() < 0.71);
                    }
                }
                for other in &props[..index] {
                    assert!(
                        prop.position.xz().distance(other.position.xz())
                            > radius + other.size.x.hypot(other.size.z) * 0.5 + 0.39
                    );
                }
            }
        }
        assert!((36..=84).contains(&count));
        eprintln!("Placed {count} supported existing catalog farm props across six farms");
    }
    #[test]
    fn farm_parcels_are_hills_only_and_whole_fields_have_gentle_grades() {
        let track = Track::hills().unwrap();
        let sites = farm_sites(&track);
        assert_eq!(sites.len(), 6);
        for (p, q) in sites {
            let (station, _, distance) = track.project_route(0, p, 0.);
            assert!((1500. ..5500.).contains(&station));
            assert!(!super::super::bridge::reserved(
                &track,
                p,
                FARM_HALF.length()
            ));
            assert!(distance > track.half_width(station) + 44.);
            assert!(parcel_road_clear(&track, p, q));
            let height = |v: Vec3| super::super::geometry::ground_height(&track, v.x, v.z);
            let grade = parcel_grade(p, q, height);
            assert!(grade.maximum <= FARM_MAX_GRADE);
            assert!(grade.relief <= FARM_MAX_RELIEF);
            assert!(grade.barn_relief <= 2.8);
            // Denser independent samples catch a cliff inside a field, rather
            // than merely accepting a level barn on a steep surrounding shelf.
            let mut maximum = 0f32;
            for x in (-32..=32).step_by(8) {
                for z in (-88..=88).step_by(8) {
                    let c = p + q * Vec3::new(x as f32, 0., z as f32);
                    let dx = (height(c + Vec3::X * 2.) - height(c - Vec3::X * 2.)) / 4.;
                    let dz = (height(c + Vec3::Z * 2.) - height(c - Vec3::Z * 2.)) / 4.;
                    maximum = maximum.max(dx.hypot(dz));
                }
            }
            assert!(maximum <= 0.38, "farm interior too steep: {maximum}");
            eprintln!(
                "gentle hill farm station={station:.1} lane_distance={distance:.1} grade={grade:?} dense_max={maximum:.3}"
            );
        }
    }
}
