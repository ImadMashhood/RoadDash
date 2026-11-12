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
    assert_eq!(entity.get::<Velocity>().unwrap().linear, Vec3::ZERO);
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
    .add_message::<crate::combat::StrikeContact>()
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

#[test]
fn prepared_damage_presentation_handles_loading_reset_and_distance_without_double_crush() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Update, visuals::update);
    let original = Transform::from_xyz(0.2, 0.1, -0.3).with_scale(Vec3::new(2., 3., 4.));
    let variant_pose = Transform::from_xyz(0.2, 0.038, -0.3).with_scale(original.scale);
    let damaged = app
        .world_mut()
        .spawn((visuals::Damaged, variant_pose, Visibility::Hidden))
        .id();
    let owner = app
        .world_mut()
        .spawn((
            GlobalTransform::default(),
            Breakable {
                state: Damage::default(),
                profile: Tuning::default().0.profiles["street_bin"].clone(),
                original: Transform::default(),
                shape: Collider::cuboid(1., 1., 1.),
                size: Vec3::ONE,
                slot: None,
                visual: Entity::PLACEHOLDER,
            },
        ))
        .id();
    let intact = app
        .world_mut()
        .spawn((
            original,
            Visibility::Inherited,
            visuals::Visual {
                owner,
                original: None,
                damaged: Some(damaged),
                prepared: false,
            },
        ))
        .id();
    let camera = app
        .world_mut()
        .spawn((Camera3d::default(), GlobalTransform::default()))
        .id();
    for ready_first in [false, true] {
        app.world_mut()
            .get_mut::<Breakable>(owner)
            .unwrap()
            .state
            .broken = true;
        if ready_first {
            app.world_mut().entity_mut(damaged).insert(visuals::Ready);
        }
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            if ready_first {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            }
        );
        assert_eq!(
            app.world().get::<Transform>(intact).unwrap().scale.y,
            original.scale.y * 0.38
        );
        // Reset before readiness, then complete loading: intact must stay visible.
        app.world_mut()
            .get_mut::<Breakable>(owner)
            .unwrap()
            .state
            .broken = false;
        app.update();
        app.world_mut().entity_mut(damaged).insert(visuals::Ready);
        app.update();
        assert_eq!(*app.world().get::<Transform>(intact).unwrap(), original);
        assert_eq!(
            *app.world().get::<Visibility>(intact).unwrap(),
            Visibility::Inherited
        );
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            Visibility::Hidden
        );
        app.world_mut()
            .get_mut::<Breakable>(owner)
            .unwrap()
            .state
            .broken = true;
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(intact).unwrap(),
            Visibility::Hidden
        );
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            Visibility::Inherited
        );
        assert_eq!(
            *app.world().get::<Transform>(damaged).unwrap(),
            variant_pose
        );
        *app.world_mut().get_mut::<GlobalTransform>(camera).unwrap() =
            GlobalTransform::from_translation(Vec3::X * 501.);
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            Visibility::Hidden
        );
        *app.world_mut().get_mut::<GlobalTransform>(camera).unwrap() = GlobalTransform::default();
        app.world_mut()
            .get_mut::<Breakable>(owner)
            .unwrap()
            .state
            .broken = false;
        app.update();
    }
}

