//! Production fixed-step regressions for RFU-AI, with normal solid bike bodies.
use super::tests::simulation;
use super::*;

#[test]
fn close_rider_pairs_and_chains_make_bounded_progress_without_mutual_stopping() {
    for count in [2, 4] {
        let mut app = simulation();
        let mut definition = app.world().resource::<Track>().definition.clone();
        for point in &mut definition.control_points {
            point.half_width = 6.5;
        }
        app.insert_resource(Track::from_definition(definition).unwrap());
        app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
        let mut owners = vec![];
        for index in 0..count {
            let mut rider = Rider::new(index + 1);
            rider.progress = 30. + index as f32 * 8.;
            rider.lane = 2.;
            // Isolate driving feedback, retaining solid bodies and real forces.
            rider.attack = 1000.;
            let (p, q) = app
                .world()
                .resource::<Track>()
                .sample(rider.progress, rider.lane);
            owners.push(
                app.world_mut()
                    .spawn((
                        rider,
                        Transform::from_translation(p).with_rotation(q),
                        Visibility::Visible,
                        recovery::bike_body(),
                    ))
                    .id(),
            );
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Race>().elapsed = 10.;
        let mut stopped = vec![0_u32; count];
        let mut max_stopped = vec![0_u32; count];
        for _ in 0..900 {
            app.update();
            for (i, owner) in owners.iter().enumerate() {
                let r = app.world().get::<Rider>(*owner).unwrap();
                stopped[i] = if r.speed < 0.5 { stopped[i] + 1 } else { 0 };
                max_stopped[i] = max_stopped[i].max(stopped[i]);
                assert_eq!(
                    app.world().get::<Recovery>(*owner).unwrap().stage,
                    RecoveryStage::Riding
                );
                assert_eq!(r.health, 100.);
            }
        }
        for (i, owner) in owners.iter().enumerate() {
            let r = app.world().get::<Rider>(*owner).unwrap();
            println!(
                "RFU-AI count={count} id={} station={} speed={} decision={:?} target={:?} stopped_ticks={}",
                r.id,
                r.progress,
                r.speed,
                r.combat_mind.decision,
                r.combat_mind.target_id(),
                max_stopped[i]
            );
            assert!(r.progress > 130. && r.speed > 5. && max_stopped[i] < 300);
        }
    }
}

#[test]
fn sidewalk_flee_goal_rejects_solid_outer_wall_before_physical_contact() {
    let mut app = simulation();
    let mut definition = app.world().resource::<Track>().definition.clone();
    for point in &mut definition.control_points {
        point.half_width = 6.5;
    }
    app.insert_resource(Track::from_definition(definition).unwrap());
    app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
    let mut rider = Rider::new(1);
    rider.progress = 0.;
    rider.lane = 2.;
    rider.speed = 20.;
    rider.attack = 1000.;
    // The nearby attacker requests the blocked right-side sidewalk line.
    while rider.combat_mind.struck_by(0) != rr_core::rival_ai::Response::Flee {}
    let (p, q) = app.world().resource::<Track>().sample(0., 2.);
    rider.motion_velocity = q * Vec3::Z * rider.speed;
    let owner = app
        .world_mut()
        .spawn((
            rider,
            Transform::from_translation(p).with_rotation(q),
            Visibility::Visible,
            recovery::bike_body(),
        ))
        .id();
    // Place the existing player attacker inside the ten-metre response
    // radius. Without a nearby target Flee requests the current lane, so the
    // test would never exercise the blocked sidewalk goal.
    let (attacker_position, attacker_rotation) = app.world().resource::<Track>().sample(0., -2.);
    let mut player_query = app.world_mut().query::<(Entity, &Rider)>();
    let attacker_entity = player_query
        .iter(app.world())
        .find(|(_, r)| r.id == 0)
        .unwrap()
        .0;
    let mut attacker = app.world_mut().get_mut::<Rider>(attacker_entity).unwrap();
    attacker.progress = 0.;
    attacker.lane = -2.;
    attacker.attack = 1000.;
    *app.world_mut()
        .get_mut::<Transform>(attacker_entity)
        .unwrap() = Transform::from_translation(attacker_position).with_rotation(attacker_rotation);
    let (p, q) = app.world().resource::<Track>().sample(40., 5.);
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::cuboid(2.0, 1.0, 3.0),
        Transform::from_translation(p + Vec3::Y).with_rotation(q),
    ));
    // Rapier must import the resident fixed wall before the first racing plan.
    for _ in 0..4 {
        app.update();
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    app.world_mut().resource_mut::<Race>().elapsed = 10.;
    for _ in 0..3 {
        app.update();
    }
    let first_plan = app.world().get::<Rider>(owner).unwrap().ai_plan.unwrap();
    let first_rider = app.world().get::<Rider>(owner).unwrap();
    assert_eq!(
        first_rider.combat_mind.decision,
        rr_core::rival_ai::Decision::Flee
    );
    assert!(
        first_rider.ai_static_guard_until > app.world().resource::<Race>().elapsed,
        "blocked flee goal had no resident-static rejection: {first_plan:?}"
    );
    assert!(
        first_plan.lane < 3.5,
        "solid sidewalk goal was accepted: {first_plan:?}"
    );
    let mut passed = false;
    for _ in 0..600 {
        app.update();
        let r = app.world().get::<Rider>(owner).unwrap();
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        assert_eq!(r.health, 100.);
        if r.progress > 50. {
            passed = true;
        }
    }
    let final_rider = app.world().get::<Rider>(owner).unwrap();
    assert!(
        passed,
        "the rider did not clear the solid-walled sidewalk: station={} lane={} speed={} plan={:?}",
        final_rider.progress, final_rider.lane, final_rider.speed, final_rider.ai_plan
    );
}

