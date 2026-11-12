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
    return_across_verge(None, false);
}

#[test]
fn hills_guardrail_gaps_allow_real_unassisted_foot_return() {
    for side in [-1, 1] {
        return_across_verge(Some(side), false);
    }
}

#[test]
fn hills_guardrail_midrun_allows_unassisted_return_via_a_real_end_or_gap() {
    return_across_verge(Some(1), true);
}

fn return_across_verge(gap_side: Option<i8>, midrun: bool) {
    let mut app = crate::game::tests::simulation();
    app.insert_resource(Track::hills().unwrap())
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>();
    app.world_mut().run_system_once(world::setup_world).unwrap();
    app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
    for _ in 0..3 {
        app.update();
    }
    let owner = app
        .world_mut()
        .query_filtered::<Entity, With<Rider>>()
        .single(app.world())
        .unwrap();
    let track = app.world().resource::<Track>();
    let straight = || {
        (1900..4300)
            .step_by(20)
            .map(|s| s as f32)
            .find(|s| {
                [-24., 0., 24.]
                    .iter()
                    .all(|d| !crate::world_gen::hills::bridge::curve_guardrail(track, *s + *d))
                    && (track.sample(*s, 0.).1 * Vec3::Z).y.abs() < 0.12
            })
            .expect("an open Hills straight")
    };
    let (station, lane) = if let Some(side) = gap_side {
        let runs = crate::world_gen::hills::guardrails::layout(track);
        if midrun {
            let run = runs
                .iter()
                .find(|r| {
                    r.side == side
                        && (r.end - r.start - rr_core::guardrails::MAX_RUN_M).abs() < 0.01
                })
                .expect("maximum continuous rail run");
            ((run.start + run.end) * 0.5, side as f32 * 12.)
        } else {
            let pair = runs
                .windows(2)
                .find(|p| {
                    p[0].side == side
                        && p[1].side == side
                        && (p[1].start - p[0].end - 16.).abs() < 0.01
                })
                .expect("deliberate gap on each turn side");
            ((pair[0].end + pair[1].start) * 0.5, side as f32 * 12.)
        }
    } else {
        (straight(), 12.)
    };
    let (p, q) = track.sample_route(0, station, lane);
    let sampled_start = p;
    let (p, q2) = track.sample_route(0, station, 0.);
    let wreck_pose = Transform::from_translation(p).with_rotation(q2);
    let start = app.world_mut().run_system_once(move |context: ReadRapierContext, ground: Query<(),With<world::Ground>>| {
        let ctx=context.single().unwrap();
        let is_ground=|e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
        let (_,toi)=ctx.cast_ray(sampled_start+Vec3::Y*30.,-Vec3::Y,60.,true,QueryFilter::default().predicate(&is_ground)).expect("actual recovery support");
        let start=sampled_start.with_y(sampled_start.y+30.-toi+0.98);
        eprintln!("recovery gap {gap_side:?} at {station}: analytic_start={:?} collision_start={start:?}",sampled_start+Vec3::Y*0.98);
        start
    }).unwrap();
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
        rider.lane = lane;
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
    for _ in 0..if midrun { 2400 } else { 600 } {
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
        "on-foot return must reach wreck: side={gap_side:?} station={station} start={start:?} runner={:?} wreck={:?} stage={:?} path={:?}",
        app.world().get::<Transform>(runner),
        app.world().get::<Transform>(wreck),
        recovery.stage,
        recovery.walk_path
    );
    assert_eq!(recovery.rescues, 0);
    assert_eq!(recovery.rescue_penalty_total, 0.);
}
