use super::*;
#[test]
fn comparison_launch_needs_no_keyboard_toggle_and_keeps_explicit_baseline() {
    let args = |values: &[&str]| values.iter().map(|v| (*v).to_owned()).collect::<Vec<_>>();
    assert!(Settings::launch_mode(&[], None).unwrap());
    assert!(Settings::launch_mode(&[], Some("biomechanical")).unwrap());
    assert!(!Settings::launch_mode(&[], Some("baseline")).unwrap());
    assert!(!Settings::launch_mode(&args(&["--ragdoll-baseline"]), None).unwrap());
    assert!(!Settings::launch_mode(&args(&["--ragdoll-baseline"]), Some("biomechanical")).unwrap());
    assert!(Settings::launch_mode(&args(&["--ragdoll-biomechanical"]), Some("baseline")).unwrap());
    assert!(
        Settings::launch_mode(
            &args(&["--ragdoll-baseline", "--ragdoll-biomechanical"]),
            None
        )
        .is_err()
    );
    assert!(
        !Settings::default().enabled,
        "physics control fixtures keep their baseline"
    );
}
#[test]
fn every_joint_keeps_limits_and_passive_motors_at_zero_activation() {
    let config = Settings::default().config;
    for i in 1..16 {
        let j = joint(&config, i);
        assert!(!j.contacts_enabled());
        for (axis, range) in AXES.into_iter().zip(config.joints[i].limits) {
            if range == [0.0, 0.0] {
                continue;
            }
            let limit = j.limits(axis).unwrap();
            assert_eq!([limit.min, limit.max], range);
            let motor = j.motor(axis).unwrap();
            assert!(motor.stiffness > 0.0 && motor.damping > 0.0 && motor.max_force > 0.0);
        }
    }
}
#[test]
fn quaternion_chart_is_hemisphere_invariant() {
    let q = Quat::from_rotation_x(1.3);
    assert!(coordinates(q).distance(coordinates(-q)) < 1e-5);
    assert!((coordinates(q).x - 1.3).abs() < 1e-5);
}
#[test]
fn ejection_seed_preserves_com_momentum_and_existing_angular_velocity() {
    let mut app = lab::fixture(true, lab::Scenario::Tumble);
    let world = app.world_mut();
    let records: Vec<_> = world
        .query::<(&Velocity, &Segment, &ColliderMassProperties)>()
        .iter(world)
        .map(|(v, s, m)| (*v, s.part, *m))
        .collect();
    assert_eq!(records.len(), 16);
    let mut momentum = Vec3::ZERO;
    for (v, _, m) in records {
        let ColliderMassProperties::Mass(m) = m else {
            panic!("explicit mass")
        };
        momentum += v.linvel * m;
        assert_eq!(v.angvel, Vec3::X * 2.0);
    }
    assert!(momentum.distance(Vec3::Z * 18.0 * 70.0) < 0.001);
}
#[test]
fn native_drop_settles_and_hinges_resist_torque_without_exploding() {
    let drop = lab::measure(true, lab::Scenario::Drop, 900);
    println!("BIOMECH_DROP {drop:?}");
    assert!(drop.finite);
    assert_eq!(drop.sleeping_bodies, 16);
    assert_eq!(drop.resting_ragdolls, 1);
    assert!(drop.max_anchor_error_m < 0.04);
    assert!(drop.final_max_speed_mps < 0.1);
    let force = lab::measure(true, lab::Scenario::HingeStress, 600);
    println!("BIOMECH_HINGE {force:?}");
    assert!(force.finite);
    assert!(force.peak_angular_radps > 0.5);
    assert!(force.max_limit_excess_rad < 0.08);
    assert!(force.max_anchor_error_m < 0.04);
}
#[test]
fn wall_impact_and_high_speed_ejection_remain_bounded() {
    for s in [lab::Scenario::Wall, lab::Scenario::EjectionHigh] {
        let r = lab::measure(true, s, 900);
        println!("BIOMECH_IMPACT {r:?}");
        assert!(r.finite);
        assert!(r.max_anchor_error_m < 0.05);
        assert!(r.max_limit_excess_rad < 0.10);
        assert!(r.peak_mechanical_energy_j < r.initial_mechanical_energy_j * 1.02);
        assert_eq!(r.sleeping_bodies, 16);
    }
}
#[test]
fn fifteen_crashes_have_bounded_entities_and_can_sleep() {
    let r = lab::measure(true, lab::Scenario::Pileup, 900);
    assert!(r.finite);
    assert_eq!(r.bodies, 240);
    assert_eq!(r.sleeping_bodies, 240);
    assert_eq!(r.resting_ragdolls, 15);
    assert!(r.max_anchor_error_m < 0.05);
}
#[test]
fn pose_motor_holds_a_configurable_elbow_target_and_yields_to_external_torque() {
    let mut app = lab::fixture(true, lab::Scenario::Drop);
    {
        let mut s = app.world_mut().resource_mut::<Settings>();
        s.config.joints[4].rest = [-0.8, 0.0, 0.0];
        s.config.joints[4].brace = [-0.8, 0.0, 0.0];
        s.config.joints[4].passive_k = 80.0;
        s.config.joints[4].passive_d = 4.0;
        s.config.joints[4].passive_torque = 80.0;
    }
    let root = app
        .world_mut()
        .query::<(Entity, &Segment)>()
        .iter(app.world())
        .find(|(_, s)| s.part == 0)
        .unwrap()
        .0;
    app.world_mut().entity_mut(root).insert(RigidBody::Fixed);
    for _ in 0..180 {
        app.update();
    }
    let e = app
        .world_mut()
        .query::<(Entity, &Segment)>()
        .iter(app.world())
        .find(|(_, s)| s.part == 4)
        .unwrap()
        .0;
    let j = app.world().get::<ImpulseJoint>(e).unwrap();
    let p = app.world().get::<Transform>(j.parent).unwrap();
    let t = app.world().get::<Transform>(e).unwrap();
    let angle = coordinates(p.rotation.inverse() * t.rotation).x;
    assert!((angle + 0.8).abs() < 0.12, "pose error {angle}");
    app.world_mut().entity_mut(e).insert(ExternalImpulse {
        impulse: Vec3::ZERO,
        torque_impulse: Vec3::X * 0.8,
    });
    app.update();
    let velocity = app.world().get::<Velocity>(e).unwrap();
    assert!(
        velocity.angvel.length() > 0.05,
        "joint must remain physical"
    );
}
#[test]
fn animation_bind_axes_are_removed_before_physical_pose_following() {
    let bindings = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let frames = bindings
        .standing
        .map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
    for q in animation_targets(frames) {
        assert!(
            q.dot(Quat::IDENTITY).abs() > 0.99999,
            "standing skin axes must not become physical flexion"
        );
    }
    let lib = crate::animation::Library::default();
    let t = animation_targets(lib.frames(lib.key(2020)));
    for q in t {
        assert!(q.is_finite() && (q.length() - 1.0).abs() < 0.0001);
    }
    assert!(coordinates(t[4]).x < 0.1);
    assert!(coordinates(t[8]).x > 0.0);
}
#[test]
fn real_ejection_keeps_preimpact_spin_even_after_the_hidden_chassis_is_stopped() {
    use bevy::ecs::system::RunSystemOnce;
    let mut app = crate::game::tests::simulation();
    app.world_mut().resource_mut::<Settings>().enabled = true;
    let library = crate::animation::Library::default();
    let pose = library.key(2030);
    app.insert_resource(library);
    {
        let w = app.world_mut();
        let mut query = w.query::<&mut crate::animation::Animator>();
        for mut a in query.iter_mut(w) {
            a.ready = true;
            a.current = pose;
        }
    }
    app.world_mut()
        .run_system_once(
            |mut commands: Commands,
             art: Res<crate::world::Art>,
             context: ReadRapierContext,
             mut riders: Query<(Entity, &mut crate::game::Rider, &mut Recovery)>| {
                let (e, mut rider, mut recovery) = riders.single_mut().unwrap();
                rider.motion_velocity = Vec3::Z * 12.0;
                rider.motion_angular_velocity = Vec3::Y * 0.8;
                crate::recovery::crash(
                    &mut commands,
                    &art,
                    e,
                    &mut rider,
                    &mut recovery,
                    Vec3::Y * 2.0,
                    Quat::IDENTITY,
                    Vec3::ZERO,
                    &context,
                );
            },
        )
        .unwrap();
    initialize(app.world_mut());
    let w = app.world_mut();
    let joints: Vec<_> = w
        .query::<(Entity, &ImpulseJoint)>()
        .iter(w)
        .map(|(e, j)| (e, j.parent))
        .collect();
    for (e, parent) in joints {
        let j = w.get::<ImpulseJoint>(e).unwrap();
        let raw: &GenericJoint = j.data.as_ref();
        let p = w.get::<Transform>(parent).unwrap();
        let c = w.get::<Transform>(e).unwrap();
        assert!(
            p.transform_point(raw.local_anchor1())
                .distance(c.transform_point(raw.local_anchor2()))
                < 0.00001
        );
    }
    let records: Vec<_> = w
        .query::<(&OwnedBody, &Velocity)>()
        .iter(w)
        .map(|(o, v)| (o.kind, *v))
        .collect();
    assert_eq!(
        records.iter().filter(|r| r.0 == BodyKind::Ragdoll).count(),
        16
    );
    for (_, v) in records {
        assert!(v.angvel.distance(Vec3::Y * 0.8) < 0.00001);
    }
}
#[test]
fn repeat_on_foot_knockdown_does_not_reuse_bike_anchor_space_animation() {
    use bevy::ecs::system::RunSystemOnce;
    let mut app = lab::fixture(false, lab::Scenario::Drop);
    app.world_mut().resource_mut::<Settings>().enabled = true;
    let library = crate::animation::Library::default();
    let mut pose = library.key(2020);
    pose.root.x += 30.0;
    app.insert_resource(library);
    app.world_mut()
        .run_system_once(move |mut commands: Commands, art: Res<crate::world::Art>| {
            let owner = commands
                .spawn((
                    crate::game::Rider::new(0),
                    EjectionAngular(Vec3::ZERO, false),
                ))
                .id();
            commands.queue(move |world: &mut World| {
                let mut a = world.get_mut::<crate::animation::Animator>(owner).unwrap();
                a.ready = true;
                a.current = pose;
            });
            crate::recovery::humanoid(
                &mut commands,
                &art,
                owner,
                4,
                Vec3::Y * 2.0,
                Quat::IDENTITY,
                Vec3::ZERO,
            );
        })
        .unwrap();
    initialize(app.world_mut());
    let w = app.world_mut();
    for (t, _, _) in w.query::<(&Transform, &OwnedBody, &Segment)>().iter(w) {
        assert!(
            t.translation.x.abs() < 2.0,
            "a bike-space visual pose must not teleport the on-foot crash"
        );
    }
}

