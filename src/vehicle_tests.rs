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
        assert!(snapshot(&mut app).0 <= rr_core::reverse::MAX_SPEED + 0.5);
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
        assert!(stopped.0 <= rr_core::reverse::MAX_SPEED + 0.5);
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
        assert!(
            steady.yaw_rate.abs() < 0.01,
            "straight-line acceleration without steering acquired yaw: {}",
            steady.yaw_rate
        );
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
        results.push(serde_json::json!({"preset":preset,"zero_to_100_seconds":seconds,"brake_distance_from_10s_speed_m":distance,"neutral_yaw_rate":steady.yaw_rate,"steady_slip_angle":steady.slip_angle}));
    }
    let mut app = app(1, 60);
    // The handling soak deliberately leaves the authored road. Its original
    // 1.3 km floor ended after ~27 s, testing a fall off the fixture instead of
    // 600 s of chassis stability. Bound the whole possible 60 km drive.
    for mut ground in app
        .world_mut()
        .query_filtered::<&mut Collider, With<Ground>>()
        .iter_mut(app.world_mut())
    {
        *ground = Collider::cuboid(100_000.0, 0.5, 100_000.0);
    }
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
fn pedestrians_stay_mounted_while_cows_keep_existing_crash_entry() {
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
            let incoming = state.velocity;
            let height = if kind == crate::npc::NpcKind::Cow {
                1.7
            } else {
                crate::human::HEIGHT
            };
            let transform =
                Transform::from_translation((p + incoming.normalize() * 3.0).with_y(0.0));
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
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let mut crashed = false;
            for _ in 0..60 {
                app.update();
                if kind == crate::npc::NpcKind::Cow {
                    assert_eq!(app.world().get::<Rider>(owner).unwrap().hop_velocity, 0.0);
                }
                if app.world().get::<Recovery>(owner).unwrap().stage == RecoveryStage::Falling {
                    crashed = true;
                    break;
                }
            }
            assert_eq!(
                crashed,
                kind == crate::npc::NpcKind::Cow,
                "class {preset} {kind:?} unexpected crash state"
            );
            if kind == crate::npc::NpcKind::Pedestrian {
                assert!(snapshot(&mut app).0 > 2.0, "pedestrian stopped bike");
            }
            assert!(!app.world().get::<crate::npc::Npc>(actor).unwrap().active());
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

#[test]
fn a_held_turn_builds_steering_and_a_parked_bike_cannot_strafe() {
    let mut app = app(1, 60);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    for _ in 0..3 {
        app.update();
    }
    let start = snapshot(&mut app).1;
    for _ in 0..120 {
        app.update();
    }
    assert!(
        snapshot(&mut app).1.distance(start) < 0.1,
        "parked steering slid the bike sideways"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyD);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..180 {
        app.update();
    }
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    for _ in 0..6 {
        app.update();
    }
    let tap = snapshot(&mut app).2.intent.steer;
    for _ in 0..24 {
        app.update();
    }
    let held = snapshot(&mut app).2.intent.steer;
    assert!(
        held > tap * 4.0,
        "held turn failed to strengthen: tap {tap}, held {held}"
    );
}

#[test]
fn either_full_lean_arrives_within_six_tenths_and_straightens_without_input() {
    for (key, sign) in [(KeyCode::KeyA, -1.0), (KeyCode::KeyD, 1.0)] {
        let mut app = app(1, 60);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        for _ in 0..180 {
            app.update();
        }
        assert!(
            snapshot(&mut app).2.lean.abs() < 0.001,
            "road curvature introduced lean"
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        for _ in 0..36 {
            app.update();
        }
        assert!(
            snapshot(&mut app).2.lean * sign > 0.5,
            "full lean was too slow"
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(key);
        for _ in 0..60 {
            app.update();
        }
        assert!(
            snapshot(&mut app).2.lean.abs() < 0.001,
            "lean did not unwind on a curving road"
        );
    }
}

#[test]
fn stronger_full_lean_turns_both_directions_without_increasing_visual_bank() {
    for (key, sign) in [(KeyCode::KeyA, -1.0), (KeyCode::KeyD, 1.0)] {
        let mut results = Vec::new();
        for strength in [1.0, 1.25] {
            let mut app = app(1, 60);
            app.world_mut().resource_mut::<Tuning>().presets[1].turn_response = strength;
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            for _ in 0..180 {
                app.update();
            }
            let start = snapshot(&mut app).1;
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
            for _ in 0..60 {
                app.update();
            }
            let (_, end, state) = snapshot(&mut app);
            results.push(((start.x - end.x) * sign, state.lean.abs()));
        }
        assert!(
            results[1].0 > results[0].0 + 0.2,
            "stronger steering did not turn farther: {results:?}"
        );
        assert!(
            (results[1].1 - results[0].1).abs() < 0.01,
            "turn gain altered visual bank"
        );
    }
}

#[test]
fn supported_bike_recovers_from_contact_yaw_without_changing_steering_bank() {
    let mut app = app(1, 60);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..180 {
        app.update();
    }
    let e = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    app.world_mut().get_mut::<Velocity>(e).unwrap().angvel.y = 8.0;
    for _ in 0..30 {
        app.update();
    }
    let v = app.world().get::<Velocity>(e).unwrap();
    assert!(v.angvel.y.abs() < 1.3);
    assert!(app.world().get::<BikeState>(e).unwrap().lean.abs() < 0.01);
    assert_eq!(
        app.world().get::<Recovery>(e).unwrap().stage,
        RecoveryStage::Riding
    );
}

#[test]
fn neutral_player_heading_is_not_pulled_back_to_the_road() {
    let mut app = app(1, 60);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..180 {
        app.update();
    }
    let world = app.world_mut();
    let mut q = world.query_filtered::<&mut Transform, With<Rider>>();
    let mut t = q.single_mut(world).unwrap();
    t.rotation = Quat::from_rotation_y(0.25) * t.rotation;
    let before = t.rotation * Vec3::Z;
    for _ in 0..12 {
        app.update();
    }
    let world = app.world_mut();
    let after = world
        .query_filtered::<&Transform, With<Rider>>()
        .single(world)
        .unwrap()
        .rotation
        * Vec3::Z;
    assert!(
        before.angle_between(after) < 0.08,
        "neutral input auto-steered: {before:?} -> {after:?}"
    );
}

#[test]
fn abandoned_motorcycle_tips_onto_its_side_after_knockoff() {
    let mut app = crate::game::tests::simulation();
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    {
        let world = app.world_mut();
        let mut q = world.query::<&mut Rider>();
        let mut rider = q.single_mut(world).unwrap();
        rider.health = 0.0;
    }
    let mut tipped = false;
    for _ in 0..120 {
        app.update();
        let world = app.world_mut();
        for (body, t) in world
            .query::<(&crate::recovery::OwnedBody, &Transform)>()
            .iter(world)
        {
            if body.kind == crate::recovery::BodyKind::Wreck
                && (t.rotation * Vec3::Y).dot(Vec3::Y).abs() < 0.5
            {
                tipped = true;
            }
        }
    }
    assert!(tipped, "wreck stayed upright");
}

#[test]
fn dynamic_bike_physically_traverses_a_solid_low_wreck() {
    let mut app = app(1, 60);
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    app.world_mut().get_mut::<Rider>(owner).unwrap().immunity = 0.0;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..120 {
        app.update();
    }
    let start = *app.world().get::<Transform>(owner).unwrap();
    let forward = start.rotation * Vec3::Z;
    let foreign = app.world_mut().spawn_empty().id();
    let wreck = app
        .world_mut()
        .spawn((
            crate::recovery::OwnedBody {
                owner: foreign,
                kind: crate::recovery::BodyKind::Wreck,
            },
            RigidBody::Dynamic,
            Collider::cuboid(1.1, 0.3, 0.5),
            ColliderMassProperties::Mass(180.0),
            Transform::from_translation((start.translation + forward * 8.0).with_y(0.3))
                .with_rotation(start.rotation),
            Velocity::zero(),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ))
        .id();
    let mut touched = false;
    for _ in 0..120 {
        app.update();
        touched |= app
            .world()
            .get::<Velocity>(wreck)
            .unwrap()
            .linvel
            .with_y(0.0)
            .length()
            > 0.01;
        assert_eq!(
            app.world()
                .get::<crate::recovery::Recovery>(owner)
                .unwrap()
                .stage,
            crate::recovery::RecoveryStage::Riding
        );
    }
    let finish = app.world().get::<Transform>(owner).unwrap().translation;
    assert!(
        (finish - start.translation).dot(forward) > 12.0,
        "bike blocked: {start:?} -> {finish:?}"
    );
    assert!(touched, "solid wreck received no physical contact response");
    assert!(app.world().get::<ColliderDisabled>(wreck).is_none());
}

#[test]
fn remount_hands_off_at_live_wreck_not_old_chassis_or_road_height() {
    for support_body in [RigidBody::Fixed, RigidBody::KinematicPositionBased] {
        for dynamic in [false, true] {
            let mut app = app(1, 60);
            if !dynamic {
                app.insert_resource(VehiclePhysicsAdapter::Assisted);
            }
            for _ in 0..4 {
                app.update();
            }
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let (p, q) = app.world().resource::<Track>().sample(200.0, 2.0);
            let pickup = p.with_y(6.13);
            app.world_mut().spawn((
                support_body,
                Ground,
                Collider::cuboid(8.0, 3.0, 8.0),
                Transform::from_translation(p.with_y(3.0)),
            ));
            let wreck = app
                .world_mut()
                .spawn((
                    RigidBody::KinematicPositionBased,
                    Transform::from_translation(pickup).with_rotation(q),
                    Velocity::zero(),
                    crate::recovery::OwnedBody {
                        owner,
                        kind: crate::recovery::BodyKind::Wreck,
                    },
                ))
                .id();
            app.world_mut().entity_mut(owner).insert(ColliderDisabled);
            {
                let mut recovery = app.world_mut().get_mut::<Recovery>(owner).unwrap();
                recovery.stage = RecoveryStage::Mounting;
                recovery.elapsed = 0.75;
                recovery.wreck = Some(wreck);
            }
            app.update();
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Riding
            );
            let actual = app.world().get::<Transform>(owner).unwrap().translation;
            assert!(
                actual.distance(pickup) < 0.25,
                "dynamic={dynamic}: {actual:?} vs {pickup:?}"
            );
            if dynamic {
                assert_eq!(app.world().get::<Rider>(owner).unwrap().hop_height, 0.0);
                for _ in 0..8 {
                    app.update();
                }
                let actual = app.world().get::<Transform>(owner).unwrap().translation;
                assert!(
                    actual.distance(pickup) < 0.5,
                    "backend restored stale crash: {actual:?}"
                );
            }
        }
    }
}

#[test]
fn brake_backs_dynamic_bike_away_from_a_solid_wall_and_throttle_returns_forward() {
    let mut app = app(1, 60);
    for _ in 0..6 {
        app.update();
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let pose = *app.world().get::<Transform>(owner).unwrap();
    let forward = pose.rotation * Vec3::Z;
    app.world_mut().spawn((
        RigidBody::Fixed,
        crate::world::Obstacle,
        Collider::cuboid(4.0, 2.0, 0.3),
        Transform::from_translation(pose.translation + forward * 1.6 + Vec3::Y)
            .with_rotation(pose.rotation),
    ));
    // Drive into the wall first, so backing out tests an actual blocked chassis.
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    for _ in 0..30 {
        app.update();
    }
    let against_wall = snapshot(&mut app);
    let advance = (against_wall.1 - pose.translation).dot(forward);
    assert!(
        advance > 0.05 && advance < 0.8,
        "wall did not stop forward drive: {advance}"
    );
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyW);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    for _ in 0..180 {
        app.update();
    }
    let (_, position, state) = snapshot(&mut app);
    assert!(
        (position - pose.translation).dot(forward) < -2.0,
        "did not back out: {position:?}"
    );
    assert!(state.velocity.dot(forward) < -0.5 && state.velocity.length() < 3.5);
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Riding
    );
    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::KeyS);
        keys.press(KeyCode::KeyW);
    }
    for _ in 0..40 {
        app.update();
    }
    assert!(snapshot(&mut app).2.velocity.dot(forward) > 0.0);
}

