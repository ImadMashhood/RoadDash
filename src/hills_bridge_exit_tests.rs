//! Exercise the default assisted controller at the authored open bridge edge.
use super::*;
use crate::world_gen::hills::bridge;
use bevy::ecs::system::RunSystemOnce;

fn fixture(station: f32, lane: f32) -> (App, Entity, Vec3, Quat) {
    let mut app = tests::simulation();
    // Garage pauses Rapier's query-pipeline update. Warm the actual bridge in
    // countdown so the first drive tick queries registered ground support.
    app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
    app.insert_resource(Track::hills().unwrap())
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>();
    app.world_mut()
        .run_system_once(
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             art: Res<Art>,
             track: Res<Track>| {
                bridge::spawn(&mut commands, &mut meshes, &mut materials, &art, &track);
            },
        )
        .unwrap();
    for _ in 0..3 {
        app.update();
    }
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let (p, q) = bridge::deck_sample(app.world().resource::<Track>(), station, lane);
    *app.world_mut().get_mut::<Transform>(owner).unwrap() =
        Transform::from_translation(p).with_rotation(q);
    {
        let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
        rider.progress = station;
        rider.route = 0;
        rider.lane = lane;
        rider.speed = 20.;
        rider.immunity = 0.;
        rider.health = 100.;
        rider.finished = None;
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    assert_eq!(
        *app.world()
            .resource::<crate::vehicle::VehiclePhysicsAdapter>(),
        crate::vehicle::VehiclePhysicsAdapter::Assisted
    );
    (app, owner, p, q)
}
fn assert_real_fall(app: &mut App, owner: Entity, deck: f32, q: Quat, side: f32) {
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Falling
    );
    let wreck = app.world().get::<Recovery>(owner).unwrap().wreck.unwrap();
    let t = app.world().get::<Transform>(wreck).unwrap();
    assert!(
        t.translation.y > deck - 2.,
        "must detach at deck height, not teleport to floor"
    );
    let v = app.world().get::<Velocity>(wreck).unwrap().linear;
    assert!(
        v.y < 0.5,
        "deck support correction must not launch upward: {v:?}"
    );
    assert!(v.dot(q * Vec3::Z) > 10., "forward momentum lost: {v:?}");
    assert!(
        v.dot(-(q * Vec3::X) * side) > 0.4,
        "lateral momentum lost: {v:?}"
    );
    // Contact with the real below-deck structure can briefly redirect the
    // wreck. Bound the full departure to three seconds and require continued
    // descent, so a body resting on a girder cannot satisfy this fixture.
    let mut earlier_wreck_height = deck;
    for tick in 0..180 {
        app.update();
        if tick == 119 {
            earlier_wreck_height = app.world().get::<Transform>(wreck).unwrap().translation.y;
        }
    }
    let final_wreck = app.world().get::<Transform>(wreck).unwrap();
    let final_velocity = app.world().get::<Velocity>(wreck).unwrap().linear;
    assert!(
        final_wreck.translation.y < earlier_wreck_height - 8. && final_velocity.y < -8.,
        "wreck must continue falling after structural contact: {:?}, {final_velocity:?}, earlier height {earlier_wreck_height}",
        final_wreck.translation
    );
    let lowest = app
        .world_mut()
        .query::<(&recovery::OwnedBody, &Transform)>()
        .iter(app.world())
        .filter(|(body, _)| body.owner == owner)
        .map(|(_, t)| t.translation.y)
        .fold(f32::INFINITY, f32::min);
    assert!(
        lowest < deck - 10.,
        "gravity must produce a real descent: {lowest} / {deck}"
    );
}
#[test]
fn assisted_steering_crosses_each_open_bridge_edge_into_a_real_fall() {
    for side in [-1., 1.] {
        let (mut app, owner, p, q) =
            fixture(bridge::GAP_STATIONS[0], side * (bridge::HALF_DECK - 0.2));
        for _ in 0..45 {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(if side > 0. {
                    KeyCode::KeyD
                } else {
                    KeyCode::KeyA
                });
            app.update();
            if app.world().get::<Recovery>(owner).unwrap().stage != RecoveryStage::Riding {
                break;
            }
        }
        assert_real_fall(&mut app, owner, p.y, q, side);
    }
}
#[test]
fn actual_combat_kick_pushes_assisted_rider_off_each_bridge_edge() {
    for side in [-1., 1.] {
        let (mut app, owner, p, q) =
            fixture(bridge::GAP_STATIONS[0], side * (bridge::HALF_DECK - 0.2));
        let right = (q * Vec3::Z).cross(Vec3::Y).normalize();
        let mut remote = Rider::new(2);
        remote.networked = true;
        remote.remote_human = true;
        let attacker = app
            .world_mut()
            .spawn((
                remote,
                Transform::from_translation(p - right * side * 1.4).with_rotation(q),
                Visibility::Visible,
                Velocity::zero(),
            ))
            .id();
        app.world_mut().write_message(crate::combat::AttackAttempt(
            attacker,
            rr_core::AttackSide::Kick,
        ));
        app.world_mut()
            .run_system_once(crate::combat::melee)
            .unwrap();
        assert!(app.world().get::<Rider>(owner).unwrap().side_velocity * side > 7.9);
        assert_eq!(app.world().get::<Rider>(owner).unwrap().health, 100.);
        app.world_mut().despawn(attacker);
        for _ in 0..10 {
            app.update();
            if app.world().get::<Recovery>(owner).unwrap().stage != RecoveryStage::Riding {
                break;
            }
        }
        assert_real_fall(&mut app, owner, p.y, q, side);
    }
}
#[test]
fn supported_bridge_hops_and_swept_solids_do_not_trigger_a_false_exit() {
    for side in [-1., 1.] {
        // Ride beside a real square hanger, then hop over supported deck.
        let (mut app, owner, p, q) = fixture(bridge::START + 44., side * (bridge::HALF_DECK - 0.5));
        app.world_mut().get_mut::<Rider>(owner).unwrap().hop_height = 2.;
        for _ in 0..10 {
            app.update();
        }
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        // A solid encountered before the edge must be swept first. This wall is
        // a test fixture; the production bridge intentionally has no low rails.
        let wall = p - q * Vec3::X * side * 0.65 + Vec3::Y * 2.;
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(0.15, 3., 12.),
            world::Obstacle,
            Transform::from_translation(wall).with_rotation(q),
        ));
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        for _ in 0..2 {
            app.update();
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
            r.hop_height = 0.;
            r.hop_velocity = 0.;
            r.side_velocity = side * 12.;
        }
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(
            app.world().get::<Recovery>(owner).unwrap().stage,
            RecoveryStage::Riding
        );
        assert!(app.world().get::<Rider>(owner).unwrap().lane.abs() < bridge::HALF_DECK);
    }
}
#[test]
fn bridge_support_handoff_does_not_apply_to_other_maps() {
    let (mut app, owner, _, _) = fixture(bridge::GAP_STATIONS[0], 7.1);
    // Same missing-support setup, with the actual Metropolis track selected:
    // the new authored bridge rule must not turn into a global ledge policy.
    app.insert_resource(Track::metropolis().unwrap());
    {
        let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
        rider.side_velocity = 20.;
    }
    app.update();
    assert_eq!(
        app.world().get::<Recovery>(owner).unwrap().stage,
        RecoveryStage::Riding
    );
}
