//! Assisted dynamic chassis adapter: two wheel rays, spring/damper suspension,
//! bounded combined tire forces and torque-based upright/heading assistance.
use crate::{
    game::{Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    track::Track,
    world::Ground,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Component)]
pub struct RideableStep;
#[derive(Resource, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehiclePhysicsAdapter {
    #[default]
    Assisted,
    Dynamic,
}
pub use rr_content::handling::Handling;
#[derive(Resource)]
pub struct Tuning {
    pub presets: Vec<Handling>,
    pub selected: usize,
    pub assist: f32,
}
impl Tuning {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let data = rr_content::handling::HandlingCatalog::from_json(json)?;
        let assist = data.presets[1].road_assist;
        Ok(Self {
            presets: data.presets,
            selected: 1,
            assist,
        })
    }
}
impl Default for Tuning {
    fn default() -> Self {
        let json = std::env::var_os("ROADDASH_HANDLING")
            .map(|p| std::fs::read_to_string(p).expect("read ROADDASH_HANDLING"))
            .unwrap_or_else(|| include_str!("../content/handling.json").into());
        Self::from_json(&json).expect("validated handling data")
    }
}
#[derive(Component, Default, Clone)]
pub struct BikeState {
    pub position: Vec3,
    pub velocity: Vec3,
    pub impact_velocity: Vec3,
    pub yaw: f32,
    pub lean: f32,
    pub slip: f32,
    pub slip_angle: f32,
    pub wheel_forces: [f32; 2],
    pub grip_force: f32,
    pub surface_mu: f32,
    pub applied_tire_force: Vec2,
    pub yaw_rate: f32,
    pub intent: rr_core::VehicleIntent,
    pub grounded: [bool; 2],
    pub suspension: [f32; 2],
    pub rpm: f32,
    pub gear: usize,
    pub damage: f32,
    pub was_riding: bool,
    pub preset: usize,
    pub wheel_spin: f32,
}
#[derive(Component)]
pub struct WheelVisual(pub bool);
#[derive(Resource, Default)]
pub struct VehicleTelemetry {
    pub tick: u64,
    pub samples: std::collections::VecDeque<serde_json::Value>,
}
fn mass(p: &Handling) -> ColliderMassProperties {
    let m = p.dry_mass + 70.0;
    ColliderMassProperties::MassProperties(MassProperties {
        mass: m,
        local_center_of_mass: Vec3::Y * 0.55,
        principal_inertia: Vec3::new(m * 0.55, m * 0.65, m * 0.18),
        principal_inertia_local_frame: Quat::IDENTITY,
    })
}
pub fn attach(
    mut commands: Commands,
    adapter: Res<VehiclePhysicsAdapter>,
    tuning: Res<Tuning>,
    riders: Query<(Entity, &Rider, &Transform), Without<BikeState>>,
) {
    if *adapter != VehiclePhysicsAdapter::Dynamic {
        return;
    }
    for (e, _, t) in &riders {
        let p = &tuning.presets[tuning.selected];
        commands.entity(e).insert((
            BikeState {
                position: t.translation,
                preset: tuning.selected,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::compound(vec![(
                Vec3::Y * 0.75,
                Quat::IDENTITY,
                Collider::cuboid(0.36, 0.28, 1.05),
            )]),
            mass(p),
            Velocity::zero(),
            ExternalForce::default(),
            Ccd::enabled(),
            Sleeping::disabled(),
            Damping {
                linear_damping: 0.02,
                angular_damping: 0.5,
            },
            Friction::coefficient(0.2),
            Restitution::coefficient(0.05),
        ));
    }
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn forces(
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    track: Res<Track>,
    tuning: Res<Tuning>,
    context: ReadRapierContext,
    ground: Query<(), With<Ground>>,
    mut bikes: Query<(
        Entity,
        &mut Rider,
        &Recovery,
        &Transform,
        &mut Velocity,
        &mut ExternalForce,
        &mut RigidBody,
        &mut ColliderMassProperties,
        &mut BikeState,
    )>,
) {
    let Ok(context) = context.single() else {
        return;
    };
    let dt = time.delta_secs();
    for (e, mut r, recovery, t, mut v, mut force, mut body, mut properties, mut state) in &mut bikes
    {
        *force = default();
        state.impact_velocity = v.linvel;
        if recovery.stage != RecoveryStage::Riding
            || race.phase != Phase::Racing
            || r.finished.is_some()
        {
            *body = RigidBody::KinematicPositionBased;
            *v = Velocity::zero();
            state.was_riding = false;
            continue;
        }
        *body = RigidBody::Dynamic;
        let p = &tuning.presets[tuning.selected];
        r.bike_mass = p.dry_mass;
        if state.preset != tuning.selected {
            *properties = mass(p);
            state.preset = tuning.selected;
        }
        if !state.was_riding {
            *v = Velocity::linear(r.motion_velocity.clamp_length_max(80.0));
            state.was_riding = true;
        }
        let (_, road) = track.sample(r.progress, 0.0);
        let forward = (road * Vec3::Z).normalize();
        let up = road * Vec3::Y;
        let right = forward.cross(up).normalize();
        let speed = v.linvel.dot(forward).max(0.0);
        let input = r.drive_intent;
        let player = r.id == 0;
        let at_end = !track.definition.closed && r.progress >= track.length - 0.1;
        let throttle = input.throttle && !at_end;
        let brake = input.brake || at_end;
        let nitro = input.nitro && throttle && r.nitro > 0.5;
        let steer = input.steer;
        let mut normal = Vec3::ZERO;
        let mut load = 0.0;
        state.grounded = [false; 2];
        state.suspension = [0.0; 2];
        state.wheel_forces = [0.0; 2];
        state.yaw_rate = v.angvel.y;
        state.intent = rr_core::VehicleIntent {
            throttle,
            brake,
            steer,
            nitro,
            throttle_amount: input.throttle_strength(),
            brake_amount: input.brake_strength(),
        };
        for (i, z) in [-0.78, 0.78].into_iter().enumerate() {
            let origin = t.translation + t.rotation * Vec3::new(0.0, 0.9, z);
            let predicate = |hit| ground.contains(hit);
            let filter = QueryFilter::default()
                .exclude_rigid_body(e)
                .exclude_sensors()
                .predicate(&predicate);
            if let Some((_, hit)) =
                context.cast_ray_and_get_normal(origin, -Vec3::Y, 1.35, true, filter)
            {
                if (t.rotation * Vec3::Y).dot(hit.normal) < 0.2 {
                    continue;
                }
                let compression = 0.9 - hit.time_of_impact;
                let contact = origin - Vec3::Y * hit.time_of_impact;
                let point_velocity = v.linvel
                    + v.angvel
                        .cross(contact - (t.translation + t.rotation * Vec3::Y * 0.55));
                let f = ((p.dry_mass + 70.0) * 9.81 * 0.5 + p.spring * compression
                    - p.damper * point_velocity.dot(hit.normal))
                .clamp(0.0, (p.dry_mass + 70.0) * 9.81 * 2.5);
                force.force += hit.normal * f;
                force.torque +=
                    (contact - (t.translation + t.rotation * Vec3::Y * 0.55)).cross(hit.normal * f);
                state.grounded[i] = true;
                state.suspension[i] = compression;
                state.wheel_forces[i] = f;
                normal += hit.normal;
                load += f;
            }
        }
        let grounded = state.grounded.iter().any(|v| *v);
        normal = if grounded {
            normal.normalize_or_zero()
        } else {
            up
        };
        let gears = [2.5, 1.9, 1.5, 1.25, 1.0, 0.85];
        let mut gear = state.gear.min(5);
        let rpm_for = |gear: usize| {
            (speed / (std::f32::consts::TAU * 0.4) * 60.0 * gears[gear] * 3.2).max(1200.0)
        };
        if rpm_for(gear) > 7600.0 && gear < 5 {
            gear += 1;
        } else if rpm_for(gear) < 3000.0 && gear > 0 {
            gear -= 1;
        }
        state.gear = gear;
        state.rpm = rpm_for(gear).min(10500.0);
        let limit = if nitro { p.nitro_speed } else { p.top_speed };
        let limit = if player {
            limit
        } else {
            limit
                .min(43.0 + r.id as f32 * 0.8)
                .min((12.0 / track.curvature(r.progress).abs().max(0.0001)).sqrt())
        };
        let surface = track.surface(r.progress, r.lane);
        let limit = if surface == rr_content::track::Surface::Grass {
            limit.min(32.0)
        } else {
            limit
        };
        let torque_curve = if state.rpm >= 10000.0 {
            0.0
        } else {
            0.7 + 0.3 * (1.0 - ((state.rpm - 5500.0) / 5000.0).powi(2)).clamp(0.0, 1.0)
        };
        let engine = if throttle && speed < limit {
            p.torque_nm * input.throttle_strength() * torque_curve * gears[gear] * 3.2 / 0.4
                * (if nitro { 1.8 } else { 1.0 })
                * (1.0 - (speed / limit).powi(8)).max(0.0)
        } else {
            0.0
        };
        let longitudinal = engine
            - if brake {
                (v.linvel.dot(forward) * (p.dry_mass + 70.0) / dt.max(0.001))
                    .clamp(-(p.dry_mass + 70.0) * 10.0, (p.dry_mass + 70.0) * 10.0)
                    * if at_end { 1.0 } else { input.brake_strength() }
            } else if !throttle {
                (v.linvel.dot(forward) * (p.dry_mass + 70.0) / dt.max(0.001))
                    .clamp(-(p.dry_mass + 70.0) * 1.8, (p.dry_mass + 70.0) * 1.8)
            } else {
                0.0
            };
        let desired_side = steer * (1.8 + speed * 0.085) * p.turn_response + r.side_velocity;
        let lateral = ((desired_side - v.linvel.dot(right)) * 3.0
            - track.curvature(r.progress) * speed * speed)
            * (p.dry_mass + 70.0);
        let grip = load * p.grip * surface.traction();
        let requested = Vec2::new(longitudinal, lateral);
        let applied = requested.clamp_length_max(grip);
        state.grip_force = grip;
        state.surface_mu = p.grip * surface.traction();
        state.applied_tire_force = applied;
        state.slip_angle = v.linvel.dot(right).atan2(speed.max(0.5));
        state.slip = if requested.length() > 1.0 {
            (1.0 - grip / requested.length()).clamp(0.0, 1.0)
        } else {
            0.0
        };
        if grounded {
            force.force += forward * applied.x + right * applied.y;
        }
        force.force -= v.linvel * speed * p.drag;
        let current_up = t.rotation * Vec3::Y;
        let upright_axis = if current_up.dot(normal) < -0.5
            && current_up.cross(normal).length_squared() < 0.01
            && speed < 3.0
        {
            t.rotation * Vec3::Z
        } else {
            current_up.cross(normal)
        };
        force.torque += upright_axis * (p.dry_mass + 70.0) * 28.0
            - Vec3::new(v.angvel.x, 0.0, v.angvel.z) * (p.dry_mass + 70.0) * 5.5;
        let heading = (t.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
        let target = (forward + right * steer * 0.12)
            .with_y(0.0)
            .normalize_or_zero();
        let yaw = heading.cross(target).y.atan2(heading.dot(target));
        force.torque.y += (yaw * (4.0 + 12.0 * tuning.assist * p.road_assist) - v.angvel.y * 3.0)
            * (p.dry_mass + 70.0);
        state.lean = ((speed * speed * track.curvature(r.progress) + steer * 3.0) / 9.81)
            .atan()
            .clamp(-1.0, 1.0);
        state.wheel_spin = (state.wheel_spin + speed / 0.4 * dt) % std::f32::consts::TAU;
    }
}
pub fn readback(
    track: Res<Track>,
    race: Res<Race>,
    mut bikes: Query<(&mut Rider, &Recovery, &Transform, &Velocity, &mut BikeState)>,
) {
    for (mut r, recovery, t, v, mut state) in &mut bikes {
        if recovery.stage == RecoveryStage::Riding && race.phase == Phase::Racing {
            let previous = r.progress;
            let (d, lane, _) = track.project(t.translation, previous);
            let d = if track.definition.closed {
                d
            } else {
                d.min(track.length)
            };
            r.gates
                .observe(&track, state.position, t.translation, previous, d);
            r.progress = d;
            r.lane = lane;
            r.speed = v.linvel.with_y(0.0).length();
            r.motion_velocity = v.linvel;
            r.lean = state.lean;
        }
        state.position = t.translation;
        state.velocity = v.linvel;
        state.yaw = (t.rotation * Vec3::Z).x.atan2((t.rotation * Vec3::Z).z);
        state.damage = 100.0 - r.health;
    }
}
pub fn live_tuning(keys: Res<ButtonInput<KeyCode>>, mut tuning: ResMut<Tuning>) {
    if keys.just_pressed(KeyCode::F5) {
        tuning.selected = (tuning.selected + 1) % tuning.presets.len();
        info!("Handling preset: {}", tuning.presets[tuning.selected].id);
    }
    if keys.just_pressed(KeyCode::F6) {
        tuning.assist = if tuning.assist > 0.4 { 0.15 } else { 0.6 };
    }
}
pub fn telemetry(
    race: Res<Race>,
    tuning: Res<Tuning>,
    bikes: Query<(
        &Rider,
        &BikeState,
        Option<&crate::content::ContentIdentity>,
        &Recovery,
    )>,
    mut log: ResMut<VehicleTelemetry>,
    time: Res<Time<Fixed>>,
) {
    if race.phase != Phase::Racing {
        return;
    }
    log.tick += 1;
    if let Some((r, b, identity, recovery)) = bikes.iter().find(|(r, _, _, _)| r.id == 0) {
        if log.samples.len() >= 3600 {
            log.samples.pop_front();
        }
        let tick = log.tick;
        log.samples.push_back(serde_json::json!({"tick":tick,"dt":time.delta_secs(),"bike_id":identity.map_or("prototype-bike-01",|id|id.bike.as_str()),"rider_id":r.id,"health":r.health,"crash_state":format!("{:?}",recovery.stage),"surface_mu":b.surface_mu,"preset":tuning.presets[tuning.selected].id,"position":b.position.to_array(),"velocity":b.velocity.to_array(),"speed_mps":r.speed,"rpm":b.rpm,"gear":b.gear+1,"slip":b.slip,"grounded":b.grounded,"suspension_m":b.suspension,"spring_force_n":b.wheel_forces,"grip_force_n":b.grip_force,"tire_force_n":b.applied_tire_force.to_array(),"slip_angle":b.slip_angle,"yaw_rate":b.yaw_rate,"intent":b.intent,"lean":b.lean,"lane":r.lane}));
    }
}

#[allow(clippy::type_complexity)]
pub fn visuals(
    bikes: Query<&BikeState>,
    roots: Query<&crate::runtime::VisualRoot>,
    mut wheels: Query<(&WheelVisual, &ChildOf, &mut Transform), With<WheelVisual>>,
) {
    for (wheel, parent, mut t) in &mut wheels {
        if let Ok(root) = roots.get(parent.parent())
            && let Ok(state) = bikes.get(root.owner)
        {
            t.translation.y = 0.42 - state.suspension[usize::from(wheel.0)].clamp(-0.2, 0.2);
            t.rotation = Quat::from_rotation_y(if wheel.0 {
                -state.intent.steer * 0.2
            } else {
                0.0
            }) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
                * Quat::from_rotation_y(state.wheel_spin);
        }
    }
}

#[cfg(test)]
#[path = "vehicle_tests.rs"]
mod tests;
