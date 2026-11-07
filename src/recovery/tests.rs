use super::*;
use bevy::ecs::system::RunSystemOnce;
fn foot_scene(distance: f32, bike_offset: f32) -> (App, Entity, Entity, Entity, Quat) {
    let mut app = crate::game::tests::simulation();
    app.insert_resource(
        crate::track::Track::from_json(include_str!("../../content/tracks/city5300.json")).unwrap(),
    );
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let track = app.world().resource::<Track>();
    let d = if distance < 0.0 {
        track.length + distance
    } else {
        distance
    };
    let (p, q) = track.sample(d, 0.0);
    let (bike_p, _) = track.sample(d + bike_offset, 0.0);
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.5, 100.0),
        crate::world::Ground,
        Transform::from_translation(p - Vec3::Y * 0.5),
    ));
    let camera = Transform::from_translation(p + Vec3::Y * 3.0)
        .with_rotation(q * Quat::from_rotation_y(std::f32::consts::PI));
    app.world_mut()
        .spawn((Camera3d::default(), camera, GlobalTransform::from(camera)));
    let (runner, pose) = app
        .world_mut()
        .run_system_once(|mut c: Commands, art: Res<Art>| world::runner(&mut c, &art, 0))
        .unwrap();
    let runner_t = Transform::from_translation(p + Vec3::Y * 0.98).with_rotation(q);
    app.world_mut().entity_mut(runner).insert((
        runner_t,
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.65, 0.28),
        KinematicCharacterController {
            translation: Some(Vec3::ZERO),
            ..default()
        },
        OwnedBody {
            owner,
            kind: BodyKind::Runner,
        },
        crate::game::Debris(f32::INFINITY),
    ));
    let wreck = app
        .world_mut()
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(0.38, 0.6, 1.1),
            Transform::from_translation(bike_p + Vec3::Y * 0.6).with_rotation(q),
            Visibility::default(),
            OwnedBody {
                owner,
                kind: BodyKind::Wreck,
            },
        ))
        .id();
    app.world_mut().entity_mut(owner).insert(ColliderDisabled);
    {
        let w = app.world_mut();
        let mut r = w.get_mut::<Rider>(owner).unwrap();
        r.progress = d;
        r.lane = 0.0;
        r.speed = 0.0;
        r.crashed = 1.0;
        let mut rec = w.get_mut::<Recovery>(owner).unwrap();
        rec.stage = RecoveryStage::Running;
        rec.runner = Some(runner);
        rec.pose = Some(pose);
        rec.wreck = Some(wreck);
        rec.previous_foot = Some(runner_t.translation);
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    (app, owner, runner, wreck, q)
}
fn step(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}
#[test]
fn manual_movement_is_camera_relative_and_blocks_unwanted_nearby_mounts() {
    for key in [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD] {
        let (mut app, owner, runner, _, q) = foot_scene(100.0, 10.0);
        let start = app.world().get::<Transform>(runner).unwrap().translation;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        step(&mut app, 30);
        let end = app.world().get::<Transform>(runner).unwrap().translation;
        let local = q.inverse() * (end - start);
        let projection = match key {
            KeyCode::KeyW => local.z,
            KeyCode::KeyS => -local.z,
            KeyCode::KeyA => local.x,
            _ => -local.x,
        };
        assert!(projection > 0.5, "key={key:?} movement={local:?}");
        let rec = app.world().get::<Recovery>(owner).unwrap();
        assert_eq!(rec.stage, RecoveryStage::Running);
        assert_eq!(rec.foot.mode, rr_core::recovery::FootMode::Manual);
    }
}
#[test]
fn walking_away_beside_the_bike_does_not_snap_or_force_a_mount() {
    let (mut app, owner, runner, _, q) = foot_scene(100.0, 1.7);
    let start = app.world().get::<Transform>(runner).unwrap().translation;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    step(&mut app, 30);
    let end = app.world().get::<Transform>(runner).unwrap().translation;
    assert!((end - start).dot(q * Vec3::Z) < -1.0);
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Running
    );
}
#[test]
fn neutral_delay_and_disabled_assist_do_not_force_mounts() {
    let (mut app, owner, _, _, _) = foot_scene(100.0, -20.0);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    step(&mut app, 5);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyD);
    step(&mut app, 10);
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().foot.mode,
        rr_core::recovery::FootMode::Manual
    );
    step(&mut app, 20);
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().foot.mode,
        rr_core::recovery::FootMode::AutoSeekBike
    );
    app.world_mut().resource_mut::<RecoveryTuning>().0.auto_seek = false;
    step(&mut app, 120);
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().foot.mode,
        rr_core::recovery::FootMode::Manual
    );
}
#[test]
fn running_away_from_the_bike_can_finish_once_with_valid_prior_gates() {
    for skipped in [false, true] {
        let (mut app, owner, runner, _, _) = foot_scene(-4.0, -20.0);
        if !skipped {
            let last = app.world().resource::<Track>().definition.checkpoints.len() - 1;
            app.world_mut().get_mut::<Rider>(owner).unwrap().gates.next = last;
        }
        let before = app.world().resource::<crate::game::Profile>().races;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        step(&mut app, 100);
        let r = app.world().get::<Rider>(owner).unwrap();
        if skipped {
            assert!(r.finished.is_none());
            assert!(!r.finished_on_foot);
            assert_eq!(app.world().resource::<crate::game::Profile>().races, before);
        } else {
            assert!(r.finished_on_foot && r.finished.is_some());
            assert_eq!(app.world().resource::<Race>().phase, Phase::Finished);
            assert!(app.world().get::<ColliderDisabled>(runner).is_some());
            assert_eq!(
                app.world().resource::<crate::game::Profile>().races,
                before + 1
            );
            step(&mut app, 120);
            assert_eq!(
                app.world().resource::<crate::game::Profile>().races,
                before + 1
            );
        }
    }
}
#[test]
fn manual_below_terrain_rescue_does_not_cross_the_finish_or_teleport_the_bike() {
    let (mut app, owner, runner, wreck, _) = foot_scene(-2.0, -20.0);
    let before = *app.world().get::<Transform>(wreck).unwrap();
    app.world_mut()
        .get_mut::<Transform>(runner)
        .unwrap()
        .translation
        .y -= 20.0;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    step(&mut app, 1);
    assert!(app.world().get::<Transform>(runner).unwrap().translation.y > -1.0);
    assert_eq!(
        app.world().get::<Transform>(wreck).unwrap().translation,
        before.translation
    );
    assert!(app.world().get::<Rider>(owner).unwrap().finished.is_none());
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Running
    );
}

