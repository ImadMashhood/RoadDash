use super::*;
use std::time::Duration;
fn camera_app(fps: u32) -> (App, Entity) {
    let mut app = tests::simulation();
    for _ in 0..6 {
        app.update();
    }
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_secs_f64(1.0 / fps as f64),
    ));
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
    let e = app
        .world_mut()
        .spawn((
            ChaseCamera,
            Transform::from_translation(p + q * Vec3::new(0.0, 3.0, -6.0)),
            Projection::Perspective(default()),
        ))
        .id();
    app.add_systems(PostUpdate, camera);
    (app, e)
}
#[test]
fn interpolated_camera_is_finite_and_continuous_at_120_and_144_hz() {
    let mut end = vec![];
    for fps in [60, 120, 144] {
        let (mut app, camera) = camera_app(fps);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        let mut previous = app.world().get::<Transform>(camera).unwrap().translation;
        for tick in 0..fps * 5 {
            if tick == fps * 2 {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(KeyCode::KeyD);
            }
            if tick == fps * 3 {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .release(KeyCode::KeyD);
            }
            app.update();
            let pose = app.world().get::<Transform>(camera).unwrap();
            assert!(pose.translation.is_finite() && pose.rotation.is_finite());
            assert!(
                pose.translation.distance(previous) < 1.5,
                "camera discontinuity at {fps} Hz tick {tick}: {:?} -> {:?}",
                previous,
                pose.translation
            );
            previous = pose.translation;
        }
        end.push(previous);
    }
    assert!(end[0].distance(end[1]) < 1.5 && end[1].distance(end[2]) < 1.5);
}
#[test]
fn camera_sphere_stays_in_front_of_solid_obstacles_and_comfort_fov_is_clamped() {
    let (mut app, e) = camera_app(144);
    let rider = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let t = *app.world().get::<Transform>(rider).unwrap();
    let focus = t.translation + Vec3::Y * 1.2;
    let (_, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
    let wall = focus - q * Vec3::Z * 3.0;
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::cuboid(8.0, 5.0, 0.15),
        Transform::from_translation(wall).with_rotation(q),
    ));
    let mut p = app
        .world_mut()
        .resource_mut::<crate::settings::Preferences>();
    p.file.comfort.fov_degrees = 95.0;
    for _ in 0..144 {
        app.update();
    }
    let camera = app.world().get::<Transform>(e).unwrap();
    assert!(
        (camera.translation - focus).dot(-(q * Vec3::Z)) < 2.7,
        "camera clipped through wall: focus {focus:?}, camera {:?}",
        camera.translation
    );
    let Projection::Perspective(proj) = app.world().get::<Projection>(e).unwrap() else {
        panic!()
    };
    assert!((proj.fov.to_degrees() - 95.0).abs() < 0.001);
}
#[test]
fn retry_style_warp_snaps_camera_to_the_new_interpolated_pose() {
    let (mut app, e) = camera_app(120);
    let old = app.world().get::<Transform>(e).unwrap().translation;
    {
        let world = app.world_mut();
        let mut q = world.query::<&mut Rider>();
        q.single_mut(world).unwrap().progress = 800.0;
    }
    for _ in 0..4 {
        app.update();
    }
    let world = app.world_mut();
    let player = world
        .query_filtered::<&Transform, With<Rider>>()
        .single(world)
        .unwrap()
        .translation;
    let camera = world.get::<Transform>(e).unwrap().translation;
    assert!(old.distance(player) > 100.0);
    assert!(
        camera.distance(player) < 30.0,
        "camera flew through the old world {camera:?} -> {player:?}"
    );
}

#[test]
fn every_chase_view_stays_close_at_rest_and_high_speed_with_slow_smoothing() {
    for fps in [60, 120, 144] {
        for (mode, limit) in [(0, 5.20), (1, 3.93), (2, 6.68)] {
            for speed in [0.0, 30.0, 80.0] {
                let (mut app, camera) = camera_app(fps);
                app.world_mut().resource_mut::<Race>().camera_mode = mode;
                app.world_mut()
                    .resource_mut::<crate::settings::Preferences>()
                    .file
                    .comfort
                    .smoothing = 2.0;
                for _ in 0..fps {
                    {
                        let world = app.world_mut();
                        world.query::<&mut Rider>().single_mut(world).unwrap().speed = speed;
                    }
                    app.update();
                    let world = app.world_mut();
                    let fixed = world.resource::<Time<Fixed>>().overstep_fraction();
                    let (t, h) = world
                        .query_filtered::<(&Transform, &crate::runtime::PoseHistory), With<Rider>>()
                        .single(world)
                        .unwrap();
                    let focus =
                        crate::runtime::interpolated(t, Some(h), fixed).translation + Vec3::Y * 1.2;
                    let distance = world
                        .get::<Transform>(camera)
                        .unwrap()
                        .translation
                        .distance(focus);
                    assert!(
                        distance <= limit,
                        "mode {mode}, speed {speed}, {fps} Hz zoomed to {distance}"
                    );
                }
            }
        }
    }
}

