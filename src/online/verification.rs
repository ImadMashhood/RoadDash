//! Multi-process test node: the production network and gameplay plugins, without GPU art.
//! This is a repeatable integration fixture, not an alternate online game simulation.
use super::*;
use bevy_rapier3d::prelude::*;
#[derive(Resource, Default, Serialize)]
pub struct Receipts {
    samples: Vec<(u64, u64, f32)>,
    max_players: usize,
    serialized_bytes: u64,
    encoded_frame_bytes: u64,
    largest_encoded_frame: usize,
    largest_snapshot: usize,
    remote_max_speed: f32,
    first_snapshot_wall_seconds: Option<f32>,
    remote_attack: bool,
    remote_crash: bool,
    remote_foot: bool,
    remote_remount: bool,
    errors: Vec<String>,
    final_recovery_stage: u8,
    remote_walk_distance: f32,
    final_runner_bike_distance: Option<f32>,
}
pub(super) fn wire_receipt(world: &mut World, bytes: usize) {
    if let Some(mut r) = world.get_resource_mut::<Receipts>() {
        r.encoded_frame_bytes += bytes as u64;
        r.largest_encoded_frame = r.largest_encoded_frame.max(bytes);
    }
}
pub(super) fn receipt(world: &mut World, snapshot: &super::replication::Snapshot) {
    if !world.contains_resource::<Receipts>() {
        return;
    }
    let bytes = serde_json::to_vec(snapshot).unwrap();
    let hash = bytes.iter().fold(0xcbf29ce484222325u64, |a, b| {
        (a ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    let wall = world
        .get_resource::<Fixture>()
        .map(|f| f.start.elapsed().as_secs_f32());
    let mut receipts = world.resource_mut::<Receipts>();
    if let Some((stage, speed, serial)) = snapshot.fixture_rider(1) {
        receipts.remote_max_speed = receipts.remote_max_speed.max(speed.abs());
        receipts.remote_attack |= serial > 0;
        receipts.remote_crash |= stage == 1;
        receipts.remote_foot |= stage == 3;
        receipts.remote_remount |= receipts.remote_foot && stage == 0;
    }
    receipts.serialized_bytes += bytes.len() as u64;
    receipts.largest_snapshot = receipts.largest_snapshot.max(bytes.len());
    if receipts.first_snapshot_wall_seconds.is_none() {
        receipts.first_snapshot_wall_seconds = wall;
    }
    if receipts.samples.len() < 2000 {
        receipts
            .samples
            .push((snapshot.tick, hash, snapshot.elapsed));
    }
}
#[derive(Resource)]
struct Fixture {
    host: bool,
    start: Instant,
    output: String,
    crashed: bool,
    native: bool,
    session_started: bool,
    announced: bool,
    start_requested: bool,
}
pub fn run(host: bool, endpoint: String, output: String) {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1. / 60.),
        )),
        TransformPlugin,
        bevy::state::app::StatesPlugin,
        crate::content::RoadContentPlugin,
        crate::runtime::RoadLifecyclePlugin,
        crate::runtime::RoadSimulationPlugin,
        crate::controls::RoadInputPlugin,
        OnlinePlugin,
    ))
    .init_resource::<ButtonInput<KeyCode>>()
    .init_resource::<crate::sound::Muted>()
    .init_resource::<crate::ui::MenuAction>()
    .insert_resource(crate::game::Persistence(false))
    .insert_resource(crate::world::Art {
        props: None,
        models: None,
        cube: Handle::default(),
        sphere: Handle::default(),
        wheel: Handle::default(),
        dark: Handle::default(),
        metal: Handle::default(),
        white: Handle::default(),
        colors: vec![Handle::default(); 15],
        helmets: vec![Handle::default(); 15],
        bike_colors: vec![Handle::default(); 15],
    })
    .insert_resource(Fixture {
        host,
        start: Instant::now(),
        output,
        crashed: false,
        native: false,
        session_started: true,
        announced: true,
        start_requested: true,
    })
    .init_resource::<Receipts>()
    .add_systems(
        PreUpdate,
        script.after(crate::controls::capture).before(super::pump),
    )
    .add_systems(
        FixedUpdate,
        crash
            .after(crate::game::drive)
            .before(crate::runtime::SimSet::AfterPhysics),
    )
    .add_systems(Last, observe);
    for id in 0..RACERS {
        app.world_mut().spawn((
            crate::game::Rider::new(id),
            Transform::default(),
            Visibility::default(),
            crate::recovery::bike_body(),
        ));
    }
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::cuboid(2000., 0.5, 2000.),
        crate::world::Ground,
        Transform::from_xyz(0., -0.5, 0.),
    ));
    app.world_mut().resource_mut::<Online>().endpoint = endpoint.clone();
    if std::env::var_os("ROADDASH_ONLINE_BOOT").is_some() {
        // The child resumes the exact private capability and selected map.
    } else if host {
        let host = open_host(&endpoint, false, OnlineMap::Metropolis).unwrap();
        let code = host.code.clone();
        app.world_mut().resource_mut::<NetState>().connection = Connection::Host(host);
        app.world_mut()
            .resource_mut::<Online>()
            .actions
            .push_back(OnlineAction::Start);
        println!("HOST_CODE={code}");
    } else {
        app.world_mut()
            .resource_mut::<Online>()
            .actions
            .push_back(OnlineAction::Join { code: endpoint });
    }
    if let AppExit::Error(code) = app.run() {
        std::process::exit(i32::from(code.get()));
    }
}
fn script(
    mut controls: ResMut<crate::controls::Controls>,
    race: Res<crate::game::Race>,
    fixture: Res<Fixture>,
    riders: Query<(&crate::game::Rider, &crate::recovery::Recovery)>,
    state: Res<NetState>,
) {
    controls.buffer.held.throttle = true;
    controls.buffer.held.throttle_amount = 1.;
    if !fixture.host && race.phase == crate::game::Phase::Racing {
        if race.elapsed < 2. {
            controls.buffer.push_attack(rr_core::AttackSide::Left);
        }
        if let Some((_, rec)) = riders.iter().find(|(r, _)| r.id == state.local_slot)
            && rec.stage == crate::recovery::RecoveryStage::Running
            && rec.elapsed < 0.5
        {
            controls.foot = Vec2::X;
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn crash(
    mut commands: Commands,
    mut fixture: ResMut<Fixture>,
    race: Res<crate::game::Race>,
    mut riders: Query<(
        Entity,
        &mut crate::game::Rider,
        &mut crate::recovery::Recovery,
        &Transform,
    )>,
    art: Res<crate::world::Art>,
    context: ReadRapierContext,
) {
    if !fixture.host || fixture.crashed || race.elapsed < 3. {
        return;
    }
    if let Some((e, mut r, mut rec, t)) = riders
        .iter_mut()
        .find(|(_, r, _, _)| r.id == 1 && r.remote_human)
    {
        crate::recovery::crash(
            &mut commands,
            &art,
            e,
            &mut r,
            &mut rec,
            t.translation,
            t.rotation,
            Vec3::ZERO,
            &context,
        );
        fixture.crashed = true;
    }
}
#[allow(clippy::too_many_arguments)]
fn observe(
    mut fixture: ResMut<Fixture>,
    mut online: ResMut<Online>,
    riders: Query<(&crate::game::Rider, &crate::recovery::Recovery, &Transform)>,
    mut receipts: ResMut<Receipts>,
    transforms: Query<&Transform>,
    mut exit: MessageWriter<AppExit>,
) {
    if fixture.host
        && !fixture.announced
        && let Some(l) = &online.lobby
    {
        println!("HOST_CODE={}", l.code);
        fixture.announced = true;
    }
    if fixture.native
        && fixture.host
        && fixture.start_requested
        && online.phase == OnlinePhase::Lobby
        && online.error.is_some()
    {
        fixture.start_requested = false;
    }
    if fixture.native
        && fixture.host
        && !fixture.start_requested
        && online.phase == OnlinePhase::Lobby
        && online
            .lobby
            .as_ref()
            .is_some_and(|l| l.players.iter().filter(|p| !p.bot).count() >= 2)
    {
        let requested = std::env::var("ROADDASH_TEST_RELOAD_MAP")
            .ok()
            .and_then(|s| match s.as_str() {
                "valley" => Some(OnlineMap::Valley),
                "city" => Some(OnlineMap::City),
                "metro" => Some(OnlineMap::Metropolis),
                _ => None,
            });
        if let Some(map) =
            requested.filter(|map| online.lobby.as_ref().is_some_and(|l| l.map != *map))
        {
            online.actions.push_back(OnlineAction::SetMap(map));
        } else {
            online.actions.push_back(OnlineAction::Start);
        }
        fixture.start_requested = true;
    }
    if fixture.native && !fixture.session_started {
        if matches!(online.phase, OnlinePhase::Countdown | OnlinePhase::Racing) {
            fixture.session_started = true;
            fixture.start = Instant::now();
        } else if fixture.start.elapsed() < Duration::from_secs(180) {
            return;
        }
    }
    receipts.max_players = receipts.max_players.max(
        online
            .lobby
            .as_ref()
            .map_or(0, |l| l.players.iter().filter(|p| !p.bot).count()),
    );
    if let Some((r, rec, _)) = riders.iter().find(|(r, _, _)| r.id == 1) {
        receipts.final_recovery_stage = rec.stage as u8;
        receipts.remote_walk_distance = rec.walk_distance;
        receipts.final_runner_bike_distance =
            rec.runner.zip(rec.wreck).and_then(|(runner, bike)| {
                Some(
                    transforms
                        .get(runner)
                        .ok()?
                        .translation
                        .distance(transforms.get(bike).ok()?.translation),
                )
            });
        receipts.remote_max_speed = receipts.remote_max_speed.max(r.speed.abs());
        receipts.remote_attack |= r.attack_serial > 0;
        receipts.remote_crash |= rec.stage == crate::recovery::RecoveryStage::Falling;
        receipts.remote_foot |= rec.stage == crate::recovery::RecoveryStage::Running;
        receipts.remote_remount |=
            receipts.remote_foot && rec.stage == crate::recovery::RecoveryStage::Riding;
    }
    if let Some(error) = &online.error
        && !receipts.errors.contains(error)
    {
        receipts.errors.push(error.clone());
    }
    let duration = std::env::var("ROADDASH_ONLINE_TEST_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(45)
        .clamp(30, 180);
    if fixture.start.elapsed() > Duration::from_secs(duration + if fixture.host { 3 } else { 0 }) {
        std::fs::write(
            &fixture.output,
            serde_json::to_vec_pretty(&*receipts).unwrap(),
        )
        .unwrap();
        let passed = receipts.samples.len() > 100
            && receipts.max_players >= 2
            && receipts.remote_max_speed > 5.
            && receipts.remote_attack
            && receipts.remote_crash
            && receipts.remote_foot
            && receipts.remote_remount
            && receipts.errors.is_empty();
        exit.write(if passed {
            AppExit::Success
        } else {
            AppExit::error()
        });
    }
}

/// Actual rendered world/props fixture; remote peers can be native or headless nodes.
pub fn run_native(host: bool, endpoint: String, output: String) {
    let mut app = crate::build_app(false);
    app.insert_resource(crate::game::Persistence(false))
        .insert_resource(Fixture {
            host,
            start: Instant::now(),
            output,
            crashed: false,
            native: true,
            session_started: false,
            announced: false,
            start_requested: false,
        })
        .init_resource::<Receipts>()
        .add_systems(
            PreUpdate,
            script
                .after(crate::controls::capture)
                .after(crate::frontend::FrontendSet::Input)
                .before(super::pump),
        )
        .add_systems(
            FixedUpdate,
            crash
                .after(crate::game::drive)
                .before(crate::runtime::SimSet::AfterPhysics),
        )
        .add_systems(Last, observe);
    let map = if app
        .world()
        .resource::<crate::track::Track>()
        .metropolis
        .is_some()
    {
        OnlineMap::Metropolis
    } else if app
        .world()
        .resource::<crate::track::Track>()
        .city_surface
        .is_some()
    {
        OnlineMap::City
    } else {
        OnlineMap::Valley
    };
    let mut online = app.world_mut().resource_mut::<Online>();
    if std::env::var_os("ROADDASH_ONLINE_BOOT").is_none() {
        online.endpoint = endpoint.clone();
        if host {
            online
                .actions
                .push_back(OnlineAction::Host { public: false, map });
        } else {
            online
                .actions
                .push_back(OnlineAction::Join { code: endpoint });
        }
    }
    if let AppExit::Error(code) = app.run() {
        std::process::exit(i32::from(code.get()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn host_finish_does_not_stop_other_humans_or_award_local_save() {
        let mut host = crate::game::Rider::new(0);
        host.networked = true;
        host.remote_human = true;
        host.gates.completed_laps = 1;
        let mut race = crate::game::Race {
            phase: crate::game::Phase::Racing,
            elapsed: 100.,
            ..default()
        };
        let mut profile = crate::game::Profile::default();
        assert!(crate::game::finish_rider(
            &mut host,
            &mut race,
            &mut profile,
            false,
            1,
            false
        ));
        assert_eq!(host.finished, Some(100.));
        assert_eq!(race.phase, crate::game::Phase::Racing);
        assert_eq!((profile.races, profile.credits), (0, 0));
        race.arrested = true;
        let mut guest = crate::game::Rider::new(1);
        guest.networked = true;
        guest.gates.completed_laps = 1;
        assert!(crate::game::finish_rider(
            &mut guest,
            &mut race,
            &mut profile,
            false,
            2,
            false
        ));
        assert_eq!(race.phase, crate::game::Phase::Racing);
        let mut offline = crate::game::Rider::new(0);
        offline.gates.completed_laps = 1;
        assert!(!crate::game::finish_rider(
            &mut offline,
            &mut race,
            &mut profile,
            false,
            1,
            false
        ));
    }
}
