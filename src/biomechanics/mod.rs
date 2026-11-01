//! Optional anatomical physical articulation. The previous rig remains a selectable control.
use crate::recovery::{BodyKind, OwnedBody, Recovery};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_core::biomechanics::CrashControl;
mod debug;
pub mod lab;
mod muscles;
#[cfg(test)]
mod tests;
pub mod view;
#[derive(Resource, Clone)]
pub struct Settings {
    pub enabled: bool,
    pub debug: bool,
    pub config: rr_content::biomechanics::Config,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            debug: false,
            config: rr_content::biomechanics::Config::from_json(include_str!(
                "../../content/biomechanics.json"
            ))
            .expect("validated muscle configuration"),
        }
    }
}
impl Settings {
    pub fn selected() -> Result<Self, String> {
        let args: Vec<_> = std::env::args().collect();
        let mut s = Self {
            enabled: args.iter().any(|a| a == "--ragdoll-biomechanical")
                || std::env::var("ROADDASH_RAGDOLL").as_deref() == Ok("biomechanical"),
            debug: args.iter().any(|a| a == "--ragdoll-debug"),
            ..default()
        };
        if let Some(i) = args.iter().position(|a| a == "--ragdoll-config") {
            let p = args
                .get(i + 1)
                .ok_or("--ragdoll-config requires JSON path")?;
            s.config = rr_content::biomechanics::Config::from_json(
                &std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"))?,
            )?;
        }
        Ok(s)
    }
}
#[derive(Component, Clone, Copy)]
pub(crate) struct Candidate(pub usize);
#[derive(Component, Clone, Copy)]
pub struct Segment {
    pub part: usize,
}
#[derive(Component)]
pub struct Brain {
    pub control: CrashControl,
    pub targets: [Quat; 16],
    pub supported: bool,
}
#[derive(Component)]
pub struct Muscle {
    pub activation: f32,
    pub target: Quat,
}
#[derive(Component, Clone, Copy)]
pub struct ImpactSeverity(pub f32);
#[derive(Component, Clone, Copy)]
pub(crate) struct EjectionAngular(pub Vec3, pub bool);
#[derive(Clone, Copy)]
pub struct Spec {
    pub position: Vec3,
    pub size: Vec3,
    pub parent: Option<usize>,
    pub anchor: Vec3,
}
pub fn specs() -> [Spec; 16] {
    let mut s = [Spec {
        position: Vec3::ZERO,
        size: Vec3::ONE,
        parent: None,
        anchor: Vec3::ZERO,
    }; 16];
    for (i, (p, size, parent)) in crate::human::SPECS.iter().enumerate() {
        s[i] = Spec {
            position: *p,
            size: *size,
            parent: parent.map(|v| v.0),
            anchor: parent.map_or(Vec3::ZERO, |v| v.1),
        };
    }
    s[2].parent = Some(11);
    s[2].anchor = Vec3::new(0.0, 1.22, 0.0);
    s[11] = Spec {
        position: Vec3::new(0.0, 1.30, 0.0),
        size: Vec3::new(0.36, 0.18, 0.25),
        parent: Some(0),
        anchor: Vec3::new(0.0, 1.40, 0.0),
    };
    for (i, parent, side) in [(12, 4, -1.0), (13, 6, 1.0)] {
        s[i] = Spec {
            position: Vec3::new(side * 0.49, 0.88, 0.0),
            size: Vec3::new(0.16, 0.16, 0.12),
            parent: Some(parent),
            anchor: Vec3::new(side * 0.49, 0.97, 0.0),
        };
        s[parent].size.y = 0.30;
    }
    for (i, parent, side) in [(14, 8, -1.0), (15, 10, 1.0)] {
        s[i] = Spec {
            position: Vec3::new(side * 0.17, 0.065, 0.08),
            size: Vec3::new(0.20, 0.12, 0.32),
            parent: Some(parent),
            anchor: Vec3::new(side * 0.17, 0.11, 0.0),
        };
        s[parent].size.y = 0.34;
    }
    s
}
pub fn shape(s: Spec) -> Collider {
    // Core segments/feet keep their full-width boxes; head/limbs use capsules.
    if s.parent.is_none() || s.size.y < s.size.z {
        Collider::cuboid(s.size.x * 0.5, s.size.y * 0.5, s.size.z * 0.5)
    } else {
        let r = s.size.x.min(s.size.z) * 0.45;
        Collider::capsule_y((s.size.y * 0.5 - r).max(0.005), r)
    }
}
const AXES: [JointAxis; 3] = [JointAxis::AngX, JointAxis::AngY, JointAxis::AngZ];
/// Rapier's angular coordinates are 2*asin(relative quaternion imaginary part),
/// not Euler angles. Match that solver chart with the quaternion hemisphere w>=0.
pub fn coordinates(q: Quat) -> Vec3 {
    let q = if q.w < 0.0 { -q } else { q };
    q.xyz()
        .clamp(Vec3::splat(-1.0), Vec3::ONE)
        .map(|v| 2.0 * v.asin())
}
fn rotation(v: [f32; 3]) -> Quat {
    Quat::from_euler(EulerRot::XYZ, v[0], v[1], v[2])
}
pub fn joint(config: &rr_content::biomechanics::Config, part: usize) -> GenericJoint {
    let s = specs();
    let c = &config.joints[part];
    let parent = s[part].parent.expect("non-root articulation");
    let mut j = GenericJointBuilder::new(if c.hinge {
        JointAxesMask::LOCKED_REVOLUTE_AXES
    } else {
        JointAxesMask::LIN_AXES
    })
    .local_anchor1(s[part].anchor - s[parent].position)
    .local_anchor2(s[part].anchor - s[part].position)
    .build();
    for (i, axis) in AXES.into_iter().enumerate() {
        if c.hinge && i > 0 {
            continue;
        }
        j.set_limits(axis, c.limits[i]);
    }
    j.set_contacts_enabled(false);
    set_drive(&mut j, c, 0.0, rotation(c.brace));
    j
}
fn set_drive(
    j: &mut GenericJoint,
    c: &rr_content::biomechanics::Joint,
    activation: f32,
    target: Quat,
) {
    let a = activation.clamp(0.0, 1.0);
    let rest = coordinates(rotation(c.rest));
    let active = coordinates(target);
    let k = c.passive_k + a * c.active_k;
    let d = c.passive_d + a * c.active_d;
    let target = (rest * c.passive_k + active * (a * c.active_k)) / k;
    for (i, axis) in AXES.into_iter().enumerate() {
        if c.hinge && i > 0 {
            continue;
        }
        j.set_motor_model(axis, MotorModel::ForceBased);
        j.set_motor_position(axis, target[i].clamp(c.limits[i][0], c.limits[i][1]), k, d);
        j.set_motor_max_force(axis, c.passive_torque + a * c.active_torque);
    }
}
/// Remove authoring bind axes before using skin orientations as physical targets.
/// Physical segments start in the common identity frame; skin bones do not.
fn bind_frames() -> &'static [Transform; 11] {
    static BIND: std::sync::OnceLock<[Transform; 11]> = std::sync::OnceLock::new();
    BIND.get_or_init(|| {
        rr_content::riding_model::Bindings::from_json(include_str!(
            "../../assets/models/rider-bindings.json"
        ))
        .expect("validated bind frames")
        .standing
        .map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)))
    })
}
fn animation_targets(frames: [Transform; 11]) -> [Quat; 11] {
    let bind = bind_frames();
    let physical: [Quat; 11] =
        std::array::from_fn(|i| (frames[i].rotation * bind[i].rotation.inverse()).normalize());
    std::array::from_fn(|i| {
        crate::human::SPECS[i].2.map_or(physical[i], |(p, _)| {
            (physical[p].inverse() * physical[i]).normalize()
        })
    })
}
fn limited_rotation(q: Quat, c: &rr_content::biomechanics::Joint) -> Quat {
    let v = coordinates(q);
    let v = Vec3::new(
        v.x.clamp(c.limits[0][0], c.limits[0][1]),
        v.y.clamp(c.limits[1][0], c.limits[1][1]),
        v.z.clamp(c.limits[2][0], c.limits[2][1]),
    )
    .map(|v| (v * 0.5).sin());
    Quat::from_xyzw(v.x, v.y, v.z, (1.0 - v.length_squared()).max(0.0).sqrt()).normalize()
}
/// Construct the initial articulated pose with coincident anchors, never by
/// teleporting a body which has already entered the physics simulation.
fn release_pose(
    config: &rr_content::biomechanics::Config,
    targets: &[Quat],
    captured: Option<[Transform; 11]>,
    origin: Vec3,
    yaw: Quat,
) -> [Transform; 16] {
    let s = specs();
    let mut pose =
        s.map(|s| Transform::from_translation(origin + yaw * s.position).with_rotation(yaw));
    if let Some(frames) = captured {
        let root_offset = bind_frames()[0]
            .to_matrix()
            .inverse()
            .transform_point3(s[0].position);
        pose[0] =
            Transform::from_translation(origin + yaw * frames[0].transform_point(root_offset))
                .with_rotation((yaw * targets[0]).normalize());
        for i in [1, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15] {
            let p = s[i].parent.unwrap();
            let rotation =
                (pose[p].rotation * limited_rotation(targets[i], &config.joints[i])).normalize();
            pose[i] = Transform::from_translation(
                pose[p].transform_point(s[i].anchor - s[p].position)
                    - rotation * (s[i].anchor - s[i].position),
            )
            .with_rotation(rotation);
        }
    }
    pose
}
/// Upgrade immediately after existing spawn/ejection; never change the baseline path.
fn initialize(world: &mut World) {
    if !world.resource::<Settings>().enabled {
        return;
    }
    let mut groups = std::collections::BTreeMap::<Entity, Vec<(usize, Entity)>>::new();
    let mut query = world.query_filtered::<(Entity, &Candidate, &OwnedBody), Without<Segment>>();
    for (e, c, o) in query.iter(world) {
        groups.entry(o.owner).or_default().push((c.0, e));
    }
    if groups.is_empty() {
        return;
    }
    let settings = world.resource::<Settings>().clone();
    for (owner, mut entries) in groups {
        entries.sort_by_key(|v| v.0);
        if entries.len() != 11 {
            continue;
        }
        let mut entities: Vec<_> = entries.iter().map(|e| e.1).collect();
        let root = entities[0];
        let transform = *world.get::<Transform>(root).unwrap();
        let q = transform.rotation;
        let s = specs();
        let origin = transform.translation - q * s[0].position;
        let velocity = world
            .get::<Velocity>(root)
            .copied()
            .unwrap_or_default()
            .linvel;
        let omega = world
            .get::<EjectionAngular>(owner)
            .map(|w| w.0)
            .filter(|w| w.is_finite())
            .unwrap_or_else(|| {
                world.get::<Velocity>(owner).map_or(Vec3::ZERO, |v| {
                    if v.angvel.is_finite() {
                        v.angvel
                    } else {
                        Vec3::ZERO
                    }
                })
            });
        let mut targets = settings
            .config
            .joints
            .iter()
            .map(|j| rotation(j.brace))
            .collect::<Vec<_>>();
        let mut captured = None;
        if let (Some(a), Some(lib)) = (
            world.get::<crate::animation::Animator>(owner),
            world.get_resource::<crate::animation::Library>(),
        ) && a.ready
            && world.get::<EjectionAngular>(owner).is_none_or(|s| s.1)
        {
            let frames = lib.frames(a.current);
            captured = Some(frames);
            let rotations = animation_targets(frames);
            targets[..11].copy_from_slice(&rotations);
            let spine = targets[2];
            targets[2] = Quat::IDENTITY.slerp(spine, 0.5);
            targets[11] = targets[2];
        }
        let poses = release_pose(&settings.config, &targets, captured, origin, q);
        let com = poses
            .iter()
            .zip(settings.config.masses)
            .map(|(t, m)| t.translation * m)
            .sum::<Vec3>()
            / 70.0;
        for _ in 11..16 {
            entities.push(world.spawn_empty().id());
        }
        for (i, &e) in entities.iter().enumerate() {
            let t = poses[i];
            let v = Velocity {
                linvel: velocity + omega.cross(t.translation - com),
                angvel: omega,
            };
            world.entity_mut(e).insert((
                Segment { part: i },
                OwnedBody {
                    owner,
                    kind: BodyKind::Ragdoll,
                },
                t,
                RigidBody::Dynamic,
                shape(s[i]),
                ColliderMassProperties::Mass(settings.config.masses[i]),
                v,
                Damping {
                    linear_damping: settings.config.linear_damping,
                    angular_damping: settings.config.angular_damping,
                },
                Sleeping::default(),
                Ccd::enabled(),
                Friction::coefficient(0.8),
                Restitution::coefficient(0.02),
                ReadMassProperties::default(),
                crate::game::Debris(f32::INFINITY),
                ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
            ));
            world.entity_mut(e).insert((
                SoftCcd { prediction: 0.35 },
                ContactForceEventThreshold(100.0),
            ));
            world.entity_mut(e).remove::<Ccd>();
            if i > 0 {
                world.entity_mut(e).insert((
                    ImpulseJoint::new(
                        entities[s[i].parent.unwrap()],
                        TypedJoint::GenericJoint(joint(&settings.config, i)),
                    ),
                    Muscle {
                        activation: -1.0,
                        target: targets[i],
                    },
                ));
            }
            if world.get::<crate::recovery::CrashEnvelope>(root).is_some() {
                world.entity_mut(e).insert(
                    crate::recovery::CrashEnvelope::new(
                        v.linvel,
                        t.translation,
                        t.rotation,
                        q * Vec3::Z,
                    )
                    .with_angular_velocity(omega),
                );
            }
        }
        let severity = world
            .entity_mut(owner)
            .take::<ImpactSeverity>()
            .map_or((velocity.length() / 65.0).clamp(0.1, 1.0), |s| s.0);
        world.entity_mut(root).insert((
            Brain {
                control: CrashControl::impact(severity, &settings.config.activation),
                targets: targets.try_into().unwrap(),
                supported: false,
            },
            AdditionalSolverIterations(settings.config.extra_solver_iterations),
        ));
        if let Some(wreck) = world.get::<Recovery>(owner).and_then(|r| r.wreck) {
            if let Some(mut v) = world.get_mut::<Velocity>(wreck) {
                v.angvel = omega;
            }
            if let Some(mut envelope) = world.get_mut::<crate::recovery::CrashEnvelope>(wreck) {
                *envelope = envelope.with_angular_velocity(omega);
            }
        }
        if let Some(mut r) = world.get_mut::<Recovery>(owner) {
            r.parts.extend_from_slice(&entities[11..]);
        }
        if let Some(mut n) = world.get_mut::<crate::npc::Npc>(owner) {
            n.parts.extend_from_slice(&entities[11..]);
        }
    }
}
pub struct BiomechanicsPlugin;
impl Plugin for BiomechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>().add_systems(
            FixedUpdate,
            (initialize, muscles::update)
                .chain()
                .after(crate::runtime::SimSet::Gameplay)
                .before(PhysicsSet::SyncBackend),
        );
        // Lab uses MinimalPlugins and does not install rendering resources.
    }
}
pub fn install_debug(app: &mut App) {
    app.add_systems(Update, debug::draw);
}
