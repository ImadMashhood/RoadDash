//! Contact-policy regressions, independent from visual asset selection.
use crate::{
    game::{self, Phase, Race, Rider, Traffic},
    recovery::{self, OwnedBody, Recovery, RecoveryStage},
    runtime::SimSet,
    vehicle::{BikeState, VehiclePhysicsAdapter},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Default)]
struct Entry(Vec<Velocity>);
fn capture_entry(mut entry: ResMut<Entry>, bodies: Query<&Velocity, With<OwnedBody>>) {
    if entry.0.is_empty() {
        entry.0.extend(bodies.iter().copied());
    }
}

#[test]
fn player_and_ai_use_80_kmh_entry_for_both_120_40_car_directions() {
    for id in [0, 1] {
        for direction in [-1.0, 1.0] {
            let mut app = game::tests::simulation();
            for _ in 0..3 {
                app.update();
            }
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let origin = app.world().get::<Transform>(owner).unwrap().translation;
            let forward = app.world().get::<Transform>(owner).unwrap().rotation * Vec3::Z;
            let before = forward * (120.0 / 3.6);
            {
                let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
                rider.id = id;
                rider.motion_velocity = -before;
                rider.speed = 120.0 / 3.6;
                rider.immunity = 0.0;
            }
            app.world_mut().entity_mut(owner).insert(BikeState {
                impact_velocity: before,
                ..default()
            });
            let car = app
                .world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        progress: 500.0,
                        lane: 0.0,
                        speed: direction * 40.0 / 3.6,
                    },
                    Transform::from_translation(origin + forward * 10.0).with_rotation(
                        Quat::from_rotation_y(if direction < 0.0 {
                            std::f32::consts::PI
                        } else {
                            0.0
                        }),
                    ),
                ))
                .id();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.init_resource::<Entry>().add_systems(
                FixedUpdate,
                capture_entry
                    .after(SimSet::Gameplay)
                    .before(PhysicsSet::SyncBackend),
            );
            app.world_mut()
                .write_message(recovery::SweptContact(owner, car));
            app.update();
            let samples = &app.world().resource::<Entry>().0;
            assert_eq!(samples.len(), 12);
            for v in samples {
                assert!(
                    (v.linear.dot(forward) * 3.6 - 80.0).abs() < 0.001,
                    "id={id} direction={direction}: {v:?}"
                );
                assert!(v.angular.length() < 0.001, "scripted spin {v:?}");
                assert!(v.linear.y <= 0.001, "scripted launch {v:?}");
            }
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Falling
            );
            println!(
                "COLLISION_ENTRY id={id} direction={direction} rider_kmh=120 car_kmh=40 retained_kmh=80 bodies=12"
            );
        }
    }
}

#[test]
fn neutral_mounted_heading_ignores_repeated_contact_spin_at_each_render_rate() {
    for fps in [30, 60, 120] {
        let mut app = game::tests::simulation();
        app.insert_resource(VehiclePhysicsAdapter::Dynamic);
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / fps as f64),
        ));
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..12 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let before = app
            .world()
            .get::<Transform>(owner)
            .unwrap()
            .rotation
            .to_euler(EulerRot::YXZ)
            .0;
        for _ in 0..fps {
            app.world_mut()
                .get_mut::<Velocity>(owner)
                .unwrap()
                .angular
                .y = 15.0;
            app.update();
            let now = app
                .world()
                .get::<Transform>(owner)
                .unwrap()
                .rotation
                .to_euler(EulerRot::YXZ)
                .0;
            assert!(
                (now - before).abs() < 0.0001,
                "contact steered bike at {fps} Hz: {before} -> {now}"
            );
        }
    }
}

