use super::*;
use std::time::Duration;
fn app(preset: usize, fps: u32) -> App {
    let mut app = crate::game::tests::simulation();
    app.insert_resource(VehiclePhysicsAdapter::Dynamic);
    app.world_mut().resource_mut::<Tuning>().selected = preset;
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_secs_f64(1.0 / fps as f64),
    ));
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
    let world = app.world_mut();
    let mut query = world.query_filtered::<&mut Transform, With<Rider>>();
    *query.single_mut(world).unwrap() = Transform::from_translation(p).with_rotation(q);
    app
}
fn snapshot(app: &mut App) -> (f32, Vec3, BikeState) {
    let world = app.world_mut();
    let (r, b) = world.query::<(&Rider, &BikeState)>().single(world).unwrap();
    (r.speed, b.position, b.clone())
}
#[test]
fn dynamic_chassis_accelerates_and_brakes_with_two_supported_wheels() {
    for preset in 0..3 {
        let mut app = app(preset, 60);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        for _ in 0..360 {
            app.update();
        }
        let (speed, _, state) = snapshot(&mut app);
        println!(
            "preset{preset} speed {speed} grounded {:?} height {} rpm {}",
            state.grounded, state.position.y, state.rpm
        );
        assert!(speed > 27.7 && speed < 70.0);
        assert!(state.grounded.iter().all(|b| *b));
        assert!(state.rpm > 1200.0);
        assert!(state.position.y > -0.3 && state.position.y < 0.8);
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyW);
            keys.press(KeyCode::KeyS);
        }
        for _ in 0..360 {
            app.update();
        }
        assert!(snapshot(&mut app).0 < 1.0);
    }
}
#[test]
fn dynamic_physics_matches_at_render_rates() {
    let mut poses = vec![];
    for fps in [30, 60, 120] {
        let mut app = app(1, fps);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        for _ in 0..fps * 3 {
            app.update();
        }
        let accelerated = snapshot(&mut app);
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyW);
            keys.press(KeyCode::KeyS);
        }
        for _ in 0..fps * 3 {
            app.update();
        }
        let stopped = snapshot(&mut app);
        assert!(stopped.0 < 0.5);
        poses.push((accelerated, stopped));
    }
    for other in &poses[1..] {
        assert!(
            (poses[0].0.0 - other.0.0).abs() < 0.5,
            "speeds {:?}",
            poses.iter().map(|v| v.0.0).collect::<Vec<_>>()
        );
        assert!(poses[0].0.1.distance(other.0.1) < 1.0);
        assert!(poses[0].1.1.distance(other.1.1) < 1.0);
    }
}
#[test]
fn a_low_sidewalk_step_is_supported_without_a_wipeout() {
    let mut app = app(1, 60);
    let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
    let root = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(root)
        .insert(Transform::from_translation(p + Vec3::Y * 0.2).with_rotation(q));
    app.world_mut().spawn((
        Ground,
        RideableStep,
        crate::world::Obstacle,
        RigidBody::Fixed,
        Collider::cuboid(20.0, 0.08, 20.0),
        Transform::from_translation(p.with_y(0.08)),
    ));
    for _ in 0..180 {
        app.update();
    }
    let (_, position, state) = snapshot(&mut app);
    assert!(
        position.y > 0.1 && position.y < 0.6,
        "curb height {}",
        position.y
    );
    assert!(state.grounded.iter().all(|b| *b));
    let r = app
        .world_mut()
        .query::<&Rider>()
        .single(app.world())
        .unwrap();
    assert_eq!(r.crashed, 0.0);
}