#[test]
fn assisted_adapter_can_back_up_without_nitro_or_finishing_the_race() {
    let mut app = crate::game::tests::simulation();
    for _ in 0..3 {
        app.update();
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let start = app.world().get::<Rider>(owner).unwrap().progress;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    for _ in 0..180 {
        app.update();
    }
    let rider = app.world().get::<Rider>(owner).unwrap();
    assert!(rider.progress < start - 2.0);
    assert!(rider.speed < -0.5 && rider.speed >= -rr_core::reverse::MAX_SPEED);
    assert!(rider.finished.is_none());
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Riding
    );
}

#[test]
fn disabled_player_nitro_matches_normal_throttle_on_both_adapters() {
    for adapter in [
        VehiclePhysicsAdapter::Assisted,
        VehiclePhysicsAdapter::Dynamic,
    ] {
        let mut results = Vec::new();
        for shift in [false, true] {
            let mut a = app(1, 60);
            a.insert_resource(adapter);
            assert!(!a.world().resource::<Tuning>().player_nitro_enabled);
            {
                let mut keys = a.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.press(KeyCode::KeyW);
                if shift {
                    keys.press(KeyCode::ShiftLeft);
                }
            }
            for _ in 0..120 {
                a.update();
            }
            let world = a.world_mut();
            let (r, t) = world.query::<(&Rider, &Transform)>().single(world).unwrap();
            assert_eq!(r.nitro, 100.);
            assert!(!r.drive_intent.nitro);
            results.push((r.speed, t.translation));
        }
        assert!((results[0].0 - results[1].0).abs() < 0.0001, "{adapter:?}");
        assert!(results[0].1.distance(results[1].1) < 0.0001, "{adapter:?}");
    }
}