#[test]
fn crash_envelope_dissipates_solver_energy_and_reverse_com_without_launching() {
    let mut app = game::tests::simulation();
    let owner = app.world_mut().spawn_empty().id();
    for x in [-2.0, 2.0] {
        app.world_mut().spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.3, 0.3, 0.3),
            ColliderMassProperties::Mass(10.0),
            ReadMassProperties::default(),
            Transform::from_xyz(x, 20.0, 0.0),
            Velocity {
                linear: Vec3::new(0.0, 70.0, -40.0),
                angular: Vec3::splat(30.0),
            },
            OwnedBody {
                owner,
                kind: recovery::BodyKind::Ragdoll,
            },
            recovery::CrashEnvelope::new(
                Vec3::Z * 20.0,
                Vec3::new(x, 20.0, 0.0),
                Quat::IDENTITY,
                Vec3::Z,
            ),
        ));
    }
    for _ in 0..12 {
        app.update();
        let w = app.world_mut();
        let mut energy = 0.0;
        let mut budget = 0.0;
        let mut momentum = Vec3::ZERO;
        for (body, t, m, v) in w
            .query::<(&OwnedBody, &Transform, &ReadMassProperties, &Velocity)>()
            .iter(w)
        {
            if body.owner != owner {
                continue;
            }
            let omega = (t.rotation * m.principal_inertia_local_frame).inverse() * v.angular;
            energy += m.mass * v.linear.length_squared()
                + (m.principal_inertia * omega * omega).element_sum();
            budget += m.mass * (400.0 + 2.0 * 9.81 * (20.0 - t.translation.y));
            momentum += v.linear * m.mass;
            assert!(v.linear.is_finite() && v.angular.is_finite());
        }
        assert!(
            energy <= budget.max(0.0) + 0.1,
            "solver injected kinetic energy: {energy} > {budget}"
        );
        assert!(momentum.z >= -0.001, "reverse centre of mass {momentum:?}");
    }
}

#[test]
fn real_rear_and_oncoming_car_contacts_detach_without_reverse_rider_travel() {
    for direction in [-1.0, 1.0] {
        let mut app = game::tests::simulation();
        for _ in 0..3 {
            app.update();
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyR);
        app.insert_resource(VehiclePhysicsAdapter::Dynamic);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        for _ in 0..12 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let t = *app.world().get::<Transform>(owner).unwrap();
        let forward = (t.rotation * Vec3::Z).with_y(0.0).normalize();
        app.world_mut().get_mut::<Velocity>(owner).unwrap().linear = forward * (120.0 / 3.6);
        let r = app.world().get::<Rider>(owner).unwrap();
        let progress = r.progress + 5.0;
        let lane = r.lane;
        let (p, q) = app
            .world()
            .resource::<crate::track::Track>()
            .sample(progress, lane);
        app.world_mut().spawn((
            Traffic {
                route: 0,
                progress,
                lane,
                speed: direction * 40.0 / 3.6,
            },
            Transform::from_translation(p).with_rotation(q),
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y * 0.95,
                Quat::IDENTITY,
                Collider::cuboid(0.95, 0.9, 1.95),
            )]),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ));
        let mut torso = None;
        for _ in 0..90 {
            app.update();
            if app.world().get::<Recovery>(owner).unwrap().stage == RecoveryStage::Falling {
                torso = app.world().get::<Recovery>(owner).unwrap().torso;
                break;
            }
        }
        let torso = torso.expect("real car contact did not detach rider");
        let start = app.world().get::<Transform>(torso).unwrap().translation;
        for _ in 0..8 {
            app.update();
        }
        let end = app.world().get::<Transform>(torso).unwrap().translation;
        let travel = (end - start).dot(forward);
        assert!(
            travel >= -0.02,
            "real contact sent rider backwards, car direction {direction}: {start:?} -> {end:?}"
        );
        println!("REAL_CAR_TRAJECTORY direction={direction} forward_travel_8_ticks_m={travel}");
    }
}