#[test]
#[ignore = "600 simulated seconds; run explicitly for the Part 03 receipt"]
fn ten_minute_aggressive_turning_soak_and_numeric_benchmarks() {
    let mut results = vec![];
    for preset in 0..3 {
        let mut app = app(preset, 60);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        let mut zero_to_hundred = None;
        for tick in 0..600 {
            app.update();
            let (speed, _, _) = snapshot(&mut app);
            if speed >= 100.0 / 3.6 && zero_to_hundred.is_none() {
                zero_to_hundred = Some(tick as f32 / 60.0);
            }
        }
        let seconds = zero_to_hundred.expect("could not reach 100km/h");
        assert!((3.0..6.5).contains(&seconds));
        let steady = snapshot(&mut app).2;
        assert!(steady.yaw_rate.abs() > 0.01 && steady.yaw_rate.abs() < 1.0);
        let start = snapshot(&mut app).1;
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyW);
            keys.press(KeyCode::KeyS);
        }
        let mut distance = 0.0;
        let mut previous = start;
        for _ in 0..600 {
            app.update();
            let (speed, p, _) = snapshot(&mut app);
            distance += p.distance(previous);
            previous = p;
            if speed < 0.5 {
                break;
            }
        }
        results.push(serde_json::json!({"preset":preset,"zero_to_100_seconds":seconds,"brake_distance_from_10s_speed_m":distance,"steady_turn_yaw_rate":steady.yaw_rate,"steady_slip_angle":steady.slip_angle}));
    }
    let mut app = app(1, 60);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for tick in 0..36000 {
        // Keep this physics/handling fixture racing independently of the race gate.
        {
            let world = app.world_mut();
            let mut query = world.query::<&mut Rider>();
            let mut r = query.single_mut(world).unwrap();
            r.gates = default();
            r.finished = None;
        }
        if tick % 90 == 0 {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyA);
            keys.release(KeyCode::KeyD);
            keys.press(if tick % 180 == 0 {
                KeyCode::KeyA
            } else {
                KeyCode::KeyD
            });
        }
        app.update();
        let (speed, p, state) = snapshot(&mut app);
        assert!(p.is_finite() && state.velocity.is_finite() && state.velocity.length() < 100.0);
        assert!(
            p.y > -0.5 && p.y < 3.0,
            "unstable vertical state at {tick}: {p:?}"
        );
        assert!(speed < 100.0);
    }
    println!(
        "PART03_BENCHMARKS {}",
        serde_json::json!({"scenarios":results,"soak_simulated_seconds":600,"fixed_hz":60})
    );
}

#[test]
fn leaning_chassis_recovers_and_a_moving_bike_climbs_a_sidewalk_edge() {
    let mut app = app(1, 60);
    let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    app.world_mut()
        .entity_mut(owner)
        .insert(Transform::from_translation(p).with_rotation(q * Quat::from_rotation_z(0.35)));
    for _ in 0..120 {
        app.update();
    }
    let t = *app.world().get::<Transform>(owner).unwrap();
    assert!(
        (t.rotation * Vec3::Y).dot(Vec3::Y) > 0.99,
        "upright assist did not recover"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..120 {
        app.update();
    }
    let t = *app.world().get::<Transform>(owner).unwrap();
    let forward = t.rotation * Vec3::Z;
    let curb = t.translation + forward * 14.0;
    app.world_mut().spawn((
        Ground,
        RideableStep,
        crate::world::Obstacle,
        RigidBody::Fixed,
        Collider::cuboid(8.0, 0.08, 10.0),
        Transform::from_translation(curb.with_y(0.08)).with_rotation(t.rotation),
    ));
    let mut rose = false;
    for _ in 0..120 {
        app.update();
        let (_, p, _) = snapshot(&mut app);
        rose |= p.y > 0.1;
    }
    assert!(rose, "suspension did not climb curb");
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Riding
    );
}
#[test]
fn handling_import_rejects_invalid_presets() {
    let mut data: serde_json::Value =
        serde_json::from_str(include_str!("../content/handling.json")).unwrap();
    data["presets"][0]["grip"] = serde_json::json!(-1.0);
    assert!(Tuning::from_json(&data.to_string()).is_err());
    data["presets"][0]["grip"] = serde_json::json!(1.0);
    data["presets"][1]["id"] = data["presets"][0]["id"].clone();
    assert!(Tuning::from_json(&data.to_string()).is_err());
}

#[test]
fn dynamic_grid_can_launch_without_ai_sideswipe_or_melee_knockouts() {
    let mut app = app(1, 60);
    for id in 1..8 {
        let r = Rider::new(id);
        let (p, q) = app.world().resource::<Track>().sample(r.progress, r.lane);
        app.world_mut().spawn((
            r,
            Transform::from_translation(p).with_rotation(q),
            Visibility::default(),
            crate::recovery::bike_body(),
        ));
    }
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..120 {
        app.update();
    }
    assert!(
        app.world_mut()
            .query::<(&Rider, &Recovery)>()
            .iter(app.world())
            .all(|(r, s)| r.health == 100.0 && s.stage == RecoveryStage::Riding)
    );
}

