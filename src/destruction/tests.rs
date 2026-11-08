use super::*;
#[test]
fn reset_restores_prop_body_and_damage() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Contacts>()
        .init_resource::<Stats>()
        .add_message::<Reset>()
        .add_systems(Update, reset);
    let profile = Tuning::default().0.profiles["bench"].clone();
    let shape = Collider::cuboid(1., 0.5, 0.5);
    let original = Transform::from_xyz(2., 3., 4.);
    let visual = app.world_mut().spawn(FrozenVisual).id();
    let e = app
        .world_mut()
        .spawn((
            RigidBody::Dynamic,
            Transform::from_xyz(9., 1., 2.),
            Velocity::linear(Vec3::X * 8.),
            Loose {
                age: 2.,
                soft: false,
            },
            Breakable {
                state: Damage {
                    spent_j: 1200.,
                    broken: true,
                    revision: 1,
                },
                profile,
                original,
                shape: shape.clone(),
                size: Vec3::ONE,
                slot: None,
                visual,
            },
        ))
        .id();
    request_reset(app.world_mut());
    app.update();
    let entity = app.world().entity(e);
    assert!(!entity.get::<Breakable>().unwrap().state.broken);
    assert_eq!(*entity.get::<Transform>().unwrap(), original);
    assert_eq!(*entity.get::<RigidBody>().unwrap(), RigidBody::Fixed);
    assert!(entity.get::<Loose>().is_none());
    assert_eq!(entity.get::<Velocity>().unwrap().linvel, Vec3::ZERO);
    assert!(app.world().entity(visual).get::<FrozenVisual>().is_none());
}