#[test]
fn upgrading_a_ragdoll_preserves_original_crash_direction_policy() {
    for reverse in [false, true] {
        let mut app = lab::fixture(false, lab::Scenario::EjectionLow);
        let w = app.world_mut();
        let bodies: Vec<_> = w
            .query::<(Entity, &crate::recovery::CrashEnvelope)>()
            .iter(w)
            .map(|(e, c)| (e, c.with_original_direction(reverse)))
            .collect();
        assert_eq!(bodies.len(), 11);
        for (e, c) in bodies {
            w.entity_mut(e).insert(c);
        }
        w.resource_mut::<Settings>().enabled = true;
        initialize(w);
        let envelopes: Vec<_> = w
            .query_filtered::<&crate::recovery::CrashEnvelope, With<Segment>>()
            .iter(w)
            .collect();
        assert_eq!(envelopes.len(), 16);
        assert!(envelopes.iter().all(|c| c.allows_reverse() == reverse));
    }
}

#[test]
fn passive_knee_resistance_reduces_deflection_under_the_same_load() {
    fn deflection(legacy: bool) -> f32 {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 4,
        })
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
        let mut config = Settings::default().config;
        if legacy {
            let k = &mut config.joints[8];
            k.passive_k = 5.0;
            k.passive_d = 2.0;
            k.passive_torque = 12.0;
        }
        let mut knee = joint(&config, 8);
        knee.set_local_anchor1(Vec3::ZERO);
        knee.set_local_anchor2(Vec3::Y * 0.22);
        let parent = app
            .world_mut()
            .spawn((RigidBody::Fixed, Transform::IDENTITY))
            .id();
        let shin = app
            .world_mut()
            .spawn((
                RigidBody::Dynamic,
                GravityScale(0.0),
                Collider::capsule_y(0.13, 0.09),
                ColliderMassProperties::Mass(3.2),
                Transform::from_xyz(0.0, -0.22, 0.0),
                Velocity::zero(),
                ExternalForce {
                    torque: Vec3::X * 3.0,
                    ..default()
                },
                ImpulseJoint::new(parent, TypedJoint::GenericJoint(knee)),
            ))
            .id();
        for _ in 0..180 {
            app.update();
        }
        (coordinates(app.world().get::<Transform>(shin).unwrap().rotation).x
            - config.joints[8].rest[0])
            .abs()
    }
    let old = deflection(true);
    let new = deflection(false);
    println!("KNEE_PASSIVE_LOAD old={old} new={new}");
    assert!(
        old > 0.3 && new < old * 0.5 && new > 0.03,
        "must resist more while still yielding"
    );
}
#[test]
fn inward_hip_loading_keeps_knees_on_their_own_sides() {
    let mut app = lab::fixture(true, lab::Scenario::Drop);
    let w = app.world_mut();
    let parts: std::collections::HashMap<_, _> = w
        .query::<(Entity, &Segment)>()
        .iter(w)
        .map(|(e, s)| (s.part, e))
        .collect();
    for (part, direction) in [(7, 1.0), (9, -1.0)] {
        w.entity_mut(parts[&part]).insert(ExternalForce {
            torque: Vec3::Z * direction * 20.0,
            ..default()
        });
    }
    let mut minimum = f32::INFINITY;
    for _ in 0..240 {
        app.update();
        let w = app.world();
        let pelvis = w.get::<Transform>(parts[&2]).unwrap();
        let knee = |part: usize| {
            let t = w.get::<Transform>(parts[&part]).unwrap();
            let anchor = t.transform_point(specs()[part].anchor - specs()[part].position);
            (pelvis.rotation.inverse() * (anchor - pelvis.translation)).x
        };
        minimum = minimum.min(knee(10) - knee(8));
    }
    println!("KNEE_SEPARATION minimum={minimum}");
    assert!(
        minimum > 0.12,
        "knees crossed or collapsed inward under paired load: {minimum}"
    );
}

