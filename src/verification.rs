//! Scripted render check: `cargo run -- --smoke-test`.
use crate::game::{Phase, Race, Rider};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

pub fn smoke_test(
    mut commands: Commands,
    mut frame: Local<u32>,
    mut race: ResMut<Race>,
    track: Res<crate::track::Track>,
    mut riders: Query<&mut Rider>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        30 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshots/01-garage.png"));
        }
        60 => {
            race.phase = Phase::Racing;
            keys.press(KeyCode::KeyW);
        }
        240 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshots/02-racing.png"));
        }
        260 => {
            for mut rider in &mut riders {
                if rider.id == 0 {
                    rider.health = 0.0;
                }
            }
        }
        280 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshots/03-crash.png"));
        }
        310 => {
            race.phase = Phase::Racing;
            for mut rider in &mut riders {
                if rider.id == 0 {
                    rider.crashed = 0.0;
                    rider.progress = track.length * crate::track::LAPS as f32 + 1.0;
                    rider.health = 100.0;
                }
            }
        }
        330 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshots/04-results.png"));
        }
        360 => {
            info!("Render smoke test completed");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