#[test]
fn dynamic_remount_does_not_reapply_precrash_momentum() {
    let mut app = app(1, 60);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..180 {
        app.update();
    }
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyW);
    {
        let world = app.world_mut();
        let mut q = world.query::<&mut Rider>();
        q.single_mut(world).unwrap().health = 0.0;
    }
    app.update();
    let mut mounted = false;
    for _ in 0..2400 {
        app.update();
        let world = app.world_mut();
        let (r, recovery, state) = world
            .query::<(&Rider, &Recovery, &BikeState)>()
            .single(world)
            .unwrap();
        if recovery.stage == RecoveryStage::Riding {
            assert!(
                state.velocity.length() < 1.0,
                "remount launched stale velocity {:?}",
                state.velocity
            );
            assert_eq!(r.health, 100.0);
            mounted = true;
            break;
        }
    }
    assert!(mounted, "dynamic recovery stuck");
}

#[test]
fn neutral_assistance_never_magnets_a_stationary_bike_to_lane_zero() {
    for assist in [0.15, 0.6] {
        let mut app = app(1, 60);
        app.world_mut().resource_mut::<Tuning>().assist = assist;
        let (p, q) = app.world().resource::<Track>().sample(-8.0, 3.0);
        {
            let world = app.world_mut();
            let mut query = world.query::<(&mut Rider, &mut Transform)>();
            let (mut r, mut t) = query.single_mut(world).unwrap();
            r.lane = 3.0;
            *t = Transform::from_translation(p).with_rotation(q);
        }
        for _ in 0..180 {
            app.update();
        }
        let world = app.world_mut();
        let r = world.query::<&Rider>().single(world).unwrap();
        assert!(
            r.lane > 2.8 && r.lane < 3.2,
            "lane magnet moved to {}",
            r.lane
        );
    }
}
#[test]
fn abandoned_bike_keeps_the_selected_presets_dry_mass() {
    for preset in 0..3 {
        let mut app = app(preset, 60);
        for _ in 0..4 {
            app.update();
        }
        {
            let world = app.world_mut();
            let mut q = world.query::<&mut Rider>();
            q.single_mut(world).unwrap().health = 0.0;
        }
        app.update();
        let expected = app.world().resource::<Tuning>().presets[preset].dry_mass;
        let world = app.world_mut();
        let masses: Vec<_> = world
            .query::<(&crate::recovery::OwnedBody, &ColliderMassProperties)>()
            .iter(world)
            .filter(|(body, _)| body.kind == crate::recovery::BodyKind::Wreck)
            .map(|(_, m)| *m)
            .collect();
        assert_eq!(masses, vec![ColliderMassProperties::Mass(expected)]);
    }
}

#[test]
fn low_speed_inverted_chassis_self_rights_without_inverted_wheel_lift() {
    let mut app = app(1, 60);
    let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    app.world_mut().entity_mut(owner).insert(
        Transform::from_translation(p + Vec3::Y * 1.8)
            .with_rotation(q * Quat::from_rotation_z(std::f32::consts::PI)),
    );
    for _ in 0..360 {
        app.update();
    }
    let t = app.world().get::<Transform>(owner).unwrap();
    assert!(
        (t.rotation * Vec3::Y).dot(Vec3::Y) > 0.98,
        "inverted bike failed self right {:?}",
        t.rotation
    );
    assert!(
        t.translation.y > -0.3 && t.translation.y < 0.8,
        "unstable self-right height {}",
        t.translation.y
    );
}

#[test]
fn analog_trigger_values_reach_the_chassis_force_model() {
    let mut speeds = vec![];
    for amount in [0.25, 1.0] {
        let mut app = app(1, 60);
        let mut pad = Gamepad::default();
        // Inverse deadzone: 0.3625 becomes normalized 0.25 at the default 0.15.
        pad.analog_mut()
            .set(GamepadButton::RightTrigger2, 0.15 + amount * 0.85);
        app.world_mut().spawn(pad);
        for _ in 0..120 {
            app.update();
        }
        speeds.push(snapshot(&mut app).0);
    }
    assert!(
        speeds[0] > 0.5 && speeds[0] < speeds[1] * 0.65,
        "analog acceleration {speeds:?}"
    );
}