#[test]
fn ai_backing_intent_uses_the_existing_reverse_adapter() {
    let mut app = app(1, 60);
    {
        let w = app.world_mut();
        let mut rider = w.query::<&mut Rider>().single_mut(w).unwrap();
        rider.id = 1;
        assert!(rider.ai_drive.step(3., 0., true, true));
    }
    let mut minimum = 0.0_f32;
    for _ in 0..45 {
        app.update();
        let w = app.world_mut();
        let (r, v) = w.query::<(&Rider, &Velocity)>().single(w).unwrap();
        let forward = w
            .resource::<Track>()
            .sample_route(r.route, r.progress, 0.)
            .1
            * Vec3::Z;
        minimum = minimum.min(v.linvel.dot(forward));
    }
    assert!(minimum < -0.5, "AI never backed up: {minimum}");
    assert!(minimum >= -rr_core::reverse::MAX_SPEED - 0.5);
}

#[test]
fn crossing_recycle_has_no_140_metre_kinematic_velocity() {
    let mut app = crate::game::tests::simulation();
    let center = Vec3::new(100., 0., 100.);
    let e = app
        .world_mut()
        .spawn((
            crate::environment::Patrol {
                center,
                axis: Vec3::X,
                extent: 70.,
                speed: 9.,
                phase: 0.,
                one_way: true,
            },
            crate::world_gen::metropolis::Crossing {
                distance: 69.95,
                junction: center,
            },
            crate::environment::CrossTraffic,
            RigidBody::KinematicPositionBased,
            Collider::cuboid(0.95, 0.9, 1.95),
            Velocity::zero(),
            Transform::from_translation(center + Vec3::X * 69.95),
        ))
        .id();
    app.update();
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    for _ in 0..5 {
        app.update();
        let v = app.world().get::<Velocity>(e).unwrap().linvel.length();
        assert!(v < 20., "recycle produced contact speed {v}");
    }
    assert!(app.world().get::<Transform>(e).unwrap().translation.x < 40.);
}

