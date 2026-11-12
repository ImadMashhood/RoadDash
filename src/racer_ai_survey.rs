//! Opt-in unteleported native race with periodic fixed-state decision receipts.
//! Slot zero uses the same racer controller; all traffic, combat and recovery stay live.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
struct Survey {
    frame: u32,
    rows: Vec<serde_json::Value>,
}
pub(crate) fn install(app: &mut App) {
    app.insert_resource(bevy::winit::WinitSettings::continuous())
        .init_resource::<Survey>()
        .add_systems(PostUpdate, run.after(super::camera));
    if std::env::var("ROADDASH_AI_SURVEY_HOLD_W").as_deref() == Ok("1") {
        app.add_systems(
            FixedUpdate,
            hold_w.before(super::drive).after(crate::recovery::recover),
        );
    }
}
fn hold_w(mut controls: ResMut<crate::controls::Controls>, race: Res<Race>) {
    if race.phase == Phase::Racing {
        controls.buffer.held = rr_core::VehicleIntent {
            throttle: true,
            throttle_amount: 1.0,
            ..default()
        };
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    mut next: ResMut<NextState<crate::runtime::AppScreen>>,
    mut race: ResMut<Race>,
    mut riders: Query<(Entity, &mut Rider, &Recovery)>,
    track: Res<Track>,
    mut bodies: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut crate::vehicle::BikeState,
        ),
        With<Rider>,
    >,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    // Opt-in, bounded diagnostic for the full-race stop reported around station
    // 3,040. The next fixed tick prints the actual proxies and static sweeps;
    // this does not alter decisions or the unteleported race state.
    if std::env::var_os("ROADDASH_AI_TRACE_3040").is_some() {
        for (_, mut rider, _) in &mut riders {
            if matches!(rider.id, 1 | 11 | 13) && (3010.0..=3180.0).contains(&rider.progress) {
                rider.ai_trace_next.get_or_insert(race.elapsed);
            } else if matches!(rider.id, 1 | 11 | 13) {
                rider.ai_trace_next = None;
            }
        }
    }
    if state.frame == 1 {
        for (_, mut rider, _) in &mut riders {
            if rider.id == 0 && std::env::var("ROADDASH_AI_SURVEY_HOLD_W").as_deref() != Ok("1") {
                rider.networked = true;
                rider.remote_human = false;
            }
        }
        // Explicit synthetic diagnostic, separate from an unteleported race receipt.
        if std::env::var("ROADDASH_AI_SURVEY_SEGMENT").as_deref() == Ok("1") {
            race.elapsed = 408.;
            for (entity, mut rider, _) in &mut riders {
                if rider.id != 0 {
                    continue;
                }
                rider.progress = 11090.;
                rider.lane = -0.844;
                rider.speed = 20.;
                rider.ai_trace_next = Some(0.);
                if let Ok((mut pose, mut velocity, mut bike)) = bodies.get_mut(entity) {
                    let (p, q) = track.sample_route(rider.route, rider.progress, rider.lane);
                    *pose = Transform::from_translation(p + Vec3::Y * 0.5).with_rotation(q);
                    rider.motion_velocity = q * Vec3::Z * rider.speed;
                    *velocity = Velocity::linear(rider.motion_velocity);
                    bike.position = pose.translation;
                    bike.velocity = rider.motion_velocity;
                    bike.impact_velocity = rider.motion_velocity;
                    bike.yaw = q.to_euler(EulerRot::YXZ).0;
                    bike.grounded = [false; 2];
                    bike.was_riding = true;
                }
            }
        }
        race.phase = Phase::Countdown;
        race.countdown = 3.;
        if std::env::var("ROADDASH_AI_SURVEY_SEGMENT").as_deref() == Ok("1") {
            race.phase = Phase::Racing;
        }
        next.set(crate::runtime::AppScreen::Race);
    }
    if state.frame.is_multiple_of(180) {
        let rows: Vec<_> = riders.iter().map(|(_, r, recovery)| serde_json::json!({
            "frame":state.frame, "elapsed":race.elapsed, "id":r.id, "route":r.route,
            "progress":r.progress, "lane":r.lane, "speed":r.speed, "finished":r.finished,
            "decision":format!("{:?}", r.combat_mind.decision), "target":r.combat_mind.target_id(),
            "combat_desired_speed":r.combat_mind.desired_speed,
            "plan":r.ai_plan.map(|p| serde_json::json!({"lane":p.lane,"speed":p.speed,"avoiding":p.avoiding})),
            "passing":r.ai_pass.active(), "recovery":format!("{:?}", recovery.stage),
            "throttle":r.drive_intent.throttle_amount, "brake":r.drive_intent.brake_amount,
            "throttle_held":r.drive_intent.throttle, "brake_held":r.drive_intent.brake,
            "effective_throttle":r.drive_intent.throttle_strength(),
            "effective_brake":r.drive_intent.brake_strength(),
            "escape_active":r.ai_escape.active(), "escape_reverse":r.ai_escape.reverse,
        })).collect();
        state.rows.extend(rows);
    }
    if state.frame.is_multiple_of(3600) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/racer-ai-{}.png",
                state.frame
            )));
    }
    let limit = std::env::var("ROADDASH_AI_SURVEY_FRAMES")
        .ok()
        .and_then(|x| x.parse::<u32>().ok())
        .unwrap_or(21600)
        .clamp(180, 36000);
    let segment = std::env::var("ROADDASH_AI_SURVEY_SEGMENT").as_deref() == Ok("1");
    let segment_passed = segment
        && riders
            .iter()
            .any(|(_, r, _)| r.id == 0 && r.progress >= track.race_interval().finish);
    let completed = slot_zero_finished(riders.iter().map(|(_, r, _)| r));
    if completed || segment_passed || state.frame >= limit {
        let finished: Vec<_> = riders
            .iter()
            .filter(|(_, r, _)| r.finished.is_some())
            .map(|(_, r, _)| r.id)
            .collect();
        let report = serde_json::json!({"frames":state.frame, "elapsed":race.elapsed,"finished_ids":finished,
            "completed":completed,"samples":state.rows,
            "hold_w_player":std::env::var("ROADDASH_AI_SURVEY_HOLD_W").as_deref()==Ok("1"),
            "synthetic_segment":segment,"segment_passed":segment_passed,
            "limits":"Scripted native race; no human balance or broad hardware certification."});
        std::fs::write(
            "screenshots/racer-ai.json",
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        println!(
            "Racer AI native race receipt: frames={} completed={}",
            state.frame, completed
        );
        exit.write(AppExit::Success);
    }
}

fn slot_zero_finished<'a>(mut riders: impl Iterator<Item = &'a Rider>) -> bool {
    riders.any(|r| r.id == 0 && r.finished.is_some())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ai_slot_finish_is_detected_without_offline_results_phase() {
        let mut rider = Rider::new(0);
        rider.networked = true;
        rider.remote_human = false;
        assert!(!slot_zero_finished(std::iter::once(&rider)));
        rider.finished = Some(300.);
        assert!(slot_zero_finished(std::iter::once(&rider)));
        rider.id = 1;
        assert!(!slot_zero_finished(std::iter::once(&rider)));
    }
}