fn physics() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        PropDestructionPlugin,
    ))
    .insert_resource(Time::<Fixed>::from_hz(60.))
    .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        std::time::Duration::from_secs_f64(1. / 60.),
    ))
    .insert_resource(TimestepMode::Fixed {
        dt: 1. / 60.,
        substeps: 3,
    })
    .insert_resource(Race {
        phase: Phase::Racing,
        ..default()
    })
    .add_message::<SweptContact>()
    .add_message::<crate::combat::AttackAttempt>()
    .add_message::<AudioCue>()
    .configure_sets(
        FixedUpdate,
        SimSet::Gameplay.before(PhysicsSet::SyncBackend),
    )
    .configure_sets(
        FixedUpdate,
        SimSet::AfterPhysics.after(PhysicsSet::Writeback),
    );
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::cuboid(40., 0.1, 40.),
        Transform::from_xyz(0., -0.1, 0.),
        crate::world::Ground,
    ));
    app
}
fn prop(app: &mut App, x: f32) -> Entity {
    let profile = Tuning::default().0.profiles["hydrant"].clone();
    let shape = Collider::compound(vec![(
        Vec3::Y,
        Quat::IDENTITY,
        Collider::cuboid(0.4, 1., 0.4),
    )]);
    let original = Transform::from_xyz(x, 0., 0.);
    let visual = app.world_mut().spawn(Transform::default()).id();
    app.world_mut()
        .spawn((
            RigidBody::Fixed,
            shape.clone(),
            original,
            ActiveEvents::COLLISION_EVENTS,
            Breakable {
                state: Damage::default(),
                profile,
                original,
                shape,
                size: Vec3::new(0.8, 2., 0.8),
                slot: None,
                visual,
            },
        ))
        .id()
}
fn projectile(app: &mut App, x: f32, speed: f32) -> Entity {
    app.world_mut()
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(0.35),
            Transform::from_xyz(x, 0.65, -2.),
            Velocity::linear(Vec3::Z * speed),
            AdditionalMassProperties::Mass(180.),
            Ccd::enabled(),
        ))
        .id()
}
fn steps(app: &mut App, count: usize) {
    for _ in 0..count {
        app.update();
    }
}
#[test]
fn real_contact_breaks_releases_and_reset_restores_backend_pose() {
    let mut app = physics();
    let e = prop(&mut app, 0.);
    let ball = projectile(&mut app, 0., 20.);
    steps(&mut app, 60);
    assert!(app.world().get::<Breakable>(e).unwrap().state.broken);
    assert_ne!(
        app.world().get::<Transform>(e).unwrap().translation,
        Vec3::ZERO
    );
    assert_eq!(
        *app.world().get::<RigidBody>(e).unwrap(),
        RigidBody::Dynamic
    );
    app.world_mut().despawn(ball);
    request_reset(app.world_mut());
    steps(&mut app, 3);
    assert_eq!(*app.world().get::<RigidBody>(e).unwrap(), RigidBody::Fixed);
    assert_eq!(
        *app.world().get::<Transform>(e).unwrap(),
        Transform::default()
    );
    assert!(!app.world().get::<Breakable>(e).unwrap().state.broken);
}
#[test]
fn budget_keeps_additional_damage_as_persistent_static_wreckage() {
    let mut app = physics();
    app.world_mut()
        .resource_mut::<Tuning>()
        .0
        .max_dynamic_bodies = 1;
    let a = prop(&mut app, -3.);
    let b = prop(&mut app, 3.);
    projectile(&mut app, -3., 20.);
    projectile(&mut app, 3., 20.);
    steps(&mut app, 45);
    assert!(app.world().get::<Breakable>(a).unwrap().state.broken);
    assert!(app.world().get::<Breakable>(b).unwrap().state.broken);
    assert_eq!(app.world().resource::<Stats>().peak_dynamic, 1);
    assert_eq!(
        app.world_mut()
            .query_filtered::<Entity, With<Loose>>()
            .iter(app.world())
            .count(),
        1
    );
}
#[test]
fn slow_contacts_and_paused_damage_are_ignored() {
    for (phase, speed, disabled) in [
        (Phase::Racing, 0.28, false),
        (Phase::Paused, 20., false),
        (Phase::Racing, 20., true),
    ] {
        let mut app = physics();
        let e = prop(&mut app, 0.);
        let source = projectile(&mut app, 0., speed);
        app.world_mut().resource_mut::<Race>().phase = phase;
        if disabled {
            app.world_mut()
                .entity_mut(source)
                .insert((ColliderDisabled, RigidBodyDisabled));
        }
        app.world_mut().write_message(SweptContact(source, e));
        steps(&mut app, 20);
        assert_eq!(app.world().get::<Breakable>(e).unwrap().state.spent_j, 0.);
    }
}
#[test]
fn two_direct_punches_break_a_bin_but_a_wall_blocks_the_same_strike() {
    for wall in [false, true] {
        let mut app = physics();
        let e = prop(&mut app, -1.3);
        app.world_mut().get_mut::<Breakable>(e).unwrap().profile =
            Tuning::default().0.profiles["street_bin"].clone();
        let rider = app
            .world_mut()
            .spawn((
                Rider::new(0),
                Transform::default(),
                Collider::cuboid(0.3, 0.9, 0.5),
            ))
            .id();
        if wall {
            app.world_mut().spawn((
                RigidBody::Fixed,
                Collider::cuboid(0.1, 2., 2.),
                Transform::from_xyz(-0.65, 1., 0.),
            ));
        }
        steps(&mut app, 3);
        for _ in 0..2 {
            app.world_mut().write_message(crate::combat::AttackAttempt(
                rider,
                rr_core::AttackSide::Right,
            ));
            steps(&mut app, 35);
        }
        assert_eq!(app.world().get::<Breakable>(e).unwrap().state.broken, !wall);
    }
}
#[test]
fn supported_slow_debris_settles_and_stays_present() {
    let mut app = physics();
    let e = prop(&mut app, 0.);
    let ball = projectile(&mut app, 0., 20.);
    steps(&mut app, 60);
    app.world_mut().despawn(ball);
    steps(&mut app, 900);
    assert!(app.world().get::<Breakable>(e).unwrap().state.broken);
    assert!(app.world().get::<Loose>(e).is_none());
    assert_eq!(*app.world().get::<RigidBody>(e).unwrap(), RigidBody::Fixed);
}