#[test]
fn player_bracing_is_supported_bounded_and_released_without_changing_limits() {
    let mut app = lab::fixture(true, lab::Scenario::Drop);
    let owner = app
        .world_mut()
        .query::<&OwnedBody>()
        .iter(app.world())
        .next()
        .unwrap()
        .owner;
    app.world_mut()
        .entity_mut(owner)
        .insert(crate::game::Rider::new(0));
    app.init_resource::<crate::controls::Controls>()
        .init_resource::<crate::recovery::RecoveryTuning>();
    for _ in 0..900 {
        app.update();
    }
    assert!(
        app.world_mut()
            .query::<&Brain>()
            .iter(app.world())
            .all(|b| b.supported)
    );
    app.world_mut()
        .resource_mut::<crate::controls::Controls>()
        .foot = Vec2::X;
    app.update();
    let limit = app
        .world()
        .resource::<crate::recovery::RecoveryTuning>()
        .0
        .brace_activation;
    for m in app.world_mut().query::<&Muscle>().iter(app.world()) {
        assert!((m.activation - limit).abs() < 1e-5);
    }
    for (s, j) in app
        .world_mut()
        .query::<(&Segment, &ImpulseJoint)>()
        .iter(app.world())
    {
        let c = &app.world().resource::<Settings>().config.joints[s.part];
        for (axis, range) in AXES.into_iter().zip(c.limits) {
            if range != [0.0, 0.0] {
                let l = j.data.as_ref().limits(axis).unwrap();
                assert_eq!([l.min, l.max], range);
            }
        }
    }
    app.world_mut()
        .resource_mut::<crate::controls::Controls>()
        .foot = Vec2::ZERO;
    app.update();
    for m in app.world_mut().query::<&Muscle>().iter(app.world()) {
        assert!(m.activation < limit);
    }
}

