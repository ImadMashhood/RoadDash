use super::*;
use bevy::ecs::system::RunSystemOnce;
use bevy_rapier3d::prelude::*;
fn step(
    lib: Res<Library>,
    mut owners: Query<(&Recovery, &mut Animator)>,
    bodies: Query<&Transform>,
    context: ReadRapierContext,
) {
    for (rec, mut a) in &mut owners {
        let body = bodies.get(rec.runner.unwrap()).unwrap();
        a.gait.advance(
            rec.walk_distance,
            rec.walk_velocity,
            body.rotation,
            1. / 60.,
        );
        a.current = lib.directional_locomotion(a.gait.phase, a.gait.speed, a.gait.direction);
        grounding::apply(&lib, &mut a, rec, body, &context, 1. / 60.);
    }
}
#[test]
fn translated_walk_run_slope_and_stop_keep_contact_and_physical_root() {
    let mut report = Vec::new();
    for (speed, grade) in [(2., 0.), (6., 0.), (2., 0.12), (6., -0.12)] {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .init_resource::<Library>()
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ));
        let rotation = Quat::from_rotation_x(-f32::atan(grade));
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(10., 0.1, 100.),
            Transform::from_xyz(0., -0.1, 0.).with_rotation(rotation),
        ));
        let runner = app
            .world_mut()
            .spawn(Transform::from_xyz(0., 0.95, 0.))
            .id();
        let owner = app
            .world_mut()
            .spawn((
                Recovery {
                    runner: Some(runner),
                    stage: RecoveryStage::Running,
                    ..default()
                },
                Animator::default(),
            ))
            .id();
        for _ in 0..3 {
            app.update();
        }
        let mut previous = [None; 2];
        let mut max_slip = 0f32;
        let mut samples = 0;
        let mut distance = 0.;
        for tick in 0..360 {
            let moving = tick < 240;
            let velocity = if moving { speed } else { 0. };
            distance += velocity / 60.;
            let body = Transform::from_xyz(0., 0.95 + distance * grade, distance);
            *app.world_mut().get_mut::<Transform>(runner).unwrap() = body;
            {
                let mut rec = app.world_mut().get_mut::<Recovery>(owner).unwrap();
                rec.walk_distance = distance;
                rec.walk_velocity = Vec3::new(0., velocity * grade, velocity);
            }
            app.world_mut().run_system_once(step).unwrap();
            assert_eq!(
                *app.world().get::<Transform>(runner).unwrap(),
                body,
                "animation cannot move physical root"
            );
            let lib = app.world().resource::<Library>();
            let a = app.world().get::<Animator>(owner).unwrap();
            let frames = lib.world_pose(a.current);
            let anchor = body.mul_transform(Transform::from_xyz(0., -0.95, 0.));
            for limb in lib.data.limbs.iter().filter(|l| l.bit >= 2) {
                let side = (limb.bit - 2) as usize;
                let sole = anchor.transform_point(
                    frames[limb.end].transform_point(Vec3::from_array(limb.offset)),
                );
                let locked =
                    moving && tick > 90 && a.plant_stance[side] && a.plant_weight[side] > 0.98;
                if locked {
                    if let Some(last) = previous[side] {
                        max_slip = max_slip.max(sole.distance(last));
                        samples += 1;
                    }
                    previous[side] = Some(sole);
                } else {
                    previous[side] = None;
                }
                assert!(sole.is_finite());
                if tick == 359 {
                    assert!(
                        sole.y - (distance * grade) < 0.12,
                        "idle foot remains suspended {sole:?}"
                    );
                }
            }
        }
        report.push((speed, grade, max_slip, samples));
        assert!(samples > 10, "must measure real stance samples");
        assert!(max_slip < 0.006, "planted foot slip {report:?}");
    }
    println!("Actual translated support slip (speed,grade,max m/tick,samples): {report:?}");
}
