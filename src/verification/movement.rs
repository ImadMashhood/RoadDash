//! Opt-in native reverse controls receipt; real input/vehicle paths, isolated save.
use super::*;
use crate::vehicle::{BikeState, VehiclePhysicsAdapter};
use bevy_rapier3d::prelude::Velocity;
#[derive(Default)]
pub(crate) struct Review {
    frame: u32,
    start: Quat,
    previous: Option<Quat>,
    samples: Vec<serde_json::Value>,
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn review(
    mut commands: Commands,
    mut state: Local<Review>,
    mut race: ResMut<Race>,
    track: Res<Track>,
    adapter: Res<VehiclePhysicsAdapter>,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &Recovery,
        &mut Transform,
        Option<&mut BikeState>,
        Option<&mut Velocity>,
    )>,
    mut exit: MessageWriter<AppExit>,
) {
    state.frame += 1;
    let frame = state.frame;
    let phase = frame.saturating_sub(90) % 420;
    let side = if frame < 510 {
        KeyCode::KeyD
    } else {
        KeyCode::KeyA
    };
    let sign = if side == KeyCode::KeyD { 1. } else { -1. };
    if frame < 90 {
        return;
    }
    for (entity, mut r, rec, mut t, mut bike, mut velocity) in &mut riders {
        if r.id != 0 {
            if frame == 90 {
                commands.entity(entity).despawn();
            }
            continue;
        }
        if phase == 0 {
            let (p, q) = track.sample(2800., -2.);
            r.progress = 2800.;
            r.lane = -2.;
            r.speed = 0.;
            r.immunity = 1000.;
            r.motion_velocity = Vec3::ZERO;
            r.drive_intent = default();
            r.steering = default();
            *t = Transform::from_translation(p).with_rotation(q);
            if let Some(ref mut v) = velocity {
                **v = Velocity::zero();
            }
            if let Some(ref mut b) = bike {
                **b = default();
            }
            state.start = q;
            state.previous = None;
            race.phase = Phase::Racing;
            for key in [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyD] {
                smoke_key(&mut commands, key, false);
            }
            smoke_key(&mut commands, KeyCode::KeyS, true);
        }
        if phase == 150 {
            smoke_key(&mut commands, side, true);
        }
        if phase == 210 {
            smoke_key(&mut commands, KeyCode::KeyS, false);
            smoke_key(&mut commands, KeyCode::KeyW, true);
        }
        if phase == 300 {
            smoke_key(&mut commands, side, false);
            smoke_key(&mut commands, KeyCode::KeyW, false);
        }
        let signed = if *adapter == VehiclePhysicsAdapter::Dynamic {
            velocity
                .as_ref()
                .expect("dynamic velocity")
                .linvel
                .dot(t.rotation * Vec3::Z)
        } else {
            r.speed
        };
        let heading = state.start.inverse() * t.rotation * Vec3::Z;
        if phase == 205 {
            assert!(signed < -0.5, "native bike must actually reverse");
            assert!(
                heading.x * sign > 0.005,
                "native reverse heading wrong: {heading:?}"
            );
        }
        if (160..300).contains(&phase) {
            if let Some(previous) = state.previous {
                assert!(previous.angle_between(t.rotation) < 0.08, "heading pop");
            }
            assert_eq!(rec.stage, RecoveryStage::Riding);
        }
        state.previous = Some(t.rotation);
        if phase.is_multiple_of(4) && (140..330).contains(&phase) {
            capture(&mut commands, &format!("reverse-{frame:04}"));
            state.samples.push(serde_json::json!({"frame":frame,"key":format!("{side:?}"),"signed_speed_mps":signed,"local_heading_x":heading.x,"position":t.translation.to_array()}));
        }
    }
    if frame == 925 {
        assert!(
            state.samples.len() > 80,
            "both native steering cases must produce observations"
        );
        std::fs::write("screenshots/reverse-receipt.json", serde_json::to_vec_pretty(&serde_json::json!({"adapter":format!("{adapter:?}"),"samples":state.samples,"limits":"scripted native control fixture; human feel/hardware acceptance open"})).unwrap()).unwrap();
        exit.write(AppExit::Success);
    }
}