#[test]
fn detached_body_contacts_never_script_a_new_mounted_wipeout() {
    for id in [0, 1] {
        for original in [false, true] {
            for own in [false, true] {
                for kind in [recovery::BodyKind::Ragdoll, recovery::BodyKind::Wreck] {
                    let mut app = game::tests::simulation();
                    app.insert_resource(crate::original_crash::OriginalCrashes(original));
                    let owner = app
                        .world_mut()
                        .query_filtered::<Entity, With<Rider>>()
                        .single(app.world())
                        .unwrap();
                    let foreign = app.world_mut().spawn_empty().id();
                    let origin = *app.world().get::<Transform>(owner).unwrap();
                    {
                        let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
                        r.id = id;
                        r.immunity = 0.0;
                        r.speed = 40.0;
                        r.motion_velocity = origin.rotation * Vec3::Z * 40.0;
                    }
                    let bodies: Vec<_> = (0..11)
                        .map(|i| {
                            app.world_mut()
                                .spawn((
                                    OwnedBody {
                                        owner: if own { owner } else { foreign },
                                        kind,
                                    },
                                    Transform::from_translation(
                                        origin.translation
                                            + origin.rotation * Vec3::Z * (2.0 + i as f32),
                                    ),
                                    Collider::ball(0.2),
                                    RigidBody::Dynamic,
                                    Velocity::zero(),
                                ))
                                .id()
                        })
                        .collect();
                    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                    for body in &bodies {
                        app.world_mut()
                            .write_message(recovery::SweptContact(owner, *body));
                    }
                    app.update();
                    assert_eq!(
                        app.world().get::<Recovery>(owner).unwrap().stage,
                        RecoveryStage::Riding,
                        "id={id}, original={original}, own={own}"
                    );
                    let hop = app.world().get::<Rider>(owner).unwrap().hop_velocity;
                    if own {
                        assert_eq!(hop, 0.0);
                    } else {
                        assert!((2.0..=4.0).contains(&hop));
                    }
                    assert_eq!(
                        app.world_mut()
                            .query::<&OwnedBody>()
                            .iter(app.world())
                            .count(),
                        11
                    );
                    for body in bodies {
                        assert!(app.world().get::<Collider>(body).is_some());
                        assert!(app.world().get::<ColliderDisabled>(body).is_none());
                    }
                }
            }
        }
    }
}

#[test]
fn actual_dynamic_wreck_contacts_hop_and_move_the_target_without_ejection() {
    for kind in [recovery::BodyKind::Wreck, recovery::BodyKind::Ragdoll] {
        let mut app = game::tests::simulation();
        app.insert_resource(VehiclePhysicsAdapter::Dynamic);
        for _ in 0..10 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let t = *app.world().get::<Transform>(owner).unwrap();
        let forward = t.rotation * Vec3::Z;
        let foreign = app.world_mut().spawn_empty().id();
        let target_start = t.translation + forward * 7.0 + Vec3::Y * 0.4;
        let target = app
            .world_mut()
            .spawn((
                OwnedBody {
                    owner: foreign,
                    kind,
                },
                RigidBody::Dynamic,
                Collider::cuboid(0.7, 0.35, 1.0),
                ColliderMassProperties::Mass(100.0),
                Transform::from_translation(target_start).with_rotation(t.rotation),
                Velocity::zero(),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
            ))
            .id();
        {
            let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
            r.speed = 25.0;
            r.motion_velocity = forward * 25.0;
            r.immunity = 0.0;
        }
        app.world_mut().get_mut::<Velocity>(owner).unwrap().linear = forward * 25.0;
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let mut rise = 0.0_f32;
        for _ in 0..90 {
            app.update();
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Riding
            );
            let p = app.world().get::<Transform>(owner).unwrap().translation;
            assert!(p.is_finite());
            rise = rise.max(p.y - t.translation.y);
        }
        let displacement = app
            .world()
            .get::<Transform>(target)
            .unwrap()
            .translation
            .distance(target_start);
        println!(
            "RIDE_OVER wreck={}: rise={rise}, target_displacement={displacement}",
            kind == recovery::BodyKind::Wreck
        );
        assert!(rise > 0.2 && rise < 2.0);
        assert!(displacement > 0.1);
    }
}

