use crate::{
    recovery::{self, Recovery, RecoveryStage},
    sound::{AudioCue, Cue},
};
use crate::{
    track::{LAPS, Track},
    world::{self, Art},
};
use bevy::{camera::Exposure, prelude::*};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub use rr_core::cast::NAMES;

use crate::{
    content::{ContentIdentity, GameContent},
    controls::Controls,
};
pub use rr_core::Phase;

#[derive(Resource)]
pub struct Race {
    pub arrested: bool,
    pub phase: Phase,
    pub countdown: f32,
    pub elapsed: f32,
    pub message: String,
    pub message_time: f32,
    pub takedowns: u32,
    pub place: usize,
    pub camera_mode: usize,
    pub shake: f32,
    pub earned: u32,
}
impl Default for Race {
    fn default() -> Self {
        Self {
            arrested: false,
            phase: Phase::Garage,
            countdown: 3.0,
            elapsed: 0.0,
            message: String::new(),
            message_time: 0.0,
            takedowns: 0,
            place: 15,
            camera_mode: 0,
            shake: 0.0,
            earned: 0,
        }
    }
}

#[derive(Resource)]
pub struct Persistence(pub bool);
impl Default for Persistence {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct Profile {
    #[serde(default = "current_save_schema")]
    pub schema_version: u32,
    #[serde(skip, default = "save_allowed")]
    writable: bool,
    pub races: u32,
    pub wins: u32,
    pub credits: u32,
    pub best_seconds: Option<f32>,
}
fn current_save_schema() -> u32 {
    rr_core::SAVE_SCHEMA_VERSION
}
fn save_allowed() -> bool {
    true
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            schema_version: current_save_schema(),
            writable: true,
            races: 0,
            wins: 0,
            credits: 0,
            best_seconds: None,
        }
    }
}
impl Profile {
    pub fn load() -> Self {
        Self::load_at(&save_path())
    }
    fn load_at(path: &std::path::Path) -> Self {
        match std::fs::read(path) {
            Ok(bytes) => match Self::decode(&bytes) {
                Ok(profile) => profile,
                Err(e) => {
                    warn!("Save preserved without overwrite: {e}");
                    Self {
                        writable: false,
                        ..default()
                    }
                }
            },
            Err(_) => Self::default(),
        }
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let profile: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if profile.schema_version != current_save_schema() {
            return Err(format!(
                "unsupported save schema {}",
                profile.schema_version
            ));
        }
        if profile
            .best_seconds
            .is_some_and(|t| !t.is_finite() || t < 0.0)
        {
            return Err("invalid best time".into());
        }
        Ok(profile)
    }
    fn save(&self) {
        self.save_at(&save_path());
    }
    fn save_at(&self, path: &std::path::Path) {
        if !self.writable {
            warn!(
                "Not overwriting an unreadable/unsupported profile; restore or move it before saving new progress"
            );
            return;
        }
        let temporary = path.with_extension("tmp");
        let result = (|| -> std::io::Result<()> {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&temporary, serde_json::to_vec_pretty(self)?)?;
            std::fs::rename(temporary, path)
        })();
        if let Err(e) = result {
            warn!("Unable to save profile: {e}");
        }
    }
}
fn save_path() -> std::path::PathBuf {
    if let Some(dir) = std::env::var_os("ROADDASH_SAVE_DIR") {
        return std::path::PathBuf::from(dir).join("profile.json");
    }
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("XDG_DATA_HOME"))
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/share"))
        })
        .unwrap_or_else(std::env::temp_dir);
    base.join("roaddash/profile.json")
}

#[derive(Component)]
#[require(Recovery, crate::animation::Animator)]
pub struct Rider {
    pub online_busted: bool,
    pub online_takedowns: u32,
    pub networked: bool,
    pub remote_human: bool,
    pub police: bool,
    pub id: usize,
    pub route: usize,
    pub progress: f32,
    pub lane: f32,
    pub speed: f32,
    pub health: f32,
    pub original_condition: rr_core::original_crash::Condition,
    pub nitro: f32,
    pub crashed: f32,
    pub immunity: f32,
    pub attack: f32,
    pub attack_side: f32,
    pub kicking: bool,
    pub lean: f32,
    pub finished: Option<f32>,
    pub finished_on_foot: bool,
    pub hop_height: f32,
    pub hop_velocity: f32,
    pub impact_cooldown: f32,
    pub motion_velocity: Vec3,
    pub motion_angular_velocity: Vec3,
    pub gates: crate::track::GateProgress,
    pub hits: rr_core::HitChain,
    pub side_velocity: f32,
    pub drive_intent: rr_core::VehicleIntent,
    pub bike_mass: f32,
    pub palette_index: usize,
    pub steering: rr_core::steering::SteeringMemory,
    pub raw_steer: f32,
    pub ai_profile: rr_core::pursuit::Profile,
    pub relationships: [f32; 10],
    pub combat_mind: rr_core::rival_ai::Mind,
    pub ai_drive: rr_core::cast::DriveMemory,
    pub ai_pass: rr_core::traffic_avoidance::Memory,
    pub ai_plan: Option<rr_core::traffic_avoidance::Plan>,
    pub ai_static_guard_until: f32,
    pub(crate) ai_trace_next: Option<f32>,
    pub ai_escape: rr_core::escape::Memory,
    pub fight_stamina: f32,
    pub attack_serial: u32,
    pub(crate) weapon_hand: usize,
    pub(crate) weapon: Option<rr_core::weapons::Weapon>,
    pub(crate) strike: Option<rr_core::motion::StrikeClock>,
    pub(crate) hit_reaction: Option<rr_core::motion::HitReaction>,
}
impl Rider {
    pub(crate) fn can_fight(&self, target: usize) -> bool {
        if self.police {
            target == 0
        } else {
            self.combat_mind.can_fight(target)
        }
    }

    pub fn human_controlled(&self) -> bool {
        if self.networked {
            self.remote_human
        } else {
            self.id == 0
        }
    }
    pub(crate) fn new(id: usize) -> Self {
        Self {
            online_busted: false,
            online_takedowns: 0,
            networked: false,
            remote_human: false,
            police: id == crate::police::OFFICER_ID,
            id,
            route: id % 2,
            progress: -8.0 - (id / 2) as f32 * 4.0,
            lane: if id.is_multiple_of(2) { -2.0 } else { 2.0 },
            speed: 0.0,
            health: 100.0,
            original_condition: crate::original_crash::initial_condition(),
            nitro: 100.0,
            crashed: 0.0,
            immunity: 2.0,
            attack: 0.0,
            attack_side: 0.0,
            kicking: false,
            lean: 0.0,
            finished: None,
            finished_on_foot: false,
            hop_height: 0.0,
            hop_velocity: 0.0,
            impact_cooldown: 0.0,
            motion_velocity: Vec3::ZERO,
            motion_angular_velocity: Vec3::ZERO,
            gates: default(),
            hits: default(),
            side_velocity: 0.0,
            drive_intent: default(),
            bike_mass: 170.0,
            palette_index: id,
            steering: default(),
            raw_steer: 0.0,
            ai_profile: rr_content::rivals::profile(id),
            relationships: rr_content::rivals::profile(id).relationships,
            fight_stamina: rr_content::rivals::profile(id).stamina,
            attack_serial: 0,
            weapon_hand: 1,
            weapon: None,
            strike: None,
            hit_reaction: None,
            combat_mind: rr_core::rival_ai::Mind::new(id),
            ai_drive: default(),
            ai_pass: default(),
            ai_plan: None,
            ai_static_guard_until: 0.,
            ai_trace_next: None,
            ai_escape: default(),
        }
    }
}
#[derive(Component)]
pub struct ChaseCamera;
#[derive(Component)]
#[require(TrafficYield)]
pub struct Traffic {
    pub route: usize,
    pub progress: f32,
    pub lane: f32,
    pub speed: f32,
}
#[derive(Component, Default)]
pub struct TrafficYield {
    pub city_only: bool,
    cruise: Option<f32>,
    lane_ratio: Option<f32>,
    honk_remaining: f32,
    last_elapsed: f32,
    lane_slot: usize,
    start_progress: Option<f32>,
    passing: rr_core::civilian::Memory,
    pass_geometry_clear: bool,
    merge_geometry_clear: bool,
    pass_geometry_remaining: f32,
    // Previous fixed-step street-relative placement, shared by all racer observers.
    lateral_sample: Option<(f32, f32)>,
    lateral_speed: f32,
    lateral_goal: Option<f32>,
}
#[derive(Component)]
pub struct Debris(pub f32);

/// Nominal home/opposing lane in the traffic sampler's own frame.
fn civilian_lanes(
    track: &Track,
    route: usize,
    station: f32,
    direction: f32,
    slot: usize,
    ratio: f32,
) -> (f32, f32, f32) {
    if let Some(m) = &track.metropolis {
        let sections = &m.street_life.lane_sections;
        (
            rr_core::streets::lane_center(sections, route, station, direction > 0., slot),
            rr_core::streets::lane_center(sections, route, station, direction < 0., 0),
            rr_core::streets::markings(sections, route, station).0,
        )
    } else {
        let home = ratio * track.half_width(station);
        (home, -home, 0.)
    }
}

const HILLS_OPENING_CARS: usize = 6;
const HILLS_BASE_CARS: usize = 24;
const HILLS_HEAVY_END: usize =
    HILLS_BASE_CARS + crate::heavy_traffic::SEMIS + crate::heavy_traffic::TRACTORS;
fn hills_car_start(index: usize) -> Option<f32> {
    match index {
        0 => Some(350.),  // follows the 450 m semi
        3 => Some(2650.), // follows the 2750 m tractor
        i if (HILLS_HEAVY_END..HILLS_HEAVY_END + HILLS_OPENING_CARS).contains(&i) => {
            Some(500. + (i - HILLS_HEAVY_END) as f32 * 70.)
        }
        _ => None,
    }
}

