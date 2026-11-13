//! Render the actual crash/get-up/run/remount sequence, then verify results.
pub(crate) mod movement;
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
    last_pickup: Option<Vec3>,
    remounts: u32,
    max_remount_error: f32,
    walk_crossing_setup: [bool; 2],
    walk_crossing_distance: [f32; 2],
    walk_crossing_diagnostic_tick: [u32; 2],
    recovery_diagnostic_tick: [u32; 2],
    rescue_counts: [u32; 2],
    rescue_penalties: [f32; 2],
    rescue_stationary_seconds: [f32; 2],
    strike_windup: bool,
    strike_contact: bool,
    manual_ticks: u32,
    manual_seen: bool,
    auto_return_seen: bool,
}
// A running stage can last one physics tick when the rider lands near the bike.
// Observe fixed transitions independently of render-frame screenshot timing.
#[allow(clippy::type_complexity)]
pub fn observe_smoke_recovery(
    mut state: ResMut<SmokeProgress>,
    riders: Query<(Entity, &Rider, &Recovery, &Transform)>,
    (bodies, controller_outputs, velocities, names): (
        Query<&Transform, Without<Rider>>,
        Query<&bevy_rapier3d::prelude::KinematicCharacterControllerOutput>,
        Query<&bevy_rapier3d::prelude::Velocity>,
        Query<&Name>,
    ),
    (track, navigation, controls): (
        Res<Track>,
        Option<Res<crate::world_gen::metropolis::navigation::Navigation>>,
        Res<crate::controls::Controls>,
    ),
    ground: Query<
        (
            Has<crate::environment::RoadChunk>,
            Has<crate::vehicle::RideableStep>,
        ),
        With<crate::world::Ground>,
    >,
    context: bevy_rapier3d::prelude::ReadRapierContext,
    mut commands: Commands,
) {
    for (owner, rider, recovery, root) in &riders {
        if rider.id != 0 {
            continue;
        }
        let index = usize::from(state.hill_started);
        if std::env::var_os("ROADDASH_VERIFY_MANUAL_RETURN").is_some()
            && !state.hill_started
            && recovery.stage == RecoveryStage::Running
        {
            if state.manual_ticks == 0 && recovery.walk_distance > 0.3 {
                smoke_key(&mut commands, KeyCode::KeyA, true);
                state.manual_ticks = 1;
            } else if state.manual_ticks > 0 {
                state.manual_ticks += 1;
                if recovery.foot.mode == rr_core::recovery::FootMode::Manual {
                    if !state.manual_seen {
                        capture(&mut commands, "manual-return-interrupt");
                    }
                    state.manual_seen = true;
                }
                if state.manual_ticks == 46 {
                    smoke_key(&mut commands, KeyCode::KeyA, false);
                }
                if state.manual_ticks > 46
                    && recovery.foot.mode == rr_core::recovery::FootMode::AutoSeekBike
                {
                    if !state.auto_return_seen {
                        capture(&mut commands, "manual-return-resume");
                    }
                    state.auto_return_seen = true;
                }
            }
        }

        if recovery.rescues > state.rescue_counts[index] {
            state.rescue_counts[index] = recovery.rescues;
            state.rescue_penalties[index] = recovery.rescue_penalty_total;
            state.rescue_stationary_seconds[index] = recovery.last_rescue_stationary;
            println!(
                "Native exceptional recovery {index}: count={} stationary={:.3}s penalty={:.3}s",
                recovery.rescues, recovery.last_rescue_stationary, recovery.rescue_penalty_total
            );
        }
        if std::env::var("ROADDASH_VERIFY_WALK_CROSSINGS").as_deref() == Ok("1")
            && recovery.stage == RecoveryStage::GettingUp
            && !state.walk_crossing_setup[index]
        {
            use bevy_rapier3d::prelude::*;
            let runner = recovery.runner.expect("getting-up runner");
            let ctx = context.single().expect("native physics context");
            let (centre, rotation) = track.street_sample(rider.route, rider.progress, 0.);
            let shape = Collider::capsule_y(0.65, 0.28);
            let supports =
                |entity, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(entity);
            let mut placement = None;
            for side in [-1., 1.] {
                for along in [0., -3., 3., -6., 6.] {
                    let p = centre
                        + rotation * Vec3::X * side * if index == 0 { 8. } else { 7.2 }
                        + rotation * Vec3::Z * along;
                    if navigation
                        .as_ref()
                        .is_some_and(|nav| !nav.clear(p.xz(), p.xz()))
                    {
                        continue;
                    }
                    let Some((support, distance)) = ctx.cast_ray(
                        p + Vec3::Y * 3.,
                        Vec3::NEG_Y,
                        6.,
                        true,
                        QueryFilter::default().predicate(&supports),
                    ) else {
                        continue;
                    };
                    let (road, sidewalk) = ground.get(support).expect("support role");
                    if (index == 0 && (road || sidewalk)) || (index == 1 && !sidewalk) {
                        continue;
                    }
                    let p = p.with_y(p.y + 3. - distance + 0.98);
                    let mut blocked = false;
                    ctx.intersect_shape(
                        p,
                        Quat::IDENTITY,
                        shape.raw.as_ref(),
                        QueryFilter::default()
                            .exclude_collider(runner)
                            .exclude_rigid_body(owner),
                        |_, _| {
                            blocked = true;
                            false
                        },
                    );
                    if !blocked {
                        placement = Some(p);
                        break;
                    }
                }
                if placement.is_some() {
                    break;
                }
            }
            let p = placement.expect("clear native foot crossing start");
            commands
                .entity(runner)
                .insert(Transform::from_translation(p).with_rotation(rotation));
            commands.queue(move |world: &mut World| {
                let mut recovery = world.get_mut::<Recovery>(owner).expect("recovering rider");
                recovery.previous_foot = Some(p);
                recovery.walk_path.clear();
                recovery.walk_goal = None;
                recovery.path_refresh = 0.;
            });
            state.walk_crossing_setup[index] = true;
            println!(
                "Native foot crossing setup {index}: station={:.3} start={p:?} (farm terrain / city sidewalk)",
                rider.progress
            );
        }
        // Diagnostics only: preserve the physical setup, all recovery assertions
        // and timeout. A stalled run must explain what its actual bodies did.
        let interval = (recovery.elapsed / 5.).floor() as u32;
        if matches!(
            recovery.stage,
            RecoveryStage::Running | RecoveryStage::GettingUp
        ) && interval >= 2
            && interval > state.recovery_diagnostic_tick[index]
        {
            state.recovery_diagnostic_tick[index] = interval;
            let runner = recovery
                .runner
                .and_then(|e| bodies.get(e).ok())
                .map(|t| t.translation);
            let wreck = recovery
                .wreck
                .and_then(|e| bodies.get(e).ok())
                .map(|t| (t.translation, t.rotation));
            let speed = recovery
                .wreck
                .and_then(|e| velocities.get(e).ok())
                .map(|v| (v.linear, v.angular));
            let output = recovery.runner.and_then(|e| controller_outputs.get(e).ok());
            let contacts: Vec<_> = output
                .into_iter()
                .flat_map(|o| o.collisions.iter())
                .take(8)
                .map(|c| {
                    (
                        c.entity,
                        names.get(c.entity).map_or("unnamed", Name::as_str),
                        ground.get(c.entity).ok(),
                        bodies.get(c.entity).ok().map(|t| t.translation),
                    )
                })
                .collect();
            let ahead = runner.zip(wreck).and_then(|(from, (p, q))| {
                let direction = (p + q * Vec3::Y * 0.85 - from)
                    .with_y(0.)
                    .normalize_or_zero();
                let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                    Some(e) != recovery.runner && Some(e) != recovery.wreck
                };
                context
                    .single()
                    .ok()?
                    .cast_ray_and_get_normal(
                        from,
                        direction,
                        2.,
                        true,
                        bevy_rapier3d::prelude::QueryFilter::default().predicate(&predicate),
                    )
                    .map(|(e, hit)| {
                        (
                            e,
                            ground.get(e).ok(),
                            hit.normal,
                            hit.time_of_impact,
                            bodies.get(e).ok().map(|t| t.translation),
                        )
                    })
            });
            println!("Native recovery avoidance ray {index}: hit={ahead:?}");
            println!(
                "Native recovery diagnostic {index}: stage={:?} elapsed={:.2} station={:.3} lane={:.3} foot={:?} input={:?} runner={runner:?} wreck={wreck:?} velocity={speed:?} walk={:.3} next={:?} controller={:?} contacts={contacts:?}",
                recovery.stage,
                recovery.elapsed,
                rider.progress,
                rider.lane,
                recovery.foot.mode,
                controls.foot,
                recovery.walk_distance,
                recovery.walk_path.first(),
                output.map(|o| (o.grounded, o.desired_translation, o.effective_translation))
            );
        }
        match recovery.stage {
            RecoveryStage::GettingUp => state.fixed_stages[0] = true,
            RecoveryStage::Running => {
                state.fixed_stages[1] = true;
                if std::env::var("ROADDASH_VERIFY_WALK_CROSSINGS").as_deref() == Ok("1") {
                    let tick = recovery.elapsed.floor() as u32;
                    if tick > state.walk_crossing_diagnostic_tick[index] {
                        state.walk_crossing_diagnostic_tick[index] = tick;
                        println!(
                            "Native walk profile {index}: elapsed={:.2}s distance={:.3}m runner={:?} wreck={:?} next={:?} controller={:?}",
                            recovery.elapsed,
                            recovery.walk_distance,
                            recovery
                                .runner
                                .and_then(|e| bodies.get(e).ok())
                                .map(|t| t.translation),
                            recovery
                                .wreck
                                .and_then(|e| bodies.get(e).ok())
                                .map(|t| t.translation),
                            recovery.walk_path.first(),
                            recovery
                                .runner
                                .and_then(|e| controller_outputs.get(e).ok())
                                .map(|o| (
                                    o.grounded,
                                    o.desired_translation,
                                    o.effective_translation
                                ))
                        );
                    }
                    assert!(
                        recovery.elapsed < 30.,
                        "native foot crossing stuck; rescue must not mask it"
                    );
                    state.walk_crossing_distance[index] =
                        state.walk_crossing_distance[index].max(recovery.walk_distance);
                }
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
            RecoveryStage::Mounting => {
                state.fixed_stages[2] = true;
                state.last_pickup = recovery
                    .wreck
                    .and_then(|w| bodies.get(w).ok())
                    .map(|t| t.translation);
            }
            RecoveryStage::Riding => {
                if let Some(pickup) = state.last_pickup.take() {
                    let error = root.translation.distance(pickup);
                    assert!(
                        error < 0.5,
                        "native remount snapped {error} m away from pickup"
                    );
                    state.max_remount_error = state.max_remount_error.max(error);
                    state.remounts += 1;
                    println!(
                        "Native remount {}: pickup error {error:.4} m",
                        state.remounts
                    );
                }
            }
            _ => {}
        }
    }
}
fn smoke_weapon_family() -> rr_core::weapons::Weapon {
    let id = std::env::var("ROADDASH_VERIFY_WEAPON_FAMILY").unwrap_or_else(|_| "club".into());
    rr_core::weapons::Weapon::ALL
        .into_iter()
        .find(|w| w.id() == id)
        .expect("unknown ROADDASH_VERIFY_WEAPON_FAMILY")
}
/// Native evidence for the real keyboard -> fixed windup -> contact path.
/// Captures the next rendered/interpolated pose; it is not a human hit-feel test.
pub fn observe_smoke_strike(
    mut commands: Commands,
    mut state: ResMut<SmokeProgress>,
    riders: Query<&Rider>,
) {
    if std::env::var("ROADDASH_VERIFY_STRIKES").as_deref() != Ok("1") {
        return;
    }
    for r in &riders {
        if r.id != 0 {
            continue;
        }
        let Some(clock) = r.strike else { continue };
        if std::env::var("ROADDASH_VERIFY_CLUB").as_deref() == Ok("1") {
            if clock.pending() {
                assert_eq!(
                    r.weapon,
                    Some(smoke_weapon_family()),
                    "native family identity lost before contact"
                );
            }
            let expected = rr_content::weapons::Weapons::builtin().ability(smoke_weapon_family());
            assert_eq!(
                clock.weapon.unwrap().lateral_reach,
                expected.lateral_reach,
                "native family ability lost"
            );
        }
        if !state.strike_windup && clock.pending() {
            state.strike_windup = true;
            println!(
                "Native buffered strike windup: age={:.6} contact={:.6} side={}",
                clock.age, clock.contact, clock.side
            );
            capture(&mut commands, "30-strike-windup");
        }
        if !state.strike_contact && !clock.pending() {
            assert!(
                clock.age >= clock.contact_window().0
                    && clock.age <= clock.contact_window().1 + 2.0 / 60.0
            );
            state.strike_contact = true;
            println!(
                "Native buffered strike contact: age={:.6} contact={:.6} side={}",
                clock.age, clock.contact, clock.side
            );
            capture(&mut commands, "31-strike-contact");
        }
    }
}
fn capture(commands: &mut Commands, name: &str) {
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(format!("screenshots/{name}.png")));
}
/// Exercise the real keyboard event path; ButtonInput::press in Update loses
/// its just-pressed edge when the native InputPlugin clears edges next frame.
pub(crate) fn retry_key(commands: &mut Commands, pressed: bool) {
    smoke_key(commands, KeyCode::KeyR, pressed);
}
pub(crate) fn smoke_key(commands: &mut Commands, key_code: KeyCode, pressed: bool) {
    commands.queue(move |world: &mut World| {
        let window = world
            .query_filtered::<Entity, With<bevy::window::PrimaryWindow>>()
            .single(world)
            .expect("native primary window");
        world.write_message(bevy::input::keyboard::KeyboardInput {
            key_code,
            logical_key: bevy::input::keyboard::Key::Character(
                if key_code == KeyCode::KeyJ { "j" } else { "r" }.into(),
            ),
            state: if pressed {
                bevy::input::ButtonState::Pressed
            } else {
                bevy::input::ButtonState::Released
            },
            text: None,
            repeat: false,
            window,
        });
    });
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn smoke_test(
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
    (telemetry, mut panel, pads, prefs, screen, art, clubs, bike_audio): (
        Res<crate::vehicle::VehicleTelemetry>,
        ResMut<crate::settings::SettingsPanel>,
        Query<&Gamepad>,
        Res<crate::settings::Preferences>,
        Res<State<crate::runtime::AppScreen>>,
        Res<crate::world::Art>,
        (
            Query<(), With<crate::weapons::Held>>,
            Query<(), With<crate::weapons::Pickup>>,
            Query<&InheritedVisibility, With<crate::weapons::ClubPart>>,
        ),
        Query<
            (
                &crate::sound::BikeEmitter,
                &Transform,
                &bevy::audio::SpatialAudioSink,
            ),
            Without<Rider>,
        >,
    ),
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    let test_route = usize::from(std::env::var("ROADDASH_SMOKE_ROUTE").as_deref() == Ok("1"));
    state.frame += 1;
    // Opt-in confirmed-contact fixture: use the real melee resolver/LOS/audio,
    // with a temporary logical attacker. It isolates victim presentation; the
    // separate keyboard fixture verifies live windup/contact timing.
    if std::env::var("ROADDASH_VERIFY_REACTIONS").as_deref() == Ok("1") {
        for (start, side, kicking) in [
            (70, -1.0, false),
            (110, 1.0, false),
            (150, -1.0, true),
            (190, 1.0, true),
        ] {
            if state.frame == start {
                commands.queue(move |world: &mut World| {
                    use bevy::ecs::system::RunSystemOnce;
                    let (target, pose) = world
                        .query::<(Entity, &Rider, &Transform)>()
                        .iter(world)
                        .find(|(_, r, _)| r.id == 0)
                        .map(|(e, _, t)| (e, *t))
                        .unwrap();
                    world.get_mut::<Rider>(target).unwrap().immunity = 0.0;
                    let right = (pose.rotation * Vec3::Z).cross(Vec3::Y).normalize();
                    let attacker = world
                        .spawn((
                            Rider::new(2),
                            Transform::from_translation(pose.translation - right * side * 1.4)
                                .with_rotation(pose.rotation),
                            Visibility::Hidden,
                        ))
                        .id();
                    world.write_message(crate::combat::AttackAttempt(
                        attacker,
                        if kicking {
                            rr_core::AttackSide::Kick
                        } else if side < 0.0 {
                            rr_core::AttackSide::Left
                        } else {
                            rr_core::AttackSide::Right
                        },
                    ));
                    world.run_system_once(crate::combat::melee).unwrap();
                    let hit = world
                        .get::<Rider>(target)
                        .unwrap()
                        .hit_reaction
                        .expect("native successful hit must create reaction");
                    assert_eq!(hit.kicking, kicking);
                    assert_eq!(hit.direction, -side);
                    println!(
                        "Native confirmed reaction: side={side} kick={kicking} at={:.6}",
                        hit.at
                    );
                    world.despawn(attacker);
                });
            }
            for offset in [0, 1, 4, 12, 25] {
                if state.frame == start + offset {
                    capture(&mut commands, &format!("reaction-{start}-{offset:02}"));
                }
            }
        }
    }
    if std::env::var("ROADDASH_VERIFY_STRIKES").as_deref() == Ok("1") {
        if state.frame == 60 && std::env::var("ROADDASH_VERIFY_CLUB").as_deref() != Ok("1") {
            smoke_key(&mut commands, KeyCode::KeyJ, true);
        }
        if state.frame == 64 {
            smoke_key(&mut commands, KeyCode::KeyJ, false);
        }
    }

    if std::env::var("ROADDASH_VERIFY_CLUB").as_deref() == Ok("1") {
        if state.frame == 61
            && let Some((_, _, t, _)) = riders.iter().find(|(r, _, _, _)| r.id == 0)
        {
            let family = smoke_weapon_family();
            if family == rr_core::weapons::Weapon::Club {
                crate::weapons::pickup(&mut commands, &art, t.translation + Vec3::Y * 0.12);
            } else {
                crate::weapons::pickup_kind(
                    &mut commands,
                    &art,
                    t.translation + Vec3::Y * 0.12,
                    family,
                );
            }
        }
        for (frame, key, down) in [
            (62, KeyCode::KeyE, true),
            (64, KeyCode::KeyE, false),
            (66, KeyCode::KeyJ, true),
            (68, KeyCode::KeyJ, false),
            (110, KeyCode::KeyQ, true),
            (112, KeyCode::KeyQ, false),
        ] {
            if state.frame == frame {
                smoke_key(&mut commands, key, down);
            }
        }
        if state.frame == 65 {
            assert!(
                riders
                    .iter()
                    .find(|(r, _, _, _)| r.id == 0)
                    .unwrap()
                    .0
                    .weapon
                    .is_some(),
                "native keyboard pickup missing"
            );
        }
        if state.frame == 114 {
            assert!(
                riders
                    .iter()
                    .find(|(r, _, _, _)| r.id == 0)
                    .unwrap()
                    .0
                    .weapon
                    .is_none(),
                "native drop missing"
            );
            println!(
                "Native original {} pickup / timed attack / drop verified",
                smoke_weapon_family().id()
            );
            let census = serde_json::json!({"held_roots":clubs.0.iter().count(),"pickup_roots":clubs.1.iter().count(),"resident_part_instances":clubs.2.iter().count(),"visible_fallback_part_instances":clubs.2.iter().filter(|v|v.get()).count(),"family":smoke_weapon_family().id(),"additional_asset_counts":"not_measured","new_physical_bodies":0,"max_held_roots":30,"max_pickup_roots":12});
            std::fs::write(
                "screenshots/weapon-census.json",
                serde_json::to_vec_pretty(&census).unwrap(),
            )
            .unwrap();
            assert_eq!(
                clubs.0.iter().count(),
                30,
                "weapon sockets did not attach to all riders"
            );
        }
    }
    if state.frame == 1 {
        let rivals = riders
            .iter()
            .filter(|(r, _, _, _)| !r.police && r.id != 0)
            .count();
        assert_eq!(rivals, 14);
        println!("AI roster verified: fourteen rivals, five fighters");
    }
    if std::env::var("ROADDASH_VERIFY_REPAIRS").as_deref() == Ok("1") {
        if state.frame == 25 {
            race.phase = Phase::Racing;
            for (mut rider, _, mut t, velocity) in &mut riders {
                if rider.id == 0 {
                    rider.route = test_route;
                    let (p, q) = track.sample_route(test_route, 3200.0, 1.0);
                    rider.progress = 3200.0;
                    rider.lane = 1.0;
                    t.translation = p;
                    t.rotation = q;
                    if let Some(mut v) = velocity {
                        *v = bevy_rapier3d::prelude::Velocity::zero();
                    }
                }
            }
        }
        if state.frame == 26 {
            retry_key(&mut commands, true);
        }
        if state.frame == 28 {
            for (r, _, t, _) in &riders {
                if r.police {
                    continue;
                }
                let spawn = &track.definition.spawns[r.id];
                let (p, _) =
                    track.sample_route(test_route, track.grid_station(spawn.distance), spawn.lane);
                assert!(
                    t.translation.distance(p) < 0.2,
                    "native retry did not restore grid for {}: phase={:?} progress={} body={:?} start={:?}",
                    r.id,
                    race.phase,
                    r.progress,
                    t.translation,
                    p
                );
                assert!(r.speed < 0.01 && r.gates.completed_laps == 0);
            }
            assert_eq!(race.phase, Phase::Countdown);
            println!(
                "Native retry passed: all fifteen racers restored from mid-course to authored grid"
            );
            capture(&mut commands, "23-restart-grid");
            retry_key(&mut commands, false);
        }
        let pose = match state.frame {
            82 | 83 => Some(("18-punch-left", -1.0, false)),
            112 | 113 => Some(("19-punch-right", 1.0, false)),
            142 | 143 => Some(("20-kick-left", -1.0, true)),
            162 | 163 => Some(("21-kick-right", 1.0, true)),
            _ => None,
        };
        if let Some((name, side, kick)) = pose {
            for (mut rider, _, _, _) in &mut riders {
                if rider.id == 0 {
                    rider.route = test_route;
                    rider.attack = 0.475;
                    rider.attack_side = side;
                    rider.kicking = kick;
                }
            }
            if [83, 113, 143, 163].contains(&state.frame) {
                capture(&mut commands, name);
            }
        }
    }
    if std::env::var("ROADDASH_VERIFY_REVERSE").as_deref() == Ok("1") {
        match state.frame {
            65 => {
                keys.release(KeyCode::KeyW);
                keys.press(KeyCode::KeyS);
            }
            125 => {
                for (r, recovery, t, velocity) in &riders {
                    if r.id == 0 {
                        let speed = velocity
                            .as_ref()
                            .expect("dynamic chassis")
                            .linear
                            .dot(t.rotation * Vec3::Z);
                        assert!(
                            speed < -0.3 && speed > -3.5,
                            "native reverse failed: {speed}"
                        );
                        assert_eq!(recovery.stage, RecoveryStage::Riding);
                        println!("Native reverse verified: {speed:.3} m/s");
                    }
                }
                capture(&mut commands, "18-reversing");
            }
            140 => {
                keys.release(KeyCode::KeyS);
                keys.press(KeyCode::KeyW);
            }
            _ => {}
        }
    }
    match state.frame {
        30 => {
            assert!(sinks.iter().count() >= 4, "audio output did not initialize");
            println!(
                "Audio output initialized: {} playback sinks",
                sinks.iter().count()
            );
            capture(&mut commands, "01-garage");
        }
        35 => panel.open = true,
        50 => capture(&mut commands, "15-comfort-settings"),
        55 => panel.open = false,
        60 => {
            race.phase = Phase::Racing;
            keys.press(KeyCode::KeyW);
        }
        170 => keys.press(KeyCode::KeyD),
        185 => capture(&mut commands, "16-steering-buildup"),
        229 => capture(&mut commands, "17-held-steering"),
        230 => {
            keys.release(KeyCode::KeyD);
            metrics.visible = true;
        }
        240 => {
            assert!(
                sinks
                    .iter()
                    .any(|s| !s.is_paused() && s.volume().to_linear() > 0.0),
                "no racing audio playing"
            );
            assert!(
                bike_audio
                    .iter()
                    .filter(|(_, _, sink)| { !sink.is_paused() && sink.volume().to_linear() > 0.0 })
                    .count()
                    > 1,
                "nearby bikes have no audible spatial engine sinks"
            );
            println!("Native spatial bike engines audible during racing");
            capture(&mut commands, "02-racing");
        }
        260 => {
            for (mut rider, _, mut transform, mut velocity) in &mut riders {
                if rider.id == 0 {
                    rider.route = test_route;
                    if track.is_hills() {
                        rider.progress = 6200.;
                        rider.lane = 0.;
                        let (p, q) = track.sample_route(test_route, rider.progress, 0.);
                        *transform = Transform::from_translation(p).with_rotation(q);
                        rider.speed = 30.;
                        rider.motion_velocity = q * Vec3::Z * rider.speed;
                        if let Some(v) = velocity.as_mut() {
                            **v = bevy_rapier3d::prelude::Velocity::linear(rider.motion_velocity);
                        }
                        contacts.clear();
                        sweeps.clear();
                    } else if std::env::var("ROADDASH_VERIFY_WALK_CROSSINGS").as_deref() == Ok("1")
                    {
                        rider.progress = 2800.;
                        rider.lane = 0.;
                        rider.speed = 0.;
                        rider.motion_velocity = Vec3::ZERO;
                        let (p, q) = track.sample_route(test_route, 2800., 0.);
                        *transform = Transform::from_translation(p).with_rotation(q);
                        if let Some(v) = velocity.as_mut() {
                            **v = bevy_rapier3d::prelude::Velocity::zero();
                        }
                        contacts.clear();
                        sweeps.clear();
                    }
                    rider.health = 0.0;
                }
            }
            keys.release(KeyCode::KeyW);
        }
        280 => {
            let wreck = riders
                .iter()
                .find(|(r, _, _, _)| r.id == 0)
                .and_then(|(_, recovery, _, _)| recovery.wreck)
                .and_then(|entity| bodies.get(entity).ok())
                .expect("local crash must have a wreck");
            let wreck_engine = wreck.translation + wreck.rotation * Vec3::Y * 0.45;
            assert!(
                bike_audio.iter().any(|(emitter, position, sink)| {
                    riders.get(emitter.0).is_ok_and(|(r, _, _, _)| r.id == 0)
                        && position.translation.distance(wreck_engine) < 3.
                        && !sink.is_paused()
                        && sink.volume().to_linear() > 0.
                }),
                "local engine did not stay audible at the wreck"
            );
            println!("Native local bike engine remained audible at the wreck");
            capture(&mut commands, "03-crash");
        }
        _ => {}
    }
    for (mut rider, recovery, mut transform, mut velocity) in &mut riders {
        if rider.id != 0 || state.frame <= 280 {
            continue;
        }
        rider.route = test_route;
        if let Some(recovered_at) = state.recovered_at {
            let elapsed = state.frame - recovered_at;
            if elapsed == 20 || elapsed == 60 {
                let sector = if elapsed == 20 {
                    rr_content::track::Sector::City
                } else {
                    rr_content::track::Sector::Hills
                };
                let index = if track.is_hills() {
                    let distance = if elapsed == 60 { 3800. } else { 6200. };
                    track.distances.partition_point(|d| *d < distance)
                } else if track.metropolis.is_some() {
                    let distance = if elapsed == 60 { 4400.0 } else { 4300.0 };
                    track.distances.partition_point(|d| *d < distance)
                } else if elapsed == 60 && track.city_surface.is_some() {
                    track
                        .distances
                        .partition_point(|d| *d < track.length * 0.72)
                } else if elapsed == 60 {
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
                let (p, q) = track.sample_route(test_route, rider.progress, 0.0);
                *transform = Transform::from_translation(p).with_rotation(q);
                if let Some(v) = velocity.as_mut() {
                    **v = bevy_rapier3d::prelude::Velocity::zero();
                }
                contacts.clear();
                sweeps.clear();
            }
            if elapsed == 45 {
                race.camera_mode = 1;
            }
            if elapsed == 60 {
                race.camera_mode = 2;
            }
            if elapsed == 130 {
                race.camera_mode = 0;
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
                rider.progress = track.race_interval().finish * LAPS as f32 + 1.0;
                rider.lane = 0.0;
                let (p, q) = track.sample_route(test_route, rider.progress, rider.lane);
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
            if std::env::var("ROADDASH_VERIFY_STRIKES").as_deref() == Ok("1") {
                assert!(
                    state.strike_windup && state.strike_contact,
                    "native buffered strike lifecycle missing"
                );
            }
            let mut report = metrics.evidence(&clock);
            report["gamepads_connected"] = serde_json::json!(pads.iter().count());
            report["comfort"] = serde_json::json!(prefs.file.comfort);
            report["fixed_recovery_stages"] = serde_json::json!(state.fixed_stages);
            report["exceptional_recovery"] = serde_json::json!({
                "counts_first_and_hill":state.rescue_counts,
                "penalties_seconds":state.rescue_penalties,
                "stationary_seconds":state.rescue_stationary_seconds,
                "extended_hills_fixture":std::env::var("ROADDASH_VERIFY_HILLS_RESCUE").as_deref()==Ok("1"),
                "unassisted_both":state.rescue_counts.iter().all(|count| *count==0)
            });
            if track.is_hills()
                && std::env::var("ROADDASH_VERIFY_HILLS_RESCUE").as_deref() == Ok("1")
            {
                assert_eq!(
                    state.rescue_counts[0], 0,
                    "first Hills recovery must remain unassisted"
                );
                assert_eq!(
                    state.remounts, 2,
                    "both physical recoveries must actually remount"
                );
                if state.rescue_counts[1] > 0 {
                    assert!(
                        state.rescue_stationary_seconds[1] >= 120.,
                        "exceptional Hills rescue fired early"
                    );
                    assert_eq!(
                        state.rescue_penalties[1],
                        5. * state.rescue_counts[1] as f32
                    );
                }
            }
            if std::env::var("ROADDASH_VERIFY_WALK_CROSSINGS").as_deref() == Ok("1") {
                assert!(
                    state.walk_crossing_setup.iter().all(|v| *v),
                    "missing native surface crossings"
                );
                assert_eq!(
                    state.remounts, 2,
                    "native foot crossings must reach both bikes"
                );
                report["walk_crossing_setups"] = serde_json::json!(state.walk_crossing_setup);
                assert!(
                    state.walk_crossing_distance.iter().all(|v| *v > 5.),
                    "native fixture must physically walk back to both bikes"
                );
                report["walk_crossing_distances_m"] =
                    serde_json::json!(state.walk_crossing_distance);
            }
            if std::env::var_os("ROADDASH_VERIFY_MANUAL_RETURN").is_some() {
                assert!(
                    state.manual_seen && state.auto_return_seen,
                    "manual interruption/automatic resume missing"
                );
                report["manual_interruption_and_auto_resume"] =
                    serde_json::json!([state.manual_seen, state.auto_return_seen]);
            }
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
            report["route"] = serde_json::json!(test_route);
            report["track"] = serde_json::json!({"id":track.definition.id.as_str(),"closed":track.definition.closed,"length_m":track.length,"race_start_m":track.race_interval().start,"race_finish_m":track.race_interval().finish,"race_length_m":track.race_interval().length(),"gates":track.definition.checkpoints.len(),"render_triangles":inspector.render_triangles,"collision_triangles":inspector.collision_triangles});
            report["recovery_at_run_capture"] = serde_json::json!({"farm_distance_and_height_delta_m":state.farm_walk,"hill_distance_and_height_delta_m":state.hill_walk});
            if let Some(adapter) = adapter.as_ref() {
                report["adapter"] = serde_json::json!({"name": adapter.name, "driver": adapter.driver, "backend": format!("{:?}", adapter.backend)});
            }
            std::fs::write(
                "screenshots/smoke-metrics.json",
                serde_json::to_vec_pretty(&report).unwrap(),
            )
            .expect("write smoke evidence");
            if track.is_hills() {
                println!(
                    "Render smoke test passed: Hills route {test_route} -> city/town/hills -> crash recovery -> results fixture; exceptional rescues {:?}",
                    state.rescue_counts
                );
            } else if track.metropolis.is_some() {
                println!(
                    "Render smoke test passed: Metropolis route {test_route} -> farm/city crashes -> both recoveries -> results fixture"
                );
            } else if track.city_surface.is_some() {
                println!(
                    "Render smoke test passed: City Run 5300 m -> two crashes -> both recoveries -> finish"
                );
            } else {
                println!(
                    "Render smoke test passed: farm crash -> remount -> city/hills -> hill crash -> remount -> finish"
                );
            }
            exit.write(AppExit::Success);
        }
    }
    let limit = if track.is_hills()
        && std::env::var("ROADDASH_VERIFY_HILLS_RESCUE").as_deref() == Ok("1")
    {
        12000
    } else {
        6000
    };
    assert!(
        state.frame < limit,
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
    ragdoll: Res<crate::biomechanics::Settings>,
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
            } else if ragdoll.enabled {
                16
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

/// Close native source-family views, using the real walking and knockdown paths.
/// This fixture never runs without --actor-skin-survey.
#[derive(Resource, Default)]
pub struct ActorSkinSurvey {
    frame: u32,
    origin: Vec3,
    rotation: Quat,
    actors: Vec<Entity>,
}

#[allow(clippy::too_many_arguments)]
pub fn actor_skin_survey(
    mut commands: Commands,
    mut state: ResMut<ActorSkinSurvey>,
    mut race: ResMut<Race>,
    track: Res<Track>,
    art: Res<crate::world::Art>,
    mut actors: Query<(
        Entity,
        &mut crate::environment::Patrol,
        &crate::props::ActorAppearance,
    )>,
    mut hits: MessageWriter<crate::combat::NpcHit>,
    mut exit: MessageWriter<AppExit>,
) {
    state.frame += 1;
    if state.frame == 30 {
        race.phase = Phase::Racing;
        let (p, q) = track.sample(1200.0, 0.0);
        state.origin = p + q * Vec3::Z * 20.0 + q * Vec3::X * 8.0;
        state.rotation = q;
        for (i, seed) in [0.0, 154.0, 294.0].into_iter().enumerate() {
            crate::environment::spawn_pedestrian(
                &mut commands,
                &art,
                state.origin + q * Vec3::X * (i as f32 - 1.0) * 2.0,
                q,
                seed,
            );
        }
    }
    if state.frame == 32 {
        let origin = state.origin;
        let mut selected: Vec<_> = actors
            .iter_mut()
            .filter(|(_, p, _)| p.center.distance(origin) < 5.0)
            .collect();
        selected.sort_by(|a, b| a.1.center.x.total_cmp(&b.1.center.x));
        for (e, mut patrol, appearance) in selected {
            patrol.extent = 0.15;
            patrol.speed = 0.08;
            println!("Actor skin survey source: {}", appearance.0);
            state.actors.push(e);
        }
        assert_eq!(state.actors.len(), 3);
    }
    if (160..700).contains(&state.frame) && (state.frame - 160) % 60 == 30 {
        let index = (state.frame - 160) / 60;
        capture(&mut commands, &format!("actor-skin-walk-{index:02}"));
    }
    if state.frame == 720 {
        for &actor in &state.actors {
            hits.write(crate::combat::NpcHit(
                actor,
                state.rotation * Vec3::Z * 4.0,
                false,
            ));
        }
    }
    for (frame, name) in [
        (730, "actor-skin-impact"),
        (780, "actor-skin-ragdoll-front"),
        (840, "actor-skin-ragdoll-rear"),
    ] {
        if state.frame == frame {
            capture(&mut commands, name);
        }
    }
    if state.frame == 900 {
        println!("Actor skin native close survey completed; visual acceptance remains open");
        exit.write(AppExit::Success);
    }
}

pub fn actor_skin_camera(
    state: Res<ActorSkinSurvey>,
    actors: Query<(&crate::npc::Npc, &Transform), Without<crate::game::ChaseCamera>>,
    bodies: Query<
        &Transform,
        (
            With<crate::recovery::OwnedBody>,
            Without<crate::game::ChaseCamera>,
        ),
    >,
    mut cameras: Query<(&mut Transform, &mut Projection), With<crate::game::ChaseCamera>>,
) {
    if state.frame < 100 || state.actors.is_empty() {
        return;
    }
    let index = ((state.frame.saturating_sub(160)) / 60).min(8) as usize;
    let family = index / 3;
    let Ok((npc, root)) = actors.get(state.actors[family]) else {
        return;
    };
    let ragdoll = state.frame >= 720;
    let target = if ragdoll {
        npc.parts
            .first()
            .and_then(|e| bodies.get(*e).ok())
            .map_or(root.translation, |t| t.translation)
            + Vec3::Y * 0.2
    } else {
        root.translation + Vec3::Y * 1.15
    };
    let view = if state.frame >= 840 {
        1
    } else if ragdoll {
        0
    } else {
        index % 3
    };
    let offset = match view {
        0 => Vec3::new(1.8, 0.5, 3.2),
        1 => Vec3::new(-1.8, 0.5, -3.2),
        _ => Vec3::new(3.5, 0.4, 0.2),
    };
    for (mut camera, mut projection) in &mut cameras {
        *camera = Transform::from_translation(target + state.rotation * offset)
            .looking_at(target, Vec3::Y);
        if let Projection::Perspective(p) = &mut *projection {
            p.fov = 45_f32.to_radians();
        }
    }
}

/// Read final propagated driver frames; never changes a physical or visual pose.
pub(crate) fn actor_skin_snapshot(
    state: Res<ActorSkinSurvey>,
    bindings: Query<(
        &crate::props::ActorBinding,
        &crate::props::ActorAppearance,
        &InheritedVisibility,
    )>,
    transforms: Query<&GlobalTransform>,
) {
    use std::io::Write;
    let path = "screenshots/actor-skin-poses.jsonl";
    if state.frame == 1 {
        std::fs::write(path, "").expect("skin pose receipt");
    }
    if !((190..700).contains(&state.frame) && (state.frame - 190).is_multiple_of(60)
        || [730, 780, 840].contains(&state.frame))
    {
        return;
    }
    let mut rows = vec![];
    for (binding, appearance, visible) in &bindings {
        if !visible.get() {
            continue;
        }
        let (drivers, file) = binding.snapshot();
        if drivers.len() != 11 {
            continue;
        }
        let matrices: Option<Vec<_>> = drivers
            .iter()
            .enumerate()
            .map(|(i, e)| {
                transforms.get(*e).ok().map(|t| {
                    (t.to_matrix() * Mat4::from_translation(-crate::human::SPECS[i].0))
                        .to_cols_array()
                })
            })
            .collect();
        let Some(matrices) = matrices else {
            continue;
        };
        let origin = Mat4::from_cols_array(&matrices[0]).transform_point3(crate::human::SPECS[0].0);
        if origin.distance(state.origin) > 25.0 {
            continue;
        }
        rows.push(serde_json::json!({"family":appearance.0,"file":file,"driver_skin_matrices_column_major":matrices}));
    }
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .expect("skin pose receipt");
    writeln!(file, "{}", serde_json::json!({"frame":state.frame,"phase":if state.frame<720 {"walk"}else if state.frame==730 {"impact"}else {"ragdoll"},"actors":rows})).expect("skin pose receipt");
}