#[test]
fn rear_knockoffs_work_without_hops_in_both_entity_orders_and_preserve_head_on_crashes() {
    for base_is_attacker in [false, true] {
        for head_on in [false, true] {
            let mut app = game::tests::simulation();
            let base = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let (progress, other_progress) = if base_is_attacker {
                (-8.0, -6.0)
            } else {
                (-8.0, -10.0)
            };
            let other_progress = if head_on { -6.0 } else { other_progress };
            let (p, q) = app
                .world()
                .resource::<crate::track::Track>()
                .sample(progress, -2.0);
            *app.world_mut().get_mut::<Transform>(base).unwrap() =
                Transform::from_translation(p).with_rotation(q);
            {
                let mut r = app.world_mut().get_mut::<Rider>(base).unwrap();
                r.progress = progress;
                r.lane = -2.0;
                r.speed = if base_is_attacker || head_on {
                    25.0
                } else {
                    0.0
                };
                r.motion_velocity = q * Vec3::Z * r.speed;
                r.immunity = 0.0;
            }
            let (p, mut q) = app
                .world()
                .resource::<crate::track::Track>()
                .sample(other_progress, -2.0);
            if head_on {
                q *= Quat::from_rotation_y(std::f32::consts::PI);
            }
            let mut other = Rider::new(1);
            other.progress = other_progress;
            other.lane = -2.0;
            other.speed = if !base_is_attacker || head_on {
                25.0
            } else {
                0.0
            };
            other.motion_velocity = q * Vec3::Z * other.speed;
            other.immunity = 0.0;
            let other = app
                .world_mut()
                .spawn((
                    other,
                    Transform::from_translation(p).with_rotation(q),
                    Visibility::default(),
                    recovery::bike_body(),
                ))
                .id();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut()
                .write_message(recovery::SweptContact(base, other));
            app.update();
            if head_on {
                assert!(
                    [base, other]
                        .iter()
                        .any(|e| app.world().get::<Recovery>(*e).unwrap().stage
                            == RecoveryStage::Falling)
                );
                assert_eq!(app.world().get::<Rider>(base).unwrap().hop_velocity, 0.0);
                assert_eq!(app.world().get::<Rider>(other).unwrap().hop_velocity, 0.0);
            } else {
                let (attacker, victim) = if base_is_attacker {
                    (base, other)
                } else {
                    (other, base)
                };
                assert_eq!(
                    app.world().get::<Recovery>(attacker).unwrap().stage,
                    RecoveryStage::Riding
                );
                assert_eq!(
                    app.world().get::<Rider>(attacker).unwrap().hop_velocity,
                    0.0
                );
                assert_eq!(
                    app.world().get::<Recovery>(victim).unwrap().stage,
                    RecoveryStage::Falling
                );
            }
        }
    }
}

#[test]
fn rear_contact_threshold_and_mass_weighted_push_do_not_add_vertical_speed() {
    use bevy::ecs::system::RunSystemOnce;
    for closing in [0.0, 59.9 / 3.6, 60.1 / 3.6] {
        let mut app = game::tests::simulation();
        let a = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let ta = *app.world().get::<Transform>(a).unwrap();
        let f = ta.rotation * Vec3::Z;
        {
            let mut r = app.world_mut().get_mut::<Rider>(a).unwrap();
            r.motion_velocity = f * (10.0 + closing);
            r.speed = 10.0 + closing;
        }
        app.world_mut().entity_mut(a).insert(BikeState {
            impact_velocity: f * (10.0 + closing),
            ..default()
        });
        let mut r = Rider::new(1);
        r.speed = 10.0;
        r.motion_velocity = f * 10.0;
        r.bike_mass = 300.0;
        let b = app
            .world_mut()
            .spawn((
                r,
                ta.with_translation(ta.translation + f * 2.0),
                Visibility::Visible,
                BikeState {
                    impact_velocity: f * 10.0,
                    ..default()
                },
            ))
            .id();
        let ma = app.world().get::<Rider>(a).unwrap().bike_mass + 70.0;
        let common = rr_core::contact::shared_normal_speed(10.0 + closing, 10.0, ma, 370.0);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().write_message(recovery::SweptContact(a, b));
        app.world_mut().run_system_once(recovery::impacts).unwrap();
        let r = app.world().get::<Rider>(a).unwrap();
        assert!((r.motion_velocity.dot(f) - common).abs() < 0.001);
        assert_eq!(r.hop_velocity, 0.0);
        assert!(r.motion_velocity.y.abs() < 0.01);
        let rec = app.world().get::<Recovery>(b).unwrap();
        assert_eq!(
            rec.stage,
            if closing >= 60.0 / 3.6 {
                RecoveryStage::Falling
            } else {
                RecoveryStage::Riding
            }
        );
        if rec.stage == RecoveryStage::Riding {
            assert!(
                (app.world().get::<Rider>(b).unwrap().motion_velocity.dot(f) - common).abs()
                    < 0.001
            );
        }
    }
}