#[test]
fn rider_physically_passes_in_opposite_lane_and_returns() {
    let mut app = simulation();
    let mut definition = app.world().resource::<Track>().definition.clone();
    for point in &mut definition.control_points {
        point.half_width = 3.;
    }
    app.insert_resource(Track::from_definition(definition).unwrap());
    app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
    let mut rider = Rider::new(1);
    rider.progress = 20.;
    rider.lane = 2.;
    rider.speed = 20.;
    rider.attack = 1000.;
    let (p, q) = app
        .world()
        .resource::<Track>()
        .sample(rider.progress, rider.lane);
    rider.motion_velocity = q * Vec3::Z * rider.speed;
    let owner = app
        .world_mut()
        .spawn((
            rider,
            Transform::from_translation(p).with_rotation(q),
            Visibility::Visible,
            recovery::bike_body(),
        ))
        .id();
    let (p, q) = app.world().resource::<Track>().sample(100., 2.);
    let car = app
        .world_mut()
        .spawn((
            Traffic {
                route: 0,
                progress: 100.,
                lane: 2.,
                speed: 10.,
            },
            Transform::from_translation(p).with_rotation(q),
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y * 0.95,
                Quat::IDENTITY,
                Collider::cuboid(0.95, 0.9, 1.95),
            )]),
        ))
        .id();
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    app.world_mut().resource_mut::<Race>().elapsed = 10.;
    let mut borrowed = false;
    let mut returned = false;
    let mut minimum = f32::MAX;
    for _ in 0..1200 {
        app.update();
        let r = app.world().get::<Rider>(owner).unwrap();
        let c = app.world().get::<Traffic>(car).unwrap();
        borrowed |= r.lane < -1.;
        returned |= borrowed && r.progress > c.progress + 15. && r.lane > 0.65;
        let a = app.world().get::<Transform>(owner).unwrap().translation;
        let b = app.world().get::<Transform>(car).unwrap().translation;
        minimum = minimum.min(a.xz().distance(b.xz()));
        assert_eq!(r.health, 100.);
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        if returned {
            break;
        }
    }
    let r = app.world().get::<Rider>(owner).unwrap();
    println!(
        "RFU-AI opposite pass borrowed={borrowed} returned={returned} minimum={minimum} progress={} lane={} speed={} plan={:?}",
        r.progress, r.lane, r.speed, r.ai_plan
    );
    assert!(borrowed && returned && minimum > 2.);
}