#[test]
fn live_buffered_punch_reaches_props_once_at_contact_and_pause_cancel_discard_it() {
    for cancel in [false, true] {
        let mut app = crate::game::tests::simulation();
        app.add_plugins(PropDestructionPlugin);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        steps(&mut app, 3);
        let (owner, pose) = app
            .world_mut()
            .query::<(Entity, &Rider, &Transform)>()
            .iter(app.world())
            .find(|(_, r, _)| r.id == 0)
            .map(|(e, _, t)| (e, *t))
            .unwrap();
        let target = prop(&mut app, 0.);
        let target_pose = Transform::from_translation(
            pose.translation + pose.rotation * Vec3::X * -1.3 - Vec3::Y * pose.translation.y,
        );
        *app.world_mut().get_mut::<Transform>(target).unwrap() = target_pose;
        app.world_mut()
            .get_mut::<Breakable>(target)
            .unwrap()
            .profile = Tuning::default().0.profiles["street_bin"].clone();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyK);
        steps(&mut app, 4);
        let clock = app
            .world()
            .get::<Rider>(owner)
            .unwrap()
            .strike
            .expect("live keyboard starts windup");
        assert!(clock.age < clock.contact);
        assert_eq!(
            app.world().get::<Breakable>(target).unwrap().state.spent_j,
            0.
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        for _ in 0..20 {
            app.update();
            let clock = app.world().get::<Rider>(owner).unwrap().strike;
            let expected = if cancel || clock.is_some_and(|c| c.age < c.contact) {
                0.
            } else {
                140.
            };
            assert_eq!(
                app.world().get::<Breakable>(target).unwrap().state.spent_j,
                expected
            );
        }
        assert_eq!(
            app.world().get::<Breakable>(target).unwrap().state.spent_j,
            0.
        );
        assert_eq!(
            app.world().get::<Rider>(owner).unwrap().strike.unwrap().age,
            clock.age
        );
        if cancel {
            app.world_mut()
                .get_mut::<crate::recovery::Recovery>(owner)
                .unwrap()
                .stage = crate::recovery::RecoveryStage::Running;
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..20 {
            app.update();
            let clock = app.world().get::<Rider>(owner).unwrap().strike;
            let expected = if cancel || clock.is_some_and(|c| c.age < c.contact) {
                0.
            } else {
                140.
            };
            assert_eq!(
                app.world().get::<Breakable>(target).unwrap().state.spent_j,
                expected
            );
        }
        assert_eq!(
            app.world().get::<Breakable>(target).unwrap().state.spent_j,
            if cancel { 0. } else { 140. }
        );
        steps(&mut app, 35);
        assert_eq!(
            app.world().get::<Breakable>(target).unwrap().state.spent_j,
            if cancel { 0. } else { 140. }
        );
    }
}

#[test]
fn resolved_kick_cannot_flip_to_a_prop_on_the_opposite_side() {
    for x in [-1.3, 1.3] {
        let mut app = physics();
        app.init_resource::<crate::combat::StrikeTimings>()
            .init_resource::<crate::weapons::WeaponRules>()
            .add_message::<crate::combat::AttackIntent>()
            .add_systems(
                FixedUpdate,
                crate::combat::schedule.in_set(SimSet::Gameplay),
            );
        let target = prop(&mut app, x);
        app.world_mut()
            .get_mut::<Breakable>(target)
            .unwrap()
            .profile = Tuning::default().0.profiles["street_bin"].clone();
        let rider = app
            .world_mut()
            .spawn((
                Rider::new(0),
                crate::recovery::Recovery::default(),
                Transform::default(),
                Collider::cuboid(0.3, 0.9, 0.5),
            ))
            .id();
        steps(&mut app, 3);
        app.world_mut().write_message(crate::combat::AttackIntent(
            rider,
            rr_core::AttackSide::Kick,
        ));
        steps(&mut app, 4);
        let clock = app.world().get::<Rider>(rider).unwrap().strike.unwrap();
        assert_eq!(clock.side, 1.);
        assert_eq!(
            app.world().get::<Breakable>(target).unwrap().state.spent_j,
            0.
        );
        steps(&mut app, 20);
        assert_eq!(
            app.world().get::<Breakable>(target).unwrap().state.broken,
            x < 0.
        );
    }
}

#[test]
fn damage_readiness_waits_for_materials_and_only_normal_mapped_mesh_tangents() {
    for normal_mapped in [false, true] {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<crate::material_detail::Queue>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<Assets<Mesh>>()
            .add_message::<AssetEvent<StandardMaterial>>()
            .add_message::<AssetEvent<Image>>()
            .add_systems(Update, visuals::update);
        let material = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                normal_map_texture: normal_mapped.then(Handle::default),
                metallic_roughness_texture: normal_mapped.then(Handle::default),
                ..default()
            });
        let mesh = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(Cuboid::default()));
        let leaf = app
            .world_mut()
            .spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone())))
            .id();
        let damaged = app
            .world_mut()
            .spawn((visuals::Damaged, Visibility::Hidden))
            .add_child(leaf)
            .id();
        let owner = app
            .world_mut()
            .spawn(Breakable {
                state: Damage {
                    broken: true,
                    ..default()
                },
                profile: Tuning::default().0.profiles["street_bin"].clone(),
                original: Transform::default(),
                shape: Collider::cuboid(1., 1., 1.),
                size: Vec3::ONE,
                slot: None,
                visual: Entity::PLACEHOLDER,
            })
            .id();
        app.world_mut().spawn((
            Transform::default(),
            Visibility::Inherited,
            visuals::Visual {
                owner,
                original: None,
                damaged: Some(damaged),
                prepared: false,
            },
        ));
        if normal_mapped {
            app.world_mut().entity_mut(damaged).insert(visuals::Ready);
        }
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            Visibility::Hidden
        );
        app.world_mut()
            .write_message(AssetEvent::Added { id: material.id() });
        use bevy::ecs::system::RunSystemOnce;
        app.world_mut()
            .run_system_once(crate::material_detail::update)
            .unwrap();
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            Visibility::Hidden
        );
        // Exercise both orders: hierarchy before material prep (normal mapped),
        // material prep before hierarchy (plain). Plain materials need no tangent.
        app.world_mut().entity_mut(damaged).insert(visuals::Ready);
        if normal_mapped {
            app.world_mut()
                .resource_mut::<Assets<Mesh>>()
                .get_mut(&mesh)
                .unwrap()
                .generate_tangents()
                .unwrap();
        }
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(damaged).unwrap(),
            Visibility::Inherited
        );
    }
}
