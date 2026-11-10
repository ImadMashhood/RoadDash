//! Opt-in native motion evidence using production civilian actors and colliders.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
struct Survey {
    frame: u32,
    selected: Vec<Entity>,
    max_blend: f32,
    max_support_error: f32,
    passed: bool,
    cases: Vec<serde_json::Value>,
}
pub(crate) fn install(app: &mut App) {
    app.init_resource::<PlanningTimings>();
    if std::env::var("ROADDASH_CIVILIAN_FLEET").is_ok_and(|v| v == "1") {
        app.insert_resource(bevy::winit::WinitSettings::continuous())
            .init_resource::<FleetSurvey>()
            .add_systems(
                PostUpdate,
                fleet_run
                    .after(super::camera)
                    .before(bevy::transform::TransformSystems::Propagate),
            );
        return;
    }
    app.insert_resource(bevy::winit::WinitSettings::continuous())
        .init_resource::<Survey>()
        .add_systems(
            PostUpdate,
            run.after(super::camera)
                .before(bevy::transform::TransformSystems::Propagate),
        );
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    mut cars: Query<
        (
            Entity,
            &mut Traffic,
            &mut TrafficYield,
            &mut Transform,
            Option<&crate::heavy_traffic::HeavyTraffic>,
        ),
        (Without<Rider>, Without<Camera3d>),
    >,
    mut riders: Query<&mut Rider>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    shapes: Query<&Collider>,
    mut exit: MessageWriter<AppExit>,
    timings: Res<PlanningTimings>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    let case = (state.frame - 1) / 2600;
    let tick = (state.frame - 1) % 2600 + 1;
    let direction = if case.is_multiple_of(2) { 1. } else { -1. };
    let kind = if case < 2 {
        crate::heavy_traffic::Kind::Tractor
    } else {
        crate::heavy_traffic::Kind::Semi
    };
    race.phase = if tick <= 120 {
        Phase::Paused
    } else {
        Phase::Racing
    };
    for mut r in &mut riders {
        r.finished = Some(0.);
    }
    for (e, parent) in &ui {
        if parent.is_none() {
            commands.entity(e).insert(Visibility::Hidden);
        }
    }
    if tick == 1 {
        state.max_blend = 0.;
        state.max_support_error = 0.;
        state.passed = false;
        race.elapsed = 10.;
        let mut ordinary: Vec<_> = cars.iter().filter(|c| c.4.is_none()).map(|c| c.0).collect();
        ordinary.sort();
        let leader = cars
            .iter()
            .find(|c| c.4.is_some_and(|h| h.kind == kind))
            .map(|c| c.0)
            .unwrap_or(ordinary[1]);
        state.selected = vec![ordinary[0], leader];
        for (i, (e, mut car, mut yielding, mut pose, _)) in cars.iter_mut().enumerate() {
            let role = state.selected.iter().position(|id| *id == e);
            let station = if track.is_hills() { 5900. } else { 1400. };
            let progress = role.map_or(9500. + i as f32 * 15., |r| {
                station + r as f32 * direction * 35.
            });
            let speed = role.map_or(0., |r| {
                if r == 0 {
                    18. * direction
                } else {
                    kind.cruise() * direction
                }
            });
            let ratio = 3.1 * direction / track.half_width(progress);
            let (home, _, _) = civilian_lanes(&track, 0, progress, direction, 0, ratio);
            *car = Traffic {
                route: 0,
                progress,
                lane: home,
                speed,
            };
            *yielding = TrafficYield {
                cruise: Some(speed),
                lane_ratio: Some(ratio),
                start_progress: Some(progress),
                ..default()
            };
            let (p, q) = track.street_sample(0, progress, home);
            *pose = Transform::from_translation(p).with_rotation(
                q * if direction < 0. {
                    Quat::from_rotation_y(std::f32::consts::PI)
                } else {
                    Quat::IDENTITY
                },
            );
        }
    }
    let (_, car, y, pose, _) = cars.get(state.selected[0]).unwrap();
    let (_, lead, _, lead_pose, _) = cars.get(state.selected[1]).unwrap();
    if tick > 120 {
        let iso = |t: &Transform| {
            bevy_rapier3d::rapier::math::Isometry::from_parts(
                t.translation.into(),
                t.rotation.into(),
            )
        };
        let overlap = bevy_rapier3d::parry::query::intersection_test(
            &iso(pose),
            shapes.get(state.selected[0]).unwrap().raw.as_ref(),
            &iso(lead_pose),
            shapes.get(state.selected[1]).unwrap().raw.as_ref(),
        )
        .expect("supported production shapes");
        assert!(
            !overlap,
            "native civilian collider overlap case={case} tick={tick}"
        );
        let (home, opposite, _) = civilian_lanes(
            &track,
            car.route,
            car.progress,
            direction,
            0,
            y.lane_ratio.unwrap(),
        );
        let supported = track
            .street_sample(
                car.route,
                car.progress,
                home + (opposite - home) * y.passing.blend,
            )
            .0;
        let error = pose.translation.distance(supported);
        state.max_support_error = state.max_support_error.max(error);
        assert!(error < 0.02, "native sampler support lost");
    }
    state.max_blend = state.max_blend.max(y.passing.blend);
    state.passed |= (car.progress - lead.progress) * direction > 15.;
    let center = pose.translation;
    let q = track.street_sample(0, car.progress, 0.).1;
    for mut camera in &mut cameras {
        *camera = Transform::from_translation(
            center - q * Vec3::Z * direction * 24. + q * Vec3::X * 17. + Vec3::Y * 13.,
        )
        .looking_at(center + q * Vec3::Z * direction * 12., Vec3::Y);
    }
    if [100, 480, 900, 1800, 2500].contains(&tick) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/civilian-{case}-{tick}.png"
            )));
    }
    if tick == 2600 {
        assert!(
            state.passed
                && state.max_blend > 0.95
                && y.passing.phase == rr_core::civilian::Phase::Follow
                && y.passing.blend == 0.,
            "native civilian pass failed case={case} blend={} car={} lead={}",
            state.max_blend,
            car.progress,
            lead.progress
        );
        let result = serde_json::json!({"direction":direction,"kind":format!("{kind:?}"),"passed":state.passed,"max_blend":state.max_blend,"car_station":car.progress,"leader_station":lead.progress,"phase":format!("{:?}",y.passing.phase),"final_blend":y.passing.blend,"collider_intersections":0,"support_error_max_m":state.max_support_error});
        state.cases.push(result);
        if case == 3 {
            std::fs::write("screenshots/civilian-overtaking.json",serde_json::to_string_pretty(&serde_json::json!({"course":if track.is_hills(){"Hills"}else{"Metropolis"},"cases":state.cases,"frames":state.frame,"planning_cpu":timings.report(),"limits":"Production traffic fixed-step motion; selected two-vehicle fixtures; Metropolis uses production car bodies at heavy-vehicle cruise. Full-fleet/biker/abort coverage is in deterministic tests, not inferred from these captures. Not GPU or full-race performance evidence."})).unwrap()).unwrap();
            exit.write(AppExit::Success);
        }
    }
}