#[test]
fn role_free_rider_chain_physically_passes_a_stationary_human_player() {
    for (count, width, compressed) in [
        (1, 6.5, false),
        (3, 6.5, false),
        (3, 3.0, false),
        (3, 3.0, true),
    ] {
        let mut app = simulation();
        let mut definition = app.world().resource::<Track>().definition.clone();
        for point in &mut definition.control_points {
            point.half_width = width;
        }
        app.insert_resource(Track::from_definition(definition).unwrap());
        app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
        let player = app
            .world_mut()
            .query::<(Entity, &Rider)>()
            .iter(app.world())
            .find(|(_, r)| r.id == 0)
            .unwrap()
            .0;
        // On the narrow road neither outbound edge clears the parked bike.
        let lane = if width < 4. { 1. } else { 2. };
        let (p, q) = app.world().resource::<Track>().sample(100., lane);
        let mut human = Rider::new(0);
        human.progress = 100.;
        human.lane = lane;
        app.world_mut().entity_mut(player).insert((
            human,
            Transform::from_translation(p).with_rotation(q),
            Velocity::zero(),
            crate::vehicle::BikeState {
                position: p,
                yaw: q.to_euler(EulerRot::YXZ).0,
                ..default()
            },
        ));
        let mut followers = Vec::new();
        for id in 1..=count {
            let mut r = Rider::new(id);
            r.progress = if compressed {
                100. - id as f32 * 4.7
            } else {
                65. - id as f32 * 15.
            };
            r.lane = lane;
            r.speed = if compressed { 0. } else { 20. };
            r.attack = 1000.; // Driving fixture; keep all physical bodies and contacts.
            let (p, q) = app.world().resource::<Track>().sample(r.progress, r.lane);
            r.motion_velocity = q * Vec3::Z * r.speed;
            followers.push(
                app.world_mut()
                    .spawn((
                        r,
                        Transform::from_translation(p).with_rotation(q),
                        Visibility::Visible,
                        recovery::bike_body(),
                    ))
                    .id(),
            );
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Race>().elapsed = 10.;
        let mut stops = vec![0; count];
        let mut longest = vec![0; count];
        let mut reverse_pulses = vec![0; count];
        let mut was_reversing = vec![false; count];
        let mut minimum = f32::MAX;
        let mut borrowed = false;
        let mut elapsed_ticks = 0;
        for tick in 0..if compressed { 1800 } else { 1200 } {
            app.update();
            elapsed_ticks = tick + 1;
            let parked = app.world().get::<Rider>(player).unwrap();
            assert!(parked.human_controlled() && parked.speed.abs() < 0.5);
            assert_eq!(parked.health, 100.);
            assert_eq!(
                app.world().get::<Recovery>(player).unwrap().stage,
                RecoveryStage::Riding
            );
            let human_position = app.world().get::<Transform>(player).unwrap().translation;
            for (index, entity) in followers.iter().enumerate() {
                let r = app.world().get::<Rider>(*entity).unwrap();
                let p = app.world().get::<Transform>(*entity).unwrap().translation;
                minimum = minimum.min(p.xz().distance(human_position.xz()));
                stops[index] = if r.speed < 0.5 { stops[index] + 1 } else { 0 };
                longest[index] = longest[index].max(stops[index]);
                let reversing = r.ai_drive.reversing();
                reverse_pulses[index] += usize::from(reversing && !was_reversing[index]);
                was_reversing[index] = reversing;
                borrowed |= r.lane < -0.65;
                if width < 4. && tick % 120 == 0 {
                    println!(
                        "NARROW compressed={compressed} tick={tick} id={} station={} lane={} speed={} target={:?} decision={:?} plan={:?} committed={:?} home={:?} escape={} throttle={} brake={}",
                        r.id,
                        r.progress,
                        r.lane,
                        r.speed,
                        r.combat_mind.target_id(),
                        r.combat_mind.decision,
                        r.ai_plan,
                        r.ai_pass.committed_lane(),
                        r.ai_pass.home_lane(),
                        r.ai_escape.active(),
                        r.drive_intent.throttle_strength(),
                        r.drive_intent.brake_strength()
                    );
                }
                let bounds = ai_free_bounds(app.world().resource::<Track>(), r.route, r.progress);
                assert!(
                    r.lane > bounds.0 - 0.5 && r.lane < bounds.1 + 0.5,
                    "compressed={compressed} id={} left usable corridor at {} {bounds:?}",
                    r.id,
                    r.lane
                );
                assert_eq!(r.health, 100.);
                assert_eq!(
                    app.world().get::<Recovery>(*entity).unwrap().stage,
                    RecoveryStage::Riding
                );
            }
            if followers.iter().all(|entity| {
                let r = app.world().get::<Rider>(*entity).unwrap();
                r.progress > parked.progress + 30. && r.speed > 5.
            }) {
                break;
            }
        }
        let parked = app.world().get::<Rider>(player).unwrap();
        let progress: Vec<_> = followers
            .iter()
            .map(|e| app.world().get::<Rider>(*e).unwrap().progress)
            .collect();
        println!(
            "RFU-AI stationary human count={count} width={width} compressed={compressed} player={} progress={progress:?} longest_stop_ticks={longest:?} reverse_pulses={reverse_pulses:?} elapsed_ticks={elapsed_ticks} minimum={minimum} borrowed={borrowed}",
            parked.progress
        );
        assert!(progress.iter().all(|p| *p > parked.progress + 30.));
        assert!(longest.iter().all(|ticks| *ticks < 300));
        if compressed {
            assert!(reverse_pulses.iter().all(|count| *count > 0 && *count <= 2));
        } else {
            assert!(reverse_pulses.iter().all(|count| *count == 0));
        }
        // Match the existing solid-bike fixture's clearance; cars need a wider envelope.
        assert!(minimum > 1.2);
        if width < 4. {
            assert!(borrowed);
        }
    }
}

#[test]
fn racer_avoids_actual_incoming_civilian_lane_change() {
    let mut app = simulation();
    app.insert_resource(Track::metropolis().unwrap());
    // Real sampler surface, with enough road before/after the encounter. The
    // legacy simulation track intentionally has no civilian passing policy.
    let mut vertices = vec![];
    let mut indices = vec![];
    for i in 0..=180_u32 {
        let station = 1000. + i as f32 * 5.;
        for lane in [-9., 9.] {
            vertices.push(
                app.world()
                    .resource::<Track>()
                    .street_sample(0, station, lane)
                    .0,
            );
        }
        if i > 0 {
            let k = i * 2;
            indices.extend([[k - 2, k - 1, k], [k - 1, k + 1, k]]);
        }
    }
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::trimesh(vertices, indices).unwrap(),
        world::Ground,
        Transform::default(),
    ));
    app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
    let existing: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .iter(app.world())
        .collect();
    for entity in existing {
        app.world_mut().despawn(entity);
    }
    let mut rider = Rider::new(1);
    rider.progress = 1150.;
    rider.lane = -3.1;
    rider.speed = 25.;
    rider.attack = 1000.;
    let (p, q) = app
        .world()
        .resource::<Track>()
        .sample(rider.progress, rider.lane);
    rider.motion_velocity = q * Vec3::Z * rider.speed;
    let owner = app
        .world_mut()
        .spawn((
            rider,
            Transform::from_translation(p).with_rotation(q),
            Visibility::Visible,
            recovery::bike_body(),
        ))
        .id();
    let mut cars = vec![];
    for (progress, speed) in [(1400., -18.), (1370., -6.)] {
        let track = app.world().resource::<Track>();
        let (home, _, _) = civilian_lanes(track, 0, progress, -1., 0, 0.);
        let (p, q) = track.street_sample(0, progress, home);
        cars.push(
            app.world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        progress,
                        lane: home,
                        speed,
                    },
                    TrafficYield {
                        cruise: Some(speed),
                        ..default()
                    },
                    Transform::from_translation(p)
                        .with_rotation(q * Quat::from_rotation_y(std::f32::consts::PI)),
                    RigidBody::KinematicPositionBased,
                    Collider::compound(vec![(
                        Vec3::Y * 0.95,
                        Quat::IDENTITY,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    )]),
                ))
                .id(),
        );
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    app.world_mut().resource_mut::<Race>().elapsed = 10.;
    let mut max_blend = 0_f32;
    let mut measured_motion = false;
    let mut avoiding = false;
    let mut minimum = f32::MAX;
    for _ in 0..900 {
        app.update();
        let r = app.world().get::<Rider>(owner).unwrap();
        assert_eq!(r.health, 100.);
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        assert!(r.lane.abs() < 9.);
        avoiding |= r.ai_plan.is_some_and(|plan| plan.avoiding);
        let rider_position = app.world().get::<Transform>(owner).unwrap().translation;
        for car in &cars {
            let pose = app.world().get::<Transform>(*car).unwrap();
            minimum = minimum.min(rider_position.xz().distance(pose.translation.xz()));
            assert!(app.world().get::<Collider>(*car).is_some());
        }
        let y = app.world().get::<TrafficYield>(cars[0]).unwrap();
        max_blend = max_blend.max(y.passing.blend);
        measured_motion |= y.lateral_speed.abs() > 0.25 && y.lateral_goal.is_some();
        if r.progress > 1650. {
            break;
        }
    }
    let r = app.world().get::<Rider>(owner).unwrap();
    println!(
        "RFU-AI civilian interaction blend={max_blend} measured_motion={measured_motion} avoiding={avoiding} minimum={minimum} station={} speed={}",
        r.progress, r.speed
    );
    assert!(max_blend > 0.5 && measured_motion && avoiding);
    assert!(minimum > 2.);
    assert!(r.progress > 1500. && r.speed > 5.);
}

