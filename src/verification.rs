//! Render the actual crash/get-up/run/remount sequence, then verify results.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    track::{LAPS, Track},
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

#[derive(Default)]
pub struct SmokeProgress {
    frame: u32,
    getting_up: bool,
    running: bool,
    mounting: bool,
    finished_at: Option<u32>,
    results: bool,
}
fn capture(commands: &mut Commands, name: &str) {
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(format!("screenshots/{name}.png")));
}
#[allow(clippy::too_many_arguments)]
pub fn smoke_test(
    mut commands: Commands,
    mut state: Local<SmokeProgress>,
    mut race: ResMut<Race>,
    track: Res<Track>,
    mut riders: Query<(&mut Rider, &Recovery)>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
    sinks: Query<&AudioSink>,
) {
    state.frame += 1;
    match state.frame {
        30 => {
            assert!(sinks.iter().count() >= 4, "audio output did not initialize");
            println!(
                "Audio output initialized: {} playback sinks",
                sinks.iter().count()
            );
            capture(&mut commands, "01-garage");
        }
        60 => {
            race.phase = Phase::Racing;
            keys.press(KeyCode::KeyW);
        }
        240 => {
            assert!(
                sinks
                    .iter()
                    .any(|s| !s.is_paused() && s.volume().to_linear() > 0.0),
                "no racing audio playing"
            );
            capture(&mut commands, "02-racing");
        }
        260 => {
            for (mut rider, _) in &mut riders {
                if rider.id == 0 {
                    rider.health = 0.0;
                }
            }
            keys.release(KeyCode::KeyW);
        }
        280 => capture(&mut commands, "03-crash"),
        _ => {}
    }
    for (mut rider, recovery) in &mut riders {
        if rider.id != 0 || state.frame <= 280 {
            continue;
        }
        match recovery.stage {
            RecoveryStage::GettingUp if !state.getting_up && recovery.elapsed > 0.25 => {
                state.getting_up = true;
                capture(&mut commands, "04-getting-up");
            }
            RecoveryStage::Running if !state.running && recovery.elapsed > 0.25 => {
                state.running = true;
                capture(&mut commands, "05-running-to-bike");
            }
            RecoveryStage::Mounting if !state.mounting && recovery.elapsed > 0.25 => {
                state.mounting = true;
                capture(&mut commands, "06-remounting");
            }
            RecoveryStage::Riding if state.mounting && state.finished_at.is_none() => {
                assert!(
                    state.getting_up && state.running,
                    "recovery skipped a stage"
                );
                rider.progress = track.length * LAPS as f32 + 1.0;
                state.finished_at = Some(state.frame);
            }
            _ => {}
        }
    }
    if let Some(finished_at) = state.finished_at {
        if state.frame > finished_at + 20 && !state.results {
            assert!(race.phase == Phase::Finished);
            state.results = true;
            capture(&mut commands, "07-results");
        }
        if state.frame > finished_at + 40 {
            println!("Render smoke test passed: crash -> get up -> run -> remount -> finish");
            exit.write(AppExit::Success);
        }
    }
    assert!(
        state.frame < 3000,
        "render smoke test timed out during recovery"
    );
}