#[test]
fn ai_routes_out_of_a_three_sided_trap_with_physical_motion() {
    let mut app = app(1, 60);
    let (origin, q) = {
        let w = app.world_mut();
        let (r, t) = w.query::<(&Rider, &Transform)>().single(w).unwrap();
        (
            t.translation,
            w.resource::<Track>()
                .sample_route(r.route, r.progress, 0.)
                .1,
        )
    };
    {
        let w = app.world_mut();
        let mut r = w.query::<&mut Rider>().single_mut(w).unwrap();
        r.id = 1;
        r.immunity = 1000.;
    }
    app.world_mut().resource_mut::<Race>().elapsed = 6.;
    for (offset, size) in [
        (Vec3::new(0., 1.5, 9.), Vec3::new(16., 3., 2.)),
        (Vec3::new(-7., 1.5, -1.), Vec3::new(2., 3., 22.)),
        (Vec3::new(7., 1.5, -1.), Vec3::new(2., 3., 22.)),
    ] {
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
            Transform::from_translation(origin + q * offset).with_rotation(q),
            crate::world::Obstacle,
        ));
    }
    let mut backed = false;
    let mut routed = false;
    let mut escaped = false;
    let mut furthest = f32::NEG_INFINITY;
    for _ in 0..2400 {
        app.update();
        let w = app.world_mut();
        let (r, t, recovery) = w
            .query::<(&Rider, &Transform, &Recovery)>()
            .single(w)
            .unwrap();
        let local = q.inverse() * (t.translation - origin);
        furthest = furthest.max(local.z);
        backed |= local.z < -12.;
        routed |= r.ai_escape.active();
        escaped |= local.z > 16.;
        assert_eq!(
            recovery.stage,
            RecoveryStage::Riding,
            "escape must use the bike, not a recovery teleport"
        );
        if escaped {
            break;
        }
    }
    assert!(
        routed && backed && escaped,
        "routed={routed}, backed={backed}, escaped={escaped}, furthest={furthest}"
    );
}