#[derive(Resource)]
struct MountedContact(Entity, Vec3, f32, bool);
fn capture_mounted(mut capture: ResMut<MountedContact>, riders: Query<(&Recovery, &Velocity)>) {
    if let Ok((recovery, v)) = riders.get(capture.0) {
        capture.2 = v.linear.dot(capture.1);
        capture.3 = recovery.stage == RecoveryStage::Riding;
    }
}
#[test]
fn pedestrian_runner_and_wreck_contacts_keep_entry_motion_after_solver_stop() {
    for kind in ["pedestrian", "runner", "wreck"] {
        let mut app = game::tests::simulation();
        for _ in 0..3 {
            app.update();
        }
        app.insert_resource(VehiclePhysicsAdapter::Dynamic);
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let pose = *app.world().get::<Transform>(owner).unwrap();
        let forward = pose.rotation * Vec3::Z;
        {
            let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
            r.speed = 30.0;
            r.motion_velocity = forward * 30.0;
            r.immunity = 0.0;
        }
        app.world_mut().entity_mut(owner).insert((
            BikeState {
                impact_velocity: forward * 30.0,
                ..default()
            },
            Velocity::zero(),
        ));
        let point = pose.translation + forward * 2.5;
        let mut walking_owner = None;
        let target = if kind == "pedestrian" {
            app.world_mut()
                .spawn((
                    crate::npc::Npc::new(
                        crate::npc::NpcKind::Pedestrian,
                        Transform::from_translation(point),
                    ),
                    Transform::from_translation(point),
                    Visibility::Visible,
                    RigidBody::KinematicPositionBased,
                    Collider::capsule_y(0.6, 0.25),
                ))
                .id()
        } else {
            let victim = app
                .world_mut()
                .spawn((
                    Rider::new(1),
                    Transform::from_translation(point + forward * 20.0),
                    Visibility::Hidden,
                ))
                .id();
            let body = app
                .world_mut()
                .spawn((
                    OwnedBody {
                        owner: victim,
                        kind: if kind == "runner" {
                            recovery::BodyKind::Runner
                        } else {
                            recovery::BodyKind::Wreck
                        },
                    },
                    Transform::from_translation(
                        point
                            + if kind == "runner" {
                                Vec3::Y * 0.95
                            } else {
                                Vec3::ZERO
                            },
                    )
                    .with_rotation(pose.rotation),
                    RigidBody::Dynamic,
                    Collider::cuboid(0.3, 0.5, 0.8),
                    ColliderMassProperties::Mass(45.0),
                    Velocity::zero(),
                ))
                .id();
            if kind == "runner" {
                walking_owner = Some(victim);
                app.world_mut().get_mut::<Recovery>(victim).unwrap().stage = RecoveryStage::Running;
                app.world_mut().get_mut::<Recovery>(victim).unwrap().runner = Some(body);
            }
            body
        };
        app.insert_resource(MountedContact(owner, forward, 0.0, false))
            .add_systems(
                FixedUpdate,
                capture_mounted
                    .after(SimSet::Gameplay)
                    .before(PhysicsSet::SyncBackend),
            );
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .write_message(recovery::SweptContact(owner, target));
        app.update();
        let result = app.world().resource::<MountedContact>();
        assert!(result.3, "{kind} ejected rider");
        assert!(result.2 > 20.0, "{kind} stopped bike: {}", result.2);
        assert!(
            app.world().get::<Rider>(owner).unwrap().hop_velocity > 0.0,
            "{kind} missed ride-over lift"
        );
        if let Some(victim) = walking_owner {
            assert_eq!(
                app.world().get::<Recovery>(victim).unwrap().stage,
                RecoveryStage::Falling,
                "walking victim was not knocked down"
            );
        }
        if kind == "pedestrian" {
            assert!(!app.world().get::<crate::npc::Npc>(target).unwrap().active());
        }
    }
}

