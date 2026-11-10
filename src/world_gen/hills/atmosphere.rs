//! Bounded, presentation-only wildlife and wheel dust. No collision or rig writes.
use crate::{
    game::{Rider, Traffic},
    track::Track,
};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
pub const DUST_CAPACITY: usize = 96;
pub const BIRD_COUNT: usize = 24;
const EMISSION_INTERVAL: f32 = 0.08;
#[derive(Component)]
pub struct DustPuff {
    pub age: f32,
    life: f32,
    drift: Vec3,
}
#[derive(Component)]
pub struct Bird {
    center: Vec3,
    radius: f32,
    phase: f32,
    rate: f32,
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
    if !track.is_hills() {
        return;
    }
    let texture = images.map(|images| images.add(sprite()));
    let quad = meshes.add(Rectangle::new(1., 1.));
    for _ in 0..DUST_CAPACITY {
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.88, 0.76, 0.56, 0.),
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
            },
        ));
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
    for i in 0..BIRD_COUNT {
        let group = i / 4;
        let station = [2200., 3900., 5900., 7900., 9800., 11700.][group];
        let (p, q) = track.sample_route(0, station, 0.);
        let center = p
            + q * Vec3::X * if group % 2 == 0 { 65. } else { -65. }
            + Vec3::Y * (35. + (i % 4) as f32 * 4.);
        let phase = i as f32 * 2.399;
        let root = commands
            .spawn((
                Transform::from_translation(center),
                Visibility::default(),
                Bird {
                    center,
                    radius: 25. + (i % 4) as f32 * 8.,
                    phase,
                    rate: 0.13 + (i % 3) as f32 * 0.025,
                },
            ))
            .id();
        let torso = commands
            .spawn((
                Mesh3d(body.clone()),
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
                    MeshMaterial3d(feather.clone()),
                    Transform::from_scale(Vec3::new(side, 1., 1.)),
                    Wing { side, phase },
                    bevy::light::NotShadowCaster,
                ))
                .id();
            commands.entity(root).add_child(e);
        }
    }
    commands.insert_resource(Atmosphere::default());
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
        t.translation = bird.center;
        t.rotation = Quat::IDENTITY;
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
        (&Wing, &mut Transform),
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
    // Camera surveys and pause freelook still update view culling; no bird/dust
    // positions, ages or emission clocks advance without active presentation time.
    for (_, t, mut visible) in &mut birds {
        *visible = if t.translation.distance_squared(camera.translation) < 650f32.powi(2) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    if dt == 0. {
        return;
    }
    state.elapsed += dt;
    state.emission_clock += dt;
    let elapsed = state.elapsed;
    for (bird, mut t, mut visible) in &mut birds {
        let a = elapsed * bird.rate + bird.phase;
        t.translation = bird.center
            + Vec3::new(
                a.cos() * bird.radius,
                (a * 1.7).sin() * 3.,
                a.sin() * bird.radius,
            );
        t.rotation = Quat::from_rotation_y(-a) * Quat::from_rotation_z(-0.12);
        *visible = if t.translation.distance_squared(camera.translation) < 650f32.powi(2) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for (wing, mut t) in &mut wings {
        t.rotation =
            Quat::from_rotation_z(wing.side * ((elapsed * 7. + wing.phase).sin() * 0.48 + 0.12));
    }
    state.animated_birds = birds.iter().count();
    let mut emissions = Vec::new();
    if state.emission_clock >= EMISSION_INTERVAL {
        state.emission_clock %= EMISSION_INTERVAL;
        if let Ok(context) = physics.single() {
            let is_ground = |e| ground.contains(e);
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
            && let Some((p, drift)) = next.next()
        {
            puff.age = 0.;
            puff.drift = drift;
            t.translation = p;
            state.emitted += 1;
        }
        if puff.age < puff.life {
            active += 1;
            t.translation += puff.drift * dt;
            t.rotation = camera.rotation;
            t.scale = Vec3::splat(0.8 + puff.age * 1.6);
            *visibility = Visibility::Inherited;
            if let Some(m) = materials.get_mut(&material.0) {
                m.base_color.set_alpha(0.38 * (1. - puff.age / puff.life));
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
        let count = app.world().entities().len();
        {
            let mut r = app.world_mut().resource_mut::<crate::game::Race>();
            r.phase = crate::game::Phase::Countdown;
            r.elapsed = 0.;
        }
        app.update();
        assert_eq!(app.world().entities().len(), count);
        assert_eq!(app.world().resource::<Atmosphere>().emitted, 0);
        assert_eq!(app.world().resource::<Atmosphere>().elapsed, 0.);
        assert_eq!(
            *app.world().get::<Visibility>(dust).unwrap(),
            Visibility::Hidden
        );
        assert_eq!(
            app.world().get::<Transform>(bird).unwrap().translation,
            Vec3::Y * 20.
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