#[test]
fn grounded_roll_converts_existing_energy_once_in_forward_and_side_directions() {
    use bevy::ecs::system::RunSystemOnce;
    for direction in [Vec3::Z, Vec3::X] {
        let mut app = lab::fixture(true, lab::Scenario::Drop);
        for _ in 0..5 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query::<&OwnedBody>()
            .iter(app.world())
            .next()
            .unwrap()
            .owner;
        app.world_mut()
            .entity_mut(owner)
            .insert(crate::game::Rider::new(0));
        for mut v in app
            .world_mut()
            .query_filtered::<&mut Velocity, With<Segment>>()
            .iter_mut(app.world_mut())
        {
            *v = Velocity::linear(direction * 8.0);
        }
        for mut b in app
            .world_mut()
            .query::<&mut Brain>()
            .iter_mut(app.world_mut())
        {
            b.supported = true;
            b.control.age = 1.0;
        }
        fn measure(app: &mut App) -> (f32, Vec3, Vec3) {
            let mut energy = 0.0;
            let mut momentum = Vec3::ZERO;
            let mut angular = Vec3::ZERO;
            for (t, m, v) in app
                .world_mut()
                .query_filtered::<(&Transform, &ReadMassProperties, &Velocity), With<Segment>>()
                .iter(app.world())
            {
                let w = (t.rotation * m.principal_inertia_local_frame).inverse() * v.angvel;
                energy += 0.5
                    * (m.mass * v.linvel.length_squared()
                        + (m.principal_inertia * w * w).element_sum());
                momentum += m.mass * v.linvel;
                angular += v.angvel;
            }
            (energy, momentum, angular)
        }
        let before = measure(&mut app);
        app.world_mut().run_system_once(reactions::roll).unwrap();
        let after = measure(&mut app);
        assert!(after.0 <= before.0 + 0.02, "energy {before:?} -> {after:?}");
        assert!(
            after.1.dot(direction) > before.1.dot(direction) * 0.95,
            "direction={direction:?} before={before:?} after={after:?}"
        );
        assert!(after.2.dot(Vec3::Y.cross(direction)) > 0.5);
        app.world_mut().run_system_once(reactions::roll).unwrap();
        let twice = measure(&mut app);
        assert!((twice.0 - after.0).abs() < 1e-5 && twice.1.distance(after.1) < 1e-5);
    }
}
#[test]
fn roll_assistance_needs_support_and_does_not_change_the_baseline() {
    use bevy::ecs::system::RunSystemOnce;
    let mut app = lab::fixture(true, lab::Scenario::Drop);
    app.update();
    let owner = app
        .world_mut()
        .query::<&OwnedBody>()
        .iter(app.world())
        .next()
        .unwrap()
        .owner;
    app.world_mut()
        .entity_mut(owner)
        .insert(crate::game::Rider::new(0));
    for mut b in app
        .world_mut()
        .query::<&mut Brain>()
        .iter_mut(app.world_mut())
    {
        b.supported = false;
        b.control.age = 1.0;
    }
    app.world_mut().run_system_once(reactions::roll).unwrap();
    assert!(
        app.world_mut()
            .query::<&Brain>()
            .iter(app.world())
            .all(|b| !b.roll_assisted)
    );
    app.world_mut().resource_mut::<Settings>().enabled = false;
    for mut b in app
        .world_mut()
        .query::<&mut Brain>()
        .iter_mut(app.world_mut())
    {
        b.supported = true;
    }
    for mut v in app
        .world_mut()
        .query::<&mut Velocity>()
        .iter_mut(app.world_mut())
    {
        *v = Velocity::linear(Vec3::Z * 8.0);
    }
    app.world_mut().run_system_once(reactions::roll).unwrap();
    for v in app.world_mut().query::<&Velocity>().iter(app.world()) {
        assert_eq!(v.linvel, Vec3::Z * 8.0);
        assert_eq!(v.angvel, Vec3::ZERO);
    }
}

