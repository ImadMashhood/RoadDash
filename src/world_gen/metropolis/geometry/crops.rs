//! Static crop meshes plus small, dirty-only persistent bend tiles. No colliders.
use super::{Batch, landscape::Footprint};
use crate::{
    game::{Race, Rider},
    recovery::{OwnedBody, collider_bounds},
    runtime::{SimSet, StartupSet},
    track::Track,
};
use bevy::{
    camera::visibility::VisibilityRange,
    light::NotShadowCaster,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};
use bevy_rapier3d::prelude::*;
use rr_core::crops::{CELLS, Field, Key, SIDE};
use std::collections::{BTreeMap, HashMap};
#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct Bend {
    #[uniform(100)]
    pub origin: Vec4,
    #[storage(101, read_only)]
    pub cells: Handle<ShaderStorageBuffer>,
    #[storage(102, read_only)]
    pub clock: Handle<ShaderStorageBuffer>,
}
impl MaterialExtension for Bend {
    fn vertex_shader() -> ShaderRef {
        "shaders/crops.wgsl".into()
    }
}
#[derive(Component)]
struct CropBatch {
    triangles: usize,
}
type CropMaterial = ExtendedMaterial<StandardMaterial, Bend>;
#[derive(Resource, Default)]
struct Crops {
    field: Field,
    buffers: BTreeMap<Key, Handle<ShaderStorageBuffer>>,
    clock: Handle<ShaderStorageBuffer>,
    time: f32,
    previous: HashMap<Entity, Transform>,
    uploaded_tile_bytes: u64,
}
mod gpu;
mod survey;
pub struct CropPlugin;
impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        gpu::install(app);
        app.add_plugins(MaterialPlugin::<CropMaterial>::default())
            .init_resource::<Crops>()
            .add_systems(Startup, spawn.after(StartupSet::World))
            .add_systems(FixedUpdate, stamp.in_set(SimSet::AfterPhysics))
            .add_systems(Update, upload);
        if std::env::var_os("ROADDASH_CROP_SURVEY").is_some() {
            survey::install(app);
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn spawn(
    mut commands: Commands,
    track: Res<Track>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CropMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut crops: ResMut<Crops>,
    census: Option<ResMut<super::super::Census>>,
) {
    let Some(m) = &track.metropolis else {
        return;
    };
    let fields: Vec<_> = m
        .fields
        .iter()
        .filter(|f| f.crop == "grain")
        .map(|f| Footprint::new(&f.shape))
        .collect();
    let solids: Vec<_> = m
        .surfaces
        .iter()
        .map(|s| &s.shape)
        .chain(m.buildings.iter().map(|b| &b.shape))
        .filter(|s| {
            s.vertices()
                .any(|[x, z]| track.nearby_station(x, z).0 < 3050.)
        })
        .map(Footprint::new)
        .collect();
    let props = super::super::farms::plan(&track);
    let mut batches: BTreeMap<Key, Batch> = BTreeMap::new();
    let mut stalks = 0;
    for row in -224..2000 {
        let s = row as f32 * 1.5;
        for col in -48i32..=48 {
            let lane = col as f32 * 1.5;
            if lane.abs() < track.half_width(s) + 4. {
                continue;
            }
            let p = track.sample(s, lane).0;
            if !fields
                .iter()
                .any(|f| f.contains(p.xz()) && f.boundary_distance(p.xz()) > 1.)
                || solids.iter().any(|f| f.overlaps(p.xz(), 0.6))
                || props
                    .iter()
                    .any(|f| f.position.xz().distance(p.xz()) < f.radius + 0.8)
            {
                continue;
            }
            let p = p.with_y(track.terrain_height(p.x, p.z) - 0.10);
            crops.field.plant(p.xz().to_array());
            let (key, index) = rr_core::crops::address(p.xz().to_array());
            let batch = batches.entry(key).or_default();
            // Three distinct wheat stems per socket; opaque stem, leaf and seed head geometry.
            for stem in 0..3 {
                let a = stem as f32 * 2.094 + (row as f32 * 0.71).sin();
                let axis = Vec3::new(a.cos(), 0., a.sin());
                let root = p + axis * 0.18;
                let h = 1.05 + 0.15 * (row as f32 + col as f32).sin();
                let parts = [(0., 0.8, 0.014), (0.78, 1., 0.065), (0.35, 0.65, 0.12)];
                // Three differently oriented stems already give the clump coverage.
                // Do not duplicate every stem/head plane; only leaves turn across it.
                for (lo, hi, width) in parts {
                    let cross = if lo == 0.35 {
                        Vec3::new(-axis.z, 0., axis.x)
                    } else {
                        axis
                    };
                    let points = [
                        root + Vec3::Y * h * lo - cross * width,
                        root + Vec3::Y * h * lo + cross * width,
                        root + Vec3::Y * h * hi + cross * width * 0.3,
                        root + Vec3::Y * h * hi - cross * width * 0.3,
                    ];
                    // uv.x is the stable tile-cell address, uv.y is height above this root.
                    batch.quad(
                        points,
                        [
                            [index as f32, lo * h],
                            [index as f32, lo * h],
                            [index as f32, hi * h],
                            [index as f32, hi * h],
                        ],
                    );
                    let start = batch.colors.len() - 4;
                    let color = if lo > 0.7 {
                        [0.48, 0.31, 0.075, 1.]
                    } else {
                        [0.22, 0.28, 0.045, 1.]
                    };
                    batch.colors[start..].fill(color);
                }
            }
            stalks += 3;
        }
    }
    crops.clock = buffers.add(gpu::buffer(vec![Vec4::ZERO]));
    let triangles = batches.values().map(|b| b.ix.len()).sum::<usize>();
    assert!(triangles <= 1_000_000, "crop geometry budget exceeded");
    if let Some(mut census) = census {
        census.crop_stems = stalks;
        census.crop_batches = batches.len();
        census.crop_triangles = triangles;
        census.render_triangles += triangles;
    }
    for (key, batch) in batches {
        let origin = Vec3::new(key.0 as f32 * SIDE as f32, 0., key.1 as f32 * SIDE as f32);
        let cells = buffers.add(gpu::buffer(vec![Vec4::ZERO; CELLS]));
        crops.buffers.insert(key, cells.clone());
        let material = materials.add(CropMaterial {
            base: StandardMaterial {
                cull_mode: None,
                perceptual_roughness: 1.,
                ..default()
            },
            extension: Bend {
                origin: origin.extend(0.),
                cells,
                clock: crops.clock.clone(),
            },
        });
        let (lo, hi) = batch.p.iter().map(|p| Vec3::from_array(*p)).fold(
            (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
            |(lo, hi), p| (lo.min(p), hi.max(p)),
        );
        let mut mesh = batch.mesh(origin);
        mesh.asset_usage = bevy::asset::RenderAssetUsages::RENDER_WORLD;
        commands.spawn((
            Name::new("Wheat / persistent bend tile"),
            if std::env::var("ROADDASH_CROP_RENDER").as_deref() == Ok("off") {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            },
            CropBatch {
                triangles: batch.ix.len(),
            },
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            Transform::from_translation(origin),
            NotShadowCaster,
            bevy::camera::primitives::Aabb {
                center: ((lo + hi) * 0.5 - origin).into(),
                half_extents: ((hi - lo) * 0.5 + Vec3::splat(1.3)).into(),
            },
            VisibilityRange {
                start_margin: 0. ..0.,
                end_margin: 70. ..105.,
                use_aabb: true,
            },
        ));
    }
    info!(
        "Crops: {stalks} wheat stems; {triangles} static triangles; {} tiles / {} deformation bytes",
        crops.buffers.len(),
        crops.buffers.len() * CELLS * 16
    );
}
#[allow(clippy::type_complexity)]
fn stamp(
    race: Res<Race>,
    track: Res<Track>,
    mut crops: ResMut<Crops>,
    mut reset: MessageReader<crate::destruction::Reset>,
    actors: Query<
        (
            Entity,
            &Transform,
            &Collider,
            Option<&ColliderDisabled>,
            Option<&RigidBodyDisabled>,
        ),
        Or<(With<Rider>, With<OwnedBody>)>,
    >,
) {
    if reset.read().count() > 0 {
        crops.field.reset();
        crops.previous.clear();
        crops.time = 0.;
    }
    if !race.phase.simulating() || crops.field.tiles.is_empty() {
        return;
    }
    crops.time += 1. / 60.;
    let time = crops.time;
    for (entity, t, shape, disabled, sleeping) in &actors {
        if disabled.is_some() || sleeping.is_some() {
            crops.previous.remove(&entity);
            continue;
        }
        let previous = crops.previous.insert(entity, *t).unwrap_or(*t);
        // Cheap spatial rejection before terrain/shape queries; city bodies never inspect crops.
        let p = t.translation;
        let q = if previous.translation.distance(p) <= 12. {
            previous.translation
        } else {
            p
        };
        let a = rr_core::crops::address((p.xz().min(q.xz()) - Vec2::splat(2.)).to_array()).0;
        let b = rr_core::crops::address((p.xz().max(q.xz()) + Vec2::splat(2.)).to_array()).0;
        if !(a.0..=b.0).any(|x| (a.1..=b.1).any(|z| crops.field.tiles.contains_key(&(x, z)))) {
            continue;
        }
        let (lo, hi) = collider_bounds(t, shape);
        let ground = track.terrain_height(t.translation.x, t.translation.z);
        if lo.y > ground + 1.2 || hi.y < ground - 0.25 {
            continue;
        }
        let radius = ((hi - lo).xz().length() * 0.5 + 0.5).clamp(0.7, 2.);
        let teleport = previous.translation.distance(t.translation) > 12.;
        let previous = if teleport { *t } else { previous };
        crops.field.stamp(
            previous.translation.xz().to_array(),
            t.translation.xz().to_array(),
            radius,
            time,
            |p, alpha| {
                let translation = previous.translation.lerp(t.translation, alpha);
                let rotation = previous.rotation.slerp(t.rotation, alpha);
                let ground = track.terrain_height(p[0], p[1]);
                [0.1, 0.55, 1.0].iter().any(|h| {
                    shape.distance_to_point(
                        translation,
                        rotation,
                        Vec3::new(p[0], ground + h, p[1]),
                        true,
                    ) < 0.55
                })
            },
        );
    }
    crops.previous.retain(|e, _| actors.contains(*e));
}
fn upload(
    mut crops: ResMut<Crops>,
    mut writes: ResMut<gpu::Writes>,
    mut last: Local<Option<f32>>,
    race: Res<Race>,
    fixed: Res<Time<Fixed>>,
) {
    let crops = &mut *crops;
    let shown = if crops.time == 0. {
        0.
    } else if race.phase.simulating() {
        crops.time + fixed.overstep_fraction() / 60.
    } else {
        last.unwrap_or(crops.time)
    };
    writes.epoch += 1;
    writes.data.clear();
    if *last != Some(shown) {
        writes.data.push((crops.clock.id(), gpu::bytes([shown; 4])));
        *last = Some(shown);
    }
    for (key, tile) in &mut crops.field.tiles {
        if !tile.dirty {
            continue;
        }
        if let Some(buffer) = crops.buffers.get(key) {
            writes
                .data
                .push((buffer.id(), gpu::bytes(tile.cells.into_iter().flatten())));
            tile.dirty = false;
            crops.uploaded_tile_bytes += (CELLS * 16) as u64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recovery::BodyKind;
    #[test]
    fn actual_grain_geometry_has_bounded_tiles_and_no_crop_colliders() {
        let mut app = App::new();
        app.insert_resource(Track::metropolis().unwrap())
            .init_resource::<Crops>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<CropMaterial>>()
            .init_resource::<Assets<ShaderStorageBuffer>>()
            .add_systems(Update, spawn);
        app.update();
        let crops = app.world().resource::<Crops>();
        assert!(!crops.field.tiles.is_empty());
        assert_eq!(crops.field.tiles.len(), crops.buffers.len());
        assert!(crops.field.tiles.len() <= rr_core::crops::MAX_TILES);
        let triangles = app
            .world()
            .resource::<Assets<Mesh>>()
            .iter()
            .map(|(_, m)| m.indices().unwrap().len() / 3)
            .sum::<usize>();
        println!(
            "Actual grain crop tiles={} deformation_bytes={} static_triangles={}",
            crops.buffers.len(),
            crops.buffers.len() * CELLS * 16,
            triangles
        );
        assert!(triangles < 1_000_000);
        let mut shapes = app.world_mut().query::<&Collider>();
        assert_eq!(shapes.iter(app.world()).count(), 0);
    }
    fn fixture(kind: BodyKind, shape: Collider, height: f32) -> (App, Entity, Vec3) {
        let mut app = App::new();
        let track = Track::metropolis().unwrap();
        let p = track.sample(600., 26.).0;
        let p = p.with_y(track.terrain_height(p.x, p.z));
        let mut crops = Crops::default();
        for x in -12..13 {
            for z in -4..5 {
                crops.field.plant([p.x + x as f32, p.z + z as f32]);
            }
        }
        app.insert_resource(track)
            .insert_resource(crops)
            .insert_resource(Race {
                phase: crate::game::Phase::Racing,
                ..default()
            })
            .add_message::<crate::destruction::Reset>()
            .add_systems(Update, stamp);
        let e = app
            .world_mut()
            .spawn((
                OwnedBody {
                    owner: Entity::PLACEHOLDER,
                    kind,
                },
                shape,
                Transform::from_translation(p + Vec3::Y * height),
            ))
            .id();
        (app, e, p)
    }
    fn count(app: &App) -> usize {
        app.world()
            .resource::<Crops>()
            .field
            .tiles
            .values()
            .flat_map(|t| t.cells.iter())
            .filter(|v| v[2] > 0.)
            .count()
    }
    #[test]
    fn physical_runner_bike_and_crash_shapes_leave_persistent_marks() {
        for (kind, shape, h) in [
            (BodyKind::Runner, Collider::capsule_y(0.5, 0.3), 0.85),
            (BodyKind::Wreck, Collider::cuboid(0.35, 0.3, 1.), 0.4),
            (BodyKind::Ragdoll, Collider::capsule_y(0.25, 0.18), 0.35),
        ] {
            let (mut app, e, p) = fixture(kind, shape, h);
            app.update();
            let initial = count(&app);
            assert!(initial > 0);
            app.world_mut().get_mut::<Transform>(e).unwrap().translation = p + Vec3::new(8., h, 0.);
            app.update();
            assert!(count(&app) > initial + 3, "fast sweep must leave a trail");
            app.world_mut().despawn(e);
            let before = count(&app);
            app.update();
            assert_eq!(count(&app), before);
            app.world_mut()
                .resource_mut::<Messages<crate::destruction::Reset>>()
                .write(crate::destruction::Reset);
            app.update();
            assert_eq!(count(&app), 0);
        }
    }
    #[test]
    fn airborne_disabled_unplanted_and_paused_actors_cannot_stamp() {
        let (mut app, e, p) = fixture(BodyKind::Runner, Collider::capsule_y(0.5, 0.3), 5.);
        app.update();
        assert_eq!(count(&app), 0);
        app.world_mut().get_mut::<Transform>(e).unwrap().translation = p + Vec3::Y * 0.85;
        app.world_mut().entity_mut(e).insert(ColliderDisabled);
        app.update();
        assert_eq!(count(&app), 0);
        app.world_mut().entity_mut(e).remove::<ColliderDisabled>();
        app.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Paused;
        let time = app.world().resource::<Crops>().time;
        app.update();
        assert_eq!(count(&app), 0);
        assert_eq!(app.world().resource::<Crops>().time, time);
        app.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Racing;
        app.world_mut().get_mut::<Transform>(e).unwrap().translation = p + Vec3::new(50., 0.85, 0.);
        app.update();
        assert_eq!(count(&app), 0);
        app.world_mut().get_mut::<Transform>(e).unwrap().translation = p + Vec3::Y * 0.85;
        app.update();
        assert!(count(&app) > 0);
    }
}