#[test]
fn rival_in_chase_boom_does_not_pull_camera_into_player() {
    let (mut app, camera) = camera_app(60);
    for _ in 0..15 {
        app.update();
    }
    let before = app.world().get::<Transform>(camera).unwrap().translation;
    let player = app
        .world_mut()
        .query_filtered::<&Transform, With<Rider>>()
        .single(app.world())
        .unwrap()
        .translation;
    app.world_mut().spawn((
        Rider::new(7),
        Transform::from_translation(before.lerp(player, 0.5)),
        Visibility::Visible,
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::ball(1.0),
    ));
    for _ in 0..10 {
        app.update();
    }
    let after = app.world().get::<Transform>(camera).unwrap().translation;
    assert!(
        after.distance(player) > 4.0,
        "rival collapsed chase boom: {after:?}"
    );
}

#[test]
fn low_ceiling_and_nearby_detached_bodies_do_not_collapse_or_flip_the_camera() {
    for fps in [60, 144] {
        for recovering in [false, true] {
            let (mut app, camera) = camera_app(fps);
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let t = *app.world().get::<Transform>(owner).unwrap();
            let focus = if recovering {
                let runner = app
                    .world_mut()
                    .spawn(Transform::from_translation(t.translation + Vec3::Y * 0.95))
                    .id();
                let mut rec = app.world_mut().get_mut::<Recovery>(owner).unwrap();
                rec.stage = RecoveryStage::GettingUp;
                rec.runner = Some(runner);
                t.translation + Vec3::Y * 1.25
            } else {
                t.translation + Vec3::Y * 1.2
            };
            app.world_mut().spawn((
                RigidBody::Fixed,
                Collider::cuboid(8.0, 0.2, 10.0),
                Transform::from_translation(focus + Vec3::Y * 0.65),
            ));
            app.world_mut().spawn((
                RigidBody::Fixed,
                Collider::ball(0.8),
                Transform::from_translation(focus - Vec3::Z * 2.0),
                crate::recovery::OwnedBody {
                    owner: Entity::PLACEHOLDER,
                    kind: crate::recovery::BodyKind::Ragdoll,
                },
            ));
            for _ in 0..fps {
                app.update();
                let pose = app.world().get::<Transform>(camera).unwrap();
                assert!(pose.rotation.is_finite() && pose.translation.is_finite());
                assert!(
                    (pose.translation - focus).xz().length() > 2.0,
                    "camera collapsed: {pose:?}"
                );
                assert!(
                    (pose.rotation * Vec3::Y).dot(Vec3::Y) > 0.5,
                    "camera flipped"
                );
            }
        }
    }
}

#[test]
fn on_foot_camera_heading_does_not_snap_when_nearest_road_tangent_changes() {
    let (mut app, camera) = camera_app(60);
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let origin = app.world().get::<Transform>(owner).unwrap().translation;
    let runner = app
        .world_mut()
        .spawn(Transform::from_translation(origin + Vec3::Y * 0.95))
        .id();
    {
        let mut r = app.world_mut().get_mut::<Recovery>(owner).unwrap();
        r.stage = RecoveryStage::GettingUp;
        r.runner = Some(runner);
    }
    for _ in 0..60 {
        app.update();
    }
    let before = *app.world().get::<Transform>(camera).unwrap();
    app.world_mut().get_mut::<Rider>(owner).unwrap().progress = 600.0;
    for _ in 0..30 {
        app.update();
    }
    let after = *app.world().get::<Transform>(camera).unwrap();
    assert!(after.translation.distance(before.translation) < 0.1);
    assert!(after.rotation.angle_between(before.rotation) < 0.02);
}