#[test]
fn static_wall_uses_normal_closing_speed_instead_of_touch_or_total_speed() {
    for (along, across, crashes) in [
        (1.0 / 3.6, 0.0, false),
        (0.2, 30.0, false),
        (20.0, 0.0, true),
    ] {
        let mut app = game::tests::simulation();
        for _ in 0..3 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let t = *app.world().get::<Transform>(owner).unwrap();
        let forward = t.rotation * Vec3::Z;
        let right = t.rotation * Vec3::X;
        let before = forward * along + right * across;
        {
            let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
            rider.immunity = 0.0;
            rider.speed = before.length();
            rider.motion_velocity = before;
        }
        app.world_mut().entity_mut(owner).insert(BikeState {
            impact_velocity: before,
            ..default()
        });
        let wall = app
            .world_mut()
            .spawn((
                crate::world::Obstacle,
                Collider::cuboid(100.0, 2.0, 0.2),
                Transform::from_translation(t.translation + forward * 3.0 + Vec3::Y)
                    .with_rotation(t.rotation),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .write_message(recovery::SweptContact(owner, wall));
        app.update();
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage == RecoveryStage::Falling,
            crashes,
            "along={along} across={across}"
        );
    }
}

#[test]
fn parking_speed_vehicle_contacts_never_trigger_scripted_wipeouts() {
    use bevy::ecs::system::RunSystemOnce;
    for kind in 0..3 {
        let mut app = game::tests::simulation();
        for _ in 0..3 {
            app.update();
        }
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let pose = *app.world().get::<Transform>(owner).unwrap();
        let v = pose.rotation * Vec3::Z * (0.5 / 3.6);
        {
            let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
            r.immunity = 0.;
            r.motion_velocity = v;
            r.speed = v.length();
        }
        app.world_mut().entity_mut(owner).insert(BikeState {
            impact_velocity: v,
            ..default()
        });
        let e = app
            .world_mut()
            .spawn((
                RigidBody::Fixed,
                crate::world::Obstacle,
                Transform::from_translation(pose.translation + pose.rotation * Vec3::Z * 2.)
                    .with_rotation(pose.rotation),
                Collider::compound(vec![(
                    Vec3::Y * 0.95,
                    Quat::IDENTITY,
                    Collider::cuboid(0.95, 0.9, 1.95),
                )]),
            ))
            .id();
        if kind == 0 {
            app.world_mut().entity_mut(e).insert(Traffic {
                route: 0,
                progress: 0.,
                lane: 0.,
                speed: 0.,
            });
        }
        if kind == 1 {
            app.world_mut()
                .entity_mut(e)
                .insert(crate::environment::CrossTraffic);
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .write_message(recovery::SweptContact(owner, e));
        app.world_mut().run_system_once(recovery::impacts).unwrap();
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding,
            "vehicle kind {kind}"
        );
        assert_eq!(app.world().get::<Rider>(owner).unwrap().health, 100.);
    }
}

#[test]
fn vehicle_observations_preserve_both_world_velocities_and_collider_heights() {
    for (direction, kind, closing_kmh) in [
        (1., rr_core::contact::VehicleApproach::Rear, 40.),
        (-1., rr_core::contact::VehicleApproach::Oncoming, 160.),
    ] {
        for height in [1.2, 2.5] {
            let mut app = game::tests::simulation();
            for _ in 0..3 {
                app.update();
            }
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let t = *app.world().get::<Transform>(owner).unwrap();
            let forward = t.rotation * Vec3::Z;
            let incoming = forward * (100. / 3.6);
            {
                let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
                r.immunity = 0.;
                r.motion_velocity = incoming;
                r.speed = incoming.length();
            }
            app.world_mut().entity_mut(owner).insert(BikeState {
                impact_velocity: incoming,
                ..default()
            });
            let object = app
                .world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        progress: 500.,
                        lane: 0.,
                        speed: direction * 60. / 3.6,
                    },
                    Transform::from_translation(t.translation + forward * 10.).with_rotation(
                        t.rotation
                            * Quat::from_rotation_y(if direction < 0. {
                                std::f32::consts::PI
                            } else {
                                0.
                            }),
                    ),
                    Collider::cuboid(1., height / 2., 2.),
                ))
                .id();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut()
                .write_message(recovery::SweptContact(owner, object));
            app.update();
            let observations = app
                .world()
                .resource::<crate::contact_telemetry::VehicleContacts>();
            let sample = observations.0.back().unwrap();
            assert_eq!(sample.rider, owner);
            assert_eq!(sample.object, object);
            assert!(sample.incoming.distance(incoming) < 0.001);
            assert!(
                sample
                    .object_velocity
                    .distance(forward * (direction * 60. / 3.6))
                    < 0.001
            );
            assert_eq!(sample.approach, kind);
            assert!(
                (sample.closing_mps * 3.6 - closing_kmh).abs() < 0.01,
                "{sample:?}"
            );
            assert!((sample.object_height_m.unwrap() - height).abs() < 0.001);
            assert!(sample.crash);
        }
    }
}