#[test]
fn frontal_leg_release_uses_limited_motors_without_a_velocity_impulse() {
    use bevy::ecs::system::RunSystemOnce;
    let mut app = lab::fixture(true, lab::Scenario::Drop);
    app.update();
    for mut b in app
        .world_mut()
        .query::<&mut Brain>()
        .iter_mut(app.world_mut())
    {
        b.frontal_release = true;
        b.control.age = 0.1;
    }
    let before: Vec<_> = app
        .world_mut()
        .query::<(Entity, &Velocity)>()
        .iter(app.world())
        .map(|(e, v)| (e, *v))
        .collect();
    app.world_mut().run_system_once(muscles::update).unwrap();
    for (e, v) in before {
        let now = app.world().get::<Velocity>(e).unwrap();
        assert_eq!(v.linvel, now.linvel);
        assert_eq!(v.angvel, now.angvel);
    }
    for (s, m) in app
        .world_mut()
        .query::<(&Segment, &Muscle)>()
        .iter(app.world())
    {
        if matches!(s.part, 7 | 9) {
            assert!(coordinates(m.target).x > 0.02 && coordinates(m.target).x <= 0.4);
        }
        if matches!(s.part, 8 | 10) {
            assert!(coordinates(m.target).x > 0.2 && coordinates(m.target).x <= 1.0);
        }
    }
}
