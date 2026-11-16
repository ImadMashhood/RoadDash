//! Matched screenshot fixture for the opt-in presentation experiments.
use super::{GridEngine, Options, Puff, RoadEffects};
use crate::{
    game::{ChaseCamera, Phase, Race, Rider},
    runtime::AppScreen,
    track::Track,
    vehicle::BikeState,
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};
use bevy_rapier3d::prelude::Velocity;

#[derive(Resource)]
struct Survey {
    shot: String,
    frame: u32,
    written: bool,
}

pub(crate) fn install(app: &mut App) {
    let shot = std::env::var("ROADDASH_POLISH_CAPTURE").unwrap_or_else(|_| "road".into());
    assert!(
        matches!(shot.as_str(), "road" | "grid" | "speed" | "brake"),
        "polish capture must be road, grid, speed or brake"
    );
    app.insert_resource(Survey {
        shot,
        frame: 0,
        written: false,
    })
    .insert_resource(bevy::winit::WinitSettings::continuous())
    .add_systems(
        FixedUpdate,
        fixture_shift
            .after(crate::runtime::SimSet::AfterPhysics)
            .before(super::shake),
    )
    .add_systems(
        PostUpdate,
        capture
            .after(crate::game::camera)
            .before(super::camera_effects)
            .before(bevy::transform::TransformSystems::Propagate),
    );
}

fn saved(_: On<ScreenshotCaptured>, mut survey: ResMut<Survey>) {
    survey.written = true;
}

fn fixture_shift(survey: Res<Survey>, mut bikes: Query<(&Rider, &mut BikeState)>) {
    if survey.shot == "speed"
        && survey.frame == 68
        && let Some((_, mut bike)) = bikes.iter_mut().find(|(rider, _)| rider.id == 0)
    {
        // Expose the previous gear after the drivetrain runs. The following
        // fixed tick restores the active gear and triggers the real camera cue.
        bike.gear = 0;
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn capture(
    mut commands: Commands,
    state: Res<State<AppScreen>>,
    mut next: ResMut<NextState<AppScreen>>,
    mut survey: ResMut<Survey>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    rev: Option<Res<GridEngine>>,
    options: Res<Options>,
    effects: Query<&RoadEffects>,
    puffs: Query<&Visibility, With<Puff>>,
    mut riders: Query<
        (
            &mut Rider,
            &mut Transform,
            Option<&mut Velocity>,
            Option<&mut BikeState>,
        ),
        Without<ChaseCamera>,
    >,
    mut camera: Query<&mut Transform, With<ChaseCamera>>,
    brake_visuals: Query<
        (Has<crate::map_fx::braking::SkidMark>, &Visibility),
        Or<(
            With<crate::map_fx::braking::SkidMark>,
            With<crate::map_fx::braking::BrakePuff>,
        )>,
    >,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(state.get(), AppScreen::Boot | AppScreen::Loading) {
        return;
    }
    if survey.written {
        // save_to_disk is also observing the event. Give its queued work a frame.
        if survey.frame >= 103 {
            exit.write(AppExit::Success);
        }
        survey.frame += 1;
        return;
    }
    survey.frame += 1;
    let Some((mut rider, mut pose, velocity, mut bike)) =
        riders.iter_mut().find(|(rider, _, _, _)| rider.id == 0)
    else {
        return;
    };
    let road = survey.shot == "road";
    let grid = survey.shot == "grid";
    let brake = survey.shot == "brake";
    let station = if road {
        4835.
    } else if grid {
        track.grid_station(track.definition.spawns[0].distance)
    } else {
        10480.
    };
    let lane = if grid {
        track.definition.spawns[0].lane
    } else {
        -1.6
    };
    let (target, frame) = track.sample_route(0, station, lane);
    if survey.frame == 1 {
        next.set(AppScreen::Race);
        race.phase = if grid || road {
            Phase::Countdown
        } else {
            Phase::Racing
        };
        race.countdown = 3.;
        rider.progress = station;
        rider.route = 0;
        rider.lane = lane;
        rider.immunity = 1000.;
        *pose = Transform::from_translation(target + Vec3::Y * if grid || road { 0. } else { 0.4 })
            .with_rotation(frame);
        if !(grid || road) {
            rider.speed = 48.;
            rider.motion_velocity = frame * Vec3::Z * 48.;
            if let Some(mut velocity) = velocity {
                velocity.linear = rider.motion_velocity;
            }
            if let Some(bike) = bike.as_deref_mut() {
                bike.position = pose.translation;
                bike.velocity = rider.motion_velocity;
                bike.impact_velocity = rider.motion_velocity;
                bike.yaw = frame.to_euler(EulerRot::YXZ).0;
                bike.was_riding = true;
            }
        }
    }
    if brake && survey.frame >= 34 {
        keys.release(KeyCode::KeyW);
        keys.press(KeyCode::KeyS);
    } else if !road {
        keys.press(KeyCode::KeyW);
    }
    if road && let Ok(mut camera) = camera.single_mut() {
        *camera = Transform::from_translation(target + frame * Vec3::new(1.3, 1.6, -5.8))
            .looking_at(target + frame * Vec3::new(0., 0.4, 16.), Vec3::Y);
    } else if grid && let Ok(mut camera) = camera.single_mut() {
        *camera = Transform::from_translation(
            pose.translation + pose.rotation * Vec3::new(2.1, 1.55, -4.9),
        )
        .looking_at(pose.translation + Vec3::Y * 0.95, Vec3::Y);
    }
    if brake && let Ok(mut camera) = camera.single_mut() {
        let heading = pose.rotation * Vec3::Z;
        let eye = pose.translation - heading * 10. + pose.rotation * Vec3::X * 4. + Vec3::Y * 3.;
        *camera = Transform::from_translation(eye)
            .looking_at(pose.translation - heading * 3. + Vec3::Y * 0.8, Vec3::Y);
    }
    if survey.frame == 70 {
        let brake_count = |mark| {
            brake_visuals
                .iter()
                .filter(|(is_mark, visibility)| {
                    *is_mark == mark && **visibility != Visibility::Hidden
                })
                .count()
        };
        let receipt = serde_json::json!({
            "shot":survey.shot,
            "station":rider.progress,
            "speed_mps":rider.speed,
            "phase":format!("{:?}", race.phase),
            "rev_rpm":rev.as_ref().map(|rev| rev.0),
            "brake_marks":brake_count(true),
            "brake_smoke":brake_count(false),
            "puffs":puffs.iter().filter(|visibility| **visibility != Visibility::Hidden).count(),
            "shader_weights":effects.iter().map(|effect| effect.weights.to_array()).next(),
            "ground_shimmer":effects.iter().map(|effect| effect.ground_strength.x).next().unwrap_or(0.),
            "hanabi":options.hanabi,
            "shake":race.shake,
        });
        std::fs::write(
            "screenshots/polish.json",
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("screenshots/polish.png"))
            .observe(saved);
    }
}