#[test]
fn solid_contacts_interrupt_both_stagger_phases_without_launch_or_baseline_change() {
    for active in [false, true] {
        for side in [-1., 1.] {
            for age in [0.1, 0.4] {
                let mut app = game::tests::simulation();
                app.world_mut()
                    .resource_mut::<crate::biomechanics::Settings>()
                    .enabled = active;
                for _ in 0..3 {
                    app.update();
                }
                let owner = app
                    .world_mut()
                    .query_filtered::<Entity, With<Rider>>()
                    .single(app.world())
                    .unwrap();
                let t = *app.world().get::<Transform>(owner).unwrap();
                let incoming = t.rotation * Vec3::Z * 3.;
                {
                    let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
                    r.immunity = 0.;
                    r.speed = 3.;
                    r.motion_velocity = incoming;
                }
                app.world_mut().entity_mut(owner).insert(BikeState {
                    impact_velocity: incoming,
                    ..default()
                });
                if active {
                    let mut stagger = rr_core::crash_behavior::Stagger::new(side);
                    stagger.age = age;
                    app.world_mut().get_mut::<Recovery>(owner).unwrap().stagger = Some(stagger);
                }
                let wall = app
                    .world_mut()
                    .spawn((
                        crate::world::Obstacle,
                        Collider::cuboid(10., 2., 0.1),
                        Transform::from_translation(
                            t.translation + t.rotation * Vec3::Z * 4. + Vec3::Y,
                        )
                        .with_rotation(t.rotation),
                    ))
                    .id();
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                app.init_resource::<Entry>().add_systems(
                    FixedUpdate,
                    capture_entry
                        .after(SimSet::Gameplay)
                        .before(PhysicsSet::SyncBackend),
                );
                app.world_mut()
                    .write_message(recovery::SweptContact(owner, wall));
                app.update();
                let rec = app.world().get::<Recovery>(owner).unwrap();
                assert_eq!(
                    rec.stage,
                    if active {
                        RecoveryStage::Falling
                    } else {
                        RecoveryStage::Riding
                    }
                );
                if active {
                    assert!(rec.stagger.is_none());
                    let entry = &app.world().resource::<Entry>().0;
                    assert!(!entry.is_empty());
                    for v in entry {
                        assert!(v.linear.distance(incoming) < 0.001, "{v:?}");
                    }
                }
            }
        }
    }
}

