//! Host-owned physics snapshots. Clients never adjudicate race state.
use super::*;
use crate::{
    game::{Race, Rider},
    recovery::{BodyKind, Recovery, RecoveryStage},
    runtime::PoseHistory,
};
use bevy_rapier3d::prelude::*;
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Pose {
    p: [f32; 3],
    q: [f32; 4],
}
impl Pose {
    pub(super) fn valid(&self) -> bool {
        self.p.iter().all(|v| v.is_finite() && v.abs() < 1_000_000.)
            && self.q.iter().all(|v| v.is_finite())
            && (0.95..1.05).contains(&self.q.iter().map(|v| v * v).sum::<f32>())
    }
    pub(super) fn from(t: &Transform) -> Self {
        Self {
            p: t.translation.to_array(),
            q: t.rotation.to_array(),
        }
    }
    pub(super) fn transform(&self) -> Transform {
        Transform::from_translation(Vec3::from_array(self.p))
            .with_rotation(Quat::from_array(self.q).normalize())
    }
}
#[derive(Serialize, Deserialize)]
struct BikeView {
    lean: f32,
    steer: f32,
    slip: f32,
    slip_angle: f32,
    grounded: [bool; 2],
    suspension: [f32; 2],
    rpm: f32,
    gear: usize,
    spin: f32,
}
impl BikeView {
    fn from(b: &crate::vehicle::BikeState) -> Self {
        Self {
            lean: b.lean,
            steer: b.raw_steer,
            slip: b.slip,
            slip_angle: b.slip_angle,
            grounded: b.grounded,
            suspension: b.suspension,
            rpm: b.rpm,
            gear: b.gear,
            spin: b.wheel_spin,
        }
    }
    fn valid(&self) -> bool {
        self.gear <= 12
            && [
                self.lean,
                self.steer,
                self.slip,
                self.slip_angle,
                self.rpm,
                self.spin,
                self.suspension[0],
                self.suspension[1],
            ]
            .iter()
            .all(|v| v.is_finite() && v.abs() < 1_000_000.)
    }
}
#[derive(Serialize, Deserialize)]
struct Racer {
    slot: usize,
    bike: Option<BikeView>,
    weapon: Option<rr_core::weapons::Weapon>,
    weapon_hand: usize,
    strike: Option<(f32, f32, f32, rr_core::AttackSide)>,
    pose: Pose,
    progress: f32,
    lane: f32,
    route: usize,
    speed: f32,
    health: f32,
    lean: f32,
    attack: f32,
    attack_side: f32,
    kicking: bool,
    serial: u32,
    takedowns: u32,
    busted: bool,
    finished: Option<f32>,
    on_foot: bool,
    stage: u8,
    elapsed: f32,
    walk_distance: f32,
    walk_velocity: [f32; 3],
    wreck: Option<Pose>,
    runner: Option<Pose>,
    parts: Vec<Pose>,
}
#[derive(Serialize, Deserialize)]
pub(super) struct Snapshot {
    pub(super) tick: u64,
    pub(super) elapsed: f32,
    racers: Vec<Racer>,
    traffic: Vec<(u32, Pose)>,
    world: super::world_state::State,
    sounds: Vec<super::audio::Event>,
}
// Fired clocks remain attached and advance until the next strike or crash.
fn valid_strike(age: f32, contact: f32, side: f32, elapsed: f32) -> bool {
    age.is_finite()
        && (0.0..=elapsed + 1.).contains(&age)
        && contact.is_finite()
        && (0.0..=10.).contains(&contact)
        && side.is_finite()
        && side.abs() <= 1.
}
impl Snapshot {
    pub(super) fn fixture_rider(&self, slot: usize) -> Option<(u8, f32, u32)> {
        self.racers
            .iter()
            .find(|r| r.slot == slot)
            .map(|r| (r.stage, r.speed, r.serial))
    }
    pub(super) fn valid(&self) -> bool {
        let mut slots = std::collections::BTreeSet::new();
        self.elapsed.is_finite()
            && (0.0..100_000.).contains(&self.elapsed)
            && self.racers.len() <= 16
            && self.traffic.len() <= 1024
            && self.world.valid()
            && self.sounds.len() <= 64
            && self.sounds.iter().all(|s| {
                s.position
                    .iter()
                    .all(|v| v.is_finite() && v.abs() < 1_000_000.)
            })
            && self.traffic.iter().all(|(_, p)| p.valid())
            && self.racers.iter().all(|r| {
                r.slot <= 15
                    && slots.insert(r.slot)
                    && r.bike.as_ref().is_none_or(BikeView::valid)
                    && r.weapon_hand <= 1
                    && r.strike.is_none_or(|(age, contact, side, _)| {
                        valid_strike(age, contact, side, self.elapsed)
                    })
                    && (!r.busted || r.finished.is_some())
                    && r.route <= 1
                    && r.stage <= 4
                    && r.pose.valid()
                    && r.parts.len() <= 16
                    && r.parts.iter().all(Pose::valid)
                    && r.wreck.as_ref().is_none_or(Pose::valid)
                    && r.runner.as_ref().is_none_or(Pose::valid)
                    && [
                        r.progress,
                        r.lane,
                        r.speed,
                        r.health,
                        r.lean,
                        r.attack,
                        r.attack_side,
                        r.elapsed,
                        r.walk_distance,
                    ]
                    .iter()
                    .all(|v| v.is_finite() && v.abs() < 1_000_000.)
                    && r.walk_velocity
                        .iter()
                        .all(|v| v.is_finite() && v.abs() < 1000.)
                    && r.finished
                        .is_none_or(|v| v.is_finite() && (0.0..100_000.).contains(&v))
            })
    }
}
fn stage(value: RecoveryStage) -> u8 {
    match value {
        RecoveryStage::Riding => 0,
        RecoveryStage::Falling => 1,
        RecoveryStage::GettingUp => 2,
        RecoveryStage::Running => 3,
        RecoveryStage::Mounting => 4,
    }
}
fn decode_stage(value: u8) -> RecoveryStage {
    match value {
        1 => RecoveryStage::Falling,
        2 => RecoveryStage::GettingUp,
        3 => RecoveryStage::Running,
        4 => RecoveryStage::Mounting,
        _ => RecoveryStage::Riding,
    }
}
fn capture(world: &mut World, tick: u64) -> Snapshot {
    let elapsed = world.resource::<Race>().elapsed;
    let viewers = super::world_state::viewers(world);
    let racers = world
        .query::<(Entity, &Rider, &Recovery, &Transform)>()
        .iter(world)
        .map(|(entity, r, rec, t)| {
            let pose = |e: Entity| world.get::<Transform>(e).map(Pose::from);
            Racer {
                slot: r.id,
                bike: world
                    .get::<crate::vehicle::BikeState>(entity)
                    .map(BikeView::from),
                weapon: r.weapon,
                weapon_hand: r.weapon_hand,
                strike: r.strike.map(|s| (s.age, s.contact, s.side, s.action)),
                pose: Pose::from(t),
                progress: r.progress,
                lane: r.lane,
                route: r.route,
                speed: r.speed,
                health: r.health,
                lean: r.lean,
                attack: r.attack,
                attack_side: r.attack_side,
                kicking: r.kicking,
                serial: r.attack_serial,
                takedowns: r.online_takedowns,
                busted: r.online_busted,
                finished: r.finished,
                on_foot: r.finished_on_foot,
                stage: stage(rec.stage),
                elapsed: rec.elapsed,
                walk_distance: rec.walk_distance,
                walk_velocity: rec.walk_velocity.to_array(),
                wreck: rec.wreck.and_then(pose),
                runner: rec.runner.and_then(pose),
                parts: rec.parts.iter().filter_map(|e| pose(*e)).collect(),
            }
        })
        .collect();
    let traffic = world
        .query_filtered::<(&super::TrafficId, &Transform), With<crate::game::Traffic>>()
        .iter(world)
        .filter(|(_, t)| super::world_state::interested(t.translation, &viewers, 750.))
        .map(|(id, t)| (id.0, Pose::from(t)))
        .collect();
    Snapshot {
        tick,
        elapsed,
        racers,
        traffic,
        world: super::world_state::capture(world, &viewers),
        sounds: world
            .get_resource::<super::audio::Events>()
            .map_or(vec![], |e| e.pending.iter().cloned().collect()),
    }
}
pub(super) fn publish(world: &mut World) {
    let Some(mut state) = world.remove_resource::<NetState>() else {
        return;
    };
    if let Connection::Host(host) = &mut state.connection {
        if host.started {
            host.tick += 1;
            if host.tick.is_multiple_of(3) {
                let mut snapshot = capture(world, host.tick);
                // Race.elapsed becomes the host's result display after everyone finishes.
                // The replicated session clock must not rewind with that presentation value.
                host.elapsed = host.elapsed.max(snapshot.elapsed);
                snapshot.elapsed = host.elapsed;
                super::verification::receipt(world, &snapshot);
                let all_finished = snapshot
                    .racers
                    .iter()
                    .filter(|r| r.slot == 0 || host.guests.iter().any(|g| g.slot == Some(r.slot)))
                    .all(|r| r.finished.is_some());
                if all_finished {
                    let finished_at = snapshot
                        .racers
                        .iter()
                        .find(|r| r.slot == 0)
                        .and_then(|r| r.finished);
                    let mut race = world.resource_mut::<Race>();
                    race.phase = crate::game::Phase::Finished;
                    if let Some(at) = finished_at {
                        race.elapsed = at;
                    }
                    world.resource_mut::<Online>().phase = OnlinePhase::Results;
                }
                let message = ServerMessage::Snapshot(snapshot);
                match wire::encode(&message, true) {
                    Ok(frame) => {
                        super::verification::wire_receipt(world, frame.len());
                        host.guests
                            .retain_mut(|g| g.peer.send_frame(frame.clone()).is_ok());
                    }
                    Err(error) => {
                        world.resource_mut::<Online>().error = Some(error.to_string());
                        host.guests.clear();
                    }
                }
                world
                    .resource_mut::<AuthorityInputs>()
                    .0
                    .retain(|slot, _| host.guests.iter().any(|g| g.slot == Some(*slot)));
            }
        }
        for input in world.resource_mut::<AuthorityInputs>().0.values_mut() {
            input.clear_edges();
        }
    }
    world.insert_resource(state);
}
#[derive(Component)]
pub struct NetworkPose {
    from: Transform,
    to: Transform,
    received: Instant,
}
impl NetworkPose {
    pub(crate) fn shown(&self) -> Transform {
        let alpha = (self.received.elapsed().as_secs_f32() / 0.05).clamp(0., 1.);
        Transform {
            translation: self.from.translation.lerp(self.to.translation, alpha),
            rotation: self.from.rotation.slerp(self.to.rotation, alpha),
            scale: self.to.scale,
        }
    }
}
pub(super) fn put_pose(world: &mut World, entity: Entity, pose: &Pose) {
    let next = pose.transform();
    let old = world.get::<NetworkPose>(entity).map_or_else(
        || world.get::<Transform>(entity).copied().unwrap_or(next),
        NetworkPose::shown,
    );
    world.entity_mut(entity).insert((
        next,
        PoseHistory {
            previous: next,
            current: next,
        },
        NetworkPose {
            from: old,
            to: next,
            received: Instant::now(),
        },
        RigidBodyDisabled,
    ));
}
pub(super) fn receive(world: &mut World) {
    let Some(mut state) = world.remove_resource::<NetState>() else {
        return;
    };
    if state.launch {
        // game::input has performed the normal reset in Update. Shuffle only the
        // grid assignment; never mutate physics tuning or the accepted rider rig.
        if world.resource::<Race>().phase == crate::game::Phase::Countdown {
            if let Connection::Host(host) = &state.connection {
                let grid = rr_core::online::shuffled_grid(host.seed);

                let entities: Vec<_> = world
                    .query::<(Entity, &Rider)>()
                    .iter(world)
                    .filter(|(_, r)| r.id < RACERS)
                    .map(|(e, r)| (e, r.id))
                    .collect();
                for (entity, id) in entities {
                    let track = world.resource::<crate::track::Track>();
                    let spawn = &track.definition.spawns[grid[id]];
                    let progress = track.grid_station(spawn.distance);
                    let lane = spawn.lane;
                    let route = world.get::<Rider>(entity).unwrap().route;
                    let (p, q) = track.sample_route(route, progress, lane);
                    let human = id == 0 || world.resource::<AuthorityInputs>().0.contains_key(&id);
                    let pose = Transform::from_translation(p).with_rotation(q);
                    let mut r = world.get_mut::<Rider>(entity).unwrap();
                    r.progress = progress;
                    r.lane = lane;
                    r.networked = true;
                    r.remote_human = human;
                    world.entity_mut(entity).insert((
                        pose,
                        PoseHistory {
                            previous: pose,
                            current: pose,
                        },
                    ));
                    if let Some(mut bike) = world.get_mut::<crate::vehicle::BikeState>(entity) {
                        bike.position = p;
                    }
                    let mut query = world.query::<&mut RapierRigidBodySet>();
                    for mut set in query.iter_mut(world) {
                        if let Some(handle) = set.entity2body().get(&entity).copied()
                            && let Some(body) = set.bodies.get_mut(handle)
                        {
                            let iso = bevy_rapier3d::rapier::math::Pose3::from_parts(
                                p.to_array().into(),
                                bevy_rapier3d::rapier::math::Rot3::from_xyzw(q.x, q.y, q.z, q.w),
                            );
                            body.set_position(iso, true);
                            body.set_next_kinematic_position(iso);
                        }
                    }
                }
            }
            world.resource_mut::<Race>().countdown = 0.;
            state.launch = false;
        }
    }
    if let Some(snapshot) = state.pending.take() {
        super::verification::receipt(world, &snapshot);
        for event in &snapshot.sounds {
            if event.id > state.last_audio {
                world
                    .resource_mut::<Messages<crate::sound::AudioCue>>()
                    .write(crate::sound::AudioCue::new(
                        event.cue,
                        Vec3::from_array(event.position),
                    ));
                state.last_audio = event.id;
            }
        }
        let local = state.local_slot;
        let place = snapshot
            .racers
            .iter()
            .find(|r| r.slot == local)
            .map_or(15, |me| {
                1 + snapshot
                    .racers
                    .iter()
                    .filter(|r| {
                        r.slot < RACERS
                            && r.slot != local
                            && (me.busted
                                || (!r.busted
                                    && match (r.finished, me.finished) {
                                        (Some(a), Some(b)) => a < b,
                                        (Some(_), None) => true,
                                        (None, Some(_)) => false,
                                        _ => r.progress > me.progress,
                                    }))
                    })
                    .count()
            });
        let entities: BTreeMap<_, _> = world
            .query::<(Entity, &Rider)>()
            .iter(world)
            .map(|(e, r)| (r.id, e))
            .collect();
        for racer in snapshot.racers {
            let Some(&entity) = entities.get(&racer.slot) else {
                continue;
            };
            let mut rec = world
                .entity_mut(entity)
                .take::<Recovery>()
                .unwrap_or_default();
            let mut queue = bevy::ecs::world::CommandQueue::default();
            let palette = world.get::<Rider>(entity).unwrap().palette_index;
            {
                let art = world.resource::<crate::world::Art>();
                let mut commands = Commands::new(&mut queue, world);
                if racer.wreck.is_some() && rec.wreck.is_none() {
                    let e = crate::world::bike(&mut commands, art, palette, false);
                    commands
                        .entity(e)
                        .insert(crate::game::Debris(f32::INFINITY))
                        .insert(crate::recovery::OwnedBody {
                            owner: entity,
                            kind: BodyKind::Wreck,
                        });
                    rec.wreck = Some(e);
                }
                if racer.runner.is_some() && rec.runner.is_none() {
                    let (e, pose) = crate::world::runner(&mut commands, art, palette);
                    commands
                        .entity(e)
                        .insert(crate::game::Debris(f32::INFINITY))
                        .insert(crate::recovery::OwnedBody {
                            owner: entity,
                            kind: BodyKind::Runner,
                        });
                    rec.runner = Some(e);
                    rec.pose = Some(pose);
                }
                if !racer.parts.is_empty() && rec.parts.is_empty() {
                    rec.parts = crate::recovery::humanoid(
                        &mut commands,
                        art,
                        entity,
                        palette,
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        Vec3::ZERO,
                    );
                    rec.torso = rec.parts.first().copied();
                }
            }
            queue.apply(world);
            world.entity_mut(entity).insert(rec);
            crate::biomechanics::initialize(world);
            rec = world.entity_mut(entity).take::<Recovery>().unwrap();
            for (current, incoming) in [
                (&mut rec.wreck, &racer.wreck),
                (&mut rec.runner, &racer.runner),
            ] {
                if let Some(pose) = incoming {
                    if let Some(e) = *current {
                        put_pose(world, e, pose);
                    }
                } else if let Some(e) = current.take() {
                    world.despawn(e);
                }
            }
            if racer.parts.is_empty() {
                for e in rec.parts.drain(..) {
                    world.despawn(e);
                }
                rec.torso = None;
            } else {
                for (e, pose) in rec.parts.iter().zip(&racer.parts) {
                    put_pose(world, *e, pose);
                }
            }
            rec.stage = decode_stage(racer.stage);
            rec.elapsed = racer.elapsed;
            rec.walk_distance = racer.walk_distance;
            rec.walk_velocity = Vec3::from_array(racer.walk_velocity);
            let riding = rec.stage == RecoveryStage::Riding;
            world.entity_mut(entity).insert((
                rec,
                if riding {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
            ));
            put_pose(world, entity, &racer.pose);
            if let Some(view) = racer.bike
                && let Some(mut b) = world.get_mut::<crate::vehicle::BikeState>(entity)
            {
                b.lean = view.lean;
                b.raw_steer = view.steer;
                b.slip = view.slip;
                b.slip_angle = view.slip_angle;
                b.grounded = view.grounded;
                b.suspension = view.suspension;
                b.rpm = view.rpm;
                b.gear = view.gear;
                b.wheel_spin = view.spin;
            }
            let strike = racer.strike.map(|(age, contact, side, action)| {
                let mut s = rr_core::motion::StrikeClock::new(action, side, 0.);
                s.age = age;
                s.contact = contact;
                s.weapon = racer
                    .weapon
                    .map(|w| world.resource::<crate::weapons::WeaponRules>().get(w));
                s
            });
            let mut rider = world.get_mut::<Rider>(entity).unwrap();
            rider.weapon = racer.weapon;
            rider.online_busted = racer.busted;
            rider.weapon_hand = racer.weapon_hand;
            rider.strike = strike;

            rider.progress = racer.progress;
            rider.lane = racer.lane;
            rider.route = racer.route;
            rider.speed = racer.speed;
            rider.health = racer.health;
            rider.lean = racer.lean;
            rider.attack = racer.attack;
            rider.attack_side = racer.attack_side;
            rider.kicking = racer.kicking;
            rider.attack_serial = racer.serial;
            rider.finished = racer.finished;
            rider.finished_on_foot = racer.on_foot;
            rider.crashed = if riding { 0. } else { 1. };
            rider.networked = true;
            if racer.slot == local {
                let mut race = world.resource_mut::<Race>();
                race.phase = if racer.finished.is_some() {
                    crate::game::Phase::Finished
                } else {
                    crate::game::Phase::Racing
                };
                race.elapsed = racer.finished.unwrap_or(snapshot.elapsed);
                race.earned = 0;
                race.arrested = racer.busted;
                race.place = place;
                race.takedowns = racer.takedowns;
                world.resource_mut::<Online>().phase = if racer.finished.is_some() {
                    OnlinePhase::Results
                } else {
                    OnlinePhase::Racing
                };
            }
        }
        let traffic: BTreeMap<_, _> = world
            .query_filtered::<(Entity, &super::TrafficId), With<crate::game::Traffic>>()
            .iter(world)
            .map(|(e, id)| (id.0, e))
            .collect();
        let present: std::collections::BTreeSet<_> =
            snapshot.traffic.iter().map(|(id, _)| *id).collect();
        for (id, &entity) in &traffic {
            world.entity_mut(entity).insert(if present.contains(id) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            });
        }
        for (id, pose) in &snapshot.traffic {
            if let Some(e) = traffic.get(id) {
                put_pose(world, *e, pose);
            }
        }
        super::world_state::apply(world, snapshot.world);
        for mut config in world.query::<&mut RapierConfiguration>().iter_mut(world) {
            config.physics_pipeline_active = false;
        }
    }
    world.insert_resource(state);
}