/// A long, unobstructed physical straight isolates engine drive from curves,
/// player mistakes and civilian planner safety braking.
fn straight_simulation() -> App {
    let mut app = simulation();
    let mut definition = app.world().resource::<Track>().definition.clone();
    definition.closed = false;
    definition.control_points.truncate(4);
    for (i, p) in definition.control_points.iter_mut().enumerate() {
        p.position = [0., 0.1, i as f32 * 2000.];
        p.bank_degrees = 0.;
        p.half_width = 8.;
    }
    definition.checkpoints.last_mut().unwrap().fraction = 1.;
    app.insert_resource(Track::from_definition(definition).unwrap());
    app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
    let world = app.world_mut();
    let riders: Vec<_> = world
        .query_filtered::<Entity, With<Rider>>()
        .iter(world)
        .collect();
    for entity in riders {
        world.despawn(entity);
    }
    let floors: Vec<_> = world
        .query_filtered::<Entity, With<world::Ground>>()
        .iter(world)
        .collect();
    for entity in floors {
        world
            .entity_mut(entity)
            .insert(Collider::cuboid(10000., 0.5, 10000.));
    }
    world.resource_mut::<Race>().phase = Phase::Racing;
    world.resource_mut::<Race>().elapsed = 10.;
    app
}

fn straight_rider(app: &mut App, id: usize, station: f32, lane: f32) -> Entity {
    let mut rider = Rider::new(id);
    rider.progress = station;
    rider.lane = lane;
    let (p, q) = app.world().resource::<Track>().sample(station, lane);
    app.world_mut()
        .spawn((
            rider,
            Transform::from_translation(p).with_rotation(q),
            Visibility::Visible,
            recovery::bike_body(),
        ))
        .id()
}

