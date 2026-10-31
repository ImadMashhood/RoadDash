//! Render the actual crash/get-up/run/remount sequence, then verify results.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    track::{LAPS, Track},
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

#[derive(Resource, Default)]
pub struct SmokeProgress {
    frame: u32,
    getting_up: bool,
    fixed_stages: [bool; 3],
    fixed_hill_running: bool,
    running: bool,
    mounting: bool,
    finished_at: Option<u32>,
    recovered_at: Option<u32>,
    hill_started: bool,
    hill_running: bool,
    farm_walk: Option<(f32, f32)>,
    hill_walk: Option<(f32, f32)>,
    results: bool,
}
// A running stage can last one physics tick when the rider lands near the bike.
// Observe fixed transitions independently of render-frame screenshot timing.
pub fn observe_smoke_recovery(
    mut state: ResMut<SmokeProgress>,
    riders: Query<(&Rider, &Recovery)>,
    bodies: Query<&Transform, Without<Rider>>,
) {
    for (rider, recovery) in &riders {
        if rider.id != 0 {
            continue;
        }
        match recovery.stage {
            RecoveryStage::GettingUp => state.fixed_stages[0] = true,
            RecoveryStage::Running => {
                state.fixed_stages[1] = true;
                let walk = recovery
                    .runner
                    .zip(recovery.wreck)
                    .and_then(|(r, b)| bodies.get(r).ok().zip(bodies.get(b).ok()))
                    .map(|(r, b)| {
                        (
                            r.translation.distance(b.translation),
                            (r.translation.y - b.translation.y).abs(),
                        )
                    });
                if state.hill_started {
                    state.fixed_hill_running = true;
                    state.hill_walk = state.hill_walk.or(walk);
                } else {
                    state.farm_walk = state.farm_walk.or(walk);
                }
            }
            RecoveryStage::Mounting => state.fixed_stages[2] = true,
            _ => {}
        }
    }
}
fn capture(commands: &mut Commands, name: &str) {
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(format!("screenshots/{name}.png")));
}
#[allow(clippy::too_many_arguments)]
pub fn smoke_test(
    mut commands: Commands,
    mut state: ResMut<SmokeProgress>,
    mut race: ResMut<Race>,
    track: Res<Track>,
    mut riders: Query<
        (
            &mut Rider,
            &Recovery,
            &mut Transform,
            Option<&mut bevy_rapier3d::prelude::Velocity>,
        ),
        With<Rider>,
    >,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
    sinks: Query<&AudioSink>,
    mut metrics: ResMut<crate::diagnostics::Metrics>,
    clock: Res<crate::runtime::SimulationClock>,
    adapter: Option<Res<bevy::render::renderer::RenderAdapterInfo>>,
    mut inspector: ResMut<crate::environment::TrackInspector>,
    mut contacts: ResMut<Messages<bevy_rapier3d::prelude::CollisionEvent>>,
    mut sweeps: ResMut<Messages<crate::recovery::SweptContact>>,
    bodies: Query<&Transform, Without<Rider>>,
    telemetry: Res<crate::vehicle::VehicleTelemetry>,
) {
    state.frame += 1;
    match state.frame {
        30 => {
            assert!(sinks.iter().count() >= 4, "audio output did not initialize");
            println!(
                "Audio output initialized: {} playback sinks",
                sinks.iter().count()
            );
            capture(&mut commands, "01-garage");
        }
        60 => {
            race.phase = Phase::Racing;
            keys.press(KeyCode::KeyW);
        }
        230 => {
            metrics.visible = true;
        }
        240 => {
            assert!(
                sinks
                    .iter()
                    .any(|s| !s.is_paused() && s.volume().to_linear() > 0.0),
                "no racing audio playing"
            );
            capture(&mut commands, "02-racing");
        }
        260 => {
            for (mut rider, _, _, _) in &mut riders {
                if rider.id == 0 {
                    rider.health = 0.0;
                }
            }
            keys.release(KeyCode::KeyW);
        }
        280 => capture(&mut commands, "03-crash"),
        _ => {}
    }
    for (mut rider, recovery, mut transform, mut velocity) in &mut riders {
        if rider.id != 0 || state.frame <= 280 {
            continue;
        }
        if let Some(recovered_at) = state.recovered_at {
            let elapsed = state.frame - recovered_at;
            if elapsed == 20 || elapsed == 60 {
                let sector = if elapsed == 20 {
                    rr_content::track::Sector::City
                } else {
                    rr_content::track::Sector::Hills
                };
                let index = if elapsed == 60 {
                    track
                        .points
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.y.total_cmp(&b.1.y))
                        .unwrap()
                        .0
                } else {
                    let distance = track
                        .definition
                        .props
                        .iter()
                        .find(|p| p.kind == rr_content::track::PropKind::Crossing)
                        .map_or_else(
                            || {
                                track
                                    .distances
                                    .iter()
                                    .copied()
                                    .find(|d| track.sector(*d) == sector)
                                    .unwrap_or(0.0)
                                    + 100.0
                            },
                            |p| p.fraction * track.length - 45.0,
                        );
                    track.distances.partition_point(|d| *d < distance)
                };
                rider.progress = track.distances[index.min(track.distances.len() - 1)];
                rider.lane = 0.0;
                rider.speed = 0.0;
                rider.motion_velocity = Vec3::ZERO;
                let (p, q) = track.sample(rider.progress, 0.0);
                *transform = Transform::from_translation(p).with_rotation(q);
                if let Some(v) = velocity.as_mut() {
                    **v = bevy_rapier3d::prelude::Velocity::zero();
                }
                contacts.clear();
                sweeps.clear();
            }
            if elapsed == 50 {
                capture(&mut commands, "08-city");
            }
            if elapsed == 90 {
                capture(&mut commands, "09-hills");
            }
            if elapsed == 92 {
                inspector.visible = true;
            }
            if elapsed == 120 {
                capture(&mut commands, "12-track-inspector");
            }
            if elapsed == 130 {
                inspector.visible = false;
            }
            if elapsed == 140 {
                rider.speed = 30.0;
                rider.health = 0.0;
                state.hill_started = true;
            }
            if elapsed == 165 {
                capture(&mut commands, "10-hill-crash");
            }
            if state.hill_started && recovery.stage == RecoveryStage::Running && !state.hill_running
            {
                state.hill_walk = recovery
                    .runner
                    .zip(recovery.wreck)
                    .and_then(|(r, b)| bodies.get(r).ok().zip(bodies.get(b).ok()))
                    .map(|(r, b)| {
                        (
                            r.translation.distance(b.translation),
                            (r.translation.y - b.translation.y).abs(),
                        )
                    });
                state.hill_running = true;
                capture(&mut commands, "11-hill-running");
            }
        }
        match recovery.stage {
            RecoveryStage::GettingUp if !state.getting_up && recovery.elapsed > 0.25 => {
                state.getting_up = true;
                capture(&mut commands, "04-getting-up");
            }
            RecoveryStage::Running if !state.running => {
                state.farm_walk = recovery
                    .runner
                    .zip(recovery.wreck)
                    .and_then(|(r, b)| bodies.get(r).ok().zip(bodies.get(b).ok()))
                    .map(|(r, b)| {
                        (
                            r.translation.distance(b.translation),
                            (r.translation.y - b.translation.y).abs(),
                        )
                    });
                state.running = true;
                capture(&mut commands, "05-running-to-bike");
            }
            RecoveryStage::Mounting if !state.mounting && recovery.elapsed > 0.25 => {
                state.mounting = true;
                capture(&mut commands, "06-remounting");
            }
            RecoveryStage::Riding if state.mounting && state.finished_at.is_none() => {
                assert!(
                    state.fixed_stages.iter().all(|seen| *seen),
                    "recovery skipped a stage"
                );
                if state.recovered_at.is_none() {
                    state.recovered_at = Some(state.frame);
                }
                if !state.hill_started || !state.fixed_hill_running {
                    continue;
                }
                rider.gates.completed_laps = LAPS;
                rider.progress = track.length * LAPS as f32 + 1.0;
                rider.lane = 0.0;
                let (p, q) = track.sample(rider.progress, rider.lane);
                *transform = Transform::from_translation(p).with_rotation(q);
                if let Some(v) = velocity.as_mut() {
                    **v = bevy_rapier3d::prelude::Velocity::zero();
                }
                // This is a results-screen fixture after recovery, not a gate
                // validation test. Discard contacts from the previous location.
                contacts.clear();
                sweeps.clear();
                state.finished_at = Some(state.frame);
            }
            _ => {}
        }
    }
    if let Some(finished_at) = state.finished_at {
        if state.frame > finished_at + 20 && !state.results {
            assert!(race.phase == Phase::Finished);
            state.results = true;
            capture(&mut commands, "07-results");
        }
        if state.frame > finished_at + 40 {
            let mut report = metrics.evidence(&clock);
            report["fixed_recovery_stages"] = serde_json::json!(state.fixed_stages);
            report["render_running_captured"] = serde_json::json!(state.running);
            report["vehicle"] = telemetry
                .samples
                .back()
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            let trace = telemetry
                .samples
                .iter()
                .map(|v| serde_json::to_string(v).unwrap())
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write("screenshots/vehicle-trace.jsonl", trace).unwrap();
            report["track"] = serde_json::json!({"id":track.definition.id.as_str(),"closed":track.definition.closed,"length_m":track.length,"gates":track.definition.checkpoints.len(),"render_triangles":inspector.render_triangles,"collision_triangles":inspector.collision_triangles});
            report["recovery_at_run_capture"] = serde_json::json!({"farm_distance_and_height_delta_m":state.farm_walk,"hill_distance_and_height_delta_m":state.hill_walk});
            if let Some(adapter) = adapter.as_ref() {
                report["adapter"] = serde_json::json!({"name": adapter.name, "driver": adapter.driver, "backend": format!("{:?}", adapter.backend)});
            }
            std::fs::write(
                "screenshots/smoke-metrics.json",
                serde_json::to_vec_pretty(&report).unwrap(),
            )
            .expect("write smoke evidence");
            println!(
                "Render smoke test passed: farm crash -> remount -> city/hills -> hill crash -> remount -> finish"
            );
            exit.write(AppExit::Success);
        }
    }
    assert!(
        state.frame < 6000,
        "render smoke test timed out during recovery"
    );
}

