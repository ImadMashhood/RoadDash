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

pub const NAMES: [&str; 8] = [
    "YOU", "VIPER", "GHOST", "ROOK", "NOVA", "JINX", "DUST", "AXEL",
];

use crate::{
    content::{ContentIdentity, GameContent},
    controls::Controls,
};
pub use rr_core::Phase;

#[derive(Resource)]
pub struct Race {
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
            phase: Phase::Garage,
            countdown: 3.0,
            elapsed: 0.0,
            message: String::new(),
            message_time: 0.0,
            takedowns: 0,
            place: 8,
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
#[require(Recovery)]
pub struct Rider {
    pub id: usize,
    pub progress: f32,
    pub lane: f32,
    pub speed: f32,
    pub health: f32,
    pub nitro: f32,
    pub crashed: f32,
    pub immunity: f32,
    pub attack: f32,
    pub attack_side: f32,
    pub lean: f32,
    pub finished: Option<f32>,
    pub hop_height: f32,
    pub hop_velocity: f32,
    pub impact_cooldown: f32,
    pub motion_velocity: Vec3,
    pub gates: crate::track::GateProgress,
    pub hits: rr_core::HitChain,
    pub side_velocity: f32,
    pub drive_intent: rr_core::VehicleIntent,
    pub bike_mass: f32,
    pub palette_index: usize,
    pub steering: rr_core::steering::SteeringMemory,
    pub raw_steer: f32,
}
impl Rider {
    pub(crate) fn new(id: usize) -> Self {
        Self {
            id,
            progress: -8.0 - (id / 2) as f32 * 4.0,
            lane: if id.is_multiple_of(2) { -2.0 } else { 2.0 },
            speed: 0.0,
            health: 100.0,
            nitro: 100.0,
            crashed: 0.0,
            immunity: 2.0,
            attack: 0.0,
            attack_side: 0.0,
            lean: 0.0,
            finished: None,
            hop_height: 0.0,
            hop_velocity: 0.0,
            impact_cooldown: 0.0,
            motion_velocity: Vec3::ZERO,
            gates: default(),
            hits: default(),
            side_velocity: 0.0,
            drive_intent: default(),
            bike_mass: 170.0,
            palette_index: id,
            steering: default(),
            raw_steer: 0.0,
        }
    }
}
#[derive(Component)]
pub struct ChaseCamera;
#[derive(Component)]
pub struct Traffic {
    pub progress: f32,
    pub lane: f32,
    pub speed: f32,
}
#[derive(Component)]
pub struct Debris(pub f32);

pub fn setup_race(
    mut commands: Commands,
    art: Res<Art>,
    track: Res<Track>,
    content: Res<GameContent>,
) {
    commands.insert_resource(Profile::load());
    for id in 0..8 {
        let mut rider = Rider::new(id);
        rider.palette_index = content.0.riders[id].palette_index;
        rider.progress = track.definition.spawns[id].distance;
        rider.lane = track.definition.spawns[id].lane;
        let (p, q) = track.sample(rider.progress, rider.lane);
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
    for i in 0..12 {
        let progress = 100.0 + i as f32 * track.length / 12.0;
        let lane = if i % 3 == 0 { 3.1 } else { -3.1 };
        let speed = if lane > 0.0 { 18.0 } else { -13.0 };
        let root = commands
            .spawn((
                Transform::default(),
                Visibility::default(),
                Traffic {
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
    let (p, q) = track.sample(-8.0, -2.0);
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
    mut riders: Query<(Entity, &mut Rider, &mut Recovery)>,
    mut debris: Query<Entity, With<Debris>>,
    mut commands: Commands,
    mut menu_action: ResMut<crate::ui::MenuAction>,
    screen: Option<Res<State<crate::runtime::AppScreen>>>,
    track: Res<Track>,
    panel: Res<crate::settings::SettingsPanel>,
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
    let activate = std::mem::take(&mut menu_action.0);
    let resume = activate && race.phase == Phase::Paused;
    if std::mem::take(&mut controls.mute) {
        muted.0 = !muted.0;
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
    let start = (std::mem::take(&mut controls.start) || (activate && !resume))
        && matches!(race.phase, Phase::Garage | Phase::Finished);
    let restart = std::mem::take(&mut controls.restart)
        && matches!(race.phase, Phase::Racing | Phase::Paused | Phase::Finished);
    if start || restart {
        let mode = race.camera_mode;
        *race = Race::default();
        race.camera_mode = mode;
        race.phase = Phase::Countdown;
        for (entity, mut rider, mut recovery) in &mut riders {
            let palette = rider.palette_index;
            *rider = Rider::new(rider.id);
            rider.palette_index = palette;
            let spawn = &track.definition.spawns[rider.id];
            rider.progress = spawn.distance;
            rider.lane = spawn.lane;
            *recovery = Recovery::default();
            commands.entity(entity).remove::<ColliderDisabled>();
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
    mut traffic: Query<(&mut Traffic, &mut Transform), Without<Rider>>,
    mut commands: Commands,
    mut profile: ResMut<Profile>,
    (content, persistence, adapter, handling): (
        Res<GameContent>,
        Res<Persistence>,
        Res<crate::vehicle::VehiclePhysicsAdapter>,
        Res<crate::vehicle::Tuning>,
    ),
    mut physics: Query<&mut RapierConfiguration>,
    mut audio: MessageWriter<AudioCue>,
    context: ReadRapierContext,
    ground: Query<(), With<world::Ground>>,
    owned: Query<&recovery::OwnedBody>,
    (mut swept, mut attacks): (
        MessageWriter<recovery::SweptContact>,
        MessageWriter<crate::combat::AttackAttempt>,
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
    let dt = time.delta_secs();
    let dynamic = *adapter == crate::vehicle::VehiclePhysicsAdapter::Dynamic;
    let player_intent = controls.buffer.held;
    let player_attack = controls
        .buffer
        .take_attack()
        .map_or(0.0, |side| side.sign());
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
    let player_progress = snapshots.iter().find(|s| s.1 == 0).map_or(0.0, |s| s.2);

    for (entity, mut r, mut recovery, mut transform, mut visibility) in &mut riders {
        if recovery.stage != RecoveryStage::Riding {
            *visibility = Visibility::Hidden;
            continue;
        }
        *visibility = Visibility::Visible;
        let previous_progress = r.progress;
        let previous_position = transform.translation;
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
            let is_player = r.id == 0;
            let throttle = if is_player {
                player_intent.throttle
            } else {
                true
            };
            let brake = is_player && player_intent.brake;
            let boost = is_player && player_intent.nitro && throttle && r.nitro > 0.5;
            let top = if is_player {
                if boost {
                    tuning.nitro_speed_mps
                } else {
                    tuning.top_speed_mps
                }
            } else {
                43.0 + r.id as f32 * 0.8
            };
            let curve = track.curvature(r.progress).abs();
            let target = if is_player {
                top
            } else {
                top.min(35.0 + 0.42 / curve.max(0.005))
            };
            let on_grass = r.lane.abs() > track.half_width(r.progress);
            let target = if on_grass { target.min(32.0) } else { target };
            if !dynamic {
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
                            * track.surface(r.progress, r.lane).traction())
                    .min(target);
                } else if r.speed > target || !throttle {
                    r.speed = (r.speed - dt * tuning.coast_drag_mps2).max(0.0);
                }
                if brake {
                    r.speed = (r.speed - dt * tuning.brake_mps2 * player_intent.brake_strength())
                        .max(0.0);
                }
            }
            if boost {
                r.nitro = (r.nitro - dt * 25.0).max(0.0);
            } else {
                r.nitro = (r.nitro + dt * 8.0).min(100.0);
            }
            let steer;
            if is_player {
                r.raw_steer = player_intent.steer;
                let speed = r.speed;
                let response = handling.presets[handling.selected].steering;
                steer = r.steering.step(player_intent.steer, speed, dt, response);
                // High speed on a tight curve pushes the rider toward the outside shoulder.
                if !dynamic {
                    r.lane += track.curvature(r.progress) * r.speed * r.speed * dt * 0.025;
                }
            } else {
                let mut desired =
                    ((race.elapsed * 0.28 + r.id as f32 * 2.4).sin() * 3.4).clamp(-4.5, 4.5);
                for (car, _) in &traffic {
                    let ahead = (car.progress - r.progress).rem_euclid(track.length);
                    if ahead < 35.0 && (r.lane - car.lane).abs() < 2.0 {
                        desired = -car.lane;
                    }
                }
                steer = if race.elapsed < 3.0 {
                    0.0
                } else {
                    ((desired - r.lane) * 1.5).clamp(-1.0, 1.0)
                };
            }
            r.drive_intent = rr_core::VehicleIntent {
                throttle,
                brake,
                steer,
                nitro: boost,
                throttle_amount: if is_player {
                    player_intent.throttle_strength()
                } else {
                    1.0
                },
                brake_amount: if is_player {
                    player_intent.brake_strength()
                } else {
                    0.0
                },
            };
            if !dynamic {
                r.lane += r.side_velocity * dt;

                r.lane += steer
                    * handling.presets[handling.selected].turn_response
                    * (tuning.steering_rate_base + r.speed * tuning.steering_rate_speed)
                    * dt;
            }
            r.side_velocity *= (-dt * 2.5).exp();
            r.lean = r
                .lean
                .lerp((steer * 46.0 / 74.0).atan(), 1.0 - (-dt * 16.0).exp());
            if on_grass && !dynamic {
                r.speed = (r.speed - dt * 1.8).max(0.0);
            }
            if !dynamic {
                r.progress += r.speed * dt;
            }
            if !dynamic && !track.definition.closed && r.progress >= track.length {
                r.progress = track.length;
                r.speed = 0.0;
            }
            // Human and AI share damage and cooldown rules. Attack only one side at once.
            let attack_side = if is_player {
                player_attack
            } else if race.elapsed > 3.0
                && r.speed > 6.0
                && (race.elapsed * 2.8 + r.id as f32).sin() > 0.88
            {
                if r.lane > 0.0 { -1.0 } else { 1.0 }
            } else {
                0.0
            };
            if attack_side != 0.0 && r.attack <= 0.0 {
                r.attack = 0.65;
                r.attack_side = attack_side;
                attacks.write(crate::combat::AttackAttempt(entity, attack_side));
            }
            if r.health <= 0.0 {
                let (origin, rotation) = if dynamic {
                    (transform.translation, transform.rotation)
                } else {
                    track.sample(r.progress, r.lane)
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
        let (p, q) = track.sample(r.progress, r.lane);
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
            let predicate = |other| {
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
                destination = transform.translation + delta * hit.time_of_impact.clamp(0.0, 1.0);
                let (progress, lane, _) = track.project(destination, r.progress);
                r.progress = progress;
                r.lane = lane;
                r.hop_height = (destination.y - p.y).max(0.0);
                swept.write(recovery::SweptContact(entity, other));
            }
        }
        if active && recovery.stage == RecoveryStage::Riding {
            let progress = r.progress;
            r.gates.observe(
                &track,
                previous_position,
                destination,
                previous_progress,
                progress,
            );
            if r.gates.completed_laps >= LAPS && r.finished.is_none() {
                r.finished = Some(race.elapsed);
                commands.entity(entity).insert(ColliderDisabled);
                if r.id == 0 {
                    race.place = 1 + snapshots.iter().filter(|s| s.1 != 0 && s.6).count();
                    race.earned = rr_core::race_award(race.place, race.takedowns);
                    profile.races += 1;
                    profile.wins += u32::from(race.place == 1);
                    profile.credits += race.earned;
                    profile.best_seconds = Some(
                        profile
                            .best_seconds
                            .map_or(race.elapsed, |best| best.min(race.elapsed)),
                    );
                    if persistence.0 {
                        profile.save();
                    }
                    race.phase = Phase::Finished;
                }
            }
        }
        if !dynamic || !active {
            if let Ok(ctx) = context.single() {
                let predicate = |e| ground.contains(e);
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
            transform.rotation = q
                * Quat::from_rotation_z(r.lean.clamp(-0.6, 0.6))
                * Quat::from_rotation_x(-r.hop_velocity * 0.025);
        }
    }
    if active && race.phase != Phase::Finished {
        race.place = 1 + riders
            .iter()
            .filter(|(_, r, _, _, _)| r.id != 0 && r.progress > player_progress)
            .count();
    }
    for (mut car, mut transform) in &mut traffic {
        if active {
            car.progress += car.speed * dt;
            if !track.definition.closed {
                if car.progress > track.length + 80.0 {
                    car.progress = -80.0;
                }
                if car.progress < -80.0 {
                    car.progress = track.length + 80.0;
                }
            }
        }
        let (p, q) = track.sample(car.progress, car.lane);
        transform.translation = p;
        transform.rotation = q * if car.speed < 0.0 {
            Quat::from_rotation_y(std::f32::consts::PI)
        } else {
            Quat::IDENTITY
        };
    }
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
    mut cameras: Query<(&mut Transform, &mut Projection), With<ChaseCamera>>,
) {
    let Some((player_entity, player, recovery)) = riders.iter().find(|(_, r, _)| r.id == 0) else {
        return;
    };
    if race.phase == Phase::Paused {
        return;
    }
    let (road, rotation) = track.sample(player.progress, player.lane);
    let recovering = recovery.stage != RecoveryStage::Riding;
    let rotation = if !recovering && race.phase != Phase::Garage {
        bodies.get(player_entity).map_or(rotation, |(t, h)| {
            let pose = crate::runtime::interpolated(t, h, fixed.overstep_fraction());
            let heading = pose.rotation * Vec3::Z;
            Quat::from_rotation_y(heading.x.atan2(heading.z))
        })
    } else {
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
        let snapped = transform.translation.distance_squared(target) > 75.0 * 75.0;
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
            let predicate = |e| !owned.get(e).is_ok_and(|body| body.owner == player_entity);
            let filter = QueryFilter::default()
                .exclude_rigid_body(player_entity)
                .exclude_sensors()
                .predicate(&predicate);
            let proxy = Collider::ball(0.3);
            if let Some((_, hit)) = context.cast_shape(
                focus,
                Quat::IDENTITY,
                delta.normalize_or_zero(),
                proxy.raw.as_ref(),
                ShapeCastOptions {
                    max_time_of_impact: delta.length(),
                    target_distance: 0.02,
                    ..default()
                },
                filter,
            ) {
                position = focus + delta.normalize_or_zero() * (hit.time_of_impact - 0.05).max(0.0);
            }
        }
        transform.translation = position;
        let looking = Transform::from_translation(position)
            .looking_at(focus + rotation * Vec3::Z * look + anticipation, Vec3::Y)
            .rotation;
        transform.rotation = transform
            .rotation
            .slerp(looking, 1.0 - (-dt * follow).exp());
        if let Projection::Perspective(p) = projection.as_mut() {
            p.fov = comfort.fov(player.speed, recovering);
        }
    }
}

/// Avoid a nearby rival's helmet filling the chase camera on a packed grid.
/// This only hides the mesh; combat and collision rules remain active.
pub fn camera_occlusion(
    camera: Query<&Transform, With<ChaseCamera>>,
    mut riders: Query<(&Rider, &Transform, &mut Visibility), Without<ChaseCamera>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    for (rider, transform, mut visibility) in &mut riders {
        if rider.id != 0
            && camera
                .translation
                .distance(transform.translation + Vec3::Y * 1.3)
                < 3.0
        {
            *visibility = Visibility::Hidden;
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::track::ROAD_HALF;
    #[test]
    fn collision_gap_wraps_at_finish() {
        assert_eq!(wrapped_gap(998.0, 1000.0), -2.0);
        assert_eq!(wrapped_gap(-998.0, 1000.0), 2.0);
        assert_eq!(wrapped_gap(12.0, 1000.0), 12.0);
    }
    #[test]
    fn grid_starts_before_timing_line() {
        for id in 0..8 {
            let r = Rider::new(id);
            assert!(r.progress < 0.0);
            assert!(r.lane.abs() < ROAD_HALF);
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
            cube: Handle::default(),
            sphere: Handle::default(),
            wheel: Handle::default(),
            dark: Handle::default(),
            metal: Handle::default(),
            white: Handle::default(),
            colors: vec![Handle::default(); 8],
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
        assert!(player(&mut app).nitro < 80.0);
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
            step(&mut app, 1);
            assert!((app.world().get::<Rider>(rival_id).unwrap().health - expected).abs() < 0.001);
            step(&mut app, 20);
        }
        assert!(app.world().get::<Rider>(rival_id).unwrap().crashed > 0.0);
        assert_eq!(app.world().resource::<Race>().takedowns, 1);
    }
    #[test]
    fn traffic_impact_crashes_and_restart_clears_debris() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        {
            let mut q = app.world_mut().query::<&mut Rider>();
            q.single_mut(app.world_mut()).unwrap().immunity = 0.0;
        }
        app.world_mut().spawn((
            Traffic {
                progress: -8.0,
                lane: -2.0,
                speed: -13.0,
            },
            Transform::default(),
            RigidBody::KinematicPositionBased,
            Collider::compound(vec![(
                Vec3::Y,
                Quat::IDENTITY,
                Collider::cuboid(0.95, 0.9, 1.95),
            )]),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ));
        step(&mut app, 3);
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
            .spawn((crate::ui::StartButton, Interaction::Pressed));
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
    fn fast_rear_contact_ragdolls_victim_without_launching_the_attacker() {
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
            assert!(app.world().get_entity(runner).is_err());
            let torso = app.world().get::<Recovery>(owner).unwrap().torso.unwrap();
            assert!(
                app.world().get::<Velocity>(torso).unwrap().linvel.length() < 100.0,
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
        assert!(
            rider.crashed > 0.0,
            "slow sideways contact failed to trigger an impact"
        );
        assert!(
            rider.lane < -1.6,
            "bike passed into the car before the crash: lane {}",
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
            let count = app.world().entities().len();
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
                                .map(|(body, v, t)| (body.kind, v.linvel, t.translation)),
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
        app.world_mut().entity_mut(wreck).insert((
            Transform::from_translation(Vec3::new(0.0, 10.5, 0.0)).with_rotation(tilt),
            Velocity::zero(),
        ));
        step(&mut app, 120);
        let p = app.world().get::<Transform>(wreck).unwrap().translation;
        assert!(p.x < -1.0 && p.y < 9.5, "wreck stuck on slope: {p:?}");
    }
    #[test]
    fn a_cow_lifts_a_rider_without_a_terrain_only_knockoff() {
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
        assert!(player(&mut app).hop_velocity > 0.0);
        assert_eq!(player(&mut app).crashed, 0.0);
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
                    t.translation.is_finite() && t.rotation.is_finite() && v.linvel.is_finite()
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
}

#[cfg(test)]
#[path = "camera_tests.rs"]
mod camera_tests;
