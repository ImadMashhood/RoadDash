//! Production fixed-step regressions for RFU-AI, with normal solid bike bodies.
use super::tests::simulation;
use super::*;

#[test]
fn killer_pairs_and_chains_make_bounded_progress_without_mutual_stopping() {
    for count in [2, 4] {
        for leading_killer_only in [false, true] {
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
                rider.combat_mind.role = if leading_killer_only && index < count - 1 {
                    rr_core::rival_ai::Role::Racer
                } else {
                    rr_core::rival_ai::Role::Killer
                };
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
                    "RFU-AI count={count} leading_only={leading_killer_only} id={} station={} speed={} decision={:?} target={:?} stopped_ticks={}",
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