#[test]
fn queued_ai_physically_passes_a_stopped_car_without_escape_or_recovery() {
    let mut app = app(1, 60);
    let station = 20.;
    let (bike_position, heading, car_position) = {
        let track = app.world().resource::<Track>();
        let (p, q) = track.sample_route(0, station, 2.);
        (p, q, track.sample_route(0, station + 16., 2.).0)
    };
    {
        let w = app.world_mut();
        let (mut r, mut t) = w
            .query::<(&mut Rider, &mut Transform)>()
            .single_mut(w)
            .unwrap();
        r.id = 1;
        r.route = 0;
        r.progress = station;
        r.lane = 2.;
        r.speed = 0.;
        *t = Transform::from_translation(bike_position).with_rotation(heading);
    }
    app.world_mut().resource_mut::<Race>().elapsed = 6.;
    app.world_mut().spawn((
        crate::environment::CrossTraffic,
        RigidBody::Fixed,
        Collider::cuboid(0.95, 0.9, 1.95),
        Transform::from_translation(car_position + Vec3::Y * 0.9).with_rotation(heading),
    ));
    let mut passed = false;
    let mut moved_aside = false;
    let mut previous = bike_position;
    for _ in 0..1200 {
        app.update();
        let w = app.world_mut();
        let (r, t, recovery) = w
            .query::<(&Rider, &Transform, &Recovery)>()
            .single(w)
            .unwrap();
        assert_eq!(recovery.stage, RecoveryStage::Riding);
        assert!(
            !r.ai_escape.active(),
            "ordinary queue must not need the escape planner"
        );
        assert!(t.translation.distance(previous) < 2., "no teleport");
        previous = t.translation;
        moved_aside |= r.lane > 4.5;
        let bounds = crate::game::ai_free_bounds(w.resource::<Track>(), r.route, r.progress);
        assert!(
            r.lane > bounds.0 - 0.5 && r.lane < bounds.1 + 0.5,
            "pass left usable corridor: {} {bounds:?}",
            r.lane
        );
        if r.progress > station + 26. {
            passed = true;
            break;
        }
    }
    assert!(
        passed && moved_aside,
        "passed={passed}, moved_aside={moved_aside}"
    );
}