#[test]
fn physical_rival_reaches_competitive_unboosted_pace_at_all_render_rates() {
    let mut rival_speeds = vec![];
    for fps in [30, 60, 120] {
        let mut speeds = vec![];
        for id in [0, 3] {
            let mut app = straight_simulation();
            app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::from_secs_f64(1. / fps as f64),
            ));
            let owner = straight_rider(&mut app, id, 200., 2.);
            if id == 0 {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(KeyCode::KeyW);
            }
            for _ in 0..fps * 20 {
                app.update();
            }
            let r = app.world().get::<Rider>(owner).unwrap();
            println!(
                "AI-PACE fps={fps} id={id} speed={} progress={} throttle={}",
                r.speed,
                r.progress,
                r.drive_intent.throttle_strength()
            );
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Riding
            );
            assert!(r.progress > 650. && r.speed > 40. && r.speed <= 55.);
            speeds.push(r.speed);
        }
        assert!(speeds[1] >= speeds[0] * 0.90, "rival/player {speeds:?}");
        rival_speeds.push(speeds[1]);
    }
    assert!(
        rival_speeds
            .iter()
            .all(|speed| (speed - rival_speeds[0]).abs() < 0.5)
    );
}

#[test]
fn physical_fighter_starts_and_lands_an_authored_strike_without_injected_attack_events() {
    for (weapon, initial_serial) in [
        (Some(rr_core::weapons::Weapon::Chain), 0),
        (None, 0),
        (None, 1),
    ] {
        let mut app = straight_simulation();
        let owner = straight_rider(&mut app, 1, 200., 2.);
        let victim = straight_rider(&mut app, 2, 200.4, 3.8);
        // Let the ordinary adapter attach both solid chassis before seeding a moving
        // encounter. No attack/contact messages or hit counters are injected.
        app.update();
        for entity in [owner, victim] {
            let q = app.world().get::<Transform>(entity).unwrap().rotation;
            let velocity = q * Vec3::Z * 38.;
            let mut r = app.world_mut().get_mut::<Rider>(entity).unwrap();
            r.speed = 38.;
            r.motion_velocity = velocity;
            r.immunity = 0.;
            if entity == owner {
                r.weapon = weapon;
                // Select both real unarmed action phases without injecting intents.
                r.attack_serial = initial_serial;
            }
            let mut state = app
                .world_mut()
                .get_mut::<crate::vehicle::BikeState>(entity)
                .unwrap();
            state.velocity = velocity;
            state.impact_velocity = velocity;
            state.was_riding = true;
            app.world_mut().get_mut::<Velocity>(entity).unwrap().linear = velocity;
        }
        let mut attempts = 0;
        let mut confirmed = false;
        for _ in 0..240 {
            app.update();
            let r = app.world().get::<Rider>(owner).unwrap();
            attempts = attempts.max(r.attack_serial.saturating_sub(initial_serial));
            let v = app.world().get::<Rider>(victim).unwrap();
            confirmed |= v.hits.count > 0 || v.hit_reaction.is_some();
            if confirmed {
                break;
            }
        }
        let r = app.world().get::<Rider>(owner).unwrap();
        let v = app.world().get::<Rider>(victim).unwrap();
        println!(
            "AI-ENGAGE weapon={weapon:?} initial_serial={initial_serial} attempts={attempts} confirmed={confirmed} gap={} lane={} speeds={}/{} decisions={:?}/{:?}",
            v.progress - r.progress,
            v.lane - r.lane,
            r.speed,
            v.speed,
            r.combat_mind.decision,
            v.combat_mind.decision
        );
        assert!(attempts > 0 && confirmed);
        assert!(r.progress > 200. && v.progress > 200. && r.speed > 20. && v.speed > 20.);
        assert!(v.combat_mind.incident_count() > 0);
        if weapon.is_none() {
            assert_eq!(v.hit_reaction.unwrap().kicking, initial_serial == 0);
            assert_eq!(v.hits.count > 0, initial_serial != 0);
        }
    }
}

