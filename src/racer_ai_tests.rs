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