/// Reproducible comparison evidence, not a guarantee of vault clearance. The
/// production traffic proxy is a solid box: test it rather than a ramp that
/// would silently make the experiment easier than the game.
#[test]
fn measured_solid_car_height_trajectories_keep_passive_control_and_finite_recovery() {
    for active in [false, true] {
        for height in [1.2, 2.5] {
            for car_kmh in [0., 60., -60.] {
                let mut app = game::tests::simulation();
                app.world_mut()
                    .resource_mut::<crate::biomechanics::Settings>()
                    .enabled = active;
                for _ in 0..3 {
                    app.update();
                }
                app.insert_resource(VehiclePhysicsAdapter::Dynamic);
                app.update();
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                let owner = app
                    .world_mut()
                    .query_filtered::<Entity, With<Rider>>()
                    .single(app.world())
                    .unwrap();
                let t = *app.world().get::<Transform>(owner).unwrap();
                let forward = (t.rotation * Vec3::Z).with_y(0.).normalize();
                {
                    let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
                    r.immunity = 0.;
                    r.speed = 100. / 3.6;
                    r.motion_velocity = forward * (100. / 3.6);
                }
                app.world_mut().get_mut::<Velocity>(owner).unwrap().linear = forward * (100. / 3.6);
                let r = app.world().get::<Rider>(owner).unwrap();
                let (progress, lane) = (r.progress + 5., r.lane);
                let (p, q) = app
                    .world()
                    .resource::<crate::track::Track>()
                    .sample(progress, lane);
                let object = app
                    .world_mut()
                    .spawn((
                        Traffic {
                            route: 0,
                            progress,
                            lane,
                            speed: car_kmh / 3.6,
                        },
                        Transform::from_translation(p).with_rotation(q),
                        RigidBody::KinematicPositionBased,
                        Collider::compound(vec![(
                            Vec3::Y * (height / 2.),
                            Quat::IDENTITY,
                            Collider::cuboid(0.95, height / 2., 1.95),
                        )]),
                        ActiveEvents::COLLISION_EVENTS,
                        ActiveCollisionTypes::all(),
                    ))
                    .id();
                let mut detached = false;
                let mut max_height = f32::NEG_INFINITY;
                let mut max_pitch = 0.0_f32;
                let mut cleared_front_above_roof = false;
                let mut previous_longitudinal = f32::NEG_INFINITY;
                for _ in 0..180 {
                    app.update();
                    let rec = app.world().get::<Recovery>(owner).unwrap();
                    if let Some(torso) = rec.torso {
                        detached = true;
                        let pose = app.world().get::<Transform>(torso).unwrap();
                        let velocity = app.world().get::<Velocity>(torso).unwrap();
                        assert!(pose.translation.is_finite() && pose.rotation.is_finite());
                        assert!(velocity.linear.is_finite() && velocity.angular.is_finite());
                        let car = app.world().get::<Transform>(object).unwrap();
                        let relative = pose.translation - car.translation;
                        max_height = max_height.max(relative.y);
                        let up = pose.rotation * Vec3::Y;
                        max_pitch = max_pitch.max(up.dot(forward).abs().clamp(0., 1.).asin());
                        let longitudinal = relative.dot(forward);
                        // Count only crossing the forward face above the solid
                        // roof while still inside its lateral footprint.
                        if previous_longitudinal <= 1.95
                            && longitudinal > 1.95
                            && relative.dot(q * Vec3::X).abs() < 0.95
                            && relative.y > height + 0.2
                        {
                            cleared_front_above_roof = true;
                        }
                        previous_longitudinal = longitudinal;
                    }
                }
                assert!(
                    detached,
                    "no actual contact active={active} height={height} car={car_kmh}"
                );
                println!(
                    "CAR_HEIGHT_TRAJECTORY active={active} rider_kmh=100 car_kmh={car_kmh} solid_height_m={height} max_torso_height_m={max_height:.4} max_torso_pitch_rad={max_pitch:.4} cleared_front_above_roof={cleared_front_above_roof} duration_ticks=180"
                );
            }
        }
    }
}

#[test]
fn downed_contact_lift_uses_pre_solver_relative_motion_for_both_bodies() {
    use bevy::ecs::system::RunSystemOnce;
    for kind in [recovery::BodyKind::Wreck, recovery::BodyKind::Ragdoll] {
        for (incoming, before, after, lifts) in [(2.0, 2.0, -5.0, false), (10.0, 0.0, 10.0, true)] {
            let mut app = game::tests::simulation();
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let pose = *app.world().get::<Transform>(owner).unwrap();
            let forward = pose.rotation * Vec3::Z;
            app.world_mut().entity_mut(owner).insert((
                BikeState {
                    impact_velocity: forward * incoming,
                    ..default()
                },
                Velocity::linear(Vec3::ZERO),
            ));
            app.world_mut()
                .get_mut::<Rider>(owner)
                .unwrap()
                .impact_cooldown = 0.0;
            let foreign = app.world_mut().spawn_empty().id();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let target = app
                .world_mut()
                .spawn((
                    OwnedBody {
                        owner: foreign,
                        kind,
                    },
                    pose.with_translation(pose.translation + forward),
                    Velocity::linear(forward * after),
                    recovery::PreviousMotion(forward * before),
                ))
                .id();
            app.world_mut()
                .write_message(recovery::SweptContact(owner, target));
            app.world_mut().run_system_once(recovery::impacts).unwrap();
            let rider = app.world().get::<Rider>(owner).unwrap();
            assert_eq!(
                rider.hop_velocity > 0.0,
                lifts,
                "wreck={} incoming={incoming} target_pre={before} target_post={after}",
                kind == recovery::BodyKind::Wreck
            );
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Riding
            );
            if !lifts {
                assert_eq!(
                    app.world().get::<Velocity>(target).unwrap().linear,
                    forward * after
                );
                assert_eq!(rider.impact_cooldown, 0.0);
            }
        }
    }
}
