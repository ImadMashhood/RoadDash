//! Opt-in native sand receipt: real authored ground and live Dynamic adapter.
use crate::{
    game::{Phase, Race, Rider},
    runtime::{AppScreen, SimulationClock},
    track::Track,
    vehicle::BikeState,
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use bevy_rapier3d::prelude::*;
#[derive(Resource, Default)]
pub struct Survey {
    case: usize,
    physics_start: Option<u64>,
    start: Option<u64>,
    records: Vec<serde_json::Value>,
    frames: u32,
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<AppScreen>>,
    track: Res<Track>,
    clock: Res<SimulationClock>,
    mut race: ResMut<Race>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut riders: Query<(&mut Rider, &mut Transform, &mut Velocity, &BikeState)>,
    ground: Query<(), With<crate::world::Ground>>,
    context: ReadRapierContext,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(screen.get(), AppScreen::Boot | AppScreen::Loading) {
        return;
    }
    state.frames += 1;
    if state.frames == 1 {
        std::fs::create_dir_all("screenshots").unwrap();
    }
    assert!(state.frames < 15000, "surface native timeout");
    if state.case >= 2 {
        race.phase = Phase::Paused;
        if state.frames.is_multiple_of(60) {
            std::fs::write(
                "screenshots/surface-survey.json",
                serde_json::to_vec_pretty(&state.records).unwrap(),
            )
            .unwrap();
            exit.write(AppExit::Success);
        }
        return;
    }
    race.phase = Phase::Racing;
    keys.press(KeyCode::KeyW);
    // Leaving the garage enables the physics pipeline on the next fixed tick.
    // Authored colliders already exist in ECS, but its query broad phase is not
    // ready for a support ray in this same Update.
    let physics_start = *state.physics_start.get_or_insert(clock.active_ticks);
    if clock.active_ticks.saturating_sub(physics_start) < 2 {
        return;
    }
    let Some((mut rider, mut body, mut velocity, bike)) =
        riders.iter_mut().find(|(r, _, _, _)| r.id == 0)
    else {
        return;
    };
    rider.immunity = 1000.;
    if state.start.is_none() {
        let station = if track.is_hills() { 11000. } else { 1200. };
        let lane = if state.case == 0 { 0. } else { 10. };
        let (mut p, q) = track.street_sample(0, station, lane);
        let is_ground = |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
        let (_, toi) = context
            .single()
            .unwrap()
            .cast_ray(
                p + Vec3::Y * 30.,
                Vec3::NEG_Y,
                80.,
                true,
                QueryFilter::default().predicate(&is_ground),
            )
            .unwrap_or_else(|| {
                panic!(
                    "surface support after {} active ticks at {p:?}",
                    clock.active_ticks.saturating_sub(physics_start)
                )
            });
        p.y += 30. - toi + 0.3;
        rider.progress = station;
        rider.route = 0;
        rider.lane = track.lane_at(0, station, p);
        rider.speed = 0.;
        *body = Transform::from_translation(p).with_rotation(q);
        *velocity = Velocity::zero();
        state.start = Some(clock.active_ticks);
        return;
    }
    let ticks = clock.active_ticks - state.start.unwrap();
    if ticks >= 240 {
        let surface = track.surface_route(rider.route, rider.progress, rider.lane);
        assert!(rider.crashed == 0., "surface fixture crashed");
        assert!(rider.speed > 2., "surface fixture did not accelerate");
        if state.case == 1 && track.is_hills() {
            assert_eq!(surface, rr_core::surface::Surface::Sand);
        }
        let record = serde_json::json!({"case":state.case,"ticks":ticks,"surface":surface,"speed_mps":rider.speed,"grounded":bike.grounded,"station":rider.progress,"lane":rider.lane,"position":body.translation.to_array(),"limits":"Actual authored terrain; native slopes differ. Matched-grade comparisons are in the adapter regression. Not a performance or human-feel approval."});
        state.records.push(record);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/surface-{}.png",
                state.case
            )));
        state.case += 1;
        state.start = None;
    }
}
