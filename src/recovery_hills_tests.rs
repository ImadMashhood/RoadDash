//! Real Hills walking regression: open straights let a rider return across the verge.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{BodyKind, OwnedBody, Recovery, RecoveryStage},
    track::Track,
    world::{self, Art},
};
use bevy::{ecs::system::RunSystemOnce, prelude::*};
use bevy_rapier3d::prelude::*;

#[test]
fn hills_open_straight_allows_unassisted_return_across_the_verge() {
    let mut app = crate::game::tests::simulation();
    app.insert_resource(Track::hills().unwrap())
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>();
    app.world_mut().run_system_once(world::setup_world).unwrap();
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let track = app.world().resource::<Track>();
    let station = (1900..4300)
        .step_by(20)
        .map(|s| s as f32)
        .find(|s| {
            [-24., 0., 24.]
                .iter()
                .all(|d| !crate::world_gen::hills::bridge::curve_guardrail(track, *s + *d))
                && (track.sample(*s, 0.).1 * Vec3::Z).y.abs() < 0.12
        })
        .expect("an open Hills straight");
    let (p, q) = track.sample_route(0, station, 12.);
    let start = p.with_y(track.terrain_height(p.x, p.z) + 0.98);
    let (p, q2) = track.sample_route(0, station, 0.);
    let wreck_pose = Transform::from_translation(p).with_rotation(q2);
    let (runner, pose) = app
        .world_mut()
        .run_system_once(|mut commands: Commands, art: Res<Art>| {
            world::runner(&mut commands, &art, 0)
        })
        .unwrap();
    app.world_mut().entity_mut(runner).insert((
        Transform::from_translation(start).with_rotation(q),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.65, 0.28),
        crate::recovery::runner_controller(),
        OwnedBody {
            owner,
            kind: BodyKind::Runner,
        },
        crate::game::Debris(f32::INFINITY),
    ));
    let wreck = app
        .world_mut()
        .spawn((
            wreck_pose,
            RigidBody::Fixed,
            Collider::compound(vec![(
                Vec3::Y * 0.85,
                Quat::IDENTITY,
                Collider::cuboid(0.38, 0.8, 1.12),
            )]),
            Visibility::default(),
            OwnedBody {
                owner,
                kind: BodyKind::Wreck,
            },
        ))
        .id();
    app.world_mut().entity_mut(owner).insert(ColliderDisabled);
    {
        let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
        rider.progress = station;
        rider.lane = 12.;
        rider.speed = 0.;
        rider.crashed = 1.;
    }
    {
        let mut recovery = app.world_mut().get_mut::<Recovery>(owner).unwrap();
        recovery.stage = RecoveryStage::Running;
        recovery.runner = Some(runner);
        recovery.pose = Some(pose);
        recovery.wreck = Some(wreck);
        recovery.previous_foot = Some(start);
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
    for _ in 0..3 {
        app.update();
    }
    app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
    let mut arrived = false;
    for _ in 0..600 {
        app.update();
        let stage = app.world().get::<Recovery>(owner).unwrap().stage;
        if matches!(stage, RecoveryStage::Mounting | RecoveryStage::Riding) {
            arrived = true;
            break;
        }
    }
    let recovery = app.world().get::<Recovery>(owner).unwrap();
    assert!(
        arrived,
        "on-foot return across an open straight must reach the actual wreck"
    );
    assert_eq!(recovery.rescues, 0);
    assert_eq!(recovery.rescue_penalty_total, 0.);
}