#[test]
fn launch_grid_holds_its_line_and_accelerates_without_police() {
    // Use the real alternating authoring grids, not a synthetic single-file
    // compression that legitimately needs avoidance as acceleration differs.
    for source in [Track::metropolis().unwrap(), Track::hills().unwrap()] {
        let spawns = source.definition.spawns;
        let mut app = straight_simulation();
        app.world_mut().resource_mut::<Race>().elapsed = 0.;
        let mut owners = vec![];
        for (id, spawn) in spawns.iter().enumerate().skip(1) {
            owners.push(straight_rider(
                &mut app,
                id,
                600. + spawn.distance,
                spawn.lane,
            ));
        }
        let mut max_displacement = 0_f32;
        let mut max_steer = 0_f32;
        let mut rollout_drift = 0_f32;
        let mut rollout_steer = 0_f32;
        for _ in 0..180 {
            app.update();
            for owner in &owners {
                let r = app.world().get::<Rider>(*owner).unwrap();
                max_displacement = max_displacement.max((r.lane - spawns[r.id].lane).abs());
                if max_steer <= 0.15 && r.drive_intent.steer.abs() > 0.15 {
                    println!(
                        "AI-LAUNCH first-steer id={} elapsed={} speed={} lane={} plan={:?}",
                        r.id,
                        app.world().resource::<Race>().elapsed,
                        r.speed,
                        r.lane,
                        r.ai_plan
                    );
                }
                max_steer = max_steer.max(r.drive_intent.steer.abs());
                if app.world().resource::<Race>().elapsed <= 1.5 {
                    rollout_drift = rollout_drift.max((r.lane - spawns[r.id].lane).abs());
                    rollout_steer = rollout_steer.max(r.drive_intent.steer.abs());
                }
                assert_eq!(r.health, 100.);
                assert_eq!(
                    app.world().get::<Recovery>(*owner).unwrap().stage,
                    RecoveryStage::Riding
                );
            }
        }
        println!("AI-LAUNCH maximum_lane_drift={max_displacement} maximum_steer={max_steer}");
        // Initial rollout stays calm; a real closing-speed hazard may request
        // a gradual pass once moving. Never force zero steering through it.
        assert!(rollout_drift < 0.4 && rollout_steer < 0.15);
        assert!(max_displacement < 1.0 && max_steer < 0.35);
        for owner in owners {
            let r = app.world().get::<Rider>(owner).unwrap();
            assert!(
                r.speed > 20.,
                "id={} speed={} plan={:?}",
                r.id,
                r.speed,
                r.ai_plan
            );
        }
    }
}