#[derive(Default)]
pub struct ActorSmoke {
    frame: u32,
    actors: Vec<Entity>,
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn actor_smoke(
    mut commands: Commands,
    mut state: Local<ActorSmoke>,
    mut race: ResMut<Race>,
    track: Res<Track>,
    mut riders: Query<(Entity, &mut Rider, &mut Transform), Without<crate::npc::Npc>>,
    mut actors: Query<(Entity, &crate::npc::Npc, &mut Transform), Without<Rider>>,
    mut hits: MessageWriter<crate::combat::NpcHit>,
    bodies: Query<
        (&crate::recovery::OwnedBody, &Transform),
        (Without<Rider>, Without<crate::npc::Npc>),
    >,
    mut exit: MessageWriter<AppExit>,
    sinks: Query<&AudioSink>,
) {
    state.frame += 1;
    if state.frame == 30 {
        race.phase = Phase::Racing;
        race.camera_mode = 0;
        let (p, q) = track.sample(1200.0, 0.0);
        for (e, mut r, mut t) in &mut riders {
            if r.id == 0 {
                r.progress = 1200.0;
                r.lane = 0.0;
                r.speed = 0.0;
                r.immunity = 1000.0;
                *t = Transform::from_translation(p).with_rotation(q);
                commands
                    .entity(e)
                    .insert(bevy_rapier3d::prelude::ColliderDisabled);
            }
        }
        for (kind, side) in [
            (crate::npc::NpcKind::Cow, -1.0),
            (crate::npc::NpcKind::Pedestrian, 1.0),
        ] {
            if let Some((e, _, mut t)) = actors.iter_mut().find(|(_, n, _)| n.kind == kind) {
                *t = Transform::from_translation(p + q * Vec3::X * side * 2.1).with_rotation(q);
                commands.entity(e).remove::<crate::environment::Patrol>();
                state.actors.push(e);
            }
        }
    }
    if state.frame == 65 {
        capture(&mut commands, "13-actors-standing");
    }
    if state.frame == 90 {
        let (_, q) = track.sample(1200.0, 0.0);
        for &e in &state.actors {
            hits.write(crate::combat::NpcHit(e, q * Vec3::Z * 9.0, false));
        }
    }
    if state.frame == 150 {
        capture(&mut commands, "14-actors-ragdoll");
    }
    if state.frame == 200 {
        assert_eq!(state.actors.len(), 2);
        let mut summary = vec![];
        for &e in &state.actors {
            let (_, npc, _) = actors.get(e).unwrap();
            let expected = if npc.kind == crate::npc::NpcKind::Cow {
                10
            } else {
                11
            };
            assert_eq!(npc.parts.len(), expected);
            for &part in &npc.parts {
                let (body, t) = bodies.get(part).unwrap();
                assert!(body.kind == crate::recovery::BodyKind::Ragdoll);
                assert!(t.translation.is_finite() && t.translation.y > -0.5);
            }
            summary.push(
                serde_json::json!({"kind":format!("{:?}",npc.kind),"bodies":npc.parts.len()}),
            );
        }
        assert!(sinks.iter().count() >= 4);
        std::fs::write(
            "screenshots/actor-smoke.json",
            serde_json::to_vec_pretty(&summary).unwrap(),
        )
        .unwrap();
        println!(
            "Actor smoke passed: independent human and quadruped ragdolls, solid contacts, bounded bodies"
        );
        exit.write(AppExit::Success);
    }
}
