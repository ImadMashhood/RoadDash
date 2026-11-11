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
    for (speed, grade, cross, uneven) in [
        (2., 0., 0., false),
        (6., 0., 0., false),
        (2., 0.12, 0., false),
        (6., -0.12, 0., false),
        (6., 0.25, 0., false),
        (6., -0.25, 0., false),
        (6., 0., 0.2, false),
        (6., 0., 0., true),
    ] {
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
        let height = |x: f32, z: f32| {
            x * cross
                + z * grade
                + if uneven {
                    (z * 0.8).sin() * 0.18 + (z * 1.7).sin() * 0.035
                } else {
                    0.
                }
        };
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for n in -20..1000 {
            let z = n as f32 * 0.1;
            let base = vertices.len() as u32;
            for (x, z) in [(-5., z), (-5., z + 0.1), (5., z + 0.1), (5., z)] {
                vertices.push(Vec3::new(x, height(x, z), z));
            }
            indices.extend([[base, base + 1, base + 2], [base, base + 2, base + 3]]);
        }
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::trimesh(vertices, indices).unwrap(),
            Transform::default(),
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
            let body = Transform::from_xyz(0., 0.95 + height(0., distance), distance);
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
                if tick > 90 && moving {
                    let surface = height(sole.x, sole.z);
                    assert!(
                        sole.y > surface - 0.035,
                        "foot clips terrain: speed={speed} grade={grade} cross={cross} tick={tick} side={side} sole={sole:?} surface={surface}"
                    );
                }
                if tick == 359 {
                    assert!(
                        sole.y - height(sole.x, sole.z) < 0.12,
                        "idle foot remains suspended {sole:?}"
                    );
                }
            }
        }
        report.push((speed, grade, cross, uneven, max_slip, samples));
        assert!(samples > 10, "must measure real stance samples");
        assert!(max_slip < 0.006, "planted foot slip {report:?}");
    }
    println!("Actual translated support slip (speed,grade,max m/tick,samples): {report:?}");
}
