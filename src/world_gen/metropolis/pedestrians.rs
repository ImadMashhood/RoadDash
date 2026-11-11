//! Authored sidewalk-to-sidewalk crossings; never modifies the NPC physical rig.
use crate::{
    environment::Pedestrian,
    game::{Phase, Race, Rider, Traffic},
    track::Track,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_core::pedestrian_crossing as rules;
#[derive(Component)]
pub struct CrossingWalker {
    pub start: Vec3,
    pub end: Vec3,
    pub active: bool,
    pub completed: u32,
    pub started: u32,
    ready_at: f32,
    returning: bool,
}
impl CrossingWalker {
    pub fn new(start: Vec3, end: Vec3, index: usize) -> Self {
        Self {
            start,
            end,
            active: false,
            completed: 0,
            started: 0,
            ready_at: 8. + (index % 8) as f32 * 3.,
            returning: false,
        }
    }
}
#[derive(Default)]
pub struct ReleaseClock {
    previous: f32,
    next: f32,
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update(
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    track: Res<Track>,
    context: ReadRapierContext,
    ground: Query<(), With<crate::world::Ground>>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut clock: Local<ReleaseClock>,
    mut walkers: Query<
        (
            &mut CrossingWalker,
            &mut Transform,
            &mut Velocity,
            Option<&ColliderDisabled>,
        ),
        With<Pedestrian>,
    >,
    traffic: Query<
        (
            &Transform,
            Option<&Velocity>,
            Option<&Rider>,
            Option<&Traffic>,
            Option<&crate::recovery::Recovery>,
        ),
        (
            Without<Pedestrian>,
            Or<(
                With<Rider>,
                With<Traffic>,
                With<crate::environment::CrossTraffic>,
                With<crate::recovery::OwnedBody>,
            )>,
        ),
    >,
) {
    if race.phase != Phase::Racing {
        return;
    }
    let reset = race.elapsed < clock.previous;
    clock.previous = race.elapsed;
    if reset {
        clock.next = race.elapsed + 8.;
    }
    let Ok(context) = context.single() else {
        return;
    };
    let mut active = walkers
        .iter()
        .filter(|(w, _, _, disabled)| w.active && disabled.is_none())
        .count();
    for (mut w, mut transform, mut velocity, disabled) in &mut walkers {
        if disabled.is_some() || reset {
            w.active = false;
            w.returning = false;
            w.ready_at = race.elapsed + rules::REST_SECONDS;
            if reset && disabled.is_none() {
                transform.translation = w.start;
                w.completed = 0;
                w.started = 0;
            }
            velocity.linvel = Vec3::ZERO;
            continue;
        }
        let (start, end) = if w.returning {
            (w.end, w.start)
        } else {
            (w.start, w.end)
        };
        if !w.active {
            let nearby = cameras
                .iter()
                .any(|c| c.translation().xz().distance(start.xz()) < 300.)
                || traffic.iter().any(|(t, _, r, _, _)| {
                    r.is_some_and(|r| r.id == 0 || r.remote_human)
                        && t.translation.xz().distance(start.xz()) < 300.
                });
            // Cooldown, global admission and distance are cheap. Forecast motion only
            // when this walker could actually step off the curb this fixed tick.
            if !rules::may_start(race.elapsed, w.ready_at, clock.next, active, nearby, true) {
                velocity.linvel = Vec3::ZERO;
                continue;
            }
            let clear = traffic.iter().all(|(t, v, r, car, recovery)| {
                if r.is_some()
                    && recovery
                        .is_some_and(|rec| rec.stage != crate::recovery::RecoveryStage::Riding)
                {
                    return true;
                }
                !rules::conflicts(
                    start.xz().to_array(),
                    end.xz().to_array(),
                    t.translation.xz().to_array(),
                    forecast_velocity(&track, t, v, car).xz().to_array(),
                )
            });
            if !rules::may_start(race.elapsed, w.ready_at, clock.next, active, nearby, clear) {
                velocity.linvel = Vec3::ZERO;
                continue;
            }
            w.active = true;
            w.started += 1;
            active += 1;
            clock.next = race.elapsed + rules::RELEASE_INTERVAL;
        }
        let delta = (end - transform.translation).with_y(0.);
        let distance = delta.length();
        let direction = delta.normalize_or_zero();
        let next = transform.translation
            + direction * (rules::WALK_SPEED * time.delta_secs()).min(distance);
        let predicate = |entity| ground.contains(entity);
        let filter = QueryFilter::only_fixed().predicate(&predicate);
        let origin = next.with_y(track.terrain_height(next.x, next.z) + 3.);
        let Some((_, toi)) = context.cast_ray(origin, Vec3::NEG_Y, 6., true, filter) else {
            velocity.linvel = Vec3::ZERO;
            continue;
        };
        let next = next.with_y(origin.y - toi + 0.015);
        velocity.linvel = (next - transform.translation) / time.delta_secs();
        transform.translation = next;
        transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
        if distance <= rules::WALK_SPEED * time.delta_secs() {
            w.active = false;
            w.completed += 1;
            w.returning = !w.returning;
            w.ready_at = race.elapsed + rules::REST_SECONDS;
            active = active.saturating_sub(1);
            velocity.linvel = Vec3::ZERO;
        }
    }
}

fn forecast_velocity(
    track: &Track,
    _transform: &Transform,
    velocity: Option<&Velocity>,
    car: Option<&Traffic>,
) -> Vec3 {
    car.map_or_else(
        || velocity.map_or(Vec3::ZERO, |v| v.linvel),
        |c| track.street_sample(c.route, c.progress, c.lane).1 * Vec3::Z * c.speed,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_car_without_velocity_predicts_approach_on_both_routes() {
        let track = Track::metropolis().unwrap();
        for route in 0..2 {
            for speed in [-15., 15.] {
                let (p, q) = track.street_sample(route, 4300., 0.);
                let car = Traffic {
                    route,
                    progress: 4300.,
                    lane: 0.,
                    speed,
                };
                let v =
                    forecast_velocity(&track, &Transform::from_translation(p), None, Some(&car));
                assert!(v.abs_diff_eq(q * Vec3::Z * speed, 1e-5));
                assert!(v.length() > 14.9);
            }
        }
    }
    #[test]
    fn authored_crossing_uses_actual_support_waits_and_completes_without_teleport() {
        for route in 0..2 {
            verify_authored_crossing(route);
        }
    }
    fn verify_authored_crossing(route: usize) {
        let track = Track::metropolis().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        let plan = m
            .pedestrians
            .iter()
            .find(|p| p.route == route && p.crossing_end.is_some())
            .expect("authored crossings");
        let plan_route = plan.route;
        let plan_station = plan.station;
        let end = plan.crossing_end.unwrap();
        let start = Vec3::new(
            plan.position[0],
            track.terrain_height(plan.position[0], plan.position[1]) + 0.15,
            plan.position[1],
        );
        let end = Vec3::new(end[0], 0., end[1]);
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1. / 60.,
            substeps: 3,
        })
        .insert_resource(Race {
            phase: Phase::Racing,
            elapsed: 0.,
            ..default()
        });
        for surface in &m.surfaces {
            if surface.kind > 1
                || !surface
                    .shape
                    .vertices()
                    .any(|p| Vec2::from_array(p).distance(start.xz()) < 40.)
            {
                continue;
            }
            let points = surface
                .shape
                .vertices()
                .map(|[x, z]| {
                    Vec3::new(
                        x,
                        track.terrain_height(x, z) + if surface.kind == 1 { 0.14 } else { 0.025 },
                        z,
                    )
                })
                .collect();
            app.world_mut().spawn((
                Collider::trimesh(
                    points,
                    surface
                        .shape
                        .triangles
                        .iter()
                        .map(|t| [t[0], t[2], t[1]])
                        .collect(),
                )
                .unwrap(),
                Transform::IDENTITY,
                crate::world::Ground,
            ));
        }
        let heading = track.street_sample(plan_route, plan_station, 0.).1 * Vec3::Z;
        let intercept_time = start.xz().distance(end.xz()) / (2. * rules::WALK_SPEED);
        let approach = start.lerp(end, 0.5) - heading * 15. * intercept_time;
        // Production main cars intentionally have no Velocity component.
        let approaching = app
            .world_mut()
            .spawn((
                Transform::from_translation(approach),
                Traffic {
                    route: plan_route,
                    progress: plan_station,
                    lane: 0.,
                    speed: 15.,
                },
            ))
            .id();
        app.insert_resource(track)
            .add_systems(FixedUpdate, update.before(PhysicsSet::SyncBackend));
        app.world_mut().spawn((
            Rider::new(0),
            Transform::from_translation(start + Vec3::Z * 35.),
            Velocity::zero(),
        ));
        let e = app
            .world_mut()
            .spawn((
                Pedestrian,
                Transform::from_translation(start),
                Velocity::zero(),
                CrossingWalker::new(start, end, 0),
            ))
            .id();
        let mut last = start;
        let mut max_step = 0f32;
        let mut min_y = f32::INFINITY;
        for frame in 0..2100 {
            if frame == 720 {
                app.world_mut().entity_mut(approaching).despawn();
            }
            app.world_mut().resource_mut::<Race>().elapsed = frame as f32 / 60.;
            app.update();
            let p = app.world().get::<Transform>(e).unwrap().translation;
            max_step = max_step.max(p.xz().distance(last.xz()));
            min_y = min_y.min(p.y);
            last = p;
            if frame < 720 {
                assert!(p.xz().distance(start.xz()) < 0.001);
            }
        }
        let state = app.world().get::<CrossingWalker>(e).unwrap();
        assert_eq!((state.started, state.completed), (1, 1));
        assert!(!state.active && last.xz().distance(end.xz()) < 0.001);
        assert!(
            max_step < rules::WALK_SPEED / 60. + 0.001,
            "no teleport: {max_step}"
        );
        assert!(min_y.is_finite());
        assert_eq!(app.world().get::<Velocity>(e).unwrap().linvel, Vec3::ZERO);
        let track = app.world().resource::<Track>();
        assert!(
            (last.y - track.terrain_height(last.x, last.z) - 0.155).abs() < 0.015,
            "final sidewalk support {last:?}"
        );
        app.world_mut().resource_mut::<Race>().elapsed = 0.;
        app.update();
        assert!(
            app.world()
                .get::<Transform>(e)
                .unwrap()
                .translation
                .abs_diff_eq(start, 0.001)
        );
    }
}