#[test]
fn disabled_police_setup_and_retry_leave_no_officer_or_patrol_hud() {
    use bevy::ecs::system::RunSystemOnce;
    let mut app = simulation();
    app.world_mut()
        .resource_mut::<crate::police::Police>()
        .deployment_enabled = false;
    app.world_mut()
        .run_system_once(crate::police::setup)
        .unwrap();
    for _ in 0..5 {
        app.update();
    }
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyR);
    for _ in 0..5 {
        app.update();
    }
    let world = app.world_mut();
    assert!(world.query::<&Rider>().iter(world).all(|r| !r.police));
    assert!(world.resource::<crate::police::Police>().hud().is_empty());
    assert!(
        !world
            .resource::<crate::police::Police>()
            .mind
            .state
            .wanted()
    );
}

#[test]
fn physical_fighter_closes_twenty_metres_and_lands_a_hit_with_normal_engine_force() {
    for (gap, lane_a, lane_b, speed) in [(20., 2., 3.8, 40.), (4., -3., 2., 25.)] {
        let mut app = straight_simulation();
        let owner = straight_rider(&mut app, 10, 200., lane_a);
        let victim = straight_rider(&mut app, 14, 200. + gap, lane_b);
        app.update();
        for entity in [owner, victim] {
            let q = app.world().get::<Transform>(entity).unwrap().rotation;
            let velocity = q * Vec3::Z * speed;
            let mut r = app.world_mut().get_mut::<Rider>(entity).unwrap();
            r.speed = speed;
            r.motion_velocity = velocity;
            r.immunity = 0.;
            if entity == owner {
                r.weapon = Some(rr_core::weapons::Weapon::Chain);
            }
            let mut state = app
                .world_mut()
                .get_mut::<crate::vehicle::BikeState>(entity)
                .unwrap();
            state.velocity = velocity;
            state.impact_velocity = velocity;
            state.was_riding = true;
            app.world_mut().get_mut::<Velocity>(entity).unwrap().linear = velocity;
        }
        let mut confirmed = false;
        for _ in 0..1200 {
            app.update();
            let v = app.world().get::<Rider>(victim).unwrap();
            confirmed |= v.hits.count > 0 || v.hit_reaction.is_some();
            if confirmed {
                break;
            }
        }
        let r = app.world().get::<Rider>(owner).unwrap();
        let v = app.world().get::<Rider>(victim).unwrap();
        println!(
            "AI-CATCHUP attack_serial={} confirmed={confirmed} gap={} speeds={}/{}",
            r.attack_serial,
            v.progress - r.progress,
            r.speed,
            v.speed
        );
        assert!(confirmed && r.attack_serial > 0);
        assert!(r.speed > 12. && v.speed > 12. && r.speed <= 55.);
    }
}

#[test]
fn opted_in_officer_remains_solid_traffic_without_becoming_a_rival_combat_target() {
    let mut app = straight_simulation();
    let fighter = straight_rider(&mut app, 10, 200., 2.);
    let officer = straight_rider(&mut app, crate::police::OFFICER_ID, 202., 3.8);
    let rival = straight_rider(&mut app, 14, 218., -2.);
    app.update();
    for entity in [fighter, officer, rival] {
        let q = app.world().get::<Transform>(entity).unwrap().rotation;
        let velocity = q * Vec3::Z * 25.;
        let mut r = app.world_mut().get_mut::<Rider>(entity).unwrap();
        r.speed = 25.;
        r.motion_velocity = velocity;
        let mut state = app
            .world_mut()
            .get_mut::<crate::vehicle::BikeState>(entity)
            .unwrap();
        state.velocity = velocity;
        state.was_riding = true;
        app.world_mut().get_mut::<Velocity>(entity).unwrap().linear = velocity;
    }
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(
        app.world()
            .get::<Rider>(fighter)
            .unwrap()
            .combat_mind
            .target_id(),
        Some(14)
    );
    assert!(app.world().get::<Collider>(officer).is_some());
    assert!(app.world().get::<RigidBody>(officer).is_some());
}