pub(super) fn clear_remote(world: &mut World) {
    for mut rider in world.query::<&mut Rider>().iter_mut(world) {
        rider.networked = false;
        rider.remote_human = false;
    }
    let entities: Vec<_> = world
        .query_filtered::<(Entity, Option<&crate::recovery::OwnedBody>), Or<(With<NetworkPose>,With<crate::recovery::OwnedBody>)>>()
        .iter(world)
        .map(|(e, o)| (e, o.is_some()))
        .collect();
    for (e, owned) in entities {
        if owned {
            world.despawn(e);
        } else {
            world
                .entity_mut(e)
                .remove::<(NetworkPose, RigidBodyDisabled)>();
        }
    }
    for mut configuration in world.query::<&mut RapierConfiguration>().iter_mut(world) {
        configuration.physics_pipeline_active = true;
    }
    let npcs: Vec<_> = world
        .query::<(Entity, &mut crate::npc::Npc, &mut Transform)>()
        .iter_mut(world)
        .map(|(entity, mut npc, mut transform)| {
            npc.parts.clear();
            npc.elapsed = 0.;
            npc.fresh = false;
            npc.hits = default();
            *transform = npc.spawn;
            entity
        })
        .collect();
    for entity in npcs {
        world
            .entity_mut(entity)
            .remove::<(ColliderDisabled, RigidBodyDisabled)>();
    }
    for mut visible in world
        .query_filtered::<&mut Visibility, Or<(
            With<Rider>,
            With<crate::npc::Npc>,
            With<crate::game::Traffic>,
            With<crate::environment::CrossTraffic>,
        )>>()
        .iter_mut(world)
    {
        *visible = Visibility::Visible;
    }
    for mut rec in world.query::<&mut Recovery>().iter_mut(world) {
        *rec = Recovery::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recovery::OwnedBody;
    fn snapshot() -> Snapshot {
        Snapshot {
            tick: 1,
            elapsed: 0.,
            racers: vec![],
            traffic: vec![],
            world: default(),
            sounds: vec![],
        }
    }
    #[test]
    fn completed_session_keeps_monotonic_clock_and_aged_strikes_valid() {
        let mut world = World::new();
        world.insert_resource(Race {
            phase: crate::game::Phase::Racing,
            elapsed: 60.,
            ..default()
        });
        world.insert_resource(Online::default());
        world.insert_resource(AuthorityInputs::default());
        let mut host = open_host("127.0.0.1:0", false, OnlineMap::Metropolis).unwrap();
        host.started = true;
        let stream = TcpStream::connect(host.listener.local_addr().unwrap()).unwrap();
        let (accepted, _) = host.listener.accept().unwrap();
        let mut client = Peer::new(stream).unwrap();
        host.guests.push(Guest {
            peer: Peer::new(accepted).unwrap(),
            slot: Some(1),
            name: "Guest".into(),
            seen: Instant::now(),
            rate_start: Instant::now(),
            rate: 0,
            sequence: 0,
        });
        world.insert_resource(NetState {
            connection: Connection::Host(host),
            ..default()
        });
        let mut local = Rider::new(0);
        local.networked = true;
        local.remote_human = true;
        local.finished = Some(20.);
        let mut strike = rr_core::motion::StrikeClock::new(rr_core::AttackSide::Left, 1., 0.2);
        strike.advance(50.);
        local.strike = Some(strike);
        world.spawn((local, Recovery::default(), Transform::default()));
        let mut remote = Rider::new(1);
        remote.networked = true;
        remote.remote_human = true;
        let remote = world
            .spawn((remote, Recovery::default(), Transform::default()))
            .id();
        let mut elapsed = Vec::new();
        for step in 0..3 {
            if step == 1 {
                world.get_mut::<Rider>(remote).unwrap().finished = Some(60.);
            }
            for _ in 0..3 {
                publish(&mut world);
            }
            if step == 0 {
                assert_eq!(world.resource::<Race>().phase, crate::game::Phase::Racing);
            } else {
                assert_eq!(world.resource::<Race>().elapsed, 20.);
            }
            let deadline = Instant::now() + Duration::from_secs(1);
            loop {
                let messages = client.receive::<ServerMessage>(wire::FRAME_LIMIT).unwrap();
                if !messages.is_empty() {
                    for message in messages {
                        let ServerMessage::Snapshot(snapshot) = message else {
                            panic!("expected snapshot")
                        };
                        assert!(
                            snapshot.valid(),
                            "retained fired strike must remain valid after result clock freezes"
                        );
                        elapsed.push(snapshot.elapsed);
                    }
                    break;
                }
                assert!(Instant::now() < deadline);
                std::thread::yield_now();
            }
        }
        assert_eq!(elapsed, vec![60., 60., 60.]);
    }
    #[test]
    fn leave_restores_downed_npc_pose_and_solid_collision() {
        let mut world = World::new();
        let spawn = Transform::from_xyz(3., 1., 5.);
        let owner = world.spawn_empty().id();
        let part = world
            .spawn((
                Transform::default(),
                OwnedBody {
                    owner,
                    kind: BodyKind::Ragdoll,
                },
            ))
            .id();
        let mut npc = crate::npc::Npc::new(crate::npc::NpcKind::Pedestrian, spawn);
        npc.parts.push(part);
        npc.elapsed = 4.;
        world.entity_mut(owner).insert((
            npc,
            Transform::from_xyz(10., 0., 20.),
            Visibility::Hidden,
            ColliderDisabled,
            RigidBodyDisabled,
        ));
        clear_remote(&mut world);
        assert!(world.get_entity(part).is_err());
        assert!(world.get::<ColliderDisabled>(owner).is_none());
        assert!(world.get::<RigidBodyDisabled>(owner).is_none());
        assert_eq!(
            world.get::<Transform>(owner).unwrap().translation,
            spawn.translation
        );
        assert!(world.get::<crate::npc::Npc>(owner).unwrap().active());
        assert_eq!(
            *world.get::<Visibility>(owner).unwrap(),
            Visibility::Visible
        );
    }
    #[test]
    fn rejects_nonfinite_zero_rotation_and_impossible_arrays() {
        let mut value = snapshot();
        assert!(value.valid());
        value.elapsed = f32::NAN;
        assert!(!value.valid());
        value.elapsed = 0.;
        value.traffic.push((
            0,
            Pose {
                p: [0.; 3],
                q: [0.; 4],
            },
        ));
        assert!(!value.valid());
        value.traffic[0].1.q = [0., 0., 0., 1.];
        assert!(value.valid());
        value.traffic[0].1.p[0] = f32::INFINITY;
        assert!(!value.valid());
    }
    #[test]
    fn visual_interpolation_advances_once_without_changing_authority() {
        let pose = NetworkPose {
            from: Transform::IDENTITY,
            to: Transform::from_xyz(10., 0., 0.),
            received: Instant::now() - Duration::from_millis(25),
        };
        let shown = pose.shown().translation.x;
        assert!((4.9..5.2).contains(&shown));
        assert_eq!(pose.to.translation.x, 10.);
        let done = NetworkPose {
            received: Instant::now() - Duration::from_secs(1),
            ..pose
        };
        assert_eq!(done.shown().translation.x, 10.);
    }
    #[test]
    fn leave_restores_authoritative_physics_and_offline_identity() {
        let mut world = World::new();
        let mut r = Rider::new(1);
        r.networked = true;
        r.remote_human = true;
        let rider = world
            .spawn((
                r,
                Recovery::default(),
                RigidBodyDisabled,
                NetworkPose {
                    from: Transform::IDENTITY,
                    to: Transform::IDENTITY,
                    received: Instant::now(),
                },
            ))
            .id();
        clear_remote(&mut world);
        assert!(!world.get::<Rider>(rider).unwrap().networked);
        assert!(world.get::<RigidBodyDisabled>(rider).is_none());
        assert!(world.get::<NetworkPose>(rider).is_none());
    }
}

#[cfg(test)]
mod retained_clock_tests {
    #[test]
    fn long_lived_fired_strikes_remain_valid() {
        assert!(super::valid_strike(25., 0.2, -1., 30.));
        assert!(super::valid_strike(120., 0.2, 1., 120.));
        assert!(!super::valid_strike(122., 0.2, 1., 120.));
        assert!(!super::valid_strike(-1., 0.2, 1., 120.));
        assert!(!super::valid_strike(1., f32::NAN, 1., 120.));
    }
}
