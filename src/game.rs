use crate::{
    recovery::{self, Recovery, RecoveryStage},
    sound::{AudioCue, Cue},
};
use crate::{
    track::{LAPS, ROAD_HALF, Track},
    world::{self, Art},
};
use bevy::{camera::Exposure, prelude::*};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub const NAMES: [&str; 8] = [
    "YOU", "VIPER", "GHOST", "ROOK", "NOVA", "JINX", "DUST", "AXEL",
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Garage,
    Countdown,
    Racing,
    Paused,
    Finished,
}

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

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct Profile {
    pub races: u32,
    pub wins: u32,
    pub credits: u32,
    pub best_seconds: Option<f32>,
}
impl Profile {
    pub fn load() -> Self {
        match std::fs::read(save_path()) {
            Ok(bytes) => match serde_json::from_slice(&bytes) {
                Ok(profile) => profile,
                Err(e) => {
                    warn!("Ignoring invalid save: {e}");
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }
    fn save(&self) {
        let path = save_path();
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
}
impl Rider {
    fn new(id: usize) -> Self {
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

pub fn setup_race(mut commands: Commands, art: Res<Art>, track: Res<Track>) {
    commands.insert_resource(Profile::load());
    for id in 0..8 {
        let rider = Rider::new(id);
        let (p, q) = track.sample(rider.progress, rider.lane);
        let root = world::bike(&mut commands, &art, id, true);
        commands.entity(root).insert((
            rider,
            Transform::from_translation(p).with_rotation(q),
            recovery::bike_body(),
        ));
    }
    for i in 0..12 {
        let progress = 100.0 + i as f32 * track.length / 12.0;
        let lane = if i % 3 == 0 { 3.1 } else { -3.1 };
        let speed = if lane > 0.0 { -13.0 } else { 18.0 };
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
        commands.entity(root).add_children(&parts);
    }
    let (p, q) = track.sample(-8.0, -2.0);
    commands.spawn((
        Camera3d::default(),
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

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut race: ResMut<Race>,
    mut riders: Query<(Entity, &mut Rider, &mut Recovery)>,
    mut debris: Query<Entity, With<Debris>>,
    mut commands: Commands,
    mut menu_action: ResMut<crate::ui::MenuAction>,
) {
    let activate = std::mem::take(&mut menu_action.0);
    let resume = activate && race.phase == Phase::Paused;
    if keys.just_pressed(KeyCode::KeyC) {
        race.camera_mode = (race.camera_mode + 1) % 3;
    }
    if keys.just_pressed(KeyCode::Escape) || resume {
        race.phase = match race.phase {
            Phase::Racing => Phase::Paused,
            Phase::Paused => Phase::Racing,
            p => p,
        };
    }
    let start = (keys.just_pressed(KeyCode::Enter) || (activate && !resume))
        && matches!(race.phase, Phase::Garage | Phase::Finished);
    let restart = keys.just_pressed(KeyCode::KeyR)
        && matches!(race.phase, Phase::Racing | Phase::Paused | Phase::Finished);
    if start || restart {
        let mode = race.camera_mode;
        *race = Race::default();
        race.camera_mode = mode;
        race.phase = Phase::Countdown;
        for (entity, mut rider, mut recovery) in &mut riders {
            *rider = Rider::new(rider.id);
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
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
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
    persistence: Res<Persistence>,
    mut physics: Query<&mut RapierConfiguration>,
    mut audio: MessageWriter<AudioCue>,
    context: ReadRapierContext,
    ground: Query<(), With<world::Ground>>,
    owned: Query<&recovery::OwnedBody>,
    mut swept: MessageWriter<recovery::SweptContact>,
) {
    // Physics has its own clock: pause debris together with the race.
    for mut config in &mut physics {
        config.physics_pipeline_active =
            !matches!(race.phase, Phase::Paused | Phase::Garage | Phase::Finished);
    }
    if race.phase == Phase::Paused || race.phase == Phase::Finished {
        return;
    }
    let dt = time.delta_secs().min(0.05);
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
    let snapshots: Vec<(Entity, usize, f32, f32, f32, bool)> = riders
        .iter()
        .map(|(e, r, _, _, _)| (e, r.id, r.progress, r.lane, r.speed, r.crashed > 0.0))
        .collect();
    let player_progress = snapshots.iter().find(|s| s.1 == 0).map_or(0.0, |s| s.2);
    let mut hits: Vec<(Entity, f32, bool)> = Vec::new();
    for (entity, mut r, mut recovery, mut transform, mut visibility) in &mut riders {
        if recovery.stage != RecoveryStage::Riding {
            *visibility = Visibility::Hidden;
            continue;
        }
        *visibility = Visibility::Visible;
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
                keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp)
            } else {
                true
            };
            let brake =
                is_player && (keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown));
            let boost = is_player
                && (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight))
                && throttle
                && r.nitro > 0.5;
            let top = if is_player {
                if boost { 75.0 } else { 55.0 }
            } else {
                43.0 + r.id as f32 * 0.8
            };
            let curve = track.curvature(r.progress).abs();
            let target = if is_player {
                top
            } else {
                top.min(35.0 + 0.42 / curve.max(0.005))
            };
            if throttle {
                r.speed = (r.speed + dt * if boost { 22.0 } else { 11.0 }).min(target);
            } else {
                r.speed = (r.speed - dt * 4.0).max(0.0);
            }
            if brake {
                r.speed = (r.speed - dt * 28.0).max(0.0);
            }
            if boost {
                r.nitro = (r.nitro - dt * 25.0).max(0.0);
            } else {
                r.nitro = (r.nitro + dt * 8.0).min(100.0);
            }
            let steer;
            if is_player {
                steer = (keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight)) as u8
                    as f32
                    - (keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft)) as u8
                        as f32;
                // High speed on a tight curve pushes the rider toward the outside shoulder.
                r.lane += track.curvature(r.progress) * r.speed * r.speed * dt * 0.10;
            } else {
                let mut desired =
                    ((race.elapsed * 0.28 + r.id as f32 * 2.4).sin() * 3.4).clamp(-4.5, 4.5);
                for (car, _) in &traffic {
                    let ahead = (car.progress - r.progress).rem_euclid(track.length);
                    if ahead < 35.0 && (r.lane - car.lane).abs() < 2.0 {
                        desired = -car.lane;
                    }
                }
                steer = ((desired - r.lane) * 1.5).clamp(-1.0, 1.0);
            }
            r.lane += steer * (1.5 + r.speed * 0.09) * dt;
            r.lean = r.lean.lerp(
                steer * 0.40 - track.curvature(r.progress) * r.speed * 0.55,
                (dt * 8.0).min(1.0),
            );
            if r.lane.abs() > ROAD_HALF {
                r.speed = (r.speed - dt * 18.0).max(0.0);
                r.health -= dt * 12.0;
            }
            r.progress += r.speed * dt;
            // Human and AI share damage and cooldown rules. Attack only one side at once.
            let attack_side = if is_player {
                if keys.just_pressed(KeyCode::KeyJ) {
                    -1.0
                } else if keys.just_pressed(KeyCode::KeyK) {
                    1.0
                } else {
                    0.0
                }
            } else if (race.elapsed * 1.7 + r.id as f32).sin() > 0.94 {
                if r.lane > 0.0 { -1.0 } else { 1.0 }
            } else {
                0.0
            };
            if attack_side != 0.0 && r.attack <= 0.0 {
                r.attack = 0.65;
                r.attack_side = attack_side;
                for &(other, _, progress, lane, _, crashed) in &snapshots {
                    let gap = wrapped_gap(progress - r.progress, track.length);
                    if other != entity
                        && !crashed
                        && gap.abs() < 3.2
                        && (lane - r.lane).abs() < 2.8
                        && (lane - r.lane) * attack_side > 0.0
                    {
                        hits.push((other, 38.0, is_player));
                        if is_player {
                            race.message = "HIT / KEEP THE PRESSURE ON".into();
                            race.message_time = 1.2;
                        }
                    }
                }
            }
            if r.health <= 0.0 {
                let (origin, rotation) = track.sample(r.progress, r.lane);
                recovery::crash(
                    &mut commands,
                    &art,
                    entity,
                    &mut r,
                    &mut recovery,
                    origin,
                    rotation,
                    Vec3::ZERO,
                );
                *visibility = Visibility::Hidden;
                audio.write(AudioCue::new(Cue::Crash, origin));
                if is_player {
                    race.shake = 1.0;
                    race.message = "WIPED OUT / GETTING BACK UP".into();
                    race.message_time = 3.5;
                }
            }
            if r.progress >= track.length * LAPS as f32 && r.finished.is_none() {
                r.finished = Some(race.elapsed);
                if is_player {
                    race.place = 1 + snapshots
                        .iter()
                        .filter(|s| s.1 != 0 && s.2 >= track.length * LAPS as f32)
                        .count();
                    race.earned = (9 - race.place as u32) * 250 + race.takedowns * 100;
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
        let (p, q) = track.sample(r.progress, r.lane);
        let mut destination = p + Vec3::Y * r.hop_height;
        let delta = destination - transform.translation;
        r.motion_velocity = if active && dt > 0.0 {
            delta / dt
        } else {
            Vec3::ZERO
        };
        if active
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
        transform.translation = destination;
        transform.rotation =
            q * Quat::from_rotation_z(r.lean) * Quat::from_rotation_x(-r.hop_velocity * 0.025);
    }
    for (victim, damage, player_attack) in hits {
        if let Ok((_, mut r, mut recovery, transform, mut visibility)) = riders.get_mut(victim)
            && r.immunity <= 0.0
            && r.crashed <= 0.0
        {
            r.health -= damage;
            audio.write(AudioCue::new(Cue::Punch, transform.translation));
            if r.health <= 0.0 {
                recovery::crash(
                    &mut commands,
                    &art,
                    victim,
                    &mut r,
                    &mut recovery,
                    transform.translation,
                    transform.rotation,
                    Vec3::ZERO,
                );
                *visibility = Visibility::Hidden;
                audio.write(AudioCue::new(Cue::Crash, transform.translation));
                if r.id != 0 && player_attack {
                    race.takedowns += 1;
                    race.message = "TAKEDOWN / +100 CREDITS".into();
                    race.message_time = 2.0;
                } else if r.id == 0 {
                    race.message = "KNOCKED OFF / GETTING BACK UP".into();
                    race.message_time = 3.5;
                    race.shake = 1.0;
                }
            }
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
    time: Res<Time>,
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

pub fn camera(
    time: Res<Time>,
    race: Res<Race>,
    track: Res<Track>,
    riders: Query<(&Rider, &Recovery)>,
    bodies: Query<&Transform, Without<ChaseCamera>>,
    context: ReadRapierContext,
    mut cameras: Query<(&mut Transform, &mut Projection), With<ChaseCamera>>,
) {
    let Some((player, recovery)) = riders.iter().find(|(r, _)| r.id == 0) else {
        return;
    };
    if race.phase == Phase::Paused {
        return;
    }
    let (road, rotation) = track.sample(player.progress, player.lane);
    let recovering = recovery.stage != RecoveryStage::Riding;
    let focus = if recovering {
        recovery
            .torso
            .or(recovery.runner)
            .and_then(|e| bodies.get(e).ok())
            .map_or(road + Vec3::Y, |t| t.translation + Vec3::Y * 0.3)
    } else {
        road + Vec3::Y * (1.2 + player.hop_height)
    };
    let (offset, look) = if recovering {
        (Vec3::new(0.0, 3.0, -6.0), 0.0)
    } else {
        match race.camera_mode {
            1 => (Vec3::new(0.0, 1.3, -4.2), 3.0),
            2 => (Vec3::new(0.0, 15.8, -18.0), 8.0),
            _ => (Vec3::new(0.0, 2.0, -5.8 - player.speed * 0.012), 4.0),
        }
    };
    let dt = time.delta_secs().min(0.05);
    for (mut transform, mut projection) in &mut cameras {
        let shake = Vec3::new(
            (time.elapsed_secs() * 81.0).sin(),
            (time.elapsed_secs() * 63.0).cos(),
            0.0,
        ) * race.shake
            * 0.15;
        let target = if race.phase == Phase::Garage {
            road + rotation * Vec3::new(5.0, 3.0, -6.0)
        } else {
            focus
                + rotation * (offset + Vec3::Z * if recovering { 0.0 } else { player.speed * 0.10 })
                + shake
        };
        let mut position = transform.translation.lerp(target, 1.0 - (-dt * 10.0).exp());
        // Prevent the camera itself from passing through solid scenery.
        if let Ok(context) = context.single() {
            let delta = position - focus;
            let filter = QueryFilter::from(
                QueryFilterFlags::EXCLUDE_DYNAMIC
                    | QueryFilterFlags::EXCLUDE_KINEMATIC
                    | QueryFilterFlags::EXCLUDE_SENSORS,
            );
            if let Some((_, distance)) = context.cast_ray(
                focus,
                delta.normalize_or_zero(),
                delta.length(),
                true,
                filter,
            ) {
                position = focus + delta.normalize_or_zero() * (distance - 0.3).max(0.5);
            }
        }
        transform.translation = position;
        let looking = Transform::from_translation(position)
            .looking_at(focus + rotation * Vec3::Z * look, Vec3::Y)
            .rotation;
        transform.rotation = transform.rotation.slerp(looking, 1.0 - (-dt * 9.0).exp());
        if let Projection::Perspective(p) = projection.as_mut() {
            p.fov = (60.0 + if recovering { 0.0 } else { player.speed * 0.17 }).to_radians();
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
mod tests {
    use super::*;
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
    fn simulation() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ))
        .init_resource::<ButtonInput<KeyCode>>()
        .add_message::<AudioCue>()
        .add_message::<recovery::SweptContact>()
        .init_resource::<Track>()
        .init_resource::<Race>()
        .init_resource::<Profile>()
        .insert_resource(Persistence(false))
        .init_resource::<crate::ui::MenuAction>()
        .insert_resource(Art {
            cube: Handle::default(),
            sphere: Handle::default(),
            wheel: Handle::default(),
            dark: Handle::default(),
            metal: Handle::default(),
            white: Handle::default(),
            colors: vec![Handle::default(); 8],
        })
        .add_systems(
            Update,
            (input, recovery::impacts, recovery::recover, drive).chain(),
        );
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
        for expected in [62.0, 24.0, 0.0] {
            {
                let mut q = app.world_mut().query::<&mut Rider>();
                for mut r in q.iter_mut(app.world_mut()) {
                    r.attack = 0.0;
                    r.progress = -8.0;
                    r.speed = 0.0;
                }
            }
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyJ);
            keys.press(KeyCode::KeyJ);
            step(&mut app, 1);
            assert_eq!(app.world().get::<Rider>(rival_id).unwrap().health, expected);
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
            q.single_mut(app.world_mut()).unwrap().progress = finish + 1.0;
        }
        step(&mut app, 1);
        assert!(app.world().resource::<Race>().phase == Phase::Finished);
        assert_eq!(app.world().resource::<Race>().place, 1);
        assert_eq!(app.world().resource::<Profile>().credits, 2000);
        step(&mut app, 5);
        assert_eq!(app.world().resource::<Profile>().races, 1);
        app.add_systems(Update, crate::ui::buttons.before(input));
        app.world_mut()
            .spawn((crate::ui::StartButton, Interaction::Pressed));
        step(&mut app, 1);
        assert!(app.world().resource::<Race>().phase == Phase::Countdown);
    }
    #[test]
    fn d_moves_right_and_a_moves_left_in_chase_camera_space() {
        let mut app = simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let (origin, q) = app.world().resource::<Track>().sample(-8.0, -2.0);
        let screen_right = (q * Vec3::Z).cross(Vec3::Y);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        step(&mut app, 1);
        let (d, lane) = {
            let r = player(&mut app);
            (r.progress, r.lane)
        };
        let right = app.world().resource::<Track>().sample(d, lane).0;
        assert!((right - origin).dot(screen_right) > 0.0, "D moved left");
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.release(KeyCode::KeyD);
            keys.press(KeyCode::KeyA);
        }
        step(&mut app, 1);
        let r = player(&mut app);
        let (d, lane) = (r.progress, r.lane);
        let left = app.world().resource::<Track>().sample(d, lane).0;
        assert!((left - right).dot(screen_right) < 0.0, "A moved right");
    }
    #[test]
    fn bike_contact_ragdolls_the_victim_and_launches_attacker_with_a_bounce() {
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
        assert!(player(&mut app).hop_velocity > 4.0);
        let mut highest: f32 = 0.0;
        let mut saw_down = false;
        let mut bounced = false;
        for _ in 0..120 {
            step(&mut app, 1);
            let r = player(&mut app);
            highest = highest.max(r.hop_height);
            if r.hop_velocity < -1.0 {
                saw_down = true;
            }
            if saw_down && r.hop_velocity > 0.5 {
                bounced = true;
            }
        }
        assert!(
            (0.9..1.8).contains(&highest),
            "unexpected launch height {highest}"
        );
        assert!(bounced, "bike did not bounce on landing");
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
        app.add_systems(PostUpdate, camera.after(PhysicsSet::Writeback));
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
            app.world_mut().get_mut::<Rider>(owner).unwrap().crashed = 1.0;
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
        step(&mut app, 30);
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
}