#[derive(Resource, Default)]
struct FleetSurvey {
    frame: u32,
    actors: std::collections::BTreeMap<Entity, (rr_core::civilian::Phase, Option<u64>)>,
    initiated: u32,
    completed: u32,
    aborted: u32,
    peak_active: usize,
    follow_camera: Option<Entity>,
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn fleet_run(
    mut commands: Commands,
    mut state: ResMut<FleetSurvey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    cars: Query<(Entity, &Traffic, &TrafficYield, &Transform), Without<Camera3d>>,
    mut riders: Query<&mut Rider>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    mut exit: MessageWriter<AppExit>,
    timings: Res<PlanningTimings>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    race.phase = Phase::Racing;
    // Production civilian positions, speed classes, lane slots and schedules are
    // untouched. Park racers so this census separates ordinary motor traffic
    // opportunities from combat/downed-person interruptions.
    for mut rider in &mut riders {
        rider.finished = Some(0.);
    }
    let mut active = 0;
    for (e, car, y, _) in &cars {
        let phase = y.passing.phase;
        if y.passing.reserved() {
            active += 1;
        }
        let previous = state.actors.get(&e).copied().unwrap_or_default();
        let mut target = previous.1;
        if phase == rr_core::civilian::Phase::Pass && previous.0 == rr_core::civilian::Phase::Follow
        {
            state.initiated += 1;
            target = y.passing.target();
            state.follow_camera = Some(e);
        }
        if phase == rr_core::civilian::Phase::Follow
            && previous.0 != rr_core::civilian::Phase::Follow
        {
            let ahead = target
                .and_then(|id| cars.iter().find(|x| x.0.to_bits() == id))
                .is_some_and(|(_, leader, _, _)| {
                    (car.progress - leader.progress) * y.cruise.unwrap_or(car.speed).signum() > 8.
                });
            if ahead {
                state.completed += 1;
            } else {
                state.aborted += 1;
            }
            target = None;
        }
        state.actors.insert(e, (phase, target));
    }
    state.peak_active = state.peak_active.max(active);
    if let Some((_, car, _, pose)) = state
        .follow_camera
        .and_then(|e| cars.get(e).ok())
        .or_else(|| cars.iter().next())
    {
        let q = track.street_sample(car.route, car.progress, 0.).1;
        for mut camera in &mut cameras {
            *camera = Transform::from_translation(
                pose.translation - q * Vec3::Z * 25. + q * Vec3::X * 17. + Vec3::Y * 14.,
            )
            .looking_at(pose.translation, Vec3::Y);
        }
    }
    if [1200, 3600, 7000].contains(&state.frame) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/civilian-fleet-{}.png",
                state.frame
            )));
    }
    if state.frame == 7200 {
        let report = serde_json::json!({"course":if track.is_hills(){"Hills"}else{"Metropolis"},"nominal_simulated_seconds":120,"race_elapsed_seconds":race.elapsed,"vehicles":cars.iter().count(),"initiated":state.initiated,"completed":state.completed,"aborted_returns":state.aborted,"active_at_end":active,"peak_active":state.peak_active,"planning_cpu":timings.report(),"limits":"Unmodified production civilian fleet; racers parked to isolate ordinary motor-traffic opportunities. Counters observe all vehicles, camera follows initiated passes. No guarantee every impediment has a safe opposing-lane window. Not whole-race/human/GPU acceptance."});
        std::fs::write(
            "screenshots/civilian-fleet.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .unwrap();
        exit.write(AppExit::Success);
    }
}

