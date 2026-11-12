//! Bounded, presentation-only wildlife and wheel dust. No collision or rig writes.
use crate::{
    game::{Rider, Traffic},
    track::Track,
};
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::VisibilityRange,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
pub const DUST_CAPACITY: usize = 96;
pub const BIRD_COUNT: usize = 24;
const EMISSION_INTERVAL: f32 = 0.08;
fn dust_alpha(age: f32, life: f32, behind_m: f32) -> f32 {
    let ease = |x: f32| x * x * (3. - 2. * x);
    0.17 * ease((age / 0.25).clamp(0., 1.))
        * ease(((life - age) / 0.35).clamp(0., 1.))
        * ease(((24. - behind_m) / 15.).clamp(0., 1.))
}
#[derive(Component)]
pub struct DustPuff {
    pub age: f32,
    life: f32,
    drift: Vec3,
    tint: Vec3,
}
#[derive(Component)]
pub struct Bird {
    center: Vec3,
    radius: f32,
    phase: f32,
    rate: f32,
    flocking: bool,
}
#[derive(Component)]
pub struct Wing {
    side: f32,
    phase: f32,
}
#[derive(Resource, Default)]
pub struct Atmosphere {
    pub elapsed: f32,
    pub emitted: u64,
    pub active_dust: usize,
    pub animated_birds: usize,
    emission_clock: f32,
    last_race_elapsed: f32,
    was_garage: bool,
}
fn sprite() -> Image {
    let size = 64;
    let mut data = Vec::with_capacity(size * size * 4);
    for y in 0..size {
        for x in 0..size {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) / size as f32 * 2. - Vec2::ONE;
            let edge = (1. - p.length_squared()).clamp(0., 1.);
            let mottling =
                0.78 + 0.12 * (p.x * 13. + p.y * 7.).sin() + 0.10 * (p.y * 19. - p.x * 5.).cos();
            data.extend([255, 255, 255, (edge.powi(2) * mottling * 255.) as u8]);
        }
    }
    Image::new(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
pub fn birds_enabled() -> bool {
    std::env::var("ROADDASH_BIRDS").as_deref() != Ok("0")
}

fn bird_range() -> VisibilityRange {
    VisibilityRange {
        start_margin: 0.0..0.0,
        end_margin: 450.0..650.0,
        use_aabb: false,
    }
}

fn flock_center(track: &Track, group: usize) -> Vec3 {
    let hills = track.is_hills();
    let station = if hills {
        [2200., 3900., 5900., 7900., 9800., 11700.][group]
    } else {
        // Rural approach, city entry, two street canyons and both civic spaces.
        // Keep street flocks ahead of riders so they silhouette above roofs.
        [1100., 3200., 4850., 7300., 8650., 9950.][group]
    };
    let (p, q) = track.sample_route(0, station, 0.);
    let side = if hills {
        if group.is_multiple_of(2) { 65. } else { -65. }
    } else {
        [35., -35., 35., 0., -35., 0.][group]
    };
    let mut center = p + q * Vec3::X * side;
    let mut altitude = p.y + if hills { 35. } else { 48. };
    if !hills && let Some(metro) = &track.metropolis {
        // The two planted civic openings give flocks a lower, readable skyline
        // without threading the resident street walls. These are authored sites,
        // not survey-camera targets.
        if let Some(kind) = match group {
            3 => Some("city_hall"),
            5 => Some("park"),
            _ => None,
        } && let Some(site) = metro.civic_sites.iter().find(|site| site.kind == kind)
        {
            center.x = site.position[0];
            center.z = site.position[1];
            altitude = track.terrain_height(center.x, center.z)
                + if kind == "city_hall" { 62. } else { 42. };
        }
        // Reserve the whole flock orbit above nearby resident roofs, not merely
        // the spawn point. This is authoring-time placement, never physics.
        for building in &metro.buildings {
            let mut min = Vec2::splat(f32::INFINITY);
            let mut max = Vec2::splat(f32::NEG_INFINITY);
            let mut base = f32::NEG_INFINITY;
            for v in building.shape.vertices() {
                let v = Vec2::from_array(v);
                min = min.min(v);
                max = max.max(v);
                base = base.max(track.terrain_height(v.x, v.y));
            }
            if center.xz().distance(center.xz().clamp(min, max)) <= 55. {
                altitude = altitude.max(base + building.height + 12.);
            }
        }
    }
    center.y = altitude;
    center
}

fn bird_at(center: Vec3, hills: bool, i: usize) -> Bird {
    let group = i / 4;
    let member = (i % 4) as f32;
    Bird {
        center: center + Vec3::Y * member * if hills { 4. } else { 2. },
        radius: if hills {
            25. + member * 8.
        } else {
            38. + member * 3.
        },
        phase: if hills {
            i as f32 * 2.399
        } else {
            group as f32 * 2.399 + member * 0.16
        },
        rate: if hills {
            0.13 + (i % 3) as f32 * 0.025
        } else {
            0.17 + group as f32 * 0.005
        },
        flocking: !hills,
    }
}

fn bird_pose(bird: &Bird, elapsed: f32) -> Transform {
    let a = elapsed * bird.rate + bird.phase;
    Transform::from_translation(
        bird.center
            + Vec3::new(
                a.cos() * bird.radius,
                (a * 1.7).sin() * 3.,
                a.sin() * bird.radius,
            ),
    )
    .with_rotation(Quat::from_rotation_y(-a) * Quat::from_rotation_z(-0.12))
}

fn wing_mesh() -> Mesh {
    let p = [
        Vec3::ZERO,
        Vec3::new(0.55, 0., -0.08),
        Vec3::new(0.95, 0., 0.23),
        Vec3::new(0.22, 0., 0.22),
    ];
    crate::environment::mesh(&p, &[[0, 1, 2], [0, 2, 3]])
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
    images: Option<&mut Assets<Image>>,
) {
    if !track.is_hills() && track.metropolis.is_none() {
        return;
    }
    commands.insert_resource(Atmosphere::default());
    if track.is_hills() {
        let texture = images.map(|images| images.add(sprite()));
        let quad = meshes.add(Rectangle::new(1., 1.));
        for _ in 0..DUST_CAPACITY {
            let material = materials.add(StandardMaterial {
                base_color: Color::srgba(0.76, 0.61, 0.38, 0.),
                base_color_texture: texture.clone(),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                double_sided: true,
                cull_mode: None,
                ..default()
            });
            commands.spawn((
                Mesh3d(quad.clone()),
                MeshMaterial3d(material),
                Transform::default(),
                Visibility::Hidden,
                bevy::light::NotShadowCaster,
                bevy::light::NotShadowReceiver,
                DustPuff {
                    age: 2.,
                    life: 1.6,
                    drift: Vec3::ZERO,
                    tint: Vec3::new(0.76, 0.61, 0.38),
                },
            ));
        }
    }
    if !birds_enabled() {
        return;
    }
    let body = meshes.add(Sphere::new(1.).mesh().uv(8, 4));
    let wing = meshes.add(wing_mesh());
    let feather = materials.add(StandardMaterial {
        base_color: Color::srgb(0.095, 0.075, 0.055),
        perceptual_roughness: 1.,
        double_sided: true,
        cull_mode: None,
        ..default()
    });
    let centers: [Vec3; 6] = std::array::from_fn(|group| flock_center(track, group));
    for i in 0..BIRD_COUNT {
        let bird = bird_at(centers[i / 4], track.is_hills(), i);
        let phase = i as f32 * 2.399;
        let initial = bird_pose(&bird, 0.);
        let root = commands.spawn((initial, Visibility::default(), bird)).id();
        let torso = commands
            .spawn((
                Mesh3d(body.clone()),
                bird_range(),
                MeshMaterial3d(feather.clone()),
                Transform::from_scale(Vec3::new(0.13, 0.11, 0.4)),
                bevy::light::NotShadowCaster,
            ))
            .id();
        commands.entity(root).add_child(torso);
        for side in [-1., 1.] {
            let e = commands
                .spawn((
                    Mesh3d(wing.clone()),
                    bird_range(),
                    MeshMaterial3d(feather.clone()),
                    Transform::from_scale(Vec3::new(side, 1., 1.)),
                    Wing { side, phase },
                    bevy::light::NotShadowCaster,
                ))
                .id();
            commands.entity(root).add_child(e);
        }
    }
}
/// Reset only visual pools when the existing race lifecycle starts a fresh run.
#[allow(clippy::type_complexity)]
pub fn reset(
    race: Res<crate::game::Race>,
    state: Option<ResMut<Atmosphere>>,
    mut dust: Query<(&mut DustPuff, &mut Visibility)>,
    mut birds: Query<(&Bird, &mut Transform), Without<Wing>>,
    mut wings: Query<(&Wing, &mut Transform), Without<Bird>>,
) {
    let Some(mut state) = state else { return };
    let garage = race.phase == crate::game::Phase::Garage;
    let restarted = race.elapsed + 0.001 < state.last_race_elapsed || (garage && !state.was_garage);
    state.last_race_elapsed = race.elapsed;
    state.was_garage = garage;
    if !restarted {
        return;
    }
    state.elapsed = 0.;
    state.emission_clock = 0.;
    state.emitted = 0;
    state.active_dust = 0;
    state.animated_birds = 0;
    for (mut p, mut visibility) in &mut dust {
        p.age = p.life;
        p.drift = Vec3::ZERO;
        *visibility = Visibility::Hidden;
    }
    for (bird, mut t) in &mut birds {
        *t = bird_pose(bird, 0.);
    }
    for (_, mut t) in &mut wings {
        t.rotation = Quat::IDENTITY;
    }
}
fn dust_eligible(
    speed: f32,
    ground_gap: f32,
    lane: f32,
    half: f32,
    wheels: Option<[bool; 2]>,
) -> bool {
    wheels.is_none_or(|contacts| contacts.into_iter().any(|grounded| grounded))
        && speed.is_finite()
        && speed.abs() > 5.
        && (-0.1..=1.6).contains(&ground_gap)
        && lane.abs() > half + 0.45
        && lane.abs() < half + 24.
}
fn active_delta(phase: crate::game::Phase, dt: f32) -> f32 {
    if phase.simulating() {
        dt.clamp(0., 0.05)
    } else {
        0.
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn animate(
    time: Res<Time>,
    race: Res<crate::game::Race>,
    track: Res<Track>,
    state: Option<ResMut<Atmosphere>>,
    cameras: Query<
        &Transform,
        (
            With<crate::game::ChaseCamera>,
            Without<DustPuff>,
            Without<Bird>,
            Without<Wing>,
        ),
    >,
    vehicles: Query<
        (
            &Transform,
            Option<&Rider>,
            Option<&Traffic>,
            Option<&crate::vehicle::BikeState>,
        ),
        (
            Or<(With<Rider>, With<Traffic>)>,
            Without<DustPuff>,
            Without<Bird>,
            Without<Wing>,
            Without<crate::game::ChaseCamera>,
        ),
    >,
    mut dust: Query<
        (
            &mut DustPuff,
            &mut Transform,
            &mut Visibility,
            &MeshMaterial3d<StandardMaterial>,
        ),
        (
            Without<Bird>,
            Without<Wing>,
            Without<Rider>,
            Without<Traffic>,
            Without<crate::game::ChaseCamera>,
        ),
    >,
    mut birds: Query<
        (&Bird, &mut Transform, &mut Visibility),
        (
            Without<DustPuff>,
            Without<Wing>,
            Without<Rider>,
            Without<Traffic>,
            Without<crate::game::ChaseCamera>,
        ),
    >,
    mut wings: Query<
        (&Wing, &mut Transform, &ChildOf),
        (
            Without<Bird>,
            Without<DustPuff>,
            Without<Rider>,
            Without<Traffic>,
            Without<crate::game::ChaseCamera>,
        ),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ground: Query<(), With<crate::world::Ground>>,
    physics: ReadRapierContext,
) {
    let Some(mut state) = state else { return };
    let Ok(camera) = cameras.single() else { return };
    let dt = active_delta(race.phase, time.delta_secs());
    // Bevy VisibilityRange applies soft dither culling to every mesh after
    // final GlobalTransform propagation, including paused/survey cameras.
    if dt == 0. {
        return;
    }
    state.elapsed += dt;
    state.emission_clock += dt;
    let elapsed = state.elapsed;
    state.animated_birds = 0;
    for (bird, mut t, _) in &mut birds {
        // Only 24 analytic poses, independent of camera hierarchy/teleports.
        // Final-camera range dithering limits draw work without stale poses.
        *t = bird_pose(bird, elapsed);
        state.animated_birds += 1;
    }
    for (wing, mut t, parent) in &mut wings {
        let Ok((bird, _, _)) = birds.get(parent.parent()) else {
            continue;
        };
        // Metro flocks alternate short flapping bursts with level soaring.
        // Hills retains its established continuous wing cadence.
        let beat = if bird.flocking {
            let cycle = (elapsed * 0.65 + wing.phase).sin();
            ((cycle + 0.25) * 2.).clamp(0., 1.)
        } else {
            1.
        };
        t.rotation = Quat::from_rotation_z(
            wing.side * ((elapsed * 7. + wing.phase).sin() * 0.48 * beat + 0.12),
        );
    }
    // Metropolis reuses birds only: no dust assets, contact queries or emissions.
    if dust.is_empty() {
        return;
    }
    let player_pose = vehicles
        .iter()
        .find(|(_, rider, _, _)| rider.is_some_and(|r| r.id == 0))
        .map(|(pose, _, _, _)| *pose);
    let mut emissions = Vec::new();
    if state.emission_clock >= EMISSION_INTERVAL {
        state.emission_clock %= EMISSION_INTERVAL;
        if let Ok(context) = physics.single() {
            let is_ground = |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
            let mut near: Vec<_> = vehicles
                .iter()
                .filter(|(t, r, _, _)| {
                    t.translation.distance_squared(camera.translation) < 180f32.powi(2)
                        && r.is_none_or(|r| r.crashed <= 0.)
                })
                .collect();
            near.sort_by(|a, b| {
                a.0.translation
                    .distance_squared(camera.translation)
                    .total_cmp(&b.0.translation.distance_squared(camera.translation))
            });
            for (t, r, car, bike) in near.into_iter().take(8) {
                let speed = r.map_or_else(|| car.map_or(0., |c| c.speed), |r| r.speed);
                let (route, s) = r.map_or_else(
                    || car.map_or((0, 0.), |c| (c.route, c.progress)),
                    |r| (r.route, r.progress),
                );
                let a = track.lane_at(route, s, t.translation);
                let b = track.lane_at(1 - route.min(1), s, t.translation);
                let l = if a.abs() < b.abs() { a } else { b };
                // Both paved branches suppress emission; only supported verge travel raises dust.
                if l.abs() <= track.half_width(s) + 0.45 || speed.abs() <= 5. {
                    continue;
                }
                let Some((_, toi)) = context.cast_ray(
                    t.translation + Vec3::Y,
                    Vec3::NEG_Y,
                    3.,
                    true,
                    QueryFilter::default().predicate(&is_ground),
                ) else {
                    continue;
                };
                if dust_eligible(
                    speed,
                    toi - 1.,
                    l,
                    track.half_width(s),
                    bike.map(|b| b.grounded),
                ) {
                    let p = t.translation + Vec3::Y * (1. - toi + 0.3) - t.rotation * Vec3::Z * 0.8;
                    emissions.push((
                        p,
                        -t.rotation * Vec3::Z * speed.signum() * 0.8 + Vec3::new(0.7, 0.7, 0.2),
                        super::geometry::sand_color(s, p.x, p.z),
                    ));
                }
            }
        }
    }
    let mut next = emissions.into_iter();
    let mut active = 0;
    for (mut puff, mut t, mut visibility, material) in &mut dust {
        puff.age += dt;
        if puff.age >= puff.life
            && let Some((p, drift, tint)) = next.next()
        {
            puff.age = 0.;
            puff.drift = drift;
            puff.tint = tint;
            t.translation = p;
            state.emitted += 1;
        }
        if puff.age < puff.life {
            active += 1;
            t.translation += puff.drift * dt * (1. - 0.4 * puff.age / puff.life);
            t.rotation = camera.rotation;
            t.scale = Vec3::splat(1.8);
            *visibility = Visibility::Inherited;
            if let Some(mut m) = materials.get_mut(&material.0) {
                let behind_m = player_pose.map_or(0., |player| {
                    let forward = (player.rotation * Vec3::Z).with_y(0.).normalize_or_zero();
                    (player.translation - t.translation).dot(forward)
                });
                m.base_color = Color::srgba(
                    puff.tint.x,
                    puff.tint.y,
                    puff.tint.z,
                    dust_alpha(puff.age, puff.life, behind_m),
                );
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    state.active_dust = active;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn translucent_sand_dust_eases_in_and_out_without_a_first_frame_pop() {
        assert_eq!(dust_alpha(0., 1.6, 0.), 0.);
        assert!(dust_alpha(0.05, 1.6, 0.) < 0.02);
        assert!((0.16..=0.17).contains(&dust_alpha(0.6, 1.6, 0.)));
        assert!(dust_alpha(1.5, 1.6, 0.) < 0.04);
        assert!(dust_alpha(0.6, 1.6, 20.) < 0.04);
        assert_eq!(dust_alpha(0.6, 1.6, 24.), 0.);
        assert_eq!(dust_alpha(1.6, 1.6, 0.), 0.);
    }
    #[test]
    fn bird_range_uses_final_propagated_parented_camera_while_paused() {
        use bevy::camera::visibility::{VisibleEntityRanges, check_visibility_ranges};
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, TransformPlugin))
            .init_resource::<VisibleEntityRanges>()
            .add_systems(
                PostUpdate,
                check_visibility_ranges.after(bevy::transform::TransformSystems::Propagate),
            );
        let parent = app
            .world_mut()
            .spawn(Transform::from_xyz(1000., 0., 0.))
            .id();
        let camera = app
            .world_mut()
            .spawn((Camera::default(), Transform::default(), ChildOf(parent)))
            .id();
        let bird = app
            .world_mut()
            .spawn((Transform::from_xyz(600., 0., 0.), bird_range()))
            .id();
        app.update();
        assert!(
            app.world()
                .resource::<VisibleEntityRanges>()
                .entity_is_in_range_of_view(bird, camera)
        );
        app.world_mut()
            .get_mut::<Transform>(parent)
            .unwrap()
            .translation
            .x = -100.;
        app.update();
        assert!(
            !app.world()
                .resource::<VisibleEntityRanges>()
                .entity_is_in_range_of_view(bird, camera)
        );
    }
    #[test]
    fn metro_flocks_remain_coherent_and_deterministic_through_long_runs() {
        let track = Track::metropolis().unwrap();
        for group in 0..6 {
            let center = flock_center(&track, group);
            let leader = bird_at(center, false, group * 4);
            for seconds in [0., 15., 60., 600., 3600.] {
                let first = bird_pose(&leader, seconds);
                for member in 0..4 {
                    let b = bird_at(center, false, group * 4 + member);
                    let pose = bird_pose(&b, seconds);
                    assert!(pose.translation.distance(first.translation) < 30.);
                    assert!(pose.translation.is_finite() && pose.rotation.is_finite());
                    assert_eq!(pose, bird_pose(&b, seconds));
                }
            }
        }
        let range = bird_range();
        assert_eq!(range.end_margin, 450.0..650.0);
        assert!(!range.use_aabb);
    }
    #[test]
    fn metro_flocks_cover_streets_and_authored_civic_openings() {
        let track = Track::metropolis().unwrap();
        let metro = track.metropolis.as_ref().unwrap();
        for (group, kind) in [(3, "city_hall"), (5, "park")] {
            let site = metro
                .civic_sites
                .iter()
                .find(|site| site.kind == kind)
                .unwrap();
            let center = flock_center(&track, group);
            assert!(center.xz().distance(Vec2::from_array(site.position)) < 0.01);
            assert!(center.y > track.terrain_height(center.x, center.z) + 38.);
        }
        for (group, station) in [(2, 4850.), (4, 8650.)] {
            let center = flock_center(&track, group);
            let (road, _) = track.sample_route(0, station, 0.);
            assert!(center.xz().distance(road.xz()) < 40.);
            assert!(center.y > road.y + 40.);
        }
    }
    #[test]
    fn both_maps_share_bounded_bird_meshes_but_only_hills_allocates_dust() {
        for track in [Track::metropolis().unwrap(), Track::hills().unwrap()] {
            let hills = track.is_hills();
            let mut app = App::new();
            app.insert_resource(track)
                .init_resource::<Assets<Mesh>>()
                .init_resource::<Assets<StandardMaterial>>()
                .add_systems(
                    Startup,
                    |mut commands: Commands,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<StandardMaterial>>,
                     track: Res<Track>| {
                        spawn(&mut commands, &mut meshes, &mut materials, &track, None);
                    },
                );
            app.update();
            let world = app.world_mut();
            assert_eq!(
                world
                    .query_filtered::<Entity, With<Bird>>()
                    .iter(world)
                    .count(),
                if birds_enabled() { BIRD_COUNT } else { 0 }
            );
            assert_eq!(
                world
                    .query_filtered::<Entity, With<DustPuff>>()
                    .iter(world)
                    .count(),
                if hills { DUST_CAPACITY } else { 0 }
            );
            assert_eq!(
                world
                    .query_filtered::<Entity, With<VisibilityRange>>()
                    .iter(world)
                    .count(),
                if birds_enabled() { BIRD_COUNT * 3 } else { 0 }
            );
            assert_eq!(world.query::<&Collider>().iter(world).count(), 0);
            assert!(world.contains_resource::<Atmosphere>());
        }
    }
    #[test]
    fn dust_requires_motion_ground_and_unpaved_wheel_position() {
        assert!(dust_eligible(18., 0.2, 9., 6.5, None));
        assert!(dust_eligible(-8., 0.1, -9., 6.5, None));
        // A near-ground airborne chassis must not emit just because its ray finds terrain.
        assert!(!dust_eligible(18., 0.2, 9., 6.5, Some([false, false])));
        assert!(dust_eligible(18., 0.2, 9., 6.5, Some([true, false])));
        assert!(dust_eligible(18., 0.2, 9., 6.5, Some([false, true])));
        for (speed, gap, lane) in [
            (0., 0.2, 9.),
            (18., 2., 9.),
            (18., 0.2, 4.),
            (18., 0.2, 40.),
            (f32::NAN, 0.2, 9.),
        ] {
            assert!(!dust_eligible(speed, gap, lane, 6.5, None));
        }
        assert_eq!(active_delta(crate::game::Phase::Paused, 1. / 60.), 0.);
        assert_eq!(active_delta(crate::game::Phase::Racing, 0.), 0.);
        assert_eq!(active_delta(crate::game::Phase::Racing, 1.), 0.05);
    }
    #[test]
    fn native_pool_lifecycle_keeps_pause_and_resets_retry_without_spawning() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(crate::game::Race {
                phase: crate::game::Phase::Racing,
                elapsed: 12.,
                ..default()
            })
            .insert_resource(Atmosphere {
                elapsed: 5.,
                emitted: 8,
                active_dust: 1,
                ..default()
            })
            .add_systems(Update, reset);
        let dust = app
            .world_mut()
            .spawn((
                DustPuff {
                    age: 0.3,
                    life: 1.6,
                    drift: Vec3::ONE,
                    tint: Vec3::new(0.76, 0.61, 0.38),
                },
                Visibility::Inherited,
            ))
            .id();
        let bird = app
            .world_mut()
            .spawn((
                Bird {
                    center: Vec3::Y * 20.,
                    radius: 25.,
                    phase: 0.,
                    rate: 0.2,
                    flocking: false,
                },
                Transform::from_translation(Vec3::ONE),
            ))
            .id();
        let wing = app
            .world_mut()
            .spawn((
                Wing {
                    side: 1.,
                    phase: 0.,
                },
                Transform::from_rotation(Quat::from_rotation_z(0.5)),
            ))
            .id();
        app.update();
        app.world_mut().resource_mut::<crate::game::Race>().phase = crate::game::Phase::Paused;
        app.update();
        assert_eq!(app.world().resource::<Atmosphere>().emitted, 8);
        assert_eq!(app.world().get::<DustPuff>(dust).unwrap().age, 0.3);
        assert_eq!(
            app.world().get::<Transform>(bird).unwrap().translation,
            Vec3::ONE
        );
        let count = app.world().entities().count_spawned();
        {
            let mut r = app.world_mut().resource_mut::<crate::game::Race>();
            r.phase = crate::game::Phase::Countdown;
            r.elapsed = 0.;
        }
        app.update();
        assert_eq!(app.world().entities().count_spawned(), count);
        assert_eq!(app.world().resource::<Atmosphere>().emitted, 0);
        assert_eq!(app.world().resource::<Atmosphere>().elapsed, 0.);
        assert_eq!(
            *app.world().get::<Visibility>(dust).unwrap(),
            Visibility::Hidden
        );
        assert_eq!(
            app.world().get::<Transform>(bird).unwrap().translation,
            Vec3::new(25., 20., 0.)
        );
        assert_eq!(
            app.world().get::<Transform>(wing).unwrap().rotation,
            Quat::IDENTITY
        );
    }
    #[test]
    fn original_dust_sprite_has_transparent_edges_and_bounded_payload() {
        let image = sprite();
        let bytes = image.data.unwrap();
        assert_eq!(bytes.len(), 64 * 64 * 4);
        assert_eq!(bytes[3], 0);
        assert!(bytes[(32 * 64 + 32) * 4 + 3] > 150);
        assert_eq!(DUST_CAPACITY, 96);
        assert_eq!(BIRD_COUNT, 24);
    }
}
