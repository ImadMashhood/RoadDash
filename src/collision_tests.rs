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
                    (v.linvel.dot(forward) * 3.6 - 80.0).abs() < 0.001,
                    "id={id} direction={direction}: {v:?}"
                );
                assert!(v.angvel.length() < 0.001, "scripted spin {v:?}");
                assert!(v.linvel.y <= 0.001, "scripted launch {v:?}");
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
            app.world_mut().get_mut::<Velocity>(owner).unwrap().angvel.y = 15.0;
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
                linvel: Vec3::new(0.0, 70.0, -40.0),
                angvel: Vec3::splat(30.0),
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
            let omega = (t.rotation * m.principal_inertia_local_frame).inverse() * v.angvel;
            energy += m.mass * v.linvel.length_squared()
                + (m.principal_inertia * omega * omega).element_sum();
            budget += m.mass * (400.0 + 2.0 * 9.81 * (20.0 - t.translation.y));
            momentum += v.linvel * m.mass;
            assert!(v.linvel.is_finite() && v.angvel.is_finite());
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
        app.world_mut().get_mut::<Velocity>(owner).unwrap().linvel = forward * (120.0 / 3.6);
        let r = app.world().get::<Rider>(owner).unwrap();
        let progress = r.progress + 5.0;
        let lane = r.lane;
        let (p, q) = app
            .world()
            .resource::<crate::track::Track>()
            .sample(progress, lane);
        app.world_mut().spawn((
            Traffic {
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
        app.world_mut().get_mut::<Velocity>(owner).unwrap().linvel = forward * 25.0;
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
fn rear_hops_work_in_both_entity_orders_and_do_not_replace_head_on_crashes() {
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
                assert!(app.world().get::<Rider>(attacker).unwrap().hop_velocity > 0.0);
                assert_eq!(
                    app.world().get::<Recovery>(victim).unwrap().stage,
                    RecoveryStage::Falling
                );
            }
        }
    }
}