#[test]
fn launch_checks_a_stationary_rider_before_using_the_clear_road_fast_path() {
    let mut app = straight_simulation();
    app.world_mut().resource_mut::<Race>().elapsed = 0.;
    let owner = straight_rider(&mut app, 10, 200., 2.);
    let blocker = straight_rider(&mut app, 0, 204., 2.);
    for _ in 0..5 {
        app.update();
        let r = app.world().get::<Rider>(owner).unwrap();
        assert!(r.ai_plan.is_some_and(|p| p.avoiding));
        assert!(!r.drive_intent.throttle);
        assert_eq!(r.health, 100.);
    }
    assert!(app.world().get::<Collider>(blocker).is_some());
    assert!(app.world().get::<Rider>(owner).unwrap().speed < 0.5);
}

#[test]
fn hills_rival_physically_clears_two_solid_oncoming_cars() {
    #[derive(Component)]
    struct Counterflow {
        station: f32,
        lane: f32,
    }
    fn move_cars(
        track: Res<Track>,
        mut cars: Query<(&mut Counterflow, &mut Transform, &mut Velocity)>,
    ) {
        for (mut car, mut pose, mut velocity) in &mut cars {
            car.station -= 13. / 60.;
            let (p, q) = track.sample(car.station, car.lane);
            *pose = Transform::from_translation(p)
                .with_rotation(q * Quat::from_rotation_y(std::f32::consts::PI));
            velocity.linear = q * Vec3::Z * -13.;
        }
    }
    let mut app = straight_simulation();
    let hills = Track::hills().unwrap();
    let mut vertices = vec![];
    for station in (-300..=1000).step_by(5) {
        for lane in [-10., 10.] {
            vertices.push(hills.sample(station as f32, lane).0);
        }
    }
    let mut triangles = vec![];
    for i in 0..vertices.len() / 2 - 1 {
        let a = (i * 2) as u32;
        triangles.extend([[a, a + 2, a + 1], [a + 1, a + 2, a + 3]]);
    }
    app.insert_resource(hills);
    let world = app.world_mut();
    let floors: Vec<_> = world
        .query_filtered::<Entity, With<world::Ground>>()
        .iter(world)
        .collect();
    for floor in floors {
        world.entity_mut(floor).insert((
            Collider::trimesh(vertices.clone(), triangles.clone()).unwrap(),
            Transform::IDENTITY,
        ));
    }
    let owner = straight_rider(&mut app, 3, -100., -1.8);
    let mut cars = vec![];
    for (station, lane) in [(160., -1.8), (180., 1.8)] {
        let (p, q) = app.world().resource::<Track>().sample(station, lane);
        cars.push(
            app.world_mut()
                .spawn((
                    Counterflow { station, lane },
                    crate::environment::CrossTraffic,
                    Transform::from_translation(p)
                        .with_rotation(q * Quat::from_rotation_y(std::f32::consts::PI)),
                    Velocity::linear(q * Vec3::Z * -13.),
                    RigidBody::KinematicPositionBased,
                    Collider::compound(vec![(
                        Vec3::Y * 0.95,
                        Quat::IDENTITY,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    )]),
                    ActiveEvents::COLLISION_EVENTS,
                ))
                .id(),
        );
    }
    app.add_systems(FixedUpdate, move_cars.before(drive));
    app.update();
    let (_, q) = app.world().resource::<Track>().sample(-100., -1.8);
    let velocity = q * Vec3::Z * 35.;
    {
        let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
        rider.speed = 35.;
        rider.motion_velocity = velocity;
        rider.attack = 1000.;
    }
    {
        let mut state = app
            .world_mut()
            .get_mut::<crate::vehicle::BikeState>(owner)
            .unwrap();
        state.velocity = velocity;
        state.impact_velocity = velocity;
        state.was_riding = true;
    }
    app.world_mut().get_mut::<Velocity>(owner).unwrap().linear = velocity;
    let mut passed = false;
    let mut max_bank = 0_f32;
    for _ in 0..600 {
        app.update();
        let r = app.world().get::<Rider>(owner).unwrap();
        max_bank = max_bank.max(r.drive_intent.steer.abs());
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        assert_eq!(r.health, 100.);
        passed = cars
            .iter()
            .all(|car| r.progress > app.world().get::<Counterflow>(*car).unwrap().station + 15.);
        if passed {
            break;
        }
    }
    let r = app.world().get::<Rider>(owner).unwrap();
    println!(
        "HILLS-PAIR passed={passed} progress={} speed={} lane={} max_bank={max_bank}",
        r.progress, r.speed, r.lane
    );
    assert!(passed && r.speed > 25. && max_bank > 0.01);
}