/// Opt-in native CPU instrumentation; absent from ordinary game sessions.
#[derive(Resource, Default)]
pub struct PlanningTimings {
    samples: Vec<(f64, f64)>,
}
impl PlanningTimings {
    pub(crate) fn record(&mut self, planner_ms: f64, traffic_ms: f64) {
        if self.samples.len() < 16384 {
            self.samples.push((planner_ms, traffic_ms));
        }
    }
    fn report(&self) -> serde_json::Value {
        let summary = |index: usize| {
            let mut values: Vec<_> = self
                .samples
                .iter()
                .map(|v| if index == 0 { v.0 } else { v.1 })
                .collect();
            values.sort_by(f64::total_cmp);
            serde_json::json!({"mean_ms":values.iter().sum::<f64>()/values.len().max(1)as f64,"p95_ms":values.get(values.len()*95/100).copied().unwrap_or(0.),"max_ms":values.last().copied().unwrap_or(0.)})
        };
        serde_json::json!({"ticks":self.samples.len(),"new_planner":summary(0),"whole_civilian_adapter":summary(1),"limits":"CPU/wall elapsed. Planner includes fleet ordering/copy, perception, FSM and full-collider geometry sweeps plus timing overhead. Whole adapter additionally includes existing following, motion/sampling/interest and bounds. No GPU/full-frame or before-after speedup claim."})
    }
}