fn surface_speed(app: &mut App) -> f32 {
    let world = app.world_mut();
    world.query::<&Rider>().single(world).unwrap().speed
}
fn surface_fixture(surface: rr_core::surface::Surface, adapter: VehiclePhysicsAdapter) -> App {
    let mut app = app(1, 60);
    let mut definition = app.world().resource::<Track>().definition.clone();
    definition.closed = false;
    definition.checkpoints.last_mut().unwrap().fraction = 1.0;
    for (i, cp) in definition.control_points.iter_mut().enumerate() {
        cp.position = [0., 0.1, i as f32 * 400.];
        cp.surface = surface;
        cp.half_width = 12.;
        cp.bank_degrees = 0.;
    }
    app.insert_resource(Track::from_definition(definition).unwrap());
    app.insert_resource(adapter);
    let (p, q) = app.world().resource::<Track>().sample(10., 0.);
    let world = app.world_mut();
    let owner = world
        .query_filtered::<Entity, With<Rider>>()
        .single(world)
        .unwrap();
    *world.get_mut::<Transform>(owner).unwrap() = Transform::from_translation(p).with_rotation(q);
    world.get_mut::<Rider>(owner).unwrap().progress = 10.;
    world.get_mut::<Rider>(owner).unwrap().lane = 0.;
    world.spawn((
        RigidBody::Fixed,
        Ground,
        Collider::cuboid(100., 0.5, 10000.),
        Transform::from_xyz(0., -0.5, 0.),
    ));
    app
}
fn change_fixture_surface(app: &mut App, surface: rr_core::surface::Surface) {
    let mut definition = app.world().resource::<Track>().definition.clone();
    for cp in &mut definition.control_points {
        cp.surface = surface;
    }
    app.insert_resource(Track::from_definition(definition).unwrap());
}
#[test]
fn sand_penalty_compares_real_adapters_and_exits_without_velocity_clamps() {
    use rr_core::surface::Surface;
    for adapter in [
        VehiclePhysicsAdapter::Dynamic,
        VehiclePhysicsAdapter::Assisted,
    ] {
        let mut results = vec![];
        for surface in [
            Surface::Asphalt,
            Surface::Dirt,
            Surface::Gravel,
            Surface::Grass,
            Surface::Sand,
        ] {
            let mut app = surface_fixture(surface, adapter);
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            for _ in 0..120 {
                app.update();
            }
            let acceleration_speed = surface_speed(&mut app);
            for _ in 0..1080 {
                app.update();
            }
            let sustained = surface_speed(&mut app);
            println!(
                "surface={surface:?} adapter={adapter:?} two_seconds={acceleration_speed:.3} twenty_seconds={sustained:.3}"
            );
            assert!(sustained > 2.0, "sand must remain escapable");
            results.push((acceleration_speed, sustained));
            if surface == Surface::Sand {
                change_fixture_surface(&mut app, Surface::Asphalt);
                for _ in 0..360 {
                    app.update();
                }
                assert!(surface_speed(&mut app) > sustained + 10.);
                let before = surface_speed(&mut app);
                change_fixture_surface(&mut app, Surface::Sand);
                app.update();
                let entry = surface_speed(&mut app);
                assert!(entry > before - 1., "entry must not clamp velocity");
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .release(KeyCode::KeyW);
                for _ in 0..120 {
                    app.update();
                }
                assert!(surface_speed(&mut app) < entry - 5.);
            }
        }
        for other in &results[..4] {
            assert!(results[4].0 < other.0, "acceleration {results:?}");
            assert!(results[4].1 < other.1, "sustained {results:?}");
        }
        let mut app = surface_fixture(Surface::Sand, adapter);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyS);
        for _ in 0..360 {
            app.update();
        }
        let reverse = if adapter == VehiclePhysicsAdapter::Dynamic {
            let world = app.world_mut();
            let (pose, velocity) = world
                .query_filtered::<(&Transform, &Velocity), With<Rider>>()
                .single(world)
                .unwrap();
            velocity.linvel.dot(pose.rotation * Vec3::Z)
        } else {
            surface_speed(&mut app)
        };
        println!("{adapter:?} sand reverse after6s={reverse:.4}");
        assert!(
            reverse < -0.5,
            "reverse remains available: {adapter:?} {reverse}"
        );
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyS);
            keys.press(KeyCode::KeyW);
        }
        for _ in 0..360 {
            app.update();
        }
        assert!(surface_speed(&mut app) > 2., "forward exits reverse");
    }
}

