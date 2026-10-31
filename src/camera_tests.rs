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