#[test]
fn both_civilian_kinds_launch_each_bike_class_above_their_standing_height() {
    for preset in 0..3 {
        for kind in [crate::npc::NpcKind::Cow, crate::npc::NpcKind::Pedestrian] {
            let mut app = app(preset, 60);
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            for _ in 0..120 {
                app.update();
            }
            let (_, p, state) = snapshot(&mut app);
            let forward = state.velocity.with_y(0.0).normalize();
            let height = if kind == crate::npc::NpcKind::Cow {
                1.7
            } else {
                crate::human::HEIGHT
            };
            let transform = Transform::from_translation((p + forward * 3.0).with_y(0.0));
            let actor = app
                .world_mut()
                .spawn((
                    crate::npc::Npc::new(kind, transform),
                    transform,
                    Visibility::default(),
                    RigidBody::Fixed,
                    Collider::compound(vec![(
                        Vec3::Y * height * 0.5,
                        Quat::IDENTITY,
                        Collider::cuboid(0.8, height * 0.5, 0.6),
                    )]),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::all(),
                ))
                .id();
            let mut peak = 0.0f32;
            let mut passed = false;
            for _ in 0..240 {
                app.update();
                let (_, position, state) = snapshot(&mut app);
                assert!(position.is_finite() && state.velocity.is_finite());
                peak = peak.max(position.y);
                passed |= (position - transform.translation).dot(forward) > 3.0;
            }
            assert!(
                peak > height + 0.25,
                "class {preset} {kind:?} clearance peak {peak}"
            );
            assert!(passed, "class {preset} {kind:?} stuck behind actor");
            println!(
                "HAZARD_CLEARANCE {}",
                serde_json::json!({"preset":preset,"kind":format!("{kind:?}"),"height_m":height,"peak_bike_origin_m":peak,"passed_obstacle":passed})
            );
            let world = app.world_mut();
            assert_eq!(
                world.query::<&Recovery>().single(world).unwrap().stage,
                RecoveryStage::Riding
            );
            assert!(
                !app.world().get::<crate::npc::Npc>(actor).unwrap().active(),
                "civilian did not ragdoll"
            );
        }
    }
}

#[test]
fn cross_traffic_contacts_trigger_a_wipeout_in_both_adapters() {
    for adapter in [
        VehiclePhysicsAdapter::Assisted,
        VehiclePhysicsAdapter::Dynamic,
    ] {
        for moving in [false, true] {
            let mut app = app(1, 60);
            app.insert_resource(adapter);
            if !moving {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(KeyCode::KeyW);
            }
            for _ in 0..120 {
                app.update();
            }
            let world = app.world_mut();
            let t = *world
                .query_filtered::<&Transform, With<Rider>>()
                .single(world)
                .unwrap();
            let forward = t.rotation * Vec3::Z;
            let axis = t.rotation * Vec3::X;
            let car = app
                .world_mut()
                .spawn((
                    crate::environment::CrossTraffic,
                    RigidBody::KinematicPositionBased,
                    Collider::compound(vec![(
                        Vec3::Y,
                        Quat::IDENTITY,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    )]),
                    Transform::from_translation(if moving {
                        t.translation - axis * 6.0
                    } else {
                        t.translation + forward * 4.0
                    })
                    .with_rotation(t.rotation * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::all(),
                    Ccd::enabled(),
                ))
                .id();
            if moving {
                let elapsed = app.world().resource::<Race>().elapsed;
                app.world_mut()
                    .entity_mut(car)
                    .insert(crate::environment::Patrol {
                        center: t.translation,
                        axis,
                        extent: 10.0,
                        speed: 9.0,
                        phase: 0.4 - elapsed * 0.9,
                        one_way: true,
                    });
            }
            let mut crashed = false;
            for _ in 0..60 {
                app.update();
                let world = app.world_mut();
                crashed |= world.query::<&Recovery>().single(world).unwrap().stage
                    != RecoveryStage::Riding;
            }
            assert!(
                crashed,
                "cross traffic failed to wipe out {adapter:?}, moving {moving}"
            );
        }
    }
}