#[test]
fn sand_coasting_and_ai_traversal_use_live_shared_adapter_rules() {
    use rr_core::surface::Surface;
    for adapter in [
        VehiclePhysicsAdapter::Dynamic,
        VehiclePhysicsAdapter::Assisted,
    ] {
        let mut coast = Vec::new();
        for surface in [
            Surface::Asphalt,
            Surface::Dirt,
            Surface::Gravel,
            Surface::Grass,
            Surface::Sand,
        ] {
            let mut app = surface_fixture(surface, adapter);
            for _ in 0..30 {
                app.update();
            }
            let world = app.world_mut();
            let mut q = world.query::<(&mut Rider, &Transform, Option<&mut Velocity>)>();
            let (mut rider, pose, velocity) = q.single_mut(world).unwrap();
            rider.speed = 30.;
            if let Some(mut velocity) = velocity {
                velocity.linvel = pose.rotation * Vec3::Z * 30.;
            }
            for _ in 0..60 {
                app.update();
            }
            coast.push(surface_speed(&mut app));
        }
        println!("{adapter:?} one_second_coast_from30={coast:?}");
        assert!(coast[..4].iter().all(|s| *s > coast[4] + 2.));
        let mut app = surface_fixture(Surface::Sand, adapter);
        let world = app.world_mut();
        world.query::<&mut Rider>().single_mut(world).unwrap().id = 1;
        for _ in 0..300 {
            app.update();
        }
        let world = app.world_mut();
        let rider = world.query::<&Rider>().single(world).unwrap();
        assert!(rider.speed > 2. && rider.speed < 18. && rider.crashed == 0.);
    }
}

#[test]
fn assisted_reverse_can_leave_authored_hills_sand_shoulder() {
    let mut app = crate::game::tests::simulation();
    app.insert_resource(Track::hills().unwrap());
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let (p, q) = app.world().resource::<Track>().sample_route(0, 8200., 10.);
    let world = app.world_mut();
    let owner = world
        .query_filtered::<Entity, With<Rider>>()
        .single(world)
        .unwrap();
    {
        let mut rider = world.get_mut::<Rider>(owner).unwrap();
        rider.progress = 8200.;
        rider.lane = 10.;
    }
    *world.get_mut::<Transform>(owner).unwrap() = Transform::from_translation(p).with_rotation(q);
    world
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    for _ in 0..360 {
        app.update();
    }
    let world = app.world_mut();
    let rider = world.query::<&Rider>().single(world).unwrap();
    assert!(rider.speed < -0.5, "sand shoulder reverse {}", rider.speed);
}

#[test]
fn reverse_heading_mirrors_keys_and_crosses_zero_in_both_adapters() {
    for adapter in [
        VehiclePhysicsAdapter::Dynamic,
        VehiclePhysicsAdapter::Assisted,
    ] {
        for (key, sign) in [(KeyCode::KeyD, 1.0), (KeyCode::KeyA, -1.0)] {
            let mut app = app(1, 60);
            app.insert_resource(adapter);
            for _ in 0..5 {
                app.update();
            }
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            for _ in 0..90 {
                app.update();
            }
            let initial = app.world().get::<Transform>(owner).unwrap().rotation;
            {
                let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.release(KeyCode::KeyW);
                keys.press(KeyCode::KeyS);
            }
            for _ in 0..240 {
                app.update();
            }
            let signed_speed = |app: &App| {
                let r = app.world().get::<Rider>(owner).unwrap();
                if adapter == VehiclePhysicsAdapter::Dynamic {
                    r.motion_velocity
                        .dot(app.world().get::<Transform>(owner).unwrap().rotation * Vec3::Z)
                } else {
                    r.speed
                }
            };
            assert!(signed_speed(&app) < -0.5);
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
            for _ in 0..45 {
                app.update();
            }
            let heading =
                initial.inverse() * app.world().get::<Transform>(owner).unwrap().rotation * Vec3::Z;
            assert!(
                heading.x * sign > 0.005,
                "reverse {adapter:?} {key:?}: {heading:?}"
            );
            {
                let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.release(KeyCode::KeyS);
                keys.press(KeyCode::KeyW);
            }
            let mut last = app.world().get::<Transform>(owner).unwrap().rotation;
            let mut forward = None;
            for _ in 0..90 {
                app.update();
                let now = app.world().get::<Transform>(owner).unwrap().rotation;
                assert!(now.angle_between(last) < 0.08, "zero-speed heading pop");
                last = now;
                if signed_speed(&app) > 1.0 && forward.is_none() {
                    forward = Some(now);
                }
            }
            assert!(
                forward.is_some(),
                "must actually cross back into forward travel"
            );
            if adapter == VehiclePhysicsAdapter::Dynamic {
                let delta = forward.unwrap().inverse() * last * Vec3::Z;
                assert!(
                    delta.x * sign < -0.005,
                    "forward heading retained: {delta:?}"
                );
            }
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Riding
            );
        }
    }
}