#[test]
fn bike_and_foot_finish_attempts_share_exactly_one_award() {
    let mut r = Rider::new(0);
    r.gates.completed_laps = crate::track::LAPS;
    let mut race = Race {
        phase: Phase::Racing,
        elapsed: 20.0,
        ..default()
    };
    let mut p = crate::game::Profile::default();
    let n = p.races;
    assert!(crate::game::finish_rider(
        &mut r, &mut race, &mut p, false, 1, true
    ));
    let credits = p.credits;
    assert!(!crate::game::finish_rider(
        &mut r, &mut race, &mut p, false, 1, false
    ));
    assert_eq!(p.races, n + 1);
    assert_eq!(p.credits, credits);
}
#[test]
fn slow_facing_interaction_mounts_but_a_moving_bike_is_rejected() {
    for speed in [0.0, 5.0] {
        let (mut app, owner, _, wreck, _) = foot_scene(100.0, 1.5);
        app.world_mut().resource_mut::<RecoveryTuning>().0.auto_seek = false;
        app.world_mut()
            .entity_mut(wreck)
            .insert(Velocity::linear(Vec3::Z * speed));
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        step(&mut app, 1);
        let rec = app.world().get::<Recovery>(owner).unwrap();
        assert_eq!(
            rec.stage,
            if speed == 0.0 {
                RecoveryStage::Mounting
            } else {
                RecoveryStage::Running
            }
        );
    }
}

#[test]
fn actual_crash_five_meters_before_finish_allows_a_manual_foot_finish() {
    for enabled in [false, true] {
        let (mut app, owner, runner, wreck, q) = foot_scene(-5.0, -20.0);
        app.world_mut().despawn(runner);
        app.world_mut().despawn(wreck);
        let (p, last) = {
            let t = app.world().resource::<Track>();
            (
                t.sample(t.length - 5.0, 0.0).0,
                t.definition.checkpoints.len() - 1,
            )
        };
        app.world_mut()
            .entity_mut(owner)
            .remove::<ColliderDisabled>();
        app.world_mut().entity_mut(owner).insert((
            Recovery::default(),
            Transform::from_translation(p).with_rotation(q),
        ));
        {
            let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
            r.health = 0.0;
            r.crashed = 0.0;
            r.gates.next = last;
        }
        app.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = enabled;
        app.world_mut().resource_mut::<RecoveryTuning>().0.auto_seek = false;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        let before = app.world().resource::<crate::game::Profile>().races;
        let mut seen = [false; 3];
        for _ in 0..1200 {
            app.update();
            let c = app.world().get::<Recovery>(owner).unwrap();
            match c.stage {
                RecoveryStage::Falling => seen[0] = true,
                RecoveryStage::GettingUp => seen[1] = true,
                RecoveryStage::Running => seen[2] = true,
                _ => {}
            }
            if app.world().resource::<Race>().phase == Phase::Finished {
                break;
            }
        }
        assert!(seen.into_iter().all(|v| v), "missing crash stage: {seen:?}");
        assert!(
            app.world().get::<Rider>(owner).unwrap().finished_on_foot,
            "enabled={enabled}"
        );
        assert_eq!(
            app.world().resource::<crate::game::Profile>().races,
            before + 1
        );
    }
}

#[test]
fn walking_backward_across_finish_is_rejected_even_with_prior_gates() {
    let (mut app, owner, runner, _, q) = foot_scene(-1.0, -20.0);
    let last = app.world().resource::<Track>().definition.checkpoints.len() - 1;
    app.world_mut().get_mut::<Rider>(owner).unwrap().gates.next = last;
    let start = app.world().get::<Transform>(runner).unwrap().translation + q * Vec3::Z * 4.0;
    app.world_mut()
        .get_mut::<Transform>(runner)
        .unwrap()
        .translation = start;
    app.world_mut()
        .get_mut::<Recovery>(owner)
        .unwrap()
        .previous_foot = Some(start);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    step(&mut app, 90);
    assert!(
        app.world()
            .get::<Transform>(runner)
            .unwrap()
            .translation
            .dot(q * Vec3::Z)
            < start.dot(q * Vec3::Z) - 4.0
    );
    assert!(app.world().get::<Rider>(owner).unwrap().finished.is_none());
}