pub fn setup_race(
    mut commands: Commands,
    art: Res<Art>,
    track: Res<Track>,
    content: Res<GameContent>,
) {
    commands.insert_resource(Profile::load());
    for id in 0..NAMES.len() {
        let mut rider = Rider::new(id);
        rider.palette_index = content.0.riders[id].palette_index;
        if let Some(spawn) = track.definition.spawns.get(id) {
            rider.progress = track.grid_station(spawn.distance);
            rider.lane = spawn.lane;
        }
        let (p, q) = track.sample_route(rider.route, rider.progress, rider.lane);
        let root = world::bike(
            &mut commands,
            &art,
            content.0.riders[id].palette_index,
            true,
        );
        commands.entity(root).insert((
            rider,
            Transform::from_translation(p).with_rotation(q),
            recovery::bike_body(),
            ContentIdentity {
                rider: content.0.riders[id].id.clone(),
                bike: content.0.riders[id].bike_id.clone(),
            },
        ));
    }
    let car_count = if track.is_hills() {
        HILLS_HEAVY_END + HILLS_OPENING_CARS
    } else {
        track
            .metropolis
            .as_ref()
            .map_or(24, |m| m.street_life.main_traffic)
    };
    for i in 0..car_count {
        let city_only = track.metropolis.is_some() && i >= 64;
        let heavy = track
            .is_hills()
            .then(|| crate::heavy_traffic::selection(i))
            .flatten();
        let progress = if let Some((_, station)) = heavy {
            station
        } else if let Some(station) = hills_car_start(i).filter(|_| track.is_hills()) {
            station
        } else if city_only {
            3400. + (i - 64) as f32 * (track.length - 3600.) / (car_count - 64).max(1) as f32
        } else if track.metropolis.is_some() {
            700. + i as f32 * (track.length - 900.) / 64.
        } else {
            100. + i as f32 * track.length
                / if track.is_hills() {
                    24.
                } else {
                    car_count as f32
                }
        };
        let mut lane =
            (if i % 3 == 0 { 1.0 } else { -1.0 }) * (track.half_width(progress) * 0.48).min(3.1);
        let mut speed: f32 = if lane > 0.0 { 18.0 } else { -13.0 };
        if let Some((kind, _)) = heavy {
            speed = kind.cruise() * speed.signum();
        }
        let route = if track.metropolis.is_some() {
            (i / 2) % 2
        } else {
            i % 2
        };
        if let Some(m) = &track.metropolis {
            speed = if i % 2 == 0 {
                15. + (i % 5) as f32
            } else {
                -12. - (i % 4) as f32
            };
            lane = rr_core::streets::lane_center(
                &m.street_life.lane_sections,
                route,
                progress,
                speed > 0.,
                (i / 4) % 2,
            );
        }
        let (position, heading) = track.street_sample(route, progress, lane);
        if track.metropolis.is_some() {
            lane = track.lane_at(route, progress, position);
        }
        let heading = if speed < 0. {
            heading * Quat::from_rotation_y(std::f32::consts::PI)
        } else {
            heading
        };
        let root = commands
            .spawn((
                Transform::from_translation(position).with_rotation(heading),
                Visibility::default(),
                TrafficYield {
                    city_only,
                    lane_slot: (i / 4) % 2,
                    start_progress: Some(progress),
                    ..default()
                },
                crate::online::TrafficId(i as u32),
                Traffic {
                    route,
                    progress,
                    lane,
                    speed,
                },
                RigidBody::KinematicPositionBased,
                Collider::compound(vec![(
                    Vec3::new(0.0, 0.95, 0.0),
                    Quat::IDENTITY,
                    Collider::cuboid(0.95, 0.9, 1.95),
                )]),
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
            ))
            .id();
        if let Some((kind, _)) = heavy {
            crate::heavy_traffic::spawn(&mut commands, &art, root, kind, i);
            continue;
        }
        if let Some(props) = &art.props {
            let visual = props.vehicle(&mut commands, i);
            crate::runtime::attach_visuals(&mut commands, root, &[visual]);
            continue;
        }
        let mut parts = vec![];
        for (p, s, mat) in [
            (
                Vec3::new(0.0, 0.70, 0.0),
                Vec3::new(1.85, 0.65, 3.8),
                art.colors[(i + 2) % 8].clone(),
            ),
            (
                Vec3::new(0.0, 1.25, -0.18),
                Vec3::new(1.62, 0.65, 1.9),
                art.dark.clone(),
            ),
            (
                Vec3::new(0.0, 0.8, 1.91),
                Vec3::new(1.4, 0.16, 0.05),
                art.white.clone(),
            ),
        ] {
            parts.push(world::part(
                &mut commands,
                art.cube.clone(),
                mat,
                p,
                s,
                Quat::IDENTITY,
            ));
        }
        for x in [-0.94, 0.94] {
            for z in [-1.15, 1.15] {
                parts.push(world::part(
                    &mut commands,
                    art.wheel.clone(),
                    art.dark.clone(),
                    Vec3::new(x, 0.4, z),
                    Vec3::ONE,
                    Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ));
            }
        }
        crate::runtime::attach_visuals(&mut commands, root, &parts);
    }
    let spawn = &track.definition.spawns[0];
    let (p, q) = track.sample(track.grid_station(spawn.distance), spawn.lane);
    commands.spawn((
        Camera3d::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
        Exposure { ev100: 13.0 },
        ChaseCamera,
        DistanceFog {
            color: Color::srgb(0.48, 0.63, 0.66),
            falloff: FogFalloff::Linear {
                start: 130.0,
                end: 630.0,
            },
            ..default()
        },
        Transform::from_translation(p + q * Vec3::new(0.0, 4.0, -8.0))
            .looking_at(p + Vec3::Y, Vec3::Y),
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn input(
    mut controls: ResMut<Controls>,
    mut muted: ResMut<crate::sound::Muted>,
    mut race: ResMut<Race>,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &mut Recovery,
        Option<&mut crate::vehicle::BikeState>,
    )>,
    mut debris: Query<Entity, With<Debris>>,
    mut commands: Commands,
    mut menu_action: ResMut<crate::ui::MenuAction>,
    screen: Option<Res<State<crate::runtime::AppScreen>>>,
    track: Res<Track>,
    panel: Res<crate::settings::SettingsPanel>,
    mut body_sets: Query<&mut RapierRigidBodySet>,
    frontend: Option<Res<crate::frontend::Frontend>>,
    mut grid_weapons: ResMut<crate::weapons::GridWeapons>,
    (online, network): (
        Option<Res<crate::online::Online>>,
        Option<Res<crate::online::NetState>>,
    ),
) {
    if panel.open {
        menu_action.0 = false;
        return;
    }
    if screen.as_ref().is_some_and(|s| {
        matches!(
            s.get(),
            crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
        )
    }) {
        return;
    }
    let online_active = online.as_ref().is_some_and(|o| o.lobby.is_some());
    if online_active {
        controls.pause = false;
        controls.restart = false;
    }
    if crate::online::remote_client(online.as_deref()) {
        if std::mem::take(&mut controls.mute) {
            muted.0 = !muted.0;
        }
        if std::mem::take(&mut controls.camera) {
            race.camera_mode = (race.camera_mode + 1) % 3;
        }
        controls.start = false;
        menu_action.0 = false;
        return;
    }
    let activate = std::mem::take(&mut menu_action.0);
    let resume = activate && race.phase == Phase::Paused;
    if std::mem::take(&mut controls.mute) {
        muted.0 = !muted.0;
    }
    if frontend.as_ref().is_some_and(|f| f.owns(race.phase))
        && !online
            .as_ref()
            .is_some_and(|o| o.phase == crate::online::OnlinePhase::Racing)
    {
        menu_action.0 = false;
        return;
    }
    if std::mem::take(&mut controls.camera) {
        race.camera_mode = (race.camera_mode + 1) % 3;
    }
    if std::mem::take(&mut controls.pause) || resume {
        race.phase = match race.phase {
            Phase::Racing => Phase::Paused,
            Phase::Paused => Phase::Racing,
            p => p,
        };
    }
    let start = (network.as_ref().is_some_and(|n| n.launch)
        || std::mem::take(&mut controls.start)
        || (activate && !resume))
        && matches!(race.phase, Phase::Garage | Phase::Finished);
    let restart = std::mem::take(&mut controls.restart)
        && matches!(race.phase, Phase::Racing | Phase::Paused | Phase::Finished);
    if start || restart {
        grid_weapons.roll(
            riders
                .iter()
                .filter(|(_, r, _, _)| r.id != 0 && !r.police)
                .map(|(_, r, _, _)| r.id),
        );
        commands.queue(crate::destruction::request_reset);
        let mode = race.camera_mode;
        *race = Race::default();
        race.camera_mode = mode;
        race.phase = Phase::Countdown;
        controls.buffer.clear_edges();
        controls.look = Vec2::ZERO;
        controls.foot = Vec2::ZERO;
        for (entity, mut rider, mut recovery, state) in &mut riders {
            crate::crash_behavior::restore_chassis(&mut commands, entity);
            let palette = rider.palette_index;
            *rider = Rider::new(rider.id);
            if !rider.police {
                rider.weapon = rr_core::cast::character(rider.id).and_then(|c| c.weapon);
            }
            if rider.police {
                rider.combat_mind.struck_by(0);
            }
            rider.palette_index = palette;
            if let Some(spawn) = track.definition.spawns.get(rider.id) {
                rider.progress = track.grid_station(spawn.distance);
                rider.lane = spawn.lane;
            }
            if rider.police {
                rider.progress = track.length * rr_content::police::builtin().patrol_fraction;
                rider.route = 0;
                rider.lane = 2.0;
            }
            *recovery = Recovery::default();
            let (position, rotation) = track.sample_route(rider.route, rider.progress, rider.lane);
            let pose = Transform::from_translation(position).with_rotation(rotation);
            // Explicit reset/teleport must update Rapier's current AND next pose.
            // Merely replacing Bevy transforms can be overwritten by backend
            // writeback when the visible hierarchical chassis is kinematic.
            for mut set in &mut body_sets {
                if let Some(handle) = set.entity2body().get(&entity).copied()
                    && let Some(body) = set.bodies.get_mut(handle)
                {
                    let location = bevy_rapier3d::rapier::math::Pose3::from_parts(
                        position.to_array().into(),
                        bevy_rapier3d::rapier::math::Rot3::from_xyzw(
                            rotation.x, rotation.y, rotation.z, rotation.w,
                        ),
                    );
                    body.set_body_type(
                        bevy_rapier3d::rapier::dynamics::RigidBodyType::KinematicPositionBased,
                        true,
                    );
                    body.set_position(location, true);
                    body.set_next_kinematic_position(location);
                    body.set_linvel(Vec3::ZERO, true);
                    body.set_angvel(Vec3::ZERO, true);
                    body.reset_forces(true);
                    body.reset_torques(true);
                }
            }
            if let Some(mut state) = state {
                let preset = state.preset;
                *state = crate::vehicle::BikeState {
                    position,
                    preset,
                    ..default()
                };
            }
            // Reset the complete authority/presentation snapshot before the next fixed tick.
            commands
                .entity(entity)
                .insert((
                    pose,
                    GlobalTransform::from(pose),
                    Velocity::zero(),
                    ExternalForce::default(),
                    RigidBody::KinematicPositionBased,
                    Visibility::Visible,
                    crate::runtime::PoseHistory {
                        previous: pose,
                        current: pose,
                    },
                ))
                .remove::<ColliderDisabled>()
                .remove::<RigidBodyDisabled>();
        }
        for e in &mut debris {
            commands.entity(e).despawn();
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn drive(
    time: Res<Time<Fixed>>,
    mut controls: ResMut<Controls>,
    track: Res<Track>,
    art: Res<Art>,
    mut race: ResMut<Race>,
    mut riders: Query<
        (
            Entity,
            &mut Rider,
            &mut Recovery,
            &mut Transform,
            &mut Visibility,
        ),
        Without<Traffic>,
    >,
    mut traffic: Query<
        (
            Entity,
            &mut Traffic,
            &mut Transform,
            &mut TrafficYield,
            Option<&ColliderDisabled>,
        ),
        Without<Rider>,
    >,
    mut commands: Commands,
    mut profile: ResMut<Profile>,
    (
        content,
        persistence,
        adapter,
        handling,
        weapons,
        network_inputs,
        mut civilian_scratch,
        mut civilian_timings,
        strike_timings,
        work,
        mut metrics,
    ): (
        Res<GameContent>,
        Res<Persistence>,
        Res<crate::vehicle::VehiclePhysicsAdapter>,
        Res<crate::vehicle::Tuning>,
        Res<crate::weapons::WeaponRules>,
        Option<Res<crate::online::AuthorityInputs>>,
        Local<(
            Vec<rr_core::civilian::Vehicle>,
            Vec<rr_core::civilian::Vehicle>,
        )>,
        Option<ResMut<civilian_survey::PlanningTimings>>,
        Res<crate::combat::StrikeTimings>,
        Local<crate::ai_work::Work>,
        Option<ResMut<crate::diagnostics::Metrics>>,
    ),
    police: Res<crate::police::Police>,
    mut physics: Query<&mut RapierConfiguration>,
    mut audio: MessageWriter<AudioCue>,
    context: ReadRapierContext,
    (ground, owned, poses, shapes, crossing_cars, names): (
        Query<(), With<world::Ground>>,
        Query<&recovery::OwnedBody>,
        Query<&GlobalTransform>,
        Query<&Collider>,
        Query<
            (
                &Transform,
                Option<&crate::environment::Patrol>,
                Option<&Velocity>,
                Option<&crate::world_gen::metropolis::Crossing>,
                Option<&ColliderDisabled>,
            ),
            (
                With<crate::environment::CrossTraffic>,
                Without<Traffic>,
                Without<Rider>,
            ),
        >,
        Query<&Name>,
    ),
    (mut swept, mut attacks): (
        MessageWriter<recovery::SweptContact>,
        MessageWriter<crate::combat::AttackIntent>,
    ),
) {
    // Physics has its own clock: pause debris together with the race.
    for mut config in &mut physics {
        config.physics_pipeline_active =
            !matches!(race.phase, Phase::Paused | Phase::Garage | Phase::Finished);
    }
    if race.phase == Phase::Paused || race.phase == Phase::Finished {
        return;
    }
    let drive_started = std::time::Instant::now();
    work.begin_tick();
    let dt = time.delta_secs();
    let dynamic = *adapter == crate::vehicle::VehiclePhysicsAdapter::Dynamic;
    let player_intent = controls.buffer.held;
    let player_attack = controls.buffer.take_attack();
    let tuning = &content.0.bikes[0];
    if race.phase == Phase::Countdown {
        race.countdown -= dt;
        if race.countdown <= 0.0 {
            race.phase = Phase::Racing;
            race.message = "GO / MAKE IT COUNT".into();
            race.message_time = 2.0;
        }
    }
    let active = race.phase == Phase::Racing;
    if active {
        race.elapsed += dt;
    }
    race.message_time = (race.message_time - dt).max(0.0);
    race.shake = (race.shake - dt * 2.0).max(0.0);
    let snapshots: Vec<(Entity, usize, f32, f32, f32, bool, bool)> = riders
        .iter()
        .map(|(e, r, _, _, _)| {
            (
                e,
                r.id,
                r.progress,
                r.lane,
                r.speed,
                r.crashed > 0.0,
                r.finished.is_some(),
            )
        })
        .collect();
    let rider_frames: std::collections::BTreeMap<_, _> = riders
        .iter()
        .map(|(_, r, _, _, _)| {
            let (_, road) = track.sample_route(r.route, r.progress, r.lane);
            let forward_speed = r.motion_velocity.dot(road * Vec3::Z);
            // Contact-solver drift on an otherwise parked bike is not oncoming
            // traffic. Preserve the sign of meaningful reverse motion.
            let forward_speed = rr_core::traffic_avoidance::rider_forward_speed(forward_speed);
            (
                r.id,
                (
                    r.route,
                    r.motion_velocity.dot(road * Vec3::NEG_X),
                    forward_speed,
                ),
            )
        })
        .collect();
    let targets: Vec<_> = riders
        .iter()
        .filter(|(_, r, _, _, _)| r.finished.is_none())
        .map(|(_, r, state, t, _)| {
            let downed = state.stage != RecoveryStage::Riding;
            let (progress, lane, _) = if downed {
                track.project_route(
                    r.route,
                    state
                        .runner
                        .or(state.torso)
                        .and_then(|e| poses.get(e).ok())
                        .map_or(t.translation, GlobalTransform::translation),
                    r.progress,
                )
            } else {
                (r.progress, r.lane, 0.0)
            };
            rr_core::pursuit::Target {
                id: r.id,
                progress,
                lane,
                speed: r.speed,
                downed,
            }
        })
        .collect();
    let player_progress = snapshots.iter().find(|s| s.1 == 0).map_or(0.0, |s| s.2);

    let mut escape_budget = 1usize;
    for (entity, mut r, mut recovery, mut transform, mut visibility) in &mut riders {
        if recovery.stage != RecoveryStage::Riding {
            *visibility = Visibility::Hidden;
            continue;
        }
        *visibility = Visibility::Visible;
        let previous_progress = r.progress;
        let previous_position = transform.translation;
        let mut suspended_bridge_pose = None;
        if active && r.finished.is_none() {
            r.immunity = (r.immunity - dt).max(0.0);
            r.impact_cooldown = (r.impact_cooldown - dt).max(0.0);
            if r.hop_height > 0.0 || r.hop_velocity > 0.0 {
                r.hop_velocity -= 9.81 * dt;
                r.hop_height += r.hop_velocity * dt;
                if r.hop_height <= 0.0 {
                    r.hop_height = 0.0;
                    if r.hop_velocity < -2.0 {
                        r.hop_velocity *= -0.32;
                    } else {
                        r.hop_velocity = 0.0;
                    }
                    audio.write(AudioCue::new(Cue::Land, transform.translation));
                }
            }
            r.attack = (r.attack - dt).max(0.0);
            let is_player = r.human_controlled();
            let remote_input = network_inputs.as_ref().and_then(|n| n.get(r.id));
            let player_intent = remote_input.map_or(player_intent, |i| i.vehicle);
            let player_attack = remote_input.map_or(player_attack, |i| i.attack);
            let police_goal = r
                .police
                .then(|| police.drive_goal(&r, &track, tuning.top_speed_mps));
            let profile = r.ai_profile;
            r.combat_mind.tick(dt);
            let character = rr_core::cast::character(r.id);
            let race_pace = if r.police {
                profile.race_pace(tuning.top_speed_mps) * character.map_or(1., |c| c.pace)
            } else {
                // Keep the planner's passing horizon based on the rider's
                // intended clear-road pace. The physical motor is capped
                // separately at the bike-class ceiling; shrinking this horizon
                // makes a slow queue appear safe to follow indefinitely.
                profile.race_pace(tuning.top_speed_mps) * character.map_or(1., |c| c.pace)
            };
            rr_core::pursuit::calm(&mut r.relationships, profile, dt);
            r.fight_stamina = (r.fight_stamina + 120.0 * dt).min(profile.stamina);
            let me = rr_core::pursuit::Target {
                id: r.id,
                progress: r.progress,
                lane: r.lane,
                speed: r.speed,
                downed: false,
            };
            let mut eligible = [me; NAMES.len() + 1];
            let mut eligible_count = 0;
            for t in &targets {
                if track.same_corridor(
                    r.route,
                    r.progress,
                    rider_frames.get(&t.id).map_or(0, |frame| frame.0),
                    t.progress,
                ) && eligible_count < eligible.len()
                {
                    eligible[eligible_count] = *t;
                    eligible_count += 1;
                }
            }
            // Combat distances use one carriageway frame even through offset
            // city bends; traffic proxies below retain their original stations.
            let mut combat_targets = eligible;
            for t in &mut combat_targets[..eligible_count] {
                let route = rider_frames.get(&t.id).map_or(0, |frame| frame.0);
                t.lane += track.street_offset(r.route, r.progress)
                    - track.street_offset(route, t.progress);
                if track.definition.closed {
                    t.progress = r.progress + wrapped_gap(t.progress - r.progress, track.length);
                }
            }
            // Police stay physical traffic, but are not race opponents. A slow
            // patrol must not divert every nearby fighter into an endless duel.
            let mut fight_targets = combat_targets;
            let mut fight_count = 0;
            for t in &combat_targets[..eligible_count] {
                if t.id != crate::police::OFFICER_ID {
                    fight_targets[fight_count] = *t;
                    fight_count += 1;
                }
            }
            let chase = if r.police {
                police
                    .target
                    .filter(|_| police.mind.state.wanted())
                    .map(|t| t.0)
            } else if is_player {
                None
            } else {
                r.combat_mind.choose(
                    race.elapsed,
                    me,
                    &fight_targets[..fight_count],
                    profile.reach,
                )
            };
            let chase_goal = if r.police {
                police_goal
            } else if is_player {
                None
            } else {
                r.combat_mind.drive(me, chase, race_pace)
            };
            if !is_player && !r.police && r.combat_mind.take_lane_handoff(me, chase) {
                // A completed pass can otherwise keep the fighter several lanes
                // away. Release only its memory; all real safety sweeps below
                // still validate the requested encounter line from scratch.
                r.ai_pass = default();
            }
            let mut throttle = if let Some((_, speed)) = police_goal {
                r.speed < speed - 0.05
            } else if is_player {
                player_intent.throttle
            } else {
                true
            };
            let mut brake = if let Some((_, speed)) = police_goal {
                r.speed > speed + 0.1
            } else if is_player {
                player_intent.brake
            } else {
                chase_goal.is_some_and(|(_, speed)| r.speed > speed + 3.0)
            };
            let boost = is_player
                && handling.player_nitro_enabled
                && player_intent.nitro
                && throttle
                && r.nitro > 0.5;
            let top = if is_player {
                if boost {
                    tuning.nitro_speed_mps
                } else {
                    tuning.top_speed_mps
                }
            } else {
                chase_goal.map_or(race_pace, |(_, speed)| speed)
            };
            let curve = [0., 0.6, 1.1, 1.6]
                .into_iter()
                .map(|seconds| {
                    track
                        .curvature_route(r.route, r.progress + r.speed.abs().max(12.) * seconds)
                        .abs()
                })
                .fold(0.0_f32, f32::max);
            let target = if is_player {
                top
            } else {
                top.min(
                    rr_core::pursuit::race_corner_speed(
                        curve,
                        rr_core::pursuit::RACE_LOOKAHEAD_LATERAL_ACCEL_MPS2,
                    )
                    .max(12.),
                )
            };
            let surface = track.surface_route(r.route, r.progress, r.lane);
            let on_grass = (r.lane - track.street_offset(r.route, r.progress)).abs()
                > track.half_width(r.progress);
            let mut target = surface.speed_limit(if on_grass { target.min(32.0) } else { target });
            let corner_braking = !is_player && r.speed > target + 1.5;
            if corner_braking {
                brake = true;
                throttle = false;
            }
            let trace_ai = r.ai_trace_next.is_some_and(|next| race.elapsed >= next);
            if trace_ai {
                r.ai_trace_next = Some(race.elapsed + 0.5);
                println!(
                    "RFU_AI_SPEED id={} elapsed={} station={} speed={} curve={} race_pace={} top={} target={} corner_braking={}",
                    r.id,
                    race.elapsed,
                    r.progress,
                    r.speed,
                    curve,
                    race_pace,
                    top,
                    target,
                    corner_braking
                );
            }
            let mut queue_launch_deficit = 0.0_f32;
            let traffic_plan = (!is_player).then(|| {
                let offset = track.street_offset(r.route, r.progress);
                let width = track
                    .metropolis
                    .as_ref()
                    .map_or(track.half_width(r.progress), |m| {
                        let (_, _, width) = rr_core::streets::markings(
                            &m.street_life.lane_sections,
                            r.route,
                            r.progress,
                        );
                        width
                    });
                // Civilian traffic keeps its convention. Rivals may use the
                // whole physical corridor, including clear sidewalk space.
                let bounds = ai_free_bounds(&track, r.route, r.progress);
                let roam = (width - 0.9).clamp(0.0, 3.4);
                // Spread default racing lines across the paved road without
                // changing the legacy first-racer's escape direction.
                let racing_lines = [-0.8, 0.7, -0.4, 0.0, 0.4];
                let launching = !r.police && race.elapsed < 5.;
                let planning_pace = if launching {
                    rr_core::rival_ai::planning_pace(race.elapsed, r.speed, target)
                } else {
                    target
                };
                let passing_line = if launching {
                    None
                } else {
                    rr_core::rival_ai::passing_lane(me, &combat_targets[..eligible_count])
                };
                let desired = if launching {
                    r.lane
                } else {
                    chase_goal.map(|(lane, _)| lane).or(passing_line)
                        .unwrap_or(offset + roam * racing_lines[r.id % racing_lines.len()])
                };
                // Explicit pass, combat and flee lines may use a clear sidewalk.
                // Clamp only to the full physical corridor; static sweeps reject
                // solid buildings or props inside it.
                let desired = desired.clamp(bounds.0, bounds.1);

                let mut cars: Vec<_> = traffic
                    .iter()
                    .filter_map(|(entity, car, body, yielding, _)| {
                        if !track.same_corridor(r.route, r.progress, car.route, car.progress) {
                            return None;
                        }
                        let coarse_gap = car.progress - r.progress;
                        let coarse_gap = if track.definition.closed {
                            (coarse_gap + track.length * 0.5).rem_euclid(track.length)
                                - track.length * 0.5
                        } else {
                            coarse_gap
                        };
                        if coarse_gap.abs() > 1520. {
                            return None;
                        }
                        let default_bounds = (Vec3::ZERO, Vec3::new(0.95, 0.9, 1.95));
                        let (center, half) = if !track.is_hills() {
                            default_bounds
                        } else {
                            shapes.get(entity).map_or(default_bounds, |shape| {
                                let (lo, hi) =
                                    recovery::collider_bounds(&Transform::IDENTITY, shape);
                                ((lo + hi) * 0.5, (hi - lo) * 0.5)
                            })
                        };
                        let offset = if center.z.abs() > 0.001 {
                            let road_forward =
                                track.sample_route(car.route, car.progress, car.lane).1 * Vec3::Z;
                            center.z
                                * if (body.rotation * Vec3::Z).dot(road_forward) >= 0. {
                                    1.
                                } else {
                                    -1.
                                }
                        } else {
                            0.
                        };
                        let mut gap = car.progress - r.progress + offset;
                        if track.definition.closed {
                            gap = (gap + track.length * 0.5).rem_euclid(track.length)
                                - track.length * 0.5;
                        }
                        (gap.abs() < 1500.0).then(|| rr_core::traffic_avoidance::Car {
                            gap,
                            // Read placed geometry, including street reservation offsets.
                            lane: track.lane_at(
                                car.route,
                                car.progress,
                                body.transform_point(center),
                            ) - track.street_offset(car.route, car.progress),
                            speed: car.speed,
                            lateral_speed: yielding.lateral_speed,
                            lateral_goal: yielding.lateral_goal,
                            half_width: half.x,
                            half_length: half.z,
                        })
                    })
                    .collect();
                let main_proxy_count = cars.len();
                let (_, road) = track.sample_route(r.route, r.progress, r.lane);
                let forward = (road * Vec3::Z).with_y(0.0).normalize();
                let lateral = (road * Vec3::NEG_X).with_y(0.0).normalize();
                for (pose, path, velocity, crossing, disabled) in &crossing_cars {
                    let delta = pose.translation - transform.translation;
                    if delta.xz().length_squared() > 350.0_f32.powi(2) || delta.y.abs() > 4.0 {
                        continue;
                    }
                    let world_velocity = if crossing.is_some() {
                        // Crossing's fixed mover measures actual displacement, including
                        // junction yielding; intended path speed would predict a held car moving.
                        velocity.map_or(Vec3::ZERO, |v| v.linear)
                    } else if disabled.is_some() {
                        Vec3::ZERO
                    } else if let Some(path) = path {
                        let phase =
                            (race.elapsed * path.speed / path.extent + path.phase).rem_euclid(4.0);
                        path.axis
                            * path.speed
                            * if path.one_way || phase < 2.0 {
                                1.0
                            } else {
                                -1.0
                            }
                    } else {
                        velocity.map_or(Vec3::ZERO, |v| v.linear)
                    };
                    let x = pose.rotation * Vec3::X;
                    let z = pose.rotation * Vec3::Z;
                    cars.push(rr_core::traffic_avoidance::Car {
                        gap: delta.dot(forward),
                        lane: r.lane - offset + delta.dot(lateral),
                        speed: world_velocity.dot(forward),
                        lateral_speed: world_velocity.dot(lateral),
                        lateral_goal: None,
                        half_width: x.dot(lateral).abs() * 0.95 + z.dot(lateral).abs() * 1.95,
                        half_length: x.dot(forward).abs() * 0.95 + z.dot(forward).abs() * 1.95,
                    });
                }
                let crossing_proxy_count = cars.len() - main_proxy_count;
                // Rider-only queues must enter the same predictive planner as
                // cars. Previously this snapshot was added only *after* a car
                // hazard, so crawling racers never planned around one another.
                let riders: Vec<_> = eligible[..eligible_count]
                    .iter()
                    .filter_map(|other| {
                        let gap = if track.definition.closed {
                            wrapped_gap(other.progress - r.progress, track.length)
                        } else {
                            other.progress - r.progress
                        };
                        (other.id != r.id && !other.downed && gap.abs() < 200.0).then_some(
                            rr_core::traffic_avoidance::Car {
                                gap,
                                lane: other.lane
                                    - track.street_offset(
                                        rider_frames.get(&other.id).map_or(0, |f| f.0),
                                        other.progress,
                                    ),
                                // Display speed is unsigned. Reverse motion must
                                // not make the next follower accelerate into a backoff.
                                speed: rider_frames.get(&other.id).map_or(other.speed, |frame| frame.2),
                                lateral_speed: rider_frames
                                    .get(&other.id)
                                    .map_or(0.0, |frame| frame.1),
                                lateral_goal: None,
                                half_width: 0.36,
                                half_length: 1.05,
                            },
                        )
                    })
                    .collect();
                // Keep persistent passing memory in street-centred coordinates,
                // so a bend cannot move a committed escape back into its leader.
                let lane = r.lane - offset;
                queue_launch_deficit = riders.iter().map(|other| {
                    rr_core::traffic_avoidance::queue_launch_deficit(lane, *other)
                }).fold(0.0_f32, f32::max);
                let desired = desired - offset;
                let bounds = (bounds.0 - offset, bounds.1 - offset);
                let in_race_frame = |mut plan: rr_core::traffic_avoidance::Plan| {
                    plan.lane += offset;
                    plan
                };
                // A calm grid must still see an actually occupied launch lane.
                // Include riders before the cheap clear-road path, without
                // pretending a stationary rider is already accelerating.
                if launching { cars.extend_from_slice(&riders); }
                let local_horizon = track.is_hills().then(|| rr_core::traffic_avoidance::local_flow_horizon(r.speed));
                let preliminary = rr_core::traffic_avoidance::Memory::default().plan_local(
                    rr_core::traffic_avoidance::Request {
                        lane, speed: r.speed, desired, cruise: planning_pace, bounds,
                    },
                    &cars, local_horizon, |_, _| true,
                );
                let rider_queue = riders.iter().any(|other| {
                    rr_core::traffic_avoidance::slow_queue_ahead(lane, if launching { planning_pace } else { race_pace }, *other)
                });
                if !preliminary.avoiding
                    && !r.ai_pass.active()
                    && !rider_queue
                    && chase_goal.is_none()
                    && passing_line.is_none()
                    && race.elapsed >= r.ai_static_guard_until
                    && (bounds.0..=bounds.1).contains(&desired)
                {
                    if trace_ai {
                        println!("RFU_AI_PLANNER id={} elapsed={} station={} lane={} speed={} preliminary={preliminary:?} proxies={cars:?}", r.id, race.elapsed, r.progress, lane, r.speed);
                    }
                    return in_race_frame(preliminary);
                }
                // Once passing, include every upright racer, including rear and
                // fast alongside traffic, not just the slow queue's leaders.
                if !launching { cars.extend(riders); }
                let trace_id = r.id;
                let progress = r.progress;
                let route = r.route;
                let speed = r.speed;
                let committed = r.ai_pass.committed_lane();
                work.begin_rider();
                let drivable = |goal: f32, pace: f32| {
                    let forecast_speed = pace.min(speed.max(4.));
                    work.static_clear(goal, forecast_speed, |cast_count| {
                    let Ok(ctx) = context.single() else {
                        return false;
                    };
                    let predicate =
                        |e, _: &bevy_rapier3d::rapier::geometry::Collider| !ground.contains(e) && owned.get(e).map_or(true, |o| o.owner != entity);
                    let mut previous = transform.translation + Vec3::Y * 0.85;
                    let probe = Collider::ball(0.65);
                    // A broad lane change or sidewalk excursion needs the full
                    // sweep even though both sides now lie inside free bounds.
                    let steps = if goal.abs() > width - 0.65 || (goal - lane).abs() > 2.5 {
                        24
                    } else {
                        12
                    };
                    // At a lower current speed the bike has more time to move
                    // laterally before reaching a fixed wall. Forecast that
                    // conservative approach instead of only the cruise request.
                    // Static geometry is checked along the actual curved road,
                    // not a straight chord through a sidewalk/building at a bend.
                    for step in 1..=steps {
                        let t = step as f32 * 0.5;
                        let station = (progress + forecast_speed * t).min(track.length);
                        // The bike's bank and steering force take time to unwind.
                        // Keep the static sweep inside a reachable lateral envelope;
                        // the traffic proxy's faster idealized lane transition can
                        // otherwise predict clearing a wall while the physical bike
                        // is still beside it.
                        let lateral = lane
                            + (goal - lane).clamp(
                                -0.6 * (t - 0.4).max(0.),
                                0.6 * (t - 0.4).max(0.),
                            )
                            + track.street_offset(route, station);
                        let (position, _) = track.sample_route(route, station, lateral);
                        let next = position + Vec3::Y * 0.85;
                        let delta = next - previous;
                        let filter = QueryFilter {
                            flags: QueryFilterFlags::EXCLUDE_DYNAMIC
                                | QueryFilterFlags::EXCLUDE_KINEMATIC,
                            ..default()
                        }
                        .exclude_rigid_body(entity)
                        .exclude_sensors()
                        .predicate(&predicate);
                        cast_count.set(cast_count.get() + 1);
                        if let Some((hit_entity, hit)) = ctx
                            .cast_shape(
                                previous,
                                Quat::IDENTITY,
                                delta.normalize_or_zero(),
                                probe.raw.as_ref(),
                                ShapeCastOptions {
                                    max_time_of_impact: delta.length(),
                                    ..default()
                                },
                                filter,
                            )
                        {
                            if trace_ai {
                                println!("RFU_AI_STATIC id={} station={} speed={} goal={} pace={} step={} hit={:?} name={:?} toi={} position={:?}", trace_id, progress, speed, goal, pace, step, hit_entity, names.get(hit_entity).ok().map(Name::as_str), hit.time_of_impact, poses.get(hit_entity).ok().map(GlobalTransform::translation));
                            }
                            return false;
                        }
                        previous = next;
                    }
                    true
                    })
                };
                let input = rr_core::traffic_avoidance::Request {
                    lane,
                    speed,
                    desired,
                    cruise: planning_pace,
                    bounds,
                };
                let plan = r.ai_pass.plan_local_ordered(input, &cars, local_horizon, work.optimized(), drivable);
                if ai_static_guard_blocked(lane, desired, committed, |goal| !drivable(goal, planning_pace)) {
                    // Continue checking the nearby fixed obstacle after a combat
                    // or passing request expires; clean cruising may resume once
                    // geometry is clear for two seconds of real plans.
                    r.ai_static_guard_until = race.elapsed + 2.;
                }
                if trace_ai {
                    println!("RFU_AI_PLANNER id={} elapsed={} station={} lane={} speed={} desired={} bounds={bounds:?} rider_queue={} launch_deficit={} preliminary={preliminary:?} final={plan:?} home={:?} main_proxies={} crossing_proxies={} proxies={cars:?}", r.id, race.elapsed, progress, lane, speed, desired, rider_queue, queue_launch_deficit, r.ai_pass.home_lane(), main_proxy_count, crossing_proxy_count);
                }
                in_race_frame(plan)
            });
            r.ai_plan = traffic_plan;
            let traffic_braking =
                traffic_plan.is_some_and(|p| p.avoiding && r.speed > p.speed + 0.25);
            if let Some(plan) = traffic_plan.filter(|p| p.avoiding) {
                target = target.min(plan.speed);
                throttle &= r.speed < target - 0.05;
                brake |= traffic_braking;
            }
            let rear_clear = !is_player
                && (r.speed < 1. || r.ai_drive.reversing())
                && context.single().is_ok_and(|ctx| {
                    let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                        !ground.contains(e) && owned.get(e).map_or(true, |o| o.owner != entity)
                    };
                    ctx.cast_shape(
                        transform.translation + Vec3::Y * 0.8,
                        transform.rotation,
                        -(transform.rotation * Vec3::Z),
                        Collider::cuboid(0.48, 0.45, 1.05).raw.as_ref(),
                        ShapeCastOptions {
                            max_time_of_impact: 3.5,
                            ..default()
                        },
                        QueryFilter::default()
                            .exclude_rigid_body(entity)
                            .predicate(&predicate),
                    )
                    .is_none()
                });
            let stalled_speed = r.speed;
            let reposition_queue = queue_launch_deficit > 0.5
                && stalled_speed.abs() < 0.65
                && traffic_plan.is_some_and(|p| p.avoiding && p.speed < 0.1);
            if reposition_queue {
                // Re-anchor escape progress after deliberate backing; the old
                // pre-backoff station would otherwise look like another stall.
                r.ai_escape = default();
            }
            let backing = !is_player
                && r.ai_drive.step_with_queue(
                    dt,
                    stalled_speed,
                    race.elapsed > 5. && target > 4. && throttle,
                    rear_clear,
                    race.elapsed > 5. && reposition_queue,
                    queue_launch_deficit > 0.,
                );
            if backing {
                r.ai_escape = default();
                throttle = false;
                brake = true;
            }
            if !dynamic {
                if (is_player || backing)
                    && let Some(acceleration) = rr_core::reverse::acceleration(
                        r.speed,
                        throttle,
                        brake,
                        if backing {
                            1.
                        } else {
                            player_intent.brake_strength()
                        },
                        dt,
                    )
                {
                    r.speed += acceleration * dt;
                } else {
                    if throttle && r.speed < target {
                        let acceleration = if boost {
                            tuning.nitro_acceleration_mps2
                        } else {
                            tuning.acceleration_mps2
                        };
                        r.speed = (r.speed
                            + dt * acceleration
                                * player_intent.throttle_strength().max(if is_player {
                                    0.0
                                } else {
                                    1.0
                                })
                                * surface.traction()
                                * surface.drive_scale())
                        .min(target);
                    } else if r.speed > target || !throttle {
                        r.speed = r.speed.signum()
                            * (r.speed.abs() - dt * tuning.coast_drag_mps2).max(0.0);
                    }
                    if brake {
                        r.speed = (r.speed
                            - dt * tuning.brake_mps2
                                * if r.police || traffic_braking {
                                    1.0
                                } else {
                                    player_intent.brake_strength()
                                })
                        .max(0.0);
                    }
                }
            }
            if !dynamic {
                r.speed += surface.rolling_acceleration(r.speed, dt) * dt;
                if !is_player && !r.police {
                    r.speed = r.speed.min(rr_core::pursuit::rival_drive_ceiling(
                        profile,
                        character.map_or(1., |c| c.pace),
                        tuning.top_speed_mps,
                        r.combat_mind.motor_request(tuning.top_speed_mps),
                    ));
                }
            }
            if boost {
                r.nitro = (r.nitro - dt * 25.0).max(0.0);
            } else {
                r.nitro = (r.nitro + dt * 8.0).min(100.0);
            }
            let mut steer;
            if is_player {
                r.raw_steer = player_intent.steer;
                let speed = r.speed.abs();
                let response = handling.presets[handling.selected].steering;
                steer = r.steering.step(player_intent.steer, speed, dt, response);
                // High speed on a tight curve pushes the rider toward the outside shoulder.
                if !dynamic {
                    r.lane +=
                        track.curvature_route(r.route, r.progress) * r.speed * r.speed * dt * 0.025;
                }
            } else {
                let plan = traffic_plan.expect("AI traffic plan");
                let desired = r.ai_drive.stable_lane(
                    plan.lane,
                    dt / character.map_or(1., |c| c.patience),
                    plan.avoiding,
                );
                steer = if race.elapsed < 3.0 && traffic_plan.is_none_or(|p| !p.avoiding) {
                    0.0
                } else {
                    let (_, road) = track.sample_route(r.route, r.progress, r.lane);
                    let lateral_speed = r.motion_velocity.dot(road * Vec3::NEG_X);
                    let next = rr_core::pursuit::lane_steering(
                        desired - r.lane,
                        lateral_speed,
                        r.speed,
                        handling.presets[handling.selected].turn_response,
                        r.steering.force,
                        dt,
                    );
                    r.steering.force = next;
                    next
                };
            }
            r.drive_intent = rr_core::VehicleIntent {
                throttle,
                brake,
                steer: if backing { -steer } else { steer },
                nitro: boost,
                throttle_amount: if is_player {
                    player_intent.throttle_strength()
                } else {
                    {
                        let speed = if traffic_plan.is_some_and(|p| p.avoiding) {
                            target
                        } else {
                            chase_goal.map_or(target, |(_, speed)| speed)
                        };
                        if r.police {
                            ((speed - r.speed) * 0.25).clamp(0.0, 1.0)
                        } else {
                            rr_core::pursuit::rival_throttle_amount(race.elapsed, r.speed, speed)
                        }
                    }
                },
                brake_amount: if is_player {
                    player_intent.brake_strength()
                } else {
                    if brake {
                        if r.police || traffic_braking || corner_braking || backing {
                            1.0
                        } else {
                            0.25
                        }
                    } else {
                        0.0
                    }
                },
            };
            if !is_player
                && dynamic
                && race.elapsed > 5.
                && !reposition_queue
                && !backing
                && let Ok(ctx) = context.single()
            {
                let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                    !ground.contains(e) && owned.get(e).map_or(true, |o| o.owner != entity)
                };
                let clear = |a: Vec2, b: Vec2| {
                    let from = Vec3::new(a.x, track.terrain_height(a.x, a.y) + 0.85, a.y);
                    let to = Vec3::new(b.x, track.terrain_height(b.x, b.y) + 0.85, b.y);
                    let delta = to - from;
                    ctx.cast_shape(
                        from,
                        Quat::IDENTITY,
                        delta.normalize_or_zero(),
                        Collider::ball(1.1).raw.as_ref(),
                        ShapeCastOptions {
                            max_time_of_impact: delta.length(),
                            stop_at_penetration: false,
                            ..default()
                        },
                        QueryFilter::default()
                            .exclude_rigid_body(entity)
                            .predicate(&predicate),
                    )
                    .is_none()
                };
                let (goal, _) = track.sample_route(
                    r.route,
                    (r.progress + 22.).min(track.length),
                    traffic_plan.map_or(0., |plan| plan.lane),
                );
                let position = transform.translation.xz();
                let goal = position + (goal.xz() - position).clamp_length_max(65.);
                let wants = target > 4.0 && r.drive_intent.throttle;
                let progress = r.progress;
                if let Some((go, reverse)) = r.ai_escape.tick(
                    dt,
                    position.to_array(),
                    progress,
                    (transform.rotation * Vec3::Z)
                        .xz()
                        .normalize_or_zero()
                        .to_array(),
                    goal.to_array(),
                    wants,
                    &mut escape_budget,
                    |a, b| clear(Vec2::from_array(a), Vec2::from_array(b)),
                ) {
                    r.ai_drive.cancel_reverse();
                    r.ai_escape.reverse = reverse && go;
                    r.drive_intent.throttle = go && !reverse;
                    r.drive_intent.brake = !go || reverse;
                    r.drive_intent.throttle_amount = if go && !reverse {
                        ((4. - r.speed) * 0.4).clamp(0., 1.)
                    } else {
                        0.
                    };
                    r.drive_intent.brake_amount = if !go || reverse || r.speed > 4.5 {
                        1.
                    } else {
                        0.
                    };
                    r.drive_intent.steer = 0.;
                    steer = 0.;
                    if r.speed > 4.5 {
                        r.drive_intent.throttle = false;
                        r.drive_intent.brake = true;
                    }
                }
            }
            if trace_ai {
                println!(
                    "RFU_AI_CONTROL elapsed={} station={} effective_throttle={} effective_brake={} escape={} reverse={}",
                    race.elapsed,
                    r.progress,
                    r.drive_intent.throttle_strength(),
                    r.drive_intent.brake_strength(),
                    r.ai_escape.active(),
                    r.ai_escape.reverse
                );
            }
            if !dynamic {
                r.lane += r.side_velocity * dt;

                r.lane += steer
                    * handling.presets[handling.selected].turn_response
                    * (tuning.steering_rate_base + r.speed * tuning.steering_rate_speed)
                    * dt;
            }
            r.side_velocity *= (-dt * 2.5).exp();
            r.lean = r.lean.lerp(
                rr_core::steering::visual_bank(steer),
                1.0 - (-dt * 16.0).exp(),
            );
            if on_grass && !dynamic && surface != rr_core::surface::Surface::Sand {
                r.speed = r.speed.signum() * (r.speed.abs() - dt * 1.8).max(0.0);
            }
            if !dynamic {
                r.progress += r.speed * dt;
            }
            if !dynamic && !track.definition.closed && r.progress >= track.race_interval().finish {
                r.progress = track.race_interval().finish;
                r.speed = 0.0;
            }
            if !dynamic
                && track.is_hills()
                && (crate::track::hills::BRIDGE_START..=crate::track::hills::BRIDGE_END)
                    .contains(&r.progress)
            {
                let (center, rotation) = track.sample_route(r.route, r.progress, 0.);
                if previous_position.y >= center.y - 2. {
                    // The assisted adapter must test the suspended deck before its
                    // generic off-road sampler can snap it onto the canyon floor.
                    suspended_bridge_pose = Some((center - rotation * Vec3::X * r.lane, rotation));
                }
            }
            // Human and AI share damage and cooldown rules. Attack only one side at once.
            let attack_side = if is_player {
                player_attack
            } else if race.elapsed > 8.0
                && r.speed > 6.0
                && r.fight_stamina
                    >= if r.weapon.is_some() {
                        weapons
                            .get(r.weapon.unwrap_or(rr_core::weapons::Weapon::Club))
                            .stamina
                    } else {
                        250.0
                    }
            {
                chase
                    .filter(|t| {
                        r.can_fight(t.id) && !t.downed && {
                            let kick = r.weapon.is_none()
                                && r.attack_serial.is_multiple_of(
                                    if character.is_some_and(|c| c.kick) {
                                        2
                                    } else {
                                        profile.kick_roll
                                    },
                                );
                            let ability = r.weapon.map(|w| weapons.get(w));
                            let windup = strike_timings.contact_after(kick, ability);
                            let velocities = [
                                rider_frames
                                    .get(&r.id)
                                    .map_or([me.speed, 0.0], |f| [f.2, f.1]),
                                rider_frames
                                    .get(&t.id)
                                    .map_or([t.speed, 0.0], |f| [f.2, f.1]),
                            ];
                            rr_core::rival_ai::contact_frame(me, *t, velocities, windup)
                                .is_some_and(|(me, target)| {
                                    if let Some(ability) = ability {
                                        rr_core::rival_ai::strike_window(
                                            me,
                                            target,
                                            windup,
                                            ability.longitudinal_reach,
                                            ability.lateral_reach,
                                        )
                                    } else {
                                        rr_core::rival_ai::direct_strike_window(me, target, windup)
                                    }
                                })
                        }
                    })
                    .map(|t| {
                        if r.weapon.is_none()
                            && r.attack_serial.is_multiple_of(
                                if character.is_some_and(|c| c.kick) {
                                    2
                                } else {
                                    profile.kick_roll
                                },
                            )
                        {
                            rr_core::AttackSide::Kick
                        } else if t.lane > r.lane {
                            rr_core::AttackSide::Right
                        } else {
                            rr_core::AttackSide::Left
                        }
                    })
            } else {
                None
            };
            if let Some(attack_side) = attack_side
                && r.attack <= 0.0
                && (r.weapon.is_none()
                    || matches!(attack_side, rr_core::AttackSide::Kick)
                    || weapons
                        .get(r.weapon.unwrap_or(rr_core::weapons::Weapon::Club))
                        .can_start(r.fight_stamina))
            {
                r.attack = if r.weapon.is_some() && attack_side != rr_core::AttackSide::Kick {
                    weapons
                        .get(r.weapon.unwrap_or(rr_core::weapons::Weapon::Club))
                        .cooldown
                } else if is_player {
                    0.65
                } else {
                    r.combat_mind.cooldown(profile.attack_cooldown())
                };
                r.attack_serial = r.attack_serial.wrapping_add(1);
                if !is_player && !r.police {
                    r.combat_mind.strike_started();
                }
                if r.weapon.is_some() && attack_side != rr_core::AttackSide::Kick {
                    r.fight_stamina -= weapons
                        .get(r.weapon.unwrap_or(rr_core::weapons::Weapon::Club))
                        .stamina;
                } else if !is_player {
                    r.fight_stamina -= 250.0;
                }
                r.kicking = matches!(attack_side, rr_core::AttackSide::Kick);
                r.attack_side = attack_side.sign();
                attacks.write(crate::combat::AttackIntent(entity, attack_side));
            }
            if r.health <= 0.0 && recovery.stagger.is_none() {
                let (origin, rotation) = if dynamic {
                    (transform.translation, transform.rotation)
                } else {
                    suspended_bridge_pose
                        .unwrap_or_else(|| track.sample_route(r.route, r.progress, r.lane))
                };
                recovery::crash(
                    &mut commands,
                    &art,
                    entity,
                    &mut r,
                    &mut recovery,
                    origin,
                    rotation,
                    Vec3::ZERO,
                    &context,
                );
                *visibility = Visibility::Hidden;
                audio.write(AudioCue::new(Cue::Crash, origin));
                if is_player {
                    race.shake = 1.0;
                    race.message = "WIPED OUT / GETTING BACK UP".into();
                    race.message_time = 3.5;
                }
            }
        }
        let (p, q) = suspended_bridge_pose
            .unwrap_or_else(|| track.sample_route(r.route, r.progress, r.lane));
        let mut destination = if dynamic && active {
            transform.translation
        } else {
            p + Vec3::Y * r.hop_height
        };
        let delta = destination - transform.translation;
        if !dynamic {
            r.motion_velocity = if active && dt > 0.0 && delta.length() < 5.0 {
                delta / dt
            } else if active {
                q * Vec3::Z * r.speed + Vec3::Y * r.hop_velocity
            } else {
                Vec3::ZERO
            };
        }
        if !dynamic
            && active
            && delta.length_squared() > 0.0000001
            && recovery.stage == RecoveryStage::Riding
            && let Ok(context) = context.single()
        {
            // Bound controller movement against real geometry, including very slow
            // lateral steering, rather than letting kinematic bikes pass through it.
            let predicate = |other, _: &bevy_rapier3d::rapier::geometry::Collider| {
                !ground.contains(other) && !owned.get(other).is_ok_and(|body| body.owner == entity)
            };
            // Dynamic debris is pushed by Rapier contacts; only immovable or
            // controller-driven geometry must bound this controller's sweep.
            let filter = QueryFilter::exclude_dynamic()
                .exclude_rigid_body(entity)
                .exclude_sensors()
                .predicate(&predicate);
            let shape = Collider::cuboid(0.38, 0.96, 1.12);
            let options = bevy_rapier3d::parry::query::ShapeCastOptions {
                max_time_of_impact: 1.0,
                target_distance: 0.01,
                stop_at_penetration: false,
                ..default()
            };
            if let Some((other, hit)) = context.cast_shape(
                transform.translation + transform.rotation * Vec3::Y,
                transform.rotation,
                delta,
                shape.raw.as_ref(),
                options,
                filter,
            ) {
                let low = shapes.get(other).ok().zip(poses.get(other).ok()).and_then(
                    |(shape, global)| {
                        let pose = global.compute_transform();
                        let (lo, hi) = recovery::collider_bounds(&pose, shape);
                        rr_core::contact::low_geometry(lo.y, hi.y, p.y).then_some(hi.y)
                    },
                );
                if let Some(top) = low {
                    let rise = (top - p.y + 0.08).max(0.0);
                    // Assisted mode follows the original's low-object height stack.
                    // Re-sweep at that height; a wall behind the low object stays solid.
                    let raised = transform.translation.with_y(p.y + rise);
                    let clear = context
                        .cast_shape(
                            raised + transform.rotation * Vec3::Y,
                            transform.rotation,
                            delta.with_y(0.0),
                            shape.raw.as_ref(),
                            options,
                            filter,
                        )
                        .map_or(1.0, |(_, hit)| hit.time_of_impact.clamp(0.0, 1.0));
                    r.hop_height = r.hop_height.max(rise);
                    r.hop_velocity = r.hop_velocity.max(0.0);
                    destination = raised + delta.with_y(0.0) * clear;
                    let (progress, lane, _) = track.project_route(r.route, destination, r.progress);
                    r.progress = progress;
                    r.lane = lane;
                } else {
                    destination =
                        transform.translation + delta * hit.time_of_impact.clamp(0.0, 1.0);
                    let (progress, lane, _) = track.project_route(r.route, destination, r.progress);
                    r.progress = progress;
                    r.lane = lane;
                    r.hop_height = (destination.y - p.y).max(0.0);
                }
                swept.write(recovery::SweptContact(entity, other));
            }
        }
        if suspended_bridge_pose.is_some()
            && recovery.stage == RecoveryStage::Riding
            && let Ok(ctx) = context.single()
        {
            let is_ground =
                |other, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(other);
            // Sweep solids above first: a blocked move must not fall through a
            // hanger or wall. Query below the deck plane, independent of hops.
            let supported = ctx
                .cast_ray(
                    destination.with_y(p.y + 0.7),
                    Vec3::NEG_Y,
                    1.2,
                    true,
                    QueryFilter::default()
                        .exclude_rigid_body(entity)
                        .predicate(&is_ground),
                )
                .is_some();
            if !supported {
                // Crossing the asphalt/deck support-height offset is not an
                // upward launch impulse. Preserve road grade and an existing hop.
                r.motion_velocity.y = (q * Vec3::Z * r.speed).y + r.hop_velocity;
                // Existing crash construction carries the just-computed lateral
                // steering/kick momentum into the real wreck and accepted rig.
                // Gravity, contacts and recovery take over; no downward teleport.
                recovery::crash(
                    &mut commands,
                    &art,
                    entity,
                    &mut r,
                    &mut recovery,
                    destination,
                    q,
                    Vec3::ZERO,
                    &context,
                );
                transform.translation = destination;
                transform.rotation = q;
                *visibility = Visibility::Hidden;
                audio.write(AudioCue::new(Cue::Crash, destination));
                if r.human_controlled() {
                    race.shake = 1.;
                    race.message = "OFF THE BRIDGE / GETTING BACK UP".into();
                    race.message_time = 3.5;
                }
                continue;
            }
        }
        if active && recovery.stage == RecoveryStage::Riding {
            let progress = r.progress;
            let route = r.route;
            r.gates.observe_route(
                &track,
                route,
                previous_position,
                destination,
                previous_progress,
                progress,
            );
            if !r.police && r.gates.completed_laps >= LAPS && r.finished.is_none() {
                finish_rider(
                    &mut r,
                    &mut race,
                    &mut profile,
                    persistence.0,
                    1 + snapshots.iter().filter(|s| s.1 != 0 && s.6).count(),
                    false,
                );
                commands.entity(entity).insert(ColliderDisabled);
            }
        }
        if !dynamic || !active {
            if let Ok(ctx) = context.single() {
                let predicate =
                    |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
                if let Some((_, hit)) = ctx.cast_ray_and_get_normal(
                    destination + Vec3::Y * 0.7,
                    -Vec3::Y,
                    1.2,
                    true,
                    QueryFilter::default()
                        .exclude_rigid_body(entity)
                        .predicate(&predicate),
                ) {
                    let height = destination.y + 0.7 - hit.time_of_impact;
                    if height - destination.y <= 0.35 {
                        destination.y = height + r.hop_height;
                    }
                }
            }
            transform.translation = destination;
            transform.rotation =
                q * Quat::from_rotation_y(rr_core::reverse::assisted_heading(
                    r.drive_intent.steer,
                    r.speed,
                )) * Quat::from_rotation_z(r.lean.clamp(-0.6, 0.6))
                    * Quat::from_rotation_x(-r.hop_velocity * 0.025);
        }
    }
    if let Some(metrics) = metrics.as_mut() {
        metrics.ai_work.merge(work.finish(drive_started));
    }
    if active && race.phase != Phase::Finished {
        race.place = 1 + riders
            .iter()
            .filter(|(_, r, _, _, _)| !r.police && r.id != 0 && r.progress > player_progress)
            .count();
    }
    let traffic_started = civilian_timings.as_ref().map(|_| std::time::Instant::now());
    let mut planner_elapsed = std::time::Duration::ZERO;
    let mut cars: Vec<_> = traffic
        .iter()
        .map(|(entity, c, _, y, _)| {
            let default_bounds = (Vec3::splat(-1.95), Vec3::splat(1.95));
            let (lo, hi) = shapes.get(entity).map_or(default_bounds, |shape| {
                recovery::collider_bounds(&Transform::IDENTITY, shape)
            });
            (
                c.route,
                c.progress,
                c.lane,
                c.speed,
                y.cruise.unwrap_or(c.speed).signum(),
                y.lane_slot,
                entity,
                hi.z.max(0.),
                (-lo.z).max(0.),
                hi.x.abs().max(lo.x.abs()),
            )
        })
        .collect();
    let interest: Vec<_> = if track.metropolis.is_some() {
        targets
            .iter()
            .map(|t| {
                track
                    .sample_route(
                        rider_frames.get(&t.id).map_or(0, |frame| frame.0),
                        t.progress,
                        t.lane,
                    )
                    .0
                    .xz()
            })
            .collect()
    } else {
        vec![]
    };
    let planner_started = civilian_timings.as_ref().map(|_| std::time::Instant::now());
    cars.sort_by_key(|c| c.6);
    let (fleet, peers) = &mut *civilian_scratch;
    fleet.clear();
    fleet.extend(cars.iter().filter_map(|c| {
        let Ok((_, car, _, y, _)) = traffic.get(c.6) else {
            return None;
        };
        let ratio = y
            .lane_ratio
            .unwrap_or(car.lane / track.half_width(car.progress));
        let (home, opposite, divider) = civilian_lanes(&track, c.0, c.1, c.4, c.5, ratio);
        Some(rr_core::civilian::Vehicle {
            id: c.6.to_bits(),
            station: c.1,
            lane: home + (opposite - home) * y.passing.blend - divider,
            home: home - divider,
            speed: c.3,
            cruise: y.cruise.unwrap_or(car.speed),
            front: c.7,
            rear: c.8,
            half_width: c.9,
            reserved: y.passing.reserved(),
        })
    }));
    if cars.len() != fleet.len() {
        cars.retain(|c| fleet.iter().any(|v| v.id == c.6.to_bits()));
    }
    if let Some(started) = planner_started {
        planner_elapsed += started.elapsed();
    }
    for index in 0..cars.len() {
        let Ok((entity, mut car, mut transform, mut yielding, disabled)) =
            traffic.get_mut(cars[index].6)
        else {
            continue;
        };
        let old_position = transform.translation;
        let cruise = *yielding.cruise.get_or_insert(car.speed);
        if race.elapsed < yielding.last_elapsed {
            if track.metropolis.is_some() || track.is_hills() {
                car.progress = yielding.start_progress.unwrap_or(car.progress);
            }
            car.speed = cruise;
            yielding.honk_remaining = 0.0;
            yielding.passing = default();
            yielding.pass_geometry_remaining = 0.;
            yielding.pass_geometry_clear = false;
            yielding.merge_geometry_clear = false;
        }
        yielding.last_elapsed = race.elapsed;
        if active {
            let direction = cruise.signum();
            let (front, rear) = cars
                .iter()
                .find(|c| c.6 == entity)
                .map_or((1.95, 1.95), |c| (c.7, c.8));
            let gap = targets
                .iter()
                .filter(|t| {
                    track.same_corridor(
                        car.route,
                        car.progress,
                        rider_frames.get(&t.id).map_or(0, |frame| frame.0),
                        t.progress,
                    )
                })
                .filter(|t| {
                    if !t.downed {
                        return false;
                    }
                    if !yielding.passing.reserved() {
                        return (t.lane - car.lane).abs() < 1.8;
                    }
                    // A returning pass must yield to a downed person in the
                    // home lane too, before its moving envelope reaches them.
                    let ratio = yielding
                        .lane_ratio
                        .unwrap_or(car.lane / track.half_width(car.progress));
                    let (home, opposite, _) = civilian_lanes(
                        &track,
                        car.route,
                        t.progress,
                        direction,
                        yielding.lane_slot,
                        ratio,
                    );
                    let lane = t.lane - track.street_offset(car.route, t.progress);
                    lane >= home.min(opposite) - 1.8 && lane <= home.max(opposite) + 1.8
                })
                .filter_map(|t| {
                    let gap = (t.progress - car.progress) * direction;
                    if track.is_hills() || yielding.passing.reserved() {
                        rr_core::traffic::occupied_gap(gap, front, rear)
                    } else {
                        (0.0..80.0).contains(&gap).then_some(gap)
                    }
                })
                .min_by(f32::total_cmp);
            let mut desired = rr_core::traffic::yielding_speed(cruise, gap);
            if track.is_hills() {
                let now = track.sample_route(car.route, car.progress, car.lane).1;
                let ahead = track
                    .sample_route(car.route, car.progress + direction * 12., car.lane)
                    .1;
                desired = desired.min(rr_core::traffic::curve_speed(
                    cruise,
                    now.angle_between(ahead),
                    12.,
                ));
            }
            if track.metropolis.is_some() || track.is_hills() {
                let planner_started = civilian_timings.as_ref().map(|_| std::time::Instant::now());
                peers.clear();
                peers.extend(
                    fleet
                        .iter()
                        .enumerate()
                        .filter(|(other, _)| {
                            *other != index
                                && track.same_corridor(
                                    car.route,
                                    car.progress,
                                    cars[*other].0,
                                    cars[*other].1,
                                )
                        })
                        .map(|(_, v)| *v),
                );
                let ratio = *yielding
                    .lane_ratio
                    .get_or_insert(car.lane / track.half_width(car.progress));
                let (home, opposite, divider) = civilian_lanes(
                    &track,
                    car.route,
                    car.progress,
                    direction,
                    yielding.lane_slot,
                    ratio,
                );
                let future = civilian_lanes(
                    &track,
                    car.route,
                    car.progress + direction * 300.,
                    direction,
                    yielding.lane_slot,
                    ratio,
                );
                yielding.pass_geometry_remaining = (yielding.pass_geometry_remaining - dt).max(0.);
                if yielding.passing.needs_clearance() && yielding.pass_geometry_remaining == 0. {
                    yielding.pass_geometry_remaining = 0.2;
                    let swept_clear = |returning: bool| {
                        let Ok(ctx) = context.single() else {
                            return false;
                        };
                        let Ok(shape) = shapes.get(entity) else {
                            return false;
                        };
                        let predicate =
                            |e, _: &bevy_rapier3d::rapier::geometry::Collider| !ground.contains(e);
                        let mut previous = transform.translation;
                        for step in 1..=12 {
                            let t = step as f32 * 0.5;
                            let station =
                                car.progress + direction * desired.max(car.speed.abs()) * t;
                            let (h, o, _) = civilian_lanes(
                                &track,
                                car.route,
                                station,
                                direction,
                                yielding.lane_slot,
                                ratio,
                            );
                            let blend = if returning {
                                (yielding.passing.blend - t / 3.).max(0.)
                            } else {
                                (yielding.passing.blend + t / 3.).min(1.)
                            };
                            let lane = h + (o - h) * blend;
                            let width = track.metropolis.as_ref().map_or_else(
                                || track.half_width(station),
                                |m| {
                                    rr_core::streets::markings(
                                        &m.street_life.lane_sections,
                                        car.route,
                                        station,
                                    )
                                    .2
                                },
                            );
                            if lane.abs() + fleet[index].half_width + 0.3 > width {
                                return false;
                            }
                            let (next, q) = track.street_sample(car.route, station, lane);
                            let delta = next - previous;
                            let filter = QueryFilter {
                                flags: QueryFilterFlags::EXCLUDE_DYNAMIC
                                    | QueryFilterFlags::EXCLUDE_KINEMATIC,
                                ..default()
                            }
                            .exclude_sensors()
                            .predicate(&predicate);
                            let rotation = q * if direction < 0. {
                                Quat::from_rotation_y(std::f32::consts::PI)
                            } else {
                                Quat::IDENTITY
                            };
                            if ctx
                                .cast_shape(
                                    previous,
                                    rotation,
                                    delta.normalize_or_zero(),
                                    shape.raw.as_ref(),
                                    ShapeCastOptions {
                                        max_time_of_impact: delta.length(),
                                        ..default()
                                    },
                                    filter,
                                )
                                .is_some()
                            {
                                return false;
                            }
                            previous = next;
                        }
                        true
                    };
                    let depart = swept_clear(false);
                    let merge = swept_clear(true);
                    yielding.pass_geometry_clear = depart;
                    yielding.merge_geometry_clear = merge;
                }
                let permitted = gap.is_none()
                    && yielding.pass_geometry_clear
                    && (future.0 - future.2 - home + divider).abs() < 0.5;
                let clearance = rr_core::civilian::Clearance {
                    depart: permitted,
                    merge: yielding.merge_geometry_clear && gap.is_none_or(|g| g >= 5.),
                    forward_distance: if track.definition.closed {
                        f32::INFINITY
                    } else if direction > 0. {
                        track.length - car.progress
                    } else {
                        car.progress
                    },
                };
                let plan = yielding.passing.step(
                    fleet[index],
                    peers,
                    opposite - divider,
                    clearance,
                    desired,
                    dt,
                );
                desired = desired.min(plan.speed);
                fleet[index].reserved = yielding.passing.reserved();
                if let Some(started) = planner_started {
                    planner_elapsed += started.elapsed();
                }
            }
            if let Some(m) = &track.metropolis
                && !yielding.passing.reserved()
            {
                let lead = cars
                    .iter()
                    .filter(|c| {
                        c.4 == direction
                            && track.same_corridor(car.route, car.progress, c.0, c.1)
                            && c.6 != entity
                            && rr_core::streets::shares_lane_ahead(
                                &m.street_life.lane_sections,
                                car.route,
                                car.progress,
                                direction > 0.,
                                yielding.lane_slot,
                                c.5,
                            )
                    })
                    .filter_map(|c| {
                        let gap = (c.1 - car.progress) * direction;
                        // Strict progress order with exact-station identity ties keeps
                        // compressed/merging queues free of circular waits.
                        rr_core::streets::leader_gap(gap, c.6 < entity).map(|gap| (gap, c.3))
                    })
                    .filter(|c| c.0 < 100.)
                    .min_by(|a, b| a.0.total_cmp(&b.0));
                desired = desired.min(rr_core::streets::following_speed(cruise, lead));
            }
            if track.is_hills() {
                for lead in cars.iter().filter(|c| {
                    !yielding.passing.reserved()
                        && c.6 != entity
                        && c.4 == direction
                        && track.same_corridor(car.route, car.progress, c.0, c.1)
                        && (c.2 - car.lane).abs() < 2.5
                }) {
                    let ahead = (lead.1 - car.progress) * direction;
                    if ahead > 0. && ahead < 100. {
                        desired = desired.min(rr_core::traffic::following_speed(
                            cruise, ahead, front, lead.8, lead.3,
                        ));
                    }
                }
            }
            let speed = car.speed.abs();
            car.speed = direction
                * if desired < speed {
                    (speed - 6.0 * dt).max(desired)
                } else {
                    (speed + 2.0 * dt).min(desired)
                };
            yielding.honk_remaining = (yielding.honk_remaining - dt).max(0.0);
            if gap.is_some_and(|gap| gap < 30.0) && yielding.honk_remaining <= 0.0 {
                audio.write(AudioCue::new(Cue::Honk, transform.translation));
                yielding.honk_remaining = 3.0;
            }
            car.progress += car.speed * dt;
            if !track.definition.closed {
                if car.progress > track.length + 80.0 {
                    car.progress = if yielding.city_only { 3250. } else { -80. };
                    yielding.passing = default();
                }
                if car.progress < if yielding.city_only { 3250. } else { -80. } {
                    car.progress = track.length + 80.0;
                    yielding.passing = default();
                }
            }
        }
        let ratio = *yielding
            .lane_ratio
            .get_or_insert(car.lane / track.half_width(car.progress));
        let (home, opposite, _) = civilian_lanes(
            &track,
            car.route,
            car.progress,
            cruise.signum(),
            yielding.lane_slot,
            ratio,
        );
        car.lane = home + (opposite - home) * yielding.passing.blend;
        let (p, q) = track.street_sample(car.route, car.progress, car.lane);
        let offset = track.street_offset(car.route, car.progress);
        let placed_lane = track.lane_at(car.route, car.progress, p) - offset;
        yielding.lateral_speed = yielding.lateral_sample.map_or(0., |(station, lane)| {
            if (car.progress - station).abs() < 5. && dt > 0. {
                (placed_lane - lane) / dt
            } else {
                0.
            }
        });
        yielding.lateral_sample = Some((car.progress, placed_lane));
        let destination = match yielding.passing.phase {
            rr_core::civilian::Phase::Pass => opposite,
            rr_core::civilian::Phase::Merge => home,
            rr_core::civilian::Phase::Follow | rr_core::civilian::Phase::Abort => car.lane,
        };
        let (goal, _) = track.street_sample(car.route, car.progress, destination);
        yielding.lateral_goal = Some(track.lane_at(car.route, car.progress, goal) - offset);
        if track.metropolis.is_some() {
            car.lane = track.lane_at(car.route, car.progress, p);
        }
        if track.metropolis.is_some() {
            let nearest = interest
                .iter()
                .map(|r| r.distance_squared(p.xz()))
                .fold(f32::INFINITY, f32::min)
                .sqrt();
            let enabled = rr_core::streets::collision_active(nearest, disabled.is_none());
            if enabled && disabled.is_some() {
                commands.entity(entity).remove::<ColliderDisabled>();
            } else if !enabled && disabled.is_none() {
                commands.entity(entity).insert(ColliderDisabled);
            }
        }
        transform.translation = p;
        transform.rotation = q * if cruise < 0.0 {
            Quat::from_rotation_y(std::f32::consts::PI)
        } else {
            Quat::IDENTITY
        };
        if old_position.distance(p) > 5. {
            crate::environment::teleport_kinematic(&mut commands, entity, *transform);
        }
    }
    if active && let (Some(started), Some(timings)) = (traffic_started, civilian_timings.as_mut()) {
        timings.record(
            planner_elapsed.as_secs_f64() * 1000.,
            started.elapsed().as_secs_f64() * 1000.,
        );
    }
}

/// One racer-level completion path for bike and on-foot gate crossings.
pub(crate) fn finish_rider(
    rider: &mut Rider,
    race: &mut Race,
    profile: &mut Profile,
    persist: bool,
    place: usize,
    on_foot: bool,
) -> bool {
    if rider.police
        || (race.arrested && !rider.networked)
        || rider.finished.is_some()
        || rider.gates.completed_laps < LAPS
    {
        return false;
    }
    rider.finished = Some(race.elapsed);
    rider.finished_on_foot = on_foot;
    if rider.id == 0 {
        race.place = place;
    }
    if rider.id == 0 && !rider.networked {
        race.place = place;
        race.earned = rr_core::race_award(place, race.takedowns);
        profile.races += 1;
        profile.wins += u32::from(place == 1);
        profile.credits += race.earned;
        profile.best_seconds = Some(
            profile
                .best_seconds
                .map_or(race.elapsed, |b| b.min(race.elapsed)),
        );
        if persist {
            profile.save();
        }
        race.phase = Phase::Finished;
    }
    true
}

pub fn wrapped_gap(gap: f32, length: f32) -> f32 {
    (gap + length * 0.5).rem_euclid(length) - length * 0.5
}

pub fn debris_cleanup(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    mut debris: Query<(Entity, &mut Debris)>,
) {
    if race.phase == Phase::Paused {
        return;
    }
    for (e, mut life) in &mut debris {
        life.0 -= time.delta_secs();
        if life.0 <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn camera(
    time: Res<Time>,
    race: Res<Race>,
    track: Res<Track>,
    riders: Query<(Entity, &Rider, &Recovery)>,
    fixed: Res<Time<Fixed>>,
    preferences: Res<crate::settings::Preferences>,
    controls: Res<Controls>,
    bodies: Query<(&Transform, Option<&crate::runtime::PoseHistory>), Without<ChaseCamera>>,
    context: ReadRapierContext,
    owned: Query<&crate::recovery::OwnedBody>,
    mut recovery_heading: Local<Option<Quat>>,
    actors: Query<(), With<crate::npc::Npc>>,
    mut cameras: Query<(&mut Transform, &mut Projection), With<ChaseCamera>>,
    online: Option<Res<crate::online::NetState>>,
    network_poses: Query<&crate::online::replication::NetworkPose>,
) {
    let Some((player_entity, player, recovery)) = riders
        .iter()
        .find(|(_, r, _)| r.id == online.as_ref().map_or(0, |s| s.local_slot))
    else {
        return;
    };
    if race.phase == Phase::Paused {
        return;
    }
    let (road, rotation) = track.sample_route(player.route, player.progress, player.lane);
    let recovering = recovery.stage != RecoveryStage::Riding;
    let rotation = if !recovering && race.phase != Phase::Garage {
        bodies.get(player_entity).map_or(rotation, |(t, h)| {
            let pose = network_poses.get(player_entity).map_or_else(
                |_| crate::runtime::interpolated(t, h, fixed.overstep_fraction()),
                |n| n.shown(),
            );
            let heading = pose.rotation * Vec3::Z;
            Quat::from_rotation_y(heading.x.atan2(heading.z))
        })
    } else {
        rotation
    };
    let rotation = if recovering {
        *recovery_heading.get_or_insert(rotation)
    } else {
        *recovery_heading = None;
        rotation
    };
    let focus = if recovering {
        recovery
            .torso
            .or(recovery.runner)
            .and_then(|e| bodies.get(e).ok())
            .map_or(road + Vec3::Y, |(t, history)| {
                crate::runtime::interpolated(t, history, fixed.overstep_fraction()).translation
                    + Vec3::Y * 0.3
            })
    } else {
        bodies.get(player_entity).map_or(road, |(t, history)| {
            crate::runtime::interpolated(t, history, fixed.overstep_fraction()).translation
        }) + Vec3::Y * 1.2
    };
    let (offset, look) = if recovering {
        (Vec3::new(0.0, 3.0, -6.0), 0.0)
    } else {
        match race.camera_mode {
            1 => (Vec3::new(0.0, 1.1, -3.4), 2.0),
            2 => (Vec3::new(0.0, 2.0, -6.0), 4.0),
            _ => (Vec3::new(0.0, 1.5, -4.6), 3.0),
        }
    };
    let dt = time.delta_secs().min(0.05);
    let comfort = &preferences.file.comfort;
    let follow = if comfort.anti_nausea {
        comfort.smoothing.min(7.0)
    } else {
        comfort.smoothing
    };
    let view_rotation = rotation
        * Quat::from_rotation_y(-controls.look.x * 1.25)
        * Quat::from_rotation_x(controls.look.y * 0.35);
    let anticipation = if recovering || comfort.anti_nausea {
        Vec3::ZERO
    } else {
        rotation * Vec3::NEG_X * player.drive_intent.steer * (player.speed * 0.025).min(1.2)
    };
    for (mut transform, mut projection) in &mut cameras {
        let shake = Vec3::new(
            (time.elapsed_secs() * 81.0).sin(),
            (time.elapsed_secs() * 63.0).cos(),
            0.0,
        ) * race.shake
            * 0.15
            * if comfort.anti_nausea {
                0.0
            } else {
                comfort.shake
            };
        let target = if race.phase == Phase::Garage {
            road + rotation * Vec3::new(5.0, 3.0, -6.0)
        } else {
            focus + view_rotation * offset + anticipation + shake
        };
        // Retry/scene warps must not fly the camera across the intervening world.
        let snapped = race.phase == Phase::Countdown
            || transform.translation.distance_squared(target) > 75.0 * 75.0;
        let blend = if snapped {
            1.0
        } else {
            1.0 - (-dt * follow).exp()
        };
        let riding_view = !recovering && race.phase != Phase::Garage;
        let lead = if riding_view {
            player.motion_velocity / follow
        } else {
            Vec3::ZERO
        };
        let mut position = transform.translation.lerp(target + lead, blend);
        if riding_view {
            // Follow lag must never become an unrequested speed-dependent zoom-out.
            position = focus + (position - focus).clamp_length_max(offset.length() + 0.35);
        }
        // Prevent the camera itself from passing through solid scenery.
        if let Ok(context) = context.single() {
            let delta = position - focus;
            let predicate = |e, _: &bevy_rapier3d::rapier::geometry::Collider| {
                riders.get(e).is_err() && owned.get(e).is_err() && !actors.contains(e)
            };
            let filter = QueryFilter::from(
                QueryFilterFlags::EXCLUDE_DYNAMIC | QueryFilterFlags::EXCLUDE_KINEMATIC,
            )
            .exclude_rigid_body(player_entity)
            .exclude_sensors()
            .predicate(&predicate);
            let proxy = Collider::ball(0.3);
            let mut best = focus;
            let mut best_distance = 0.0;
            // Keep the chase direction under low ceilings rather than collapsing onto the head.
            for (pivot, boom) in [
                (focus, delta),
                (focus, delta.with_y(0.0)),
                (focus - Vec3::Y * 0.5, delta.with_y(0.0)),
            ] {
                let direction = boom.normalize_or_zero();
                let hit = context.cast_shape(
                    pivot,
                    Quat::IDENTITY,
                    direction,
                    proxy.raw.as_ref(),
                    ShapeCastOptions {
                        max_time_of_impact: boom.length(),
                        target_distance: 0.02,
                        ..default()
                    },
                    filter,
                );
                let length = hit.map_or(boom.length(), |(_, h)| (h.time_of_impact - 0.05).max(0.0));
                let candidate = pivot + direction * length;
                let distance = (candidate - focus).xz().length();
                if distance > best_distance {
                    best = candidate;
                    best_distance = distance;
                }
                if length >= boom.length() - 0.01 {
                    break;
                }
            }
            position = best;
        }
        transform.translation = position;
        let look_target = focus + rotation * Vec3::Z * look + anticipation;
        let looking = if (look_target - position).xz().length_squared() < 0.0025 {
            transform.rotation
        } else {
            Transform::from_translation(position)
                .looking_at(look_target, Vec3::Y)
                .rotation
        };
        transform.rotation = transform
            .rotation
            .slerp(looking, 1.0 - (-dt * follow).exp());
        if let Projection::Perspective(p) = projection.as_mut() {
            p.fov = comfort.fov(player.speed, recovering);
        }
    }
}

/// Avoid a rival's helmet filling the chase camera on a packed grid.
/// This only hides the mesh; combat and collision rules remain active.
const CAMERA_OCCLUSION_RADIUS_M: f32 = 1.4;
fn rival_obscures_camera(camera: Vec3, rival: Vec3) -> bool {
    camera.distance(rival + Vec3::Y * 1.3) < CAMERA_OCCLUSION_RADIUS_M
}
pub fn camera_occlusion(
    state: Option<Res<crate::online::NetState>>,
    camera: Query<&Transform, With<ChaseCamera>>,
    mut riders: Query<(&Rider, &Transform, &mut Visibility), Without<ChaseCamera>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let local_slot = state.as_ref().map_or(0, |s| s.local_slot);
    for (rider, transform, mut visibility) in &mut riders {
        if rider.id != local_slot
            && rival_obscures_camera(camera.translation, transform.translation)
        {
            *visibility = Visibility::Hidden;
        }
    }
}

// Inspect these explicit routes independently of candidate search order. Rejected
// speculative alternatives must not change next tick's guard or exhaustively
// querying candidates would alter behavior compared with first-safe selection.
fn ai_static_guard_blocked(
    current: f32,
    desired: f32,
    committed: Option<f32>,
    blocked: impl Fn(f32) -> bool,
) -> bool {
    blocked(current) || blocked(desired) || committed.is_some_and(blocked)
}

// Rival bounds include both sides of the road, walkable sidewalk and a
// narrow outer clearance. Static swept shapes decide whether each candidate
// is actually usable; there is no right-side or oncoming-lane permission flag.
pub(crate) fn ai_free_bounds(track: &Track, route: usize, progress: f32) -> (f32, f32) {
    let center = track.street_offset(route, progress);
    let width = (0..=120)
        .step_by(10)
        .map(|ahead| {
            let station = (progress + ahead as f32).min(track.length - 0.01);
            track
                .metropolis
                .as_ref()
                .map_or(track.half_width(station), |m| {
                    rr_core::streets::markings(&m.street_life.lane_sections, route, station).2
                })
        })
        .fold(f32::MAX, f32::min);
    let sidewalk_and_clearance = if track.metropolis.is_some()
        || (track.is_hills() && (-300. ..1000.).contains(&progress))
    {
        2.7
    } else if let Some(city) = &track.city_surface {
        city.sidewalk_width_m + 0.3
    } else {
        1.5
    };
    let half = width + sidewalk_and_clearance - 0.65;
    (center - half, center + half)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::track::ROAD_HALF;
    #[test]
    fn following_riders_remain_visible_outside_the_camera_contact_zone() {
        let camera = Vec3::new(0.0, 2.0, -6.0);
        assert!(rival_obscures_camera(camera, Vec3::new(0.0, 0.7, -5.0)));
        assert!(!rival_obscures_camera(camera, Vec3::new(0.0, 0.7, -3.8)));
        assert!(!rival_obscures_camera(camera, Vec3::new(2.0, 0.7, -5.0)));
    }
    #[test]
    fn static_guard_tracks_requested_or_committed_lines_only() {
        assert!(ai_static_guard_blocked(2., -3., None, |x| x == 2.));
        assert!(ai_static_guard_blocked(2., -3., None, |x| x == -3.));
        assert!(ai_static_guard_blocked(2., -3., Some(-4.), |x| x == -4.));
        assert!(!ai_static_guard_blocked(2., -3., Some(-4.), |x| x == 4.8));
    }
    #[test]
    fn metropolis_free_corridor_never_erases_current_road() {
        let t = Track::metropolis().unwrap();
        let mut failed = vec![];
        for route in 0..2 {
            for station in (500..11500).step_by(5) {
                let bounds = ai_free_bounds(&t, route, station as f32);
                if bounds.0 > bounds.1 {
                    failed.push((route, station, bounds));
                }
            }
        }
        assert!(
            failed.is_empty(),
            "{} erased envelopes, first {:?}",
            failed.len(),
            &failed[..failed.len().min(15)]
        );
    }
    #[test]
    fn police_still_only_target_the_wanted_player_after_incident_coin() {
        let mut officer = Rider::new(crate::police::OFFICER_ID);
        for _ in 0..4 {
            officer.combat_mind.struck_by(0);
            assert!(officer.can_fight(0));
            assert!(!officer.can_fight(1));
        }
    }
    #[test]
    fn rival_bounds_include_both_sides_and_clear_sidewalk_band() {
        let track = Track::metropolis().unwrap();
        for station in [1600., 3040., 7200.] {
            let (left, right) = ai_free_bounds(&track, 0, station);
            let center = track.street_offset(0, station);
            let (_, _, pavement) = rr_core::streets::markings(
                &track.metropolis.as_ref().unwrap().street_life.lane_sections,
                0,
                station,
            );
            assert!(left < center - pavement - 1.);
            assert!(right > center + pavement + 1.);
        }
    }
    #[test]
    fn fleeing_rider_near_edge_cannot_plan_beyond_free_corridor() {
        let mut app = simulation();
        let mut definition = app.world().resource::<Track>().definition.clone();
        for point in &mut definition.control_points {
            point.half_width = 3.;
        }
        app.insert_resource(Track::from_definition(definition).unwrap());
        let player = app
            .world_mut()
            .query::<(Entity, &Rider)>()
            .iter(app.world())
            .find(|(_, r)| r.id == 0)
            .unwrap()
            .0;
        let (p, q) = app.world().resource::<Track>().sample(20., 2.5);
        {
            let mut rider = app.world_mut().get_mut::<Rider>(player).unwrap();
            rider.progress = 20.;
            rider.lane = 2.5;
        }
        app.world_mut()
            .entity_mut(player)
            .insert(Transform::from_translation(p).with_rotation(q));
        let mut rival = Rider::new(1);
        rival.progress = 20.;
        rival.lane = 3.5;
        rival.speed = 20.;
        while rival.combat_mind.struck_by(0) != rr_core::rival_ai::Response::Flee {}
        let (p, q) = app.world().resource::<Track>().sample(20., 3.5);
        let entity = app
            .world_mut()
            .spawn((
                rival,
                Transform::from_translation(p).with_rotation(q),
                Visibility::Visible,
                recovery::bike_body(),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Race>().elapsed = 10.;
        step(&mut app, 3);
        let rider = app.world().get::<Rider>(entity).unwrap();
        let bounds = ai_free_bounds(app.world().resource::<Track>(), rider.route, rider.progress);
        assert_eq!(
            rider.combat_mind.decision,
            rr_core::rival_ai::Decision::Flee
        );
        assert!(rider.ai_plan.unwrap().lane <= bounds.1 + 0.001);
    }
    #[test]
    fn metropolis_rider_can_take_an_empty_opposite_racing_line() {
        let mut app = simulation();
        app.insert_resource(Track::metropolis().unwrap());
        app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Assisted);
        let start = 1600.;
        let lane = {
            let track = app.world().resource::<Track>();
            let (left, right) = ai_free_bounds(track, 0, start);
            (left + right) * 0.5
        };
        let mut rider = Rider::new(4); // Its free racing line is across the centre.
        rider.progress = start;
        rider.lane = lane;
        rider.speed = 25.;
        let (position, rotation) = app.world().resource::<Track>().sample(start, lane);
        let entity = app
            .world_mut()
            .spawn((
                rider,
                Transform::from_translation(position).with_rotation(rotation),
                Visibility::Visible,
                recovery::bike_body(),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Race>().elapsed = 10.;
        step(&mut app, 5);
        let planned = app.world().get::<Rider>(entity).unwrap().ai_plan.unwrap();
        let track = app.world().resource::<Track>();
        let (divider, _, _) = rr_core::streets::markings(
            &track.metropolis.as_ref().unwrap().street_life.lane_sections,
            0,
            start,
        );
        assert!(
            planned.lane > track.street_offset(0, start) + divider,
            "rider forced home: {planned:?}"
        );
        step(&mut app, 240);
        let racer = app.world().get::<Rider>(entity).unwrap();
        let track = app.world().resource::<Track>();
        let (divider, _, _) = rr_core::streets::markings(
            &track.metropolis.as_ref().unwrap().street_life.lane_sections,
            0,
            racer.progress,
        );
        let boundary = track.street_offset(0, racer.progress) + divider;
        assert!(
            racer.lane > boundary,
            "rider stayed on the outbound side: lane={} progress={} divider={boundary}",
            racer.lane,
            racer.progress,
        );
    }
    #[test]
    fn metropolis_clear_street_rider_leaves_stall_and_passes_stopped_leader() {
        for start in [2940., 3250.] {
            let mut app = simulation();
            app.insert_resource(Track::metropolis().unwrap());
            app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Assisted);
            // Those exact stations previously had inverted look-ahead bounds.
            let lane = {
                let track = app.world().resource::<Track>();
                let (a, b) = ai_free_bounds(track, 0, start);
                (a + b) * 0.5
            };
            let mut rider = Rider::new(1);
            rider.progress = start;
            rider.lane = lane;
            let (p, q) = app.world().resource::<Track>().sample(start, lane);
            let owner = app
                .world_mut()
                .spawn((
                    rider,
                    Transform::from_translation(p).with_rotation(q),
                    Visibility::Visible,
                    recovery::bike_body(),
                ))
                .id();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut().resource_mut::<Race>().elapsed = 10.;
            step(&mut app, 180);
            let r = app.world().get::<Rider>(owner).unwrap();
            assert!(
                r.progress > start + 8.,
                "stalled at {start}: {}/{}",
                r.progress,
                r.speed
            );
        }
        let mut app = simulation();
        app.insert_resource(Track::metropolis().unwrap());
        app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Assisted);
        // Reproduce the reported queue window near station 3,040.
        let start = 3040.0;
        let lane = app.world().resource::<Track>().street_offset(0, start) + 1.2;
        let mut playerq = app.world_mut().query::<(Entity, &Rider)>();
        let leader = playerq
            .iter(app.world())
            .find(|(_, r)| r.id == 0)
            .unwrap()
            .0;
        let offset = app.world().resource::<Track>().street_offset(0, start);
        let leaderlane = lane
            + app
                .world()
                .resource::<Track>()
                .street_offset(0, start + 18.)
            - offset;
        let (p, q) = app
            .world()
            .resource::<Track>()
            .sample(start + 18., leaderlane);
        {
            let mut r = app.world_mut().get_mut::<Rider>(leader).unwrap();
            r.progress = start + 18.;
            r.lane = leaderlane;
        }
        app.world_mut()
            .entity_mut(leader)
            .insert(Transform::from_translation(p).with_rotation(q));
        let mut rider = Rider::new(1);
        rider.progress = start;
        rider.lane = lane;
        rider.speed = 2.;
        let (p, q) = app.world().resource::<Track>().sample(start, lane);
        let owner = app
            .world_mut()
            .spawn((
                rider,
                Transform::from_translation(p).with_rotation(q),
                Visibility::Visible,
                recovery::bike_body(),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().resource_mut::<Race>().elapsed = 10.;
        let mut sidegap = 0_f32;
        for _ in 0..900 {
            app.update();
            let track = app.world().resource::<Track>();
            let r = app.world().get::<Rider>(owner).unwrap();
            sidegap =
                sidegap.max((r.lane - track.street_offset(0, r.progress) - (lane - offset)).abs());
            assert_eq!(r.health, 100.);
        }
        let r = app.world().get::<Rider>(owner).unwrap();
        assert!(
            r.progress > start + 35.,
            "never passed stopped leader: {start}/{}",
            r.progress
        );
        assert!(sidegap > 1.7, "no lateral pass: {sidegap}");
    }
    #[test]
    fn traffic_brakes_and_rate_limits_horns_then_resumes_when_rider_clears() {
        #[derive(Resource, Default)]
        struct Honks(u32);
        let mut app = simulation();
        app.init_resource::<Honks>().add_systems(
            FixedUpdate,
            (|mut cues: MessageReader<AudioCue>, mut count: ResMut<Honks>| {
                count.0 += cues.read().filter(|c| matches!(c.kind, Cue::Honk)).count() as u32;
            })
            .after(crate::runtime::SimSet::Gameplay),
        );
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        let (p, q) = app.world().resource::<Track>().sample(500.0, 2.0);
        *app.world_mut().get_mut::<Transform>(owner).unwrap() =
            Transform::from_translation(p).with_rotation(q);
        {
            let mut r = app.world_mut().get_mut::<Rider>(owner).unwrap();
            r.progress = 500.0;
            r.lane = 2.0;
        }
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stage = RecoveryStage::Falling;
        let car = app
            .world_mut()
            .spawn((
                Traffic {
                    route: 0,
                    progress: 450.0,
                    lane: 2.0,
                    speed: 18.0,
                },
                Transform::default(),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for _ in 0..600 {
            app.update();
        }
        let stopped = app.world().get::<Traffic>(car).unwrap();
        assert!(
            stopped.progress < 495.1 && stopped.progress > 493.0,
            "{}",
            stopped.progress
        );
        assert!(stopped.speed < 0.1);
        assert!((2..=4).contains(&app.world().resource::<Honks>().0));
        app.world_mut().get_mut::<Recovery>(owner).unwrap().stage = RecoveryStage::Riding;
        for _ in 0..120 {
            app.update();
        }
        assert!(app.world().get::<Traffic>(car).unwrap().speed > 3.5);
    }
    #[test]
    fn collision_detachment_uses_forward_pre_solver_velocity_not_the_bounce() {
        #[derive(Resource, Default)]
        struct Detached(Vec<Vec3>);
        let mut app = simulation();
        step(&mut app, 3);
        let owner = {
            let w = app.world_mut();
            let mut q = w.query::<(Entity, &mut Rider)>();
            let (e, mut r) = q.single_mut(w).unwrap();
            r.immunity = 0.0;
            r.motion_velocity = Vec3::NEG_Z * 12.0;
            r.speed = 12.0;
            e
        };
        let origin = app.world().get::<Transform>(owner).unwrap().translation;
        let before = Vec3::Z * (100.0 / 3.6);
        app.world_mut()
            .entity_mut(owner)
            .insert(crate::vehicle::BikeState {
                impact_velocity: before,
                ..default()
            });
        let car = app
            .world_mut()
            .spawn((
                Traffic {
                    route: 0,
                    progress: 300.0,
                    lane: 0.0,
                    speed: 60.0 / 3.6,
                },
                Transform::from_translation(origin + Vec3::Z * 10.),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.init_resource::<Detached>().add_systems(
            FixedUpdate,
            (|mut snapshot: ResMut<Detached>, bodies: Query<(&recovery::OwnedBody, &Velocity)>| {
                if snapshot.0.is_empty() {
                    snapshot.0.extend(bodies.iter().map(|(_, v)| v.linear));
                }
            })
            .after(crate::runtime::SimSet::Gameplay)
            .before(PhysicsSet::SyncBackend),
        );
        app.world_mut()
            .write_message(recovery::SweptContact(owner, car));
        step(&mut app, 1);
        let samples = &app.world().resource::<Detached>().0;
        assert_eq!(samples.len(), 12);
        for v in samples {
            assert!(
                v.distance(before * 0.4) < 0.001,
                "knockoff inherited reversed solver velocity: {v:?}"
            );
        }
    }
    #[test]
    fn retry_teleports_dynamic_body_and_history_even_on_a_zero_tick_frame() {
        use crate::runtime::PoseHistory;
        use crate::vehicle::{BikeState, VehiclePhysicsAdapter};
        for (phase, zero_tick) in [Phase::Racing, Phase::Paused, Phase::Finished]
            .into_iter()
            .flat_map(|p| [(p, true), (p, false)])
        {
            let mut app = simulation();
            step(&mut app, 3);
            *app.world_mut().resource_mut::<VehiclePhysicsAdapter>() =
                VehiclePhysicsAdapter::Dynamic;
            step(&mut app, 1);
            let far = app.world().resource::<Track>().sample(3200.0, 1.0);
            let pose = Transform::from_translation(far.0).with_rotation(far.1);
            let owner = {
                let w = app.world_mut();
                let mut q = w.query::<(Entity, &mut Rider)>();
                let (e, mut r) = q.single_mut(w).unwrap();
                r.progress = 3200.0;
                r.lane = 1.0;
                r.speed = 30.0;
                e
            };
            let child = app.world_mut().spawn(Transform::default()).id();
            app.world_mut().entity_mut(owner).add_child(child);
            step(&mut app, 2);
            app.world_mut().entity_mut(owner).insert((
                pose,
                GlobalTransform::from(pose),
                Velocity::linear(Vec3::Z * 30.0),
                PoseHistory {
                    previous: pose,
                    current: pose,
                },
                BikeState {
                    position: far.0,
                    velocity: Vec3::Z * 30.0,
                    lean: 0.5,
                    was_riding: true,
                    ..default()
                },
            ));
            app.world_mut().resource_mut::<Race>().phase = phase;
            app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                if zero_tick {
                    std::time::Duration::ZERO
                } else {
                    std::time::Duration::from_secs_f64(1.0 / 60.0)
                },
            ));
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyR);
            app.update();
            let spawn = &app.world().resource::<Track>().definition.spawns[0];
            let (start, _) = app
                .world()
                .resource::<Track>()
                .sample(spawn.distance, spawn.lane);
            assert!(
                app.world()
                    .get::<Transform>(owner)
                    .unwrap()
                    .translation
                    .distance(start)
                    < 0.001,
                "retry left body halfway along the course"
            );
            assert!(
                app.world()
                    .get::<GlobalTransform>(owner)
                    .unwrap()
                    .translation()
                    .distance(start)
                    < 0.001
            );
            let backend_position = {
                let w = app.world_mut();
                let mut query = w.query::<&RapierRigidBodySet>();
                query
                    .iter(w)
                    .find_map(|set| {
                        set.entity2body()
                            .get(&owner)
                            .and_then(|h| set.bodies.get(*h))
                            .map(|body| body.translation())
                    })
                    .unwrap()
            };
            assert!(
                backend_position.distance(start) < 0.001,
                "backend retry pose was stale: {backend_position:?}"
            );
            let h = app.world().get::<PoseHistory>(owner).unwrap();
            assert_eq!(h.previous.translation, start);
            assert_eq!(h.current.translation, start);
            assert!(
                app.world().get::<Velocity>(owner).unwrap().linear.length() < 0.01,
                "retry retained travelling velocity"
            );
            assert!(!app.world().get::<BikeState>(owner).unwrap().was_riding);
            app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            ));
            step(&mut app, 190);
            assert!(
                player(&mut app).progress < 10.0,
                "first physics tick restored old progress"
            );
        }
    }
    #[test]
    fn unseen_rear_crash_skips_bodies_waits_five_seconds_and_reset_cancels_it() {
        let mut app = simulation();
        let (front, fq, back, bq) = {
            let track = app.world().resource::<Track>();
            let (p, q) = track.sample(1000.0, 0.0);
            let (b, bq) = track.sample(900.0, 0.0);
            (p, q, b, bq)
        };
        let player = app
            .world_mut()
            .query_filtered::<Entity, With<Rider>>()
            .single(app.world())
            .unwrap();
        {
            let mut r = app.world_mut().get_mut::<Rider>(player).unwrap();
            r.progress = 1000.0;
            r.immunity = 1000.0;
        }
        *app.world_mut().get_mut::<Transform>(player).unwrap() =
            Transform::from_translation(front).with_rotation(fq);
        app.world_mut().spawn((
            Camera3d::default(),
            Transform::from_translation(front - fq * Vec3::Z * 6.0 + Vec3::Y * 3.0)
                .looking_at(front + fq * Vec3::Z * 30.0, Vec3::Y),
        ));
        let mut rider = Rider::new(1);
        rider.progress = 900.0;
        rider.lane = 0.0;
        rider.health = 0.0;
        let enemy = app
            .world_mut()
            .spawn((
                rider,
                Transform::from_translation(back).with_rotation(bq),
                Visibility::Visible,
                recovery::bike_body(),
            ))
            .id();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        step(&mut app, 1);
        assert!(
            app.world()
                .get::<Recovery>(enemy)
                .unwrap()
                .deferred
                .is_some()
        );
        assert!(app.world().get::<Recovery>(enemy).unwrap().parts.is_empty());
        assert!(app.world().get::<RigidBodyDisabled>(enemy).is_some());
        assert_eq!(
            app.world_mut()
                .query::<&recovery::OwnedBody>()
                .iter(app.world())
                .filter(|b| b.owner == enemy)
                .count(),
            0
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        step(&mut app, 60);
        assert_eq!(
            app.world()
                .get::<Recovery>(enemy)
                .unwrap()
                .deferred
                .unwrap()
                .ticks,
            300
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        step(&mut app, 299);
        assert!(
            app.world()
                .get::<Recovery>(enemy)
                .unwrap()
                .deferred
                .is_some()
        );
        step(&mut app, 1);
        assert!(
            app.world()
                .get::<Recovery>(enemy)
                .unwrap()
                .deferred
                .is_none()
        );
        assert!(app.world().get::<RigidBodyDisabled>(enemy).is_none());
        let r = app.world().get::<Rider>(enemy).unwrap();
        assert!((r.progress - 900.0).abs() < 0.2 && r.lane.abs() < 0.1 && r.health == 100.0);
        app.world_mut().get_mut::<Rider>(enemy).unwrap().health = 0.0;
        step(&mut app, 1);
        assert!(
            app.world()
                .get::<Recovery>(enemy)
                .unwrap()
                .deferred
                .is_some()
        );
        app.world_mut()
            .resource_mut::<crate::controls::Controls>()
            .restart = true;
        step(&mut app, 1);
        assert!(
            app.world()
                .get::<Recovery>(enemy)
                .unwrap()
                .deferred
                .is_none()
        );
        assert!(app.world().get::<RigidBodyDisabled>(enemy).is_none());
    }
    #[test]
    fn collision_gap_wraps_at_finish() {
        assert_eq!(wrapped_gap(998.0, 1000.0), -2.0);
        assert_eq!(wrapped_gap(-998.0, 1000.0), 2.0);
        assert_eq!(wrapped_gap(12.0, 1000.0), 12.0);
    }
    #[test]
    fn grid_starts_before_timing_line() {
        for id in 0..NAMES.len() {
            let r = Rider::new(id);
            assert!(r.progress < 0.0);
            assert!(r.lane.abs() < ROAD_HALF);
        }
    }
    #[test]
    fn predictive_ai_field_queues_without_car_or_rear_end_impacts() {
        let mut outcomes = Vec::new();
        for straight_control in [true, false] {
            let mut app = simulation();
            let mut definition = app.world().resource::<Track>().definition.clone();
            for point in &mut definition.control_points {
                point.half_width = 3.0;
            }
            app.insert_resource(Track::from_definition(definition).unwrap());
            app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
            let mut owners = Vec::new();
            for id in 1..=4 {
                let mut rider = Rider::new(id);
                rider.progress = 35.0 - id as f32 * 15.0;
                rider.lane = 2.0;
                rider.speed = 35.0;
                let (p, q) = app
                    .world()
                    .resource::<Track>()
                    .sample(rider.progress, rider.lane);
                rider.motion_velocity = q * Vec3::Z * rider.speed;
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
            let (p, q) = app.world().resource::<Track>().sample(110.0, 2.0);
            app.world_mut().spawn((
                Traffic {
                    route: 0,
                    progress: 110.,
                    lane: 2.,
                    speed: 0.,
                },
                Transform::from_translation(p).with_rotation(q),
                RigidBody::KinematicPositionBased,
                Collider::compound(vec![(
                    Vec3::Y * 0.95,
                    Quat::IDENTITY,
                    Collider::cuboid(0.95, 0.9, 1.95),
                )]),
            ));
            // Both directions are sealed in this queue-yield control. A separate
            // physical test verifies borrowing an actually open opposite lane.
            let (p, q) = app.world().resource::<Track>().sample(110., -2.);
            app.world_mut().spawn((
                Traffic {
                    route: 0,
                    progress: 110.,
                    lane: -2.,
                    speed: 0.,
                },
                Transform::from_translation(p).with_rotation(q),
                RigidBody::KinematicPositionBased,
                Collider::compound(vec![(
                    Vec3::Y * 0.95,
                    Quat::IDENTITY,
                    Collider::cuboid(0.95, 0.9, 1.95),
                )]),
            ));
            if straight_control {
                // Counterfactual traffic-unaware intent only: the same solid contact,
                // chassis, crash and recovery systems remain enabled.
                app.add_systems(
                    FixedUpdate,
                    (|mut riders: Query<&mut Rider>| {
                        for mut rider in &mut riders {
                            if rider.id != 0 {
                                rider.drive_intent = rr_core::VehicleIntent {
                                    throttle: true,
                                    throttle_amount: 1.0,
                                    ..default()
                                };
                            }
                        }
                    })
                    .after(drive)
                    .before(crate::vehicle::forces),
                );
            }
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut().resource_mut::<Race>().elapsed = 10.0;
            let mut struck = [false; 4];
            let mut nearest_pair = f32::MAX;
            for _ in 0..600 {
                app.update();
                for (i, owner) in owners.iter().enumerate() {
                    let r = app.world().get::<Rider>(*owner).unwrap();
                    struck[i] |= r.health < 100.0
                        || app.world().get::<Recovery>(*owner).unwrap().stage
                            != RecoveryStage::Riding;
                    for other in &owners[i + 1..] {
                        let a = app.world().get::<Transform>(*owner).unwrap().translation;
                        let b = app.world().get::<Transform>(*other).unwrap().translation;
                        nearest_pair = nearest_pair.min(a.xz().distance(b.xz()));
                    }
                }
            }
            let impacts = struck.iter().filter(|hit| **hit).count();
            println!(
                "AI field straight_control={straight_control} impacted_riders={impacts} minimum_pair_distance={nearest_pair}"
            );
            if !straight_control {
                assert_eq!(impacts, 0);
                assert!(nearest_pair > 2.2);
                for owner in owners {
                    let r = app.world().get::<Rider>(owner).unwrap();
                    assert!(
                        // The dynamic chassis may still creep while its
                        // zero-speed avoidance plan finishes braking.
                        r.progress < 106.0 && r.speed < 2.0,
                        "sealed queue rider {} reached progress={} lane={} speed={} plan={:?}",
                        r.id,
                        r.progress,
                        r.lane,
                        r.speed,
                        r.ai_plan
                    );
                }
            }
            outcomes.push(impacts);
        }
        assert!(
            outcomes[0] > outcomes[1],
            "traffic-unaware control must demonstrate the obstruction"
        );
    }
    #[test]
    fn predictive_ai_yields_to_moving_crossing_cars_and_resumes() {
        for metropolis_mover in [false, true] {
            let mut app = simulation();
            if metropolis_mover {
                app.insert_resource(Track::metropolis().unwrap());
            } else {
                let mut definition = app.world().resource::<Track>().definition.clone();
                for point in &mut definition.control_points {
                    point.half_width = 3.0;
                }
                app.insert_resource(Track::from_definition(definition).unwrap());
            }
            let lane = if metropolis_mover { -2.0 } else { 2.0 };
            app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
            let (p, q) = app.world().resource::<Track>().sample(20.0, lane);
            if metropolis_mover {
                // Local flat physical support isolates the drive adapter; use the real
                // Metropolis path, offsets, markings and crossing mover above it.
                app.world_mut().spawn((
                    RigidBody::Fixed,
                    world::Ground,
                    Collider::cuboid(650., 0.5, 650.),
                    Transform::from_translation(p - Vec3::Y * 0.6),
                ));
            }
            let mut rider = Rider::new(1);
            rider.progress = 20.0;
            rider.lane = lane;
            rider.speed = 20.0;
            rider.motion_velocity = q * Vec3::Z * 20.0;
            let owner = app
                .world_mut()
                .spawn((
                    rider,
                    Transform::from_translation(p).with_rotation(q),
                    Visibility::Visible,
                    recovery::bike_body(),
                ))
                .id();
            let (junction, q) = app.world().resource::<Track>().sample(65.0, 0.0);
            let axis = q * Vec3::NEG_X;
            let rotation = Quat::from_rotation_y(axis.x.atan2(axis.z));
            let center = junction + axis * (lane - 2.1);
            let car = app
                .world_mut()
                .spawn((
                    crate::environment::CrossTraffic,
                    crate::environment::Patrol {
                        center,
                        axis,
                        extent: 70.,
                        speed: 6.,
                        phase: 1.0 - 11.9 / 70.0 - 10.0 * 6.0 / 70.0,
                        one_way: true,
                    },
                    Transform::from_translation(center - axis * 11.9).with_rotation(rotation),
                    Velocity::linear(axis * 6.0),
                    RigidBody::KinematicPositionBased,
                    Collider::compound(vec![(
                        Vec3::Y * 0.95,
                        Quat::IDENTITY,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    )]),
                    crate::world::Obstacle,
                    ActiveEvents::COLLISION_EVENTS,
                    Ccd::enabled(),
                ))
                .id();
            if metropolis_mover {
                app.world_mut()
                    .entity_mut(car)
                    .insert(crate::world_gen::metropolis::Crossing {
                        distance: -11.9,
                        junction,
                    });
            }
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut().resource_mut::<Race>().elapsed = 10.0;
            let mut braking = false;
            let mut resumed = false;
            let mut minimum = f32::MAX;
            let mut lowest_speed = 20.0_f32;
            for _ in 0..420 {
                app.update();
                let r = app.world().get::<Rider>(owner).unwrap();
                braking |= r.drive_intent.brake_amount > 0.9;
                lowest_speed = lowest_speed.min(r.speed);
                let a = app.world().get::<Transform>(owner).unwrap().translation;
                let b = app.world().get::<Transform>(car).unwrap().translation;
                minimum = minimum.min(a.xz().distance(b.xz()));
                let cleared = (b - junction).dot(axis) > lane + 3.5;
                resumed |=
                    braking && cleared && r.drive_intent.throttle && r.speed > lowest_speed + 3.0;
                assert_eq!(
                    app.world().get::<Recovery>(owner).unwrap().stage,
                    RecoveryStage::Riding,
                    "crossing mover={metropolis_mover} impact at {}",
                    r.progress
                );
                assert_eq!(r.health, 100.0);
            }
            let r = app.world().get::<Rider>(owner).unwrap();
            println!(
                "crossing mover={metropolis_mover} braking={braking} resumed={resumed} lowest_speed={lowest_speed} min_distance={minimum} progress={}",
                r.progress
            );
            assert!(minimum > 2.0 && r.progress > 90.0);
            assert!(!braking || resumed, "braked but did not resume");
        }
    }
    #[test]
    fn predictive_ai_traffic_uses_real_dynamic_braking_and_passing() {
        for (width, car_speed) in [(3.0, 0.0), (3.0, 12.0), (6.5, 0.0)] {
            let mut app = simulation();
            let mut definition = app.world().resource::<Track>().definition.clone();
            for point in &mut definition.control_points {
                point.half_width = width;
            }
            app.insert_resource(Track::from_definition(definition).unwrap());
            app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
            let (p, q) = app.world().resource::<Track>().sample(20.0, 2.0);
            let mut rider = Rider::new(1);
            rider.progress = 20.0;
            rider.lane = 2.0;
            rider.speed = 35.0;
            rider.motion_velocity = q * Vec3::Z * 35.0;
            let owner = app
                .world_mut()
                .spawn((
                    rider,
                    Transform::from_translation(p).with_rotation(q),
                    Visibility::Visible,
                    recovery::bike_body(),
                ))
                .id();
            let (p, q) = app.world().resource::<Track>().sample(110.0, 2.0);
            let car = app
                .world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        progress: 110.0,
                        lane: 2.0,
                        speed: car_speed,
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
            app.world_mut().resource_mut::<Race>().elapsed = 10.0;
            if width < 4. {
                // A sealed opposite lane remains a legitimate yield. Empty
                // opposite road is now separately covered as a safe pass case.
                let (p, q) = app.world().resource::<Track>().sample(110., -2.);
                app.world_mut().spawn((
                    Traffic {
                        route: 0,
                        progress: 110.,
                        lane: -2.,
                        speed: car_speed,
                    },
                    Transform::from_translation(p).with_rotation(q),
                    RigidBody::KinematicPositionBased,
                    Collider::compound(vec![(
                        Vec3::Y * 0.95,
                        Quat::IDENTITY,
                        Collider::cuboid(0.95, 0.9, 1.95),
                    )]),
                ));
            }
            let mut braking = false;
            let mut minimum = f32::MAX;
            for _ in 0..600 {
                app.update();
                let r = app.world().get::<Rider>(owner).unwrap();
                braking |= r.drive_intent.brake_amount > 0.9;
                let a = app.world().get::<Transform>(owner).unwrap().translation;
                let b = app.world().get::<Transform>(car).unwrap().translation;
                minimum = minimum.min(a.xz().distance(b.xz()));
                assert_eq!(
                    app.world().get::<Recovery>(owner).unwrap().stage,
                    RecoveryStage::Riding,
                    "width {width}, speed {car_speed}: crashed at {}",
                    r.progress
                );
                assert_eq!(r.health, 100.0);
            }
            let r = app.world().get::<Rider>(owner).unwrap();
            let c = app.world().get::<Traffic>(car).unwrap();
            println!(
                "AI traffic width={width} car_speed={car_speed} rider_s={} car_s={} rider_speed={} min_distance={minimum} braking={braking}",
                r.progress, c.progress, r.speed
            );
            assert!(minimum > 2.0);
            if width < 4.0 {
                assert!(braking);
                assert!(r.progress < c.progress - 4.0);
                assert!(r.speed < car_speed + 2.0);
            } else {
                assert!(r.progress > c.progress + 10.0, "failed to pass");
            }
        }
    }
    #[test]
    fn crawling_rider_queue_passes_without_a_car_trigger() {
        for car_queue in [false, true] {
            let mut app = simulation();
            let mut definition = app.world().resource::<Track>().definition.clone();
            for point in &mut definition.control_points {
                point.half_width = 6.5;
            }
            app.insert_resource(Track::from_definition(definition).unwrap());
            app.insert_resource(crate::vehicle::VehiclePhysicsAdapter::Dynamic);
            let mut owners = Vec::new();
            for id in 1..=4 {
                let mut rider = Rider::new(id);
                rider.progress = 25. + id as f32 * 12.;
                rider.lane = 2.;
                rider.speed = 2.;
                let (p, q) = app
                    .world()
                    .resource::<Track>()
                    .sample(rider.progress, rider.lane);
                rider.motion_velocity = q * Vec3::Z * rider.speed;
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
            let obstruction = if car_queue {
                let (p, q) = app.world().resource::<Track>().sample(95., 2.);
                app.world_mut()
                    .spawn((
                        Traffic {
                            route: 0,
                            progress: 95.,
                            lane: 2.,
                            speed: 2.,
                        },
                        Transform::from_translation(p).with_rotation(q),
                        RigidBody::KinematicPositionBased,
                        Collider::compound(vec![(
                            Vec3::Y * 0.95,
                            Quat::IDENTITY,
                            Collider::cuboid(0.95, 0.9, 1.95),
                        )]),
                    ))
                    .id()
            } else {
                // A real solid slow rider; only its intent is held at walking speed.
                app.add_systems(
                    FixedUpdate,
                    (|mut riders: Query<&mut Rider>| {
                        for mut r in &mut riders {
                            if r.id == 4 {
                                r.drive_intent = rr_core::VehicleIntent {
                                    throttle: r.speed < 2.,
                                    throttle_amount: 0.2,
                                    brake: r.speed > 2.2,
                                    brake_amount: 0.3,
                                    ..default()
                                };
                            }
                        }
                    })
                    .after(drive)
                    .before(crate::vehicle::forces),
                );
                owners[3]
            };
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut().resource_mut::<Race>().elapsed = 10.;
            let mut nearest = f32::MAX;
            let mut max_lane = 0_f32;
            let mut streak = [0usize; 4];
            let mut sustained = [0usize; 4];
            let mut nearest_obstruction = f32::MAX;
            for _ in 0..1800 {
                app.update();
                for (i, owner) in owners.iter().enumerate() {
                    let r = app.world().get::<Rider>(*owner).unwrap();
                    max_lane = max_lane.max(r.lane.abs());
                    streak[i] = if r.lane.abs() > 3.5 { streak[i] + 1 } else { 0 };
                    sustained[i] = sustained[i].max(streak[i]);
                    if *owner != obstruction {
                        let a = app.world().get::<Transform>(*owner).unwrap().translation;
                        let b = app
                            .world()
                            .get::<Transform>(obstruction)
                            .unwrap()
                            .translation;
                        nearest_obstruction = nearest_obstruction.min(a.xz().distance(b.xz()));
                    }
                    assert_eq!(
                        r.health, 100.,
                        "car_queue={car_queue} id={} progress={}",
                        r.id, r.progress
                    );
                    assert_eq!(
                        app.world().get::<Recovery>(*owner).unwrap().stage,
                        RecoveryStage::Riding
                    );
                    let bounds =
                        ai_free_bounds(app.world().resource::<Track>(), r.route, r.progress);
                    assert!(
                        r.lane > bounds.0 - 0.5 && r.lane < bounds.1 + 0.5,
                        "car_queue={car_queue} id={} outside usable corridor {} {bounds:?}",
                        r.id,
                        r.lane
                    );
                    for other in &owners[i + 1..] {
                        let a = app.world().get::<Transform>(*owner).unwrap().translation;
                        let b = app.world().get::<Transform>(*other).unwrap().translation;
                        nearest = nearest.min(a.xz().distance(b.xz()));
                    }
                }
            }
            let lead = if car_queue {
                app.world().get::<Traffic>(obstruction).unwrap().progress
            } else {
                app.world().get::<Rider>(obstruction).unwrap().progress
            };
            let results: Vec<_> = owners
                .iter()
                .map(|e| {
                    let r = app.world().get::<Rider>(*e).unwrap();
                    (r.id, r.progress, r.speed, r.lane)
                })
                .collect();
            println!(
                "crawling queue car={car_queue} obstruction={lead} nearest={nearest} obstruction_clearance={nearest_obstruction} max_lane={max_lane} sustained_ticks={sustained:?} results={results:?}"
            );
            for owner in &owners[..3] {
                let r = app.world().get::<Rider>(*owner).unwrap();
                assert!(
                    r.progress > lead + 8.,
                    "queue did not drain car={car_queue}: {results:?}"
                );
            }
            // A bike is 0.72m wide; its side-by-side distance need not
            // equal the conservative two-metre car clearance fixture.
            assert!(nearest > 1.2 && nearest_obstruction > if car_queue { 2. } else { 1.2 });
            assert!(max_lane > 4.);
            assert!(
                sustained[..3].iter().all(|ticks| *ticks >= 30),
                "no sustained physical pass: {sustained:?}"
            );
        }
    }
    pub(crate) fn simulation() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            crate::content::RoadContentPlugin,
            bevy::state::app::StatesPlugin,
            crate::runtime::RoadLifecyclePlugin,
            crate::runtime::RoadSimulationPlugin,
            crate::controls::RoadInputPlugin,
        ))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ))
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<crate::sound::Muted>()
        .init_resource::<crate::ui::MenuAction>()
        .insert_resource(Persistence(false))
        .insert_resource(Art {
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
        .add_systems(Last, |mut keys: ResMut<ButtonInput<KeyCode>>| keys.clear());
        app.world_mut().spawn((
            Rider::new(0),
            Transform::default(),
            Visibility::default(),
            recovery::bike_body(),
        ));
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(650.0, 0.5, 650.0),
            world::Ground,
            Transform::from_xyz(0.0, -0.5, 0.0),
        ));
        app.update();
        app
    }
    fn step(app: &mut App, count: usize) {
        for _ in 0..count {
            app.update();
        }
    }
    fn player(app: &mut App) -> &Rider {
        app.world_mut()
            .query::<&Rider>()
            .iter(app.world())
            .find(|r| r.id == 0)
            .unwrap()
    }
    #[test]
    fn countdown_throttle_nitro_and_pause() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
        step(&mut app, 220);
        assert!(app.world().resource::<Race>().phase == Phase::Racing);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        step(&mut app, 60);
        assert!(player(&mut app).speed > 9.0);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ShiftLeft);
        step(&mut app, 60);
        assert_eq!(player(&mut app).nitro, 100.0);
        assert!(!player(&mut app).drive_intent.nitro);
        let progress = player(&mut app).progress;
        let elapsed = app.world().resource::<Race>().elapsed;
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        step(&mut app, 60);
        assert_eq!(player(&mut app).progress, progress);
        assert_eq!(app.world().resource::<Race>().elapsed, elapsed);
    }
    #[test]
    fn crash_spawns_jointed_bodies_then_recovers() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            let mut rider = q.single_mut(app.world_mut()).unwrap();
            rider.health = 0.0;
            rider.speed = 35.0;
        }
        step(&mut app, 1);
        assert!(player(&mut app).crashed > 0.0);
        assert_eq!(
            app.world_mut()
                .query::<&ImpulseJoint>()
                .iter(app.world())
                .count(),
            10
        );
        assert_eq!(
            app.world_mut().query::<&Debris>().iter(app.world()).count(),
            12
        );
        step(&mut app, 120);
        for (t, _) in app
            .world_mut()
            .query::<(&Transform, &Debris)>()
            .iter(app.world())
        {
            assert!(t.translation.is_finite());
            assert!(
                t.translation.y > -0.5,
                "body tunneled below ground: {:?}",
                t.translation
            );
        }
        let mut saw_get_up = false;
        let mut saw_running = false;
        let mut saw_mounting = false;
        for _ in 0..1500 {
            step(&mut app, 1);
            let world = app.world_mut();
            let stage = world.query::<&Recovery>().iter(world).next().unwrap().stage;
            saw_get_up |= stage == RecoveryStage::GettingUp;
            saw_running |= stage == RecoveryStage::Running;
            saw_mounting |= stage == RecoveryStage::Mounting;
            if stage == RecoveryStage::Riding {
                break;
            }
        }
        assert!(
            saw_get_up && saw_running && saw_mounting,
            "recovery skipped a visible stage"
        );
        assert_eq!(player(&mut app).crashed, 0.0);
        assert_eq!(player(&mut app).health, 100.0);
        assert!(player(&mut app).immunity > 0.5);
        assert_eq!(
            app.world_mut().query::<&Debris>().iter(app.world()).count(),
            0
        );
    }
    #[test]
    fn directional_melee_damages_the_target_and_awards_one_takedown() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let mut rival = Rider::new(1);
        rival.lane = -4.0;
        rival.progress = -8.0;
        rival.immunity = 0.0;
        let rival_id = app
            .world_mut()
            .spawn((rival, Transform::default(), Visibility::default()))
            .id();
        for expected in [200.0 / 3.0, 100.0 / 3.0, 0.0] {
            {
                let mut q = app.world_mut().query::<&mut Rider>();
                for mut r in q.iter_mut(app.world_mut()) {
                    r.attack = 0.0;
                    r.progress = -8.0;
                    r.lane = if r.id == 0 { -2.0 } else { -3.7 };
                    r.speed = 0.0;
                }
            }
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyJ);
            keys.press(KeyCode::KeyJ);
            step(&mut app, 12);
            assert!((app.world().get::<Rider>(rival_id).unwrap().health - expected).abs() < 0.001);
            step(&mut app, 20);
        }
        assert!(app.world().get::<Rider>(rival_id).unwrap().crashed > 0.0);
        assert_eq!(app.world().resource::<Race>().takedowns, 1);
    }
    #[test]
    fn traffic_impact_crashes_and_restart_clears_debris() {
        let mut app = simulation();
        for _ in 0..3 {
            app.update();
        }
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyR);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            q.single_mut(app.world_mut()).unwrap().immunity = 0.0;
        }
        let progress = player(&mut app).progress + 3.5;
        let lane = player(&mut app).lane;
        let (car_position, car_heading) = app.world().resource::<Track>().sample(progress, lane);
        app.world_mut().spawn((
            Traffic {
                route: 0,
                progress,
                lane,
                speed: -13.0,
            },
            Transform::from_translation(car_position)
                .with_rotation(car_heading * Quat::from_rotation_y(std::f32::consts::PI)),
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y,
                Quat::IDENTITY,
                Collider::cuboid(0.95, 0.9, 1.95),
            )]),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ));
        step(&mut app, 8);
        assert!(player(&mut app).crashed > 0.0);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        step(&mut app, 1);
        assert!(app.world().resource::<Race>().phase == Phase::Countdown);
        assert_eq!(player(&mut app).health, 100.0);
        assert_eq!(player(&mut app).speed, 0.0);
        assert_eq!(
            app.world_mut().query::<&Debris>().iter(app.world()).count(),
            0
        );
        assert_eq!(
            app.world_mut()
                .query::<&ImpulseJoint>()
                .iter(app.world())
                .count(),
            0
        );
    }
    #[test]
    fn finish_awards_results_once_and_button_starts_next_race() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let finish = app.world().resource::<Track>().length * LAPS as f32;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            let mut r = q.single_mut(app.world_mut()).unwrap();
            r.progress = finish + 1.0;
            r.gates.completed_laps = LAPS;
        }
        step(&mut app, 1);
        assert!(app.world().resource::<Race>().phase == Phase::Finished);
        assert_eq!(app.world().resource::<Race>().place, 1);
        assert_eq!(app.world().resource::<Profile>().credits, 2000);
        step(&mut app, 5);
        assert_eq!(app.world().resource::<Profile>().races, 1);
        // RoadInputPlugin already wires the menu button before input.
        app.world_mut()
            .spawn((crate::ui::StartButton, bevy::ui::Pressed));
        step(&mut app, 1);
        assert!(app.world().resource::<Race>().phase == Phase::Countdown);
    }
    #[test]
    fn d_moves_right_and_a_moves_left_in_chase_camera_space() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let world = app.world_mut();
            world.query::<&mut Rider>().single_mut(world).unwrap().speed = 10.0;
        }
        let (_, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
        let screen_right = (q * Vec3::Z).cross(Vec3::Y);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        step(&mut app, 12);
        let (d, lane) = {
            let r = player(&mut app);
            (r.progress, r.lane)
        };
        let right = app.world().resource::<Track>().sample(d, lane).0;
        let base = app.world().resource::<Track>().sample(d, -2.0).0;
        assert!((right - base).dot(screen_right) > 0.0, "D moved left");
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyD);
            keys.press(KeyCode::KeyA);
        }
        // Reference steering must unwind existing right force before turning left.
        step(&mut app, 45);
        assert!(
            player(&mut app).steering.force < 0.0,
            "A never countersteered left"
        );
        assert!(
            player(&mut app).lane < lane,
            "A moved right after countersteering"
        );
    }
    #[test]
    fn fast_rear_contact_ragdolls_victim_without_hopping_the_attacker() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            q.single_mut(app.world_mut()).unwrap().speed = 22.0;
        }
        let mut victim = Rider::new(1);
        victim.progress = -6.0;
        victim.lane = -2.0;
        let (p, q) = app
            .world()
            .resource::<Track>()
            .sample(victim.progress, victim.lane);
        let victim = app
            .world_mut()
            .spawn((
                victim,
                Transform::from_translation(p).with_rotation(q),
                Visibility::default(),
                recovery::bike_body(),
            ))
            .id();
        step(&mut app, 2);
        assert_eq!(
            app.world().get::<Recovery>(victim).unwrap().stage,
            RecoveryStage::Falling
        );
        assert_eq!(player(&mut app).hop_velocity, 0.0);
        assert_eq!(player(&mut app).crashed, 0.0);
    }

    #[test]
    fn crash_camera_tracks_the_moving_torso_instead_of_the_road_position() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
        let camera_entity = app
            .world_mut()
            .spawn((
                ChaseCamera,
                Transform::from_translation(p - q * Vec3::Z * 6.0 + Vec3::Y * 4.0),
                Projection::Perspective(default()),
            ))
            .id();
        app.add_systems(PostUpdate, camera);
        {
            let mut query = app.world_mut().query::<&mut Rider>();
            let mut r = query.single_mut(app.world_mut()).unwrap();
            r.health = 0.0;
            r.speed = 40.0;
        }
        step(&mut app, 60);
        let torso = app
            .world_mut()
            .query::<&Recovery>()
            .single(app.world())
            .unwrap()
            .torso
            .unwrap();
        let torso = app.world().get::<Transform>(torso).unwrap().translation;
        let camera = app
            .world()
            .get::<Transform>(camera_entity)
            .unwrap()
            .translation;
        assert!(torso.with_y(0.0).distance(p.with_y(0.0)) > 10.0);
        assert!(
            camera.with_y(0.0).distance(torso.with_y(0.0)) < 10.0,
            "camera failed to follow torso: {camera:?} vs {torso:?}"
        );
    }
    #[test]
    fn ragdoll_sized_bodies_collide_with_trees_cars_bikes_and_wrecks() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(0.4, 2.0, 0.4),
            Transform::from_xyz(-20.0, 1.0, 0.0),
            world::Obstacle,
        ));
        app.world_mut().spawn((
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y,
                Quat::IDENTITY,
                Collider::cuboid(0.95, 0.9, 1.95),
            )]),
            Transform::from_xyz(0.0, 0.0, 0.0),
            ActiveCollisionTypes::all(),
        ));
        app.world_mut()
            .spawn((recovery::bike_body(), Transform::from_xyz(20.0, 0.0, 0.0)));
        let wreck = app
            .world_mut()
            .spawn((
                RigidBody::Dynamic,
                Collider::cuboid(0.4, 0.8, 1.12),
                ColliderMassProperties::Mass(170.0),
                Transform::from_xyz(40.0, 0.85, 0.0),
                Ccd::enabled(),
            ))
            .id();
        let mut projectiles = Vec::new();
        for x in [-20.0, 0.0, 20.0, 40.0] {
            projectiles.push(
                app.world_mut()
                    .spawn((
                        RigidBody::Dynamic,
                        Collider::ball(0.22),
                        ColliderMassProperties::Mass(5.0),
                        Transform::from_xyz(x, 1.0, -5.0),
                        Velocity::linear(Vec3::Z * 25.0),
                        GravityScale(0.0),
                        Ccd::enabled(),
                        Restitution::coefficient(0.4),
                    ))
                    .id(),
            );
        }
        step(&mut app, 90);
        for projectile in projectiles {
            let p = app
                .world()
                .get::<Transform>(projectile)
                .unwrap()
                .translation;
            assert!(p.z < 2.0, "body tunneled through solid at {p:?}");
        }
        assert!(
            app.world().get::<Transform>(wreck).unwrap().translation.z > 0.05,
            "wreck did not react to impact"
        );
    }
    #[test]
    fn biker_can_knock_down_a_recovering_rider_again_without_duplicate_ragdolls() {
        for stage in [RecoveryStage::GettingUp, RecoveryStage::Running] {
            let mut app = simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let owner = app
                .world_mut()
                .query::<(Entity, &Rider)>()
                .iter(app.world())
                .find(|(_, r)| r.id == 0)
                .unwrap()
                .0;
            let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
            let wreck = app
                .world_mut()
                .spawn((
                    RigidBody::Dynamic,
                    Collider::cuboid(0.4, 0.5, 1.1),
                    Transform::from_translation(p + q * Vec3::Z * 25.0 + Vec3::Y),
                    recovery::OwnedBody {
                        owner,
                        kind: recovery::BodyKind::Wreck,
                    },
                    Debris(f32::INFINITY),
                ))
                .id();
            let runner = app
                .world_mut()
                .spawn((
                    RigidBody::KinematicPositionBased,
                    Collider::capsule_y(0.65, 0.28),
                    KinematicCharacterController::default(),
                    Transform::from_translation(p + Vec3::Y * 0.95),
                    recovery::OwnedBody {
                        owner,
                        kind: recovery::BodyKind::Runner,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::all(),
                    Debris(f32::INFINITY),
                ))
                .id();
            app.world_mut().entity_mut(owner).insert(ColliderDisabled);
            {
                let mut r = app.world_mut().get_mut::<Recovery>(owner).unwrap();
                r.stage = stage;
                r.wreck = Some(wreck);
                r.runner = Some(runner);
            }
            {
                let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
                rider.crashed = 1.0;
                rider.motion_velocity = Vec3::X * 1000.0;
            }
            let mut attacker = Rider::new(1);
            attacker.progress = -9.5;
            attacker.lane = -2.0;
            attacker.speed = 25.0;
            let (p, q) = app
                .world()
                .resource::<Track>()
                .sample(attacker.progress, attacker.lane);
            let attacker = app
                .world_mut()
                .spawn((
                    attacker,
                    Transform::from_translation(p).with_rotation(q),
                    Visibility::default(),
                    recovery::bike_body(),
                ))
                .id();
            step(&mut app, 3);
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Falling
            );
            assert!(app.world().get::<Rider>(attacker).unwrap().hop_velocity > 0.0);
            assert_eq!(app.world().get::<Rider>(attacker).unwrap().crashed, 0.0);
            assert!(app.world().get_entity(runner).is_err());
            let torso = app.world().get::<Recovery>(owner).unwrap().torso.unwrap();
            assert!(
                app.world().get::<Velocity>(torso).unwrap().linear.length() < 100.0,
                "repeat knockdown inherited stale riding velocity"
            );
            step(&mut app, 5);
            let body_count = app
                .world_mut()
                .query::<&recovery::OwnedBody>()
                .iter(app.world())
                .filter(|b| b.owner == owner && b.kind == recovery::BodyKind::Ragdoll)
                .count();
            assert_eq!(body_count, 11);
        }
    }
    #[test]
    fn slow_sideways_steering_cannot_pass_through_a_solid_car() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let world = app.world_mut();
            world.query::<&mut Rider>().single_mut(world).unwrap().speed = 6.0;
        }
        let (p, q) = app.world().resource::<Track>().sample(-8.0, -0.5);
        app.world_mut().spawn((
            Traffic {
                route: 0,
                progress: -8.0,
                lane: -0.5,
                speed: 0.0,
            },
            Transform::from_translation(p).with_rotation(q),
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y,
                Quat::IDENTITY,
                Collider::cuboid(0.95, 0.9, 1.95),
            )]),
            ActiveCollisionTypes::all(),
            ActiveEvents::COLLISION_EVENTS,
        ));
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        step(&mut app, 60);
        let rider = player(&mut app);
        assert_eq!(rider.crashed, 0., "a slow scrape must stay mounted");
        assert!(
            rider.lane < -1.6,
            "bike passed through a solid car during a slow scrape: lane {}",
            rider.lane
        );
    }
    #[test]
    fn legacy_profile_is_upgraded_without_losing_progress() {
        let legacy = br#"{"races":5,"wins":2,"credits":1750,"best_seconds":65.25}"#;
        let profile = Profile::decode(legacy).unwrap();
        assert_eq!(profile.schema_version, 1);
        assert_eq!(profile.credits, 1750);
        let roundtrip = Profile::decode(&serde_json::to_vec(&profile).unwrap()).unwrap();
        assert_eq!(roundtrip.best_seconds, Some(65.25));
        assert!(
            Profile::decode(
                br#"{"schema_version":99,"races":5,"wins":2,"credits":1750,"best_seconds":null}"#
            )
            .is_err()
        );
        assert!(Profile::decode(b"{broken").is_err());
    }
    #[test]
    fn fixed_simulation_matches_at_30_60_and_120_render_fps() {
        let mut runs = Vec::new();
        for fps in [30u64, 60, 120] {
            let mut app = simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            let frames = fps * 2;
            let nanos = 120 * app.world().resource::<Time<Fixed>>().timestep().as_nanos() as u64;
            for i in 0..frames {
                let dt = nanos / frames + u64::from(i < nanos % frames);
                app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                    std::time::Duration::from_nanos(dt),
                ));
                app.update();
            }
            let r = player(&mut app);
            let pose = (r.progress, r.lane, r.speed);
            let ticks = app
                .world()
                .resource::<crate::runtime::SimulationClock>()
                .ticks;
            assert_eq!(ticks, 120, "wrong fixed ticks at {fps} FPS");
            runs.push(pose);
        }
        for run in &runs[1..] {
            assert!((run.0 - runs[0].0).abs() < 0.001);
            assert!((run.1 - runs[0].1).abs() < 0.001);
            assert!((run.2 - runs[0].2).abs() < 0.001);
        }
    }
    #[test]
    fn attack_press_survives_a_render_frame_without_a_fixed_tick() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_millis(1),
        ));
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyJ);
        step(&mut app, 1);
        assert_eq!(player(&mut app).attack, 0.0);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyJ);
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_millis(20),
        ));
        step(&mut app, 1);
        assert!(player(&mut app).attack > 0.6);
        assert_eq!(
            app.world_mut()
                .resource_mut::<Controls>()
                .buffer
                .take_attack(),
            None
        );
    }
    #[test]
    fn one_hundred_pause_resume_crash_reset_cycles_do_not_leak_state() {
        let mut app = simulation();
        let mut stable_count = None;
        for _ in 0..100 {
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            {
                let mut q = app.world_mut().query::<&mut Rider>();
                q.single_mut(app.world_mut()).unwrap().health = 0.0;
            }
            step(&mut app, 1);
            {
                let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.release(KeyCode::Escape);
                keys.press(KeyCode::Escape);
            }
            step(&mut app, 1);
            assert_eq!(app.world().resource::<Race>().phase, Phase::Paused);
            let snapshot: Vec<_> = app
                .world_mut()
                .query::<(Entity, &Transform, &Debris)>()
                .iter(app.world())
                .map(|(e, t, _)| (e, t.translation))
                .collect();
            step(&mut app, 3);
            for (e, p) in snapshot {
                assert_eq!(app.world().get::<Transform>(e).unwrap().translation, p);
            }
            {
                let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.release(KeyCode::Escape);
                keys.press(KeyCode::Escape);
            }
            step(&mut app, 1);
            assert_eq!(app.world().resource::<Race>().phase, Phase::Racing);
            {
                let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.release(KeyCode::KeyR);
                keys.press(KeyCode::KeyR);
            }
            step(&mut app, 1);
            assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
            assert_eq!(
                app.world_mut().query::<&Debris>().iter(app.world()).count(),
                0
            );
            assert_eq!(
                app.world_mut()
                    .query::<&ImpulseJoint>()
                    .iter(app.world())
                    .count(),
                0
            );
            assert_eq!(
                app.world_mut()
                    .query::<&ColliderDisabled>()
                    .iter(app.world())
                    .count(),
                0
            );
            let count = app.world().entities().count_spawned();
            if let Some(expected) = stable_count {
                assert_eq!(count, expected, "reset leaked entities");
            } else {
                stable_count = Some(count);
            }
        }
    }
    #[test]
    fn lifecycle_visits_loading_menu_race_and_results() {
        use crate::runtime::{AppScreen, RoadLifecyclePlugin};
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::state::app::StatesPlugin,
            crate::content::RoadContentPlugin,
            RoadLifecyclePlugin,
        ))
        .init_resource::<Race>();
        assert_eq!(
            *app.world().resource::<State<AppScreen>>().get(),
            AppScreen::Boot
        );
        step(&mut app, 1);
        assert_eq!(
            *app.world().resource::<State<AppScreen>>().get(),
            AppScreen::Loading
        );
        step(&mut app, 1);
        assert_eq!(
            *app.world().resource::<State<AppScreen>>().get(),
            AppScreen::MainMenu
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
        step(&mut app, 2);
        assert_eq!(
            *app.world().resource::<State<AppScreen>>().get(),
            AppScreen::Race
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Finished;
        step(&mut app, 2);
        assert_eq!(
            *app.world().resource::<State<AppScreen>>().get(),
            AppScreen::Results
        );
    }
    #[test]
    fn corrupt_and_future_profile_files_are_not_overwritten() {
        let dir = std::env::temp_dir().join(format!(
            "roaddash-save-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("profile.json");
        for original in [
            b"{broken".as_slice(),
            br#"{"schema_version":99,"races":5,"wins":2,"credits":1750,"best_seconds":null}"#
                .as_slice(),
        ] {
            std::fs::write(&file, original).unwrap();
            let mut profile = Profile::load_at(&file);
            profile.credits = 99999;
            profile.save_at(&file);
            assert_eq!(std::fs::read(&file).unwrap(), original);
        }
        std::fs::write(
            &file,
            br#"{"races":5,"wins":2,"credits":1750,"best_seconds":65.25}"#,
        )
        .unwrap();
        let profile = Profile::load_at(&file);
        profile.save_at(&file);
        assert_eq!(Profile::load_at(&file).credits, 1750);
        assert!(!file.with_extension("tmp").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn crash_separates_rider_and_preserves_momentum_without_an_upward_boost() {
        #[derive(Resource, Default)]
        struct Snapshot(Vec<(recovery::BodyKind, Vec3, Vec3)>);
        for momentum in [Vec3::ZERO, Vec3::new(8.0, -3.0, 31.0)] {
            let mut app = simulation();
            app.init_resource::<Snapshot>().add_systems(
                FixedUpdate,
                (|mut snapshot: ResMut<Snapshot>,
                  bodies: Query<(&recovery::OwnedBody, &Velocity, &Transform)>| {
                    if snapshot.0.is_empty() {
                        snapshot.0.extend(
                            bodies
                                .iter()
                                .map(|(body, v, t)| (body.kind, v.linear, t.translation)),
                        );
                    }
                })
                .after(crate::runtime::SimSet::Gameplay)
                .before(PhysicsSet::SyncBackend),
            );
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            {
                let mut query = app.world_mut().query::<&mut Rider>();
                let mut rider = query.single_mut(app.world_mut()).unwrap();
                rider.speed = 35.0;
                rider.motion_velocity = momentum;
                rider.health = 0.0;
            }
            step(&mut app, 1);
            let samples = &app.world().resource::<Snapshot>().0;
            assert_eq!(samples.len(), 12);
            let wreck = samples
                .iter()
                .find(|s| s.0 == recovery::BodyKind::Wreck)
                .unwrap();
            if momentum == Vec3::ZERO {
                assert!(wreck.1.length() > 34.0, "forward momentum was scaled away");
            } else {
                assert!(
                    wreck.1.distance(momentum) < 0.001,
                    "measured momentum was changed"
                );
            }
            for (_, velocity, _) in samples {
                assert!(
                    (velocity.y - momentum.y).abs() < 0.001,
                    "artificial upward velocity {velocity:?}"
                );
                assert!(velocity.distance(wreck.1) < 0.001);
            }
            let torso = samples
                .iter()
                .find(|s| s.0 == recovery::BodyKind::Ragdoll)
                .unwrap();
            assert!(
                torso.2.with_y(0.0).distance(wreck.2.with_y(0.0)) > 1.0,
                "rider stayed on the seat"
            );
            let (owner, recovery) = app
                .world_mut()
                .query::<(Entity, &Recovery)>()
                .iter(app.world())
                .next()
                .unwrap();
            assert_eq!(
                *app.world().get::<Visibility>(owner).unwrap(),
                Visibility::Hidden
            );
            let wreck = recovery.wreck.unwrap();
            let mut descendants = vec![wreck];
            while let Some(e) = descendants.pop() {
                assert!(
                    app.world().get::<world::SeatedRiderVisual>(e).is_none(),
                    "wreck still has a seated rider"
                );
                if let Some(children) = app.world().get::<Children>(e) {
                    descendants.extend(children.iter());
                }
            }
        }
    }
    #[test]
    fn remount_reaches_a_fallen_bike_in_sideways_upside_down_and_vertical_poses() {
        for rotation in [
            Quat::IDENTITY,
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            Quat::from_rotation_z(std::f32::consts::PI),
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ] {
            for side in [-1.0, 1.0] {
                let mut app = simulation();
                app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
                let owner = app
                    .world_mut()
                    .query::<(Entity, &Rider)>()
                    .iter(app.world())
                    .find(|(_, r)| r.id == 0)
                    .unwrap()
                    .0;
                let (p, yaw) = app.world().resource::<Track>().sample(-8.0, -2.0);
                let q = yaw * rotation;
                let extent = (q * Vec3::X * 0.38).y.abs()
                    + (q * Vec3::Y * 0.8).y.abs()
                    + (q * Vec3::Z * 1.12).y.abs();
                let height = extent - (q * Vec3::Y * 0.85).y + 0.02;
                let wreck_pose = Transform::from_translation(p.with_y(height)).with_rotation(q);
                let center = wreck_pose.translation + q * Vec3::Y * 0.85;
                let wreck = app
                    .world_mut()
                    .spawn((
                        RigidBody::Fixed,
                        Collider::compound(vec![(
                            Vec3::Y * 0.85,
                            Quat::IDENTITY,
                            Collider::cuboid(0.38, 0.8, 1.12),
                        )]),
                        wreck_pose,
                        recovery::OwnedBody {
                            owner,
                            kind: recovery::BodyKind::Wreck,
                        },
                        Debris(f32::INFINITY),
                    ))
                    .id();
                let runner = app
                    .world_mut()
                    .spawn((
                        RigidBody::KinematicPositionBased,
                        Collider::capsule_y(0.65, 0.28),
                        KinematicCharacterController {
                            apply_impulse_to_dynamic_bodies: false,
                            ..default()
                        },
                        Transform::from_translation(
                            (center + yaw * Vec3::X * side * 4.0).with_y(0.97),
                        ),
                        recovery::OwnedBody {
                            owner,
                            kind: recovery::BodyKind::Runner,
                        },
                        Debris(f32::INFINITY),
                    ))
                    .id();
                app.world_mut().entity_mut(owner).insert(ColliderDisabled);
                {
                    let mut state = app.world_mut().get_mut::<Recovery>(owner).unwrap();
                    state.stage = RecoveryStage::Running;
                    state.wreck = Some(wreck);
                    state.runner = Some(runner);
                }
                app.world_mut().get_mut::<Rider>(owner).unwrap().crashed = 1.0;
                for _ in 0..480 {
                    step(&mut app, 1);
                    if app.world().get::<Recovery>(owner).unwrap().stage == RecoveryStage::Riding {
                        break;
                    }
                }
                assert_eq!(
                    app.world().get::<Recovery>(owner).unwrap().stage,
                    RecoveryStage::Riding,
                    "unreachable remount for {rotation:?} side {side}"
                );
                assert!(app.world().get_entity(wreck).is_err());
                assert!(app.world().get_entity(runner).is_err());
            }
        }
    }
    #[test]
    fn releasing_or_exhausting_boost_decelerates_instead_of_clamping_speed() {
        for exhausted in [false, true] {
            let mut app = simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            {
                let mut q = app.world_mut().query::<&mut Rider>();
                let mut r = q.single_mut(app.world_mut()).unwrap();
                r.speed = 75.0;
                r.nitro = if exhausted { 0.0 } else { 50.0 };
            }
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            if exhausted {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(KeyCode::ShiftLeft);
            }
            step(&mut app, 1);
            let speed = player(&mut app).speed;
            assert!(speed > 74.0 && speed < 75.0, "instant speed clamp: {speed}");
        }
    }
    #[test]
    fn grass_allows_forward_driving_without_damage_and_ai_returns_to_the_road() {
        for (id, lane) in [(0, 18.0), (1, 18.0), (3, -22.0), (5, 12.0)] {
            let mut app = simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let (p, q) = app.world().resource::<Track>().sample(300.0, lane);
            {
                let mut query = app.world_mut().query::<(&mut Rider, &mut Transform)>();
                let (mut r, mut t) = query.single_mut(app.world_mut()).unwrap();
                r.id = id;
                r.progress = 300.0;
                r.lane = lane;
                r.immunity = 0.0;
                *t = Transform::from_translation(p).with_rotation(q);
            }
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyW);
            step(&mut app, 600);
            let r = app
                .world_mut()
                .query::<&Rider>()
                .single(app.world())
                .unwrap();
            assert!(
                r.progress > 380.0,
                "grass stopped rider {id}: {}",
                r.progress
            );
            assert_eq!(r.health, 100.0);
            assert_eq!(r.crashed, 0.0);
            if id != 0 {
                assert!(r.lane.abs() < 6.5, "AI stayed in grass: {}", r.lane);
            }
        }
    }
    #[test]
    fn a_crashed_bike_slides_down_a_solid_slope() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            q.single_mut(app.world_mut()).unwrap().health = 0.0;
        }
        step(&mut app, 1);
        let wreck = app
            .world_mut()
            .query::<&Recovery>()
            .single(app.world())
            .unwrap()
            .wreck
            .unwrap();
        let tilt = Quat::from_rotation_z(30.0_f32.to_radians());
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(20.0, 0.3, 10.0),
            Transform::from_xyz(0.0, 10.0, 0.0).with_rotation(tilt),
            Friction::coefficient(0.9),
        ));
        // This test deliberately teleports the wreck up ten metres to isolate
        // slope friction; discard the old crash's ground-level energy envelope.
        app.world_mut()
            .entity_mut(wreck)
            .remove::<recovery::CrashEnvelope>();
        app.world_mut().entity_mut(wreck).insert((
            Transform::from_translation(Vec3::new(0.0, 10.5, 0.0)).with_rotation(tilt),
            Velocity::zero(),
        ));
        step(&mut app, 120);
        let p = app.world().get::<Transform>(wreck).unwrap().translation;
        assert!(p.x < -1.0 && p.y < 9.5, "wreck stuck on slope: {p:?}");
    }
    #[test]
    fn a_cow_causes_a_momentum_limited_crash_without_a_launch() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let (p, q) = app.world().resource::<Track>().sample(-5.5, -2.0);
        app.world_mut().spawn((
            crate::environment::Cow,
            RigidBody::Fixed,
            Collider::compound(vec![(
                Vec3::Y,
                Quat::IDENTITY,
                Collider::cuboid(0.6, 0.7, 1.0),
            )]),
            Transform::from_translation(p).with_rotation(q),
            ActiveEvents::COLLISION_EVENTS,
        ));
        {
            let mut query = app.world_mut().query::<&mut Rider>();
            query.single_mut(app.world_mut()).unwrap().speed = 22.0;
        }
        step(&mut app, 4);
        assert_eq!(player(&mut app).hop_velocity, 0.0);
        assert!(player(&mut app).crashed > 0.0);
    }
    #[test]
    fn one_hundred_high_speed_drops_keep_joint_anchors_and_hinge_axes_stable() {
        let mut app = simulation();
        for cycle in 0..100 {
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            {
                let mut q = app.world_mut().query::<&mut Rider>();
                let mut r = q.single_mut(app.world_mut()).unwrap();
                r.health = 0.0;
                r.speed = 45.0;
            }
            step(&mut app, 61);
            let world = app.world_mut();
            for (t, v) in world.query::<(&Transform, &Velocity)>().iter(world) {
                assert!(
                    t.translation.is_finite() && t.rotation.is_finite() && v.linear.is_finite()
                );
                assert!(t.translation.y > -0.5, "terrain clipping in drop {cycle}");
            }
            for (t, j) in world.query::<(&Transform, &ImpulseJoint)>().iter(world) {
                let parent = world.get::<Transform>(j.parent).unwrap();
                let data: &GenericJoint = j.data.as_ref();
                assert!(
                    (parent.translation + parent.rotation * data.local_anchor1())
                        .distance(t.translation + t.rotation * data.local_anchor2())
                        < 0.3,
                    "joint separated in drop {cycle}"
                );
                if data
                    .locked_axes()
                    .contains(JointAxesMask::ANG_Y | JointAxesMask::ANG_Z)
                {
                    assert!(
                        (parent.rotation * Vec3::X).dot(t.rotation * Vec3::X) > 0.95,
                        "hinge twisted in drop {cycle}"
                    );
                }
            }
            {
                let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                keys.release(KeyCode::KeyR);
                keys.press(KeyCode::KeyR);
            }
            step(&mut app, 1);
        }
    }
    #[test]
    fn point_to_point_driving_crosses_every_gate_and_awards_one_result() {
        let mut app = simulation();
        app.insert_resource(
            Track::from_json(include_str!("../content/tracks/valley.json")).unwrap(),
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let (p, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
        {
            let mut query = app
                .world_mut()
                .query_filtered::<&mut Transform, With<Rider>>();
            *query.single_mut(app.world_mut()).unwrap() =
                Transform::from_translation(p).with_rotation(q);
        }
        // Follow the centerline through the real fixed controller without any
        // progress/finish teleport. Geometry/traffic are separate collision tests.
        app.world_mut()
            .resource_mut::<Time<Fixed>>()
            .advance_by(std::time::Duration::from_secs_f64(1.0 / 60.0));
        for _ in 0..10000 {
            let (d, lane, speed) = {
                let r = player(&mut app);
                (r.progress, r.lane, r.speed)
            };
            let curve = app.world().resource::<Track>().curvature(d);
            let steer = ((-lane * 2.0 - curve * speed * speed * 0.10) / (1.8 + speed * 0.108))
                .clamp(-1.0, 1.0);
            app.world_mut().resource_mut::<Controls>().buffer.held = rr_core::VehicleIntent {
                throttle: true,
                steer,
                ..default()
            };
            app.world_mut().run_schedule(FixedUpdate);
            if app.world().resource::<Race>().phase == Phase::Finished {
                break;
            }
        }
        let r = player(&mut app);
        let details = format!(
            "progress {} lane {} speed {} gates {:?}",
            r.progress, r.lane, r.speed, r.gates
        );
        assert_eq!(
            app.world().resource::<Race>().phase,
            Phase::Finished,
            "{details}"
        );
        assert_eq!(player(&mut app).gates.completed_laps, 1);
        assert_eq!(app.world().resource::<Profile>().races, 1);
        for _ in 0..60 {
            app.world_mut().run_schedule(FixedUpdate);
        }
        assert_eq!(app.world().resource::<Profile>().races, 1);
    }
    #[test]
    fn metropolis_retry_restores_shifted_grid_and_clears_finish_credit() {
        let mut app = simulation();
        // A retry is an in-game action; Boot/Loading intentionally suppress input.
        step(&mut app, 3);
        assert!(!matches!(
            app.world()
                .resource::<State<crate::runtime::AppScreen>>()
                .get(),
            crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
        ));
        app.insert_resource(Track::metropolis().unwrap());
        app.world_mut().resource_mut::<Race>().phase = Phase::Finished;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            let mut r = q.single_mut(app.world_mut()).unwrap();
            r.progress = 11500.;
            r.gates.completed_laps = 1;
            r.finished = Some(100.);
        }
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        step(&mut app, 1);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
        let r = player(&mut app);
        assert_eq!(r.progress, 212.);
        assert_eq!(r.finished, None);
        assert_eq!(r.gates.next, 1);
        assert_eq!(r.gates.completed_laps, 0);
    }
    #[test]
    fn missing_a_gate_stops_at_the_open_finish_without_payout_and_retry_resets_gates() {
        let mut app = simulation();
        let track = Track::from_json(include_str!("../content/tracks/valley.json")).unwrap();
        let length = track.length;
        let (p, q) = track.sample(length - 4.0, 0.0);
        app.insert_resource(track);
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut query = app.world_mut().query::<(&mut Rider, &mut Transform)>();
            let (mut r, mut t) = query.single_mut(app.world_mut()).unwrap();
            r.progress = length - 4.0;
            r.lane = 0.0;
            r.speed = 20.0;
            *t = Transform::from_translation(p).with_rotation(q);
        }
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        step(&mut app, 120);
        assert_eq!(player(&mut app).progress, length);
        assert_eq!(player(&mut app).finished, None);
        assert_eq!(app.world().resource::<Profile>().races, 0);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        step(&mut app, 1);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
        assert_eq!(player(&mut app).gates.next, 1);
        assert_eq!(player(&mut app).gates.completed_laps, 0);
    }
    #[test]
    fn original_contact_mode_applies_distinct_rider_and_wreck_results_then_recovers() {
        use crate::original_crash::{self as original, OriginalCrashes};
        for depleted in [false, true] {
            let mut app = simulation();
            app.insert_resource(OriginalCrashes(true));
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let (origin, frame) = app.world().resource::<Track>().sample(10.0, -2.0);
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let incoming = frame * Vec3::Z * (120.0 / 3.6);
            let other_velocity = frame * Vec3::Z * (-40.0 / 3.6);
            let car_origin = origin + frame * Vec3::Z * 2.0;
            let car = app
                .world_mut()
                .spawn((
                    Traffic {
                        route: 0,
                        speed: 40.0 / 3.6,
                        progress: 10.0,
                        lane: -2.0,
                    },
                    Transform::from_translation(car_origin)
                        .with_rotation(frame * Quat::from_rotation_y(std::f32::consts::PI)),
                ))
                .id();
            let condition = {
                let mut rider = app.world_mut().get_mut::<Rider>(owner).unwrap();
                rider.progress = 10.0;
                rider.lane = -2.0;
                rider.speed = 120.0 / 3.6;
                rider.motion_velocity = incoming;
                rider.immunity = 0.0;
                if depleted {
                    rider.original_condition.stamina = 1;
                }
                rider.original_condition
            };
            *app.world_mut().get_mut::<Transform>(owner).unwrap() =
                Transform::from_translation(origin).with_rotation(frame);
            if let Some(mut bike) = app.world_mut().get_mut::<crate::vehicle::BikeState>(owner) {
                bike.impact_velocity = incoming;
            }
            let (expected, reference_frame) = original::plan(
                incoming,
                other_velocity,
                frame,
                car_origin - origin,
                true,
                condition,
            );
            app.world_mut()
                .resource_mut::<Messages<recovery::SweptContact>>()
                .write(recovery::SweptContact(owner, car));
            use bevy::ecs::system::RunSystemOnce;
            app.world_mut().run_system_once(recovery::impacts).unwrap();
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Falling
            );
            let wreck = app.world().get::<Recovery>(owner).unwrap().wreck.unwrap();
            assert!(
                app.world()
                    .get::<Velocity>(wreck)
                    .unwrap()
                    .linear
                    .distance(original::velocity(expected.bike, reference_frame))
                    < 0.002
            );
            let state = app.world().get::<Recovery>(owner).unwrap();
            assert_eq!(state.parts.len(), 11, "physical rig changed");
            for part in &state.parts {
                assert!(
                    app.world()
                        .get::<Velocity>(*part)
                        .unwrap()
                        .linear
                        .distance(original::velocity(expected.rider.unwrap(), reference_frame))
                        < 0.002
                );
            }
            app.world_mut().despawn(car);
            for _ in 0..1800 {
                step(&mut app, 1);
                if app.world().get::<Recovery>(owner).unwrap().stage == RecoveryStage::Riding {
                    break;
                }
            }
            let state = app.world().get::<Recovery>(owner).unwrap();
            let runner_pose = state
                .runner
                .and_then(|e| app.world().get::<Transform>(e).map(|t| t.translation));
            let wreck_pose = state
                .wreck
                .and_then(|e| app.world().get::<Transform>(e).map(|t| t.translation));
            let wreck_speed = state
                .wreck
                .and_then(|e| app.world().get::<Velocity>(e).map(|v| v.linear.length()));
            let movement = state.runner.and_then(|e| {
                app.world()
                    .get::<KinematicCharacterControllerOutput>(e)
                    .map(|output| {
                        (
                            output.desired_translation,
                            output.effective_translation,
                            output.grounded,
                            output
                                .collisions
                                .iter()
                                .map(|c| {
                                    let entity = c.entity;
                                    (
                                        entity,
                                        app.world().get::<world::Ground>(entity).is_some(),
                                        app.world().get::<recovery::OwnedBody>(entity).map(|o| {
                                            (
                                                o.kind == recovery::BodyKind::Ragdoll,
                                                o.owner == owner,
                                            )
                                        }),
                                        app.world().get::<Transform>(entity).map(|t| t.translation),
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
            });
            let requested = state.runner.and_then(|e| {
                app.world()
                    .get::<KinematicCharacterController>(e)
                    .and_then(|c| c.translation)
            });
            assert!(
                state.stage == RecoveryStage::Riding,
                "depleted={depleted} stage={:?} elapsed={} runner={runner_pose:?} wreck={wreck_pose:?} speed={wreck_speed:?} path_len={} goal={:?} stationary={} walk_distance={} progress={} movement={movement:?} requested={requested:?}",
                state.stage,
                state.elapsed,
                state.walk_path.len(),
                state.walk_goal,
                state.walk_watch.stationary_seconds(),
                state.walk_distance,
                app.world().get::<Rider>(owner).unwrap().progress,
            );
            assert_eq!(
                app.world().get::<Rider>(owner).unwrap().original_condition,
                original::initial_condition()
            );
        }
    }

    #[test]
    fn matched_speed_contacts_and_small_side_taps_do_not_cause_wipeouts() {
        for (side, relative) in [(false, 0.0), (false, 3.0), (true, 1.5)] {
            let mut app = simulation();
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            let (p, q) = app.world().resource::<Track>().sample(10.0, -2.0);
            let owner = app
                .world_mut()
                .query_filtered::<Entity, With<Rider>>()
                .single(app.world())
                .unwrap();
            let other = app
                .world_mut()
                .spawn((
                    Rider::new(1),
                    Recovery::default(),
                    Transform::from_translation(
                        p + q * if side { Vec3::X * 0.7 } else { Vec3::Z * 2.0 },
                    ),
                    Visibility::default(),
                ))
                .id();
            {
                let world = app.world_mut();
                let mut r = world.get_mut::<Rider>(owner).unwrap();
                r.speed = 25.0;
                r.motion_velocity = q
                    * (Vec3::Z * (25.0 + if side { 0.0 } else { relative })
                        + Vec3::X * if side { relative } else { 0.0 });
            }
            app.world_mut()
                .get_mut::<Transform>(owner)
                .unwrap()
                .translation = p;
            app.world_mut()
                .get_mut::<Rider>(other)
                .unwrap()
                .motion_velocity = q * Vec3::Z * 25.0;
            app.world_mut()
                .resource_mut::<Messages<recovery::SweptContact>>()
                .write(recovery::SweptContact(owner, other));
            app.world_mut().run_schedule(FixedUpdate);
            assert_eq!(
                app.world().get::<Recovery>(owner).unwrap().stage,
                RecoveryStage::Riding
            );
            assert_eq!(
                app.world().get::<Recovery>(other).unwrap().stage,
                RecoveryStage::Riding
            );
        }
    }
    #[test]
    fn crowded_grid_has_no_immediate_combat_knockout() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        for id in 1..8 {
            let r = Rider::new(id);
            let (p, q) = app.world().resource::<Track>().sample(r.progress, r.lane);
            app.world_mut().spawn((
                r,
                Transform::from_translation(p).with_rotation(q),
                Visibility::default(),
                recovery::bike_body(),
            ));
        }
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        step(&mut app, 120);
        assert!(
            app.world_mut()
                .query::<&Recovery>()
                .iter(app.world())
                .all(|r| r.stage == RecoveryStage::Riding)
        );
    }
    #[test]
    fn hills_heavy_traffic_follows_full_trailer_bounds_and_resets_after_pause() {
        use bevy::ecs::system::RunSystemOnce;
        let mut app = simulation();
        app.insert_resource(Track::hills().unwrap());
        let ids = app
            .world_mut()
            .run_system_once(|mut commands: Commands, art: Res<Art>, track: Res<Track>| {
                [
                    (crate::heavy_traffic::Kind::Semi, 6000., 12., 24),
                    (crate::heavy_traffic::Kind::Tractor, 6025., 6., 32),
                ]
                .map(|(kind, progress, speed, index)| {
                    let (p, q) = track.street_sample(0, progress, 3.1);
                    let entity = commands
                        .spawn((
                            Traffic {
                                route: 0,
                                progress,
                                lane: 3.1,
                                speed,
                            },
                            TrafficYield {
                                start_progress: Some(progress),
                                ..default()
                            },
                            Transform::from_translation(p).with_rotation(q),
                            Visibility::default(),
                            RigidBody::KinematicPositionBased,
                        ))
                        .id();
                    crate::heavy_traffic::spawn(&mut commands, &art, entity, kind, index);
                    entity
                })
            })
            .unwrap();
        // Preserve this following/pause fixture by making the opposing lane
        // legitimately unavailable; unobstructed overtakes have separate tests.
        let (p, q) = app
            .world()
            .resource::<Track>()
            .street_sample(0, 6150., -3.1);
        app.world_mut().spawn((
            Traffic {
                route: 0,
                progress: 6150.,
                lane: -3.1,
                speed: -18.,
            },
            TrafficYield {
                cruise: Some(-18.),
                ..default()
            },
            Transform::from_translation(p)
                .with_rotation(q * Quat::from_rotation_y(std::f32::consts::PI)),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(0.95, 0.9, 1.95),
        ));
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let mut minimum_gap = f32::INFINITY;
        for _ in 0..360 {
            app.update();
            let follower = app.world().get::<Traffic>(ids[0]).unwrap();
            let leader = app.world().get::<Traffic>(ids[1]).unwrap();
            minimum_gap = minimum_gap.min(leader.progress - follower.progress);
        }
        assert!(
            minimum_gap > 10.,
            "full-size tractor/cab gap must remain physical: {minimum_gap}"
        );
        assert!(app.world().get::<Traffic>(ids[0]).unwrap().speed < 10.);
        let stopped = *app.world().get::<Transform>(ids[0]).unwrap();
        app.world_mut().resource_mut::<Race>().phase = Phase::Paused;
        step(&mut app, 30);
        assert_eq!(*app.world().get::<Transform>(ids[0]).unwrap(), stopped);
        {
            let mut race = app.world_mut().resource_mut::<Race>();
            race.elapsed = 0.;
            race.phase = Phase::Countdown;
        }
        app.update();
        assert_eq!(app.world().get::<Traffic>(ids[0]).unwrap().progress, 6000.);
        assert_eq!(app.world().get::<Traffic>(ids[1]).unwrap().progress, 6025.);
    }
}

#[cfg(test)]
#[path = "camera_tests.rs"]
mod camera_tests;

#[cfg(test)]
#[path = "traffic_stall_tests.rs"]
mod traffic_stall_tests;

#[path = "traffic_stall_survey.rs"]
pub(crate) mod traffic_stall_survey;

#[cfg(test)]
#[path = "civilian_tests.rs"]
mod civilian_tests;
#[cfg(test)]
#[path = "hills_bridge_exit_tests.rs"]
mod hills_bridge_exit_tests;

#[cfg(test)]
#[path = "racer_ai_tests.rs"]
mod racer_ai_tests;

#[path = "racer_ai_survey.rs"]
pub(crate) mod racer_ai_survey;

#[path = "civilian_survey.rs"]
pub(crate) mod civilian_survey;
