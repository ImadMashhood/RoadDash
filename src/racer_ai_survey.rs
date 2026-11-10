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
}
#[allow(clippy::too_many_arguments)]
fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    mut next: ResMut<NextState<crate::runtime::AppScreen>>,
    mut race: ResMut<Race>,
    mut riders: Query<(&mut Rider, &Recovery)>,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    if state.frame == 1 {
        for (mut rider, _) in &mut riders {
            if rider.id == 0 {
                rider.networked = true;
                rider.remote_human = false;
            }
        }
        race.phase = Phase::Countdown;
        race.countdown = 3.;
        next.set(crate::runtime::AppScreen::Race);
    }
    if state.frame.is_multiple_of(180) {
        let rows: Vec<_> = riders.iter().map(|(r, recovery)| serde_json::json!({
            "frame":state.frame, "elapsed":race.elapsed, "id":r.id, "route":r.route,
            "progress":r.progress, "lane":r.lane, "speed":r.speed, "finished":r.finished,
            "decision":format!("{:?}", r.combat_mind.decision), "target":r.combat_mind.target_id(),
            "combat_desired_speed":r.combat_mind.desired_speed,
            "plan":r.ai_plan.map(|p| serde_json::json!({"lane":p.lane,"speed":p.speed,"avoiding":p.avoiding})),
            "passing":r.ai_pass.active(), "recovery":format!("{:?}", recovery.stage),
            "throttle":r.drive_intent.throttle_amount, "brake":r.drive_intent.brake_amount,
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
    let completed = slot_zero_finished(riders.iter().map(|(r, _)| r));
    if completed || state.frame >= limit {
        let finished: Vec<_> = riders
            .iter()
            .filter(|(r, _)| r.finished.is_some())
            .map(|(r, _)| r.id)
            .collect();
        let report = serde_json::json!({"frames":state.frame, "elapsed":race.elapsed,"finished_ids":finished,
            "completed":completed,"samples":state.rows,
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
