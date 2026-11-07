use super::*;
use rr_core::physical_reaction::{Input, Mode};
#[derive(Resource)]
pub(super) struct NativeTuning(pub rr_content::physical_reaction::Config);
impl Default for NativeTuning {
    fn default() -> Self {
        Self(
            rr_content::physical_reaction::Config::from_json(include_str!(
                "../../content/native_crash.json"
            ))
            .expect("validated native crash behavior"),
        )
    }
}
#[derive(Default)]
pub(super) struct Measure {
    speed: f32,
    angular: f32,
    supported: bool,
    severity: f32,
    activation: f32,
    age: f32,
    targets: [Quat; 16],
    frontal_release: bool,
    mass: f32,
    momentum: Vec3,
    root: Transform,
    normal: Vec3,
    near_ground: bool,
    spin: [Vec3; 16],
    reaction: Option<(Mode, f32, f32)>,
    local_velocity: Vec3,
}
/// Reciprocal native constraint motors only. Never writes rigid-body poses or velocities.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(super) fn update(
    settings: Res<Settings>,
    reaction: Res<super::reactions::Tuning>,
    native: Res<NativeTuning>,
    time: Res<Time<Fixed>>,
    race: Option<Res<crate::game::Race>>,
    context: ReadRapierContext,
    controls: Option<Res<crate::controls::Controls>>,
    recovery_tuning: Option<Res<crate::recovery::RecoveryTuning>>,
    riders: Query<&crate::game::Rider>,
    mut forces: MessageReader<ContactForceEvent>,
    mut brains: Query<(&OwnedBody, &mut Brain)>,
    segments: Query<(
        Entity,
        &OwnedBody,
        &Velocity,
        &Segment,
        &Transform,
        &ReadMassProperties,
    )>,
    mut joints: Query<(&OwnedBody, &Segment, &mut ImpulseJoint, &mut Muscle)>,
    bodies: Query<&RigidBody>,
    mut scratch: Local<std::collections::HashMap<Entity, Measure>>,
) {
    if !settings.enabled || race.is_some_and(|r| !r.phase.simulating()) {
        return;
    }
    scratch.clear();
    let context = context.single().ok();
    for (e, o, v, segment, t, mass) in &segments {
        let stats = scratch.entry(o.owner).or_default();
        stats.speed = stats.speed.max(v.linvel.length());
        stats.angular = stats.angular.max(v.angvel.length());
        stats.mass += mass.mass;
        stats.momentum += v.linvel * mass.mass;
        stats.spin[segment.part] = v.angvel;
        if segment.part == 0 {
            stats.root = *t;
        }
        // A real load-bearing torso/pelvis contact, never a wall or an upright foot.
        if matches!(segment.part, 0 | 2 | 11)
            && let Some(context) = &context
        {
            for pair in context.contact_pairs_with(e) {
                let other = if pair.collider1() == Some(e) {
                    pair.collider2()
                } else {
                    pair.collider1()
                };
                if !pair.has_any_active_contact()
                    || !other
                        .is_some_and(|e| bodies.get(e).map_or(true, |b| *b == RigidBody::Fixed))
                {
                    continue;
                }
                for manifold in pair.manifolds() {
                    let normal = manifold.normal();
                    if normal.y.abs() >= 0.6
                        && manifold
                            .solver_contacts()
                            .any(|c| c.point().y < t.translation.y + 0.1)
                    {
                        stats.supported = true;
                        stats.normal += if normal.y > 0.0 { normal } else { -normal };
                    }
                }
            }
        }
    }
    for event in forces.read() {
        for e in [event.collider1, event.collider2] {
            if let Ok((_, o, ..)) = segments.get(e) {
                let stats = scratch.entry(o.owner).or_default();
                stats.severity = stats.severity.max(
                    (event.total_force_magnitude * time.delta_secs() / 70.0 / 25.0).clamp(0.0, 1.0),
                );
            }
        }
    }
    for (owner, mut brain) in &mut brains {
        let Some(stats) = scratch.get_mut(&owner.owner) else {
            continue;
        };
        brain.control.secondary_impact(stats.severity);
        brain.supported = stats.supported;
        brain.control.step(
            time.delta_secs(),
            stats.speed,
            stats.angular,
            stats.supported,
            &settings.config.activation,
        );
        let input_brace = controls
            .as_ref()
            .is_some_and(|c| c.foot.length_squared() > 0.01)
            && riders.get(owner.owner).is_ok_and(|r| r.id == 0)
            && stats.supported
            && brain.control.age >= 0.8;
        stats.activation = if input_brace {
            brain.control.activation.max(
                recovery_tuning
                    .as_ref()
                    .map_or(0.12, |t| t.0.brace_activation),
            )
        } else {
            brain.control.activation
        };
        stats.age = brain.control.age;
        stats.targets = brain.targets;
        stats.frontal_release = brain.frontal_release;
        let velocity = stats.momentum / stats.mass.max(1.0);
        let mut normal = stats.normal.normalize_or_zero();
        if normal == Vec3::ZERO {
            normal = Vec3::Y;
        }
        if let Some(context) = &context {
            let predicate = |e| bodies.get(e).map_or(true, |b| *b == RigidBody::Fixed);
            if let Some((_, hit)) = context.cast_ray_and_get_normal(
                stats.root.translation,
                Vec3::NEG_Y,
                1.8,
                true,
                QueryFilter::default()
                    .exclude_sensors()
                    .predicate(&predicate),
            ) {
                stats.near_ground = hit.normal.y >= 0.6 && velocity.y < -0.1;
                if !stats.supported {
                    normal = hit.normal;
                }
            }
        }
        let facing = stats.root.rotation * Vec3::Z;
        let side = stats.root.rotation * Vec3::X;
        let mode = brain.reaction.step(
            time.delta_secs(),
            Input {
                supported: stats.supported,
                near_ground: stats.near_ground,
                speed: velocity.length(),
                spin: stats.angular,
                facing_up: facing.dot(normal),
                side_up: side.dot(normal),
            },
            native.0.max_speed,
            native.0.max_spin,
            native.0.attempt_seconds,
            native.0.max_attempts,
        );
        let requested = match mode {
            Mode::Protect => {
                native.0.protect_activation
                    * (native.0.max_speed / velocity.length().max(1.0)).clamp(0.2, 1.0)
            }
            Mode::Roll => native.0.roll_activation,
            Mode::Slide => native.0.slide_activation,
            _ => 0.0,
        };
        let spec = specs();
        let rate = settings
            .config
            .joints
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| {
                let parent = spec[i].parent.unwrap();
                c.active_torque * (stats.spin[i] - stats.spin[parent]).abs().element_sum()
            })
            .sum();
        let activation = brain
            .reaction
            .activation(requested, rate, time.delta_secs());
        let local_velocity = stats.root.rotation.inverse() * velocity;
        stats.reaction = Some((mode, activation, brain.reaction.direction));
        // The protection target follows approach direction in the current torso frame.
        stats.local_velocity = local_velocity;
    }
    for (owner, s, mut joint, mut muscle) in &mut joints {
        let Some(stats) = scratch.get(&owner.owner) else {
            continue;
        };
        let c = &settings.config.joints[s.part];
        let mut target = stats.targets[s.part].slerp(
            rotation(c.brace),
            (stats.age / settings.config.activation.impact_seconds).clamp(0.0, 1.0),
        );
        let mut activation = stats.activation;
        if let Some((mode, strength, direction)) = stats.reaction
            && strength > 0.0
            && let Some(pose) = reaction_target(s.part, mode, direction, stats.local_velocity)
        {
            target = rotation(pose);
            activation = activation.max(strength);
        }
        if stats.frontal_release
            && reaction.0.release_activation > 0.0
            && stats.age < reaction.0.release_seconds
            && matches!(s.part, 7..=10)
        {
            let weight = (std::f32::consts::PI * stats.age / reaction.0.release_seconds)
                .sin()
                .max(0.0);
            let angle = if matches!(s.part, 7 | 9) {
                reaction.0.hip_back
            } else {
                reaction.0.knee_back
            };
            target = target.slerp(rotation([angle, 0.0, 0.0]), weight);
            activation = activation.max(reaction.0.release_activation * weight);
        }
        if (muscle.activation - activation).abs() > 0.00001
            || muscle.target.dot(target).abs() < 0.99999
        {
            set_drive(joint.data.as_mut(), c, activation, target);
            muscle.activation = activation;
            muscle.target = target;
        }
    }
}
fn reaction_target(
    part: usize,
    mode: Mode,
    direction: f32,
    local_velocity: Vec3,
) -> Option<[f32; 3]> {
    let side = if matches!(part, 3 | 4 | 7 | 8 | 12 | 14) {
        -1.0
    } else {
        1.0
    };
    match mode {
        Mode::Protect => match part {
            1 => Some([-0.25, 0.0, 0.0]),
            3 | 5 => Some([
                if local_velocity.z >= 0.0 { -0.95 } else { 0.55 },
                0.0,
                (local_velocity.x * 0.08 + side * 0.25).clamp(-0.8, 0.8),
            ]),
            4 | 6 => Some([-0.45, 0.0, 0.0]),
            12 | 13 => Some([0.0, 0.0, 0.0]),
            _ => None,
        },
        Mode::Roll => match part {
            1 => Some([-0.25, 0.0, 0.0]),
            2 | 11 => Some([0.12, direction * 0.25, direction * 0.18]),
            3 | 5 => Some([-0.35, 0.0, side * 0.45 + direction * 0.35]),
            4 | 6 => Some([-0.8, 0.0, 0.0]),
            7 | 9 => Some([
                if side * direction > 0.0 { 0.65 } else { -0.15 },
                0.0,
                side * 0.15,
            ]),
            8 | 10 => Some([if side * direction > 0.0 { 0.9 } else { 0.2 }, 0.0, 0.0]),
            _ => None,
        },
        Mode::Slide => match part {
            1 => Some([-0.3, 0.0, 0.0]),
            3 | 5 => Some([-0.2, 0.0, side * 0.2]),
            4 | 6 => Some([-0.7, 0.0, 0.0]),
            7 | 9 => Some([0.25, 0.0, side * 0.1]),
            8 | 10 => Some([0.45, 0.0, 0.0]),
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn status(
    brains: Query<(&OwnedBody, &Brain)>,
    riders: Query<&crate::game::Rider>,
    mut labels: Query<&mut Text, With<crate::crash_behavior::CompareLabel>>,
) {
    if labels.is_empty() {
        return;
    }
    let state = brains
        .iter()
        .find(|(owner, _)| riders.get(owner.owner).is_ok_and(|r| r.id == 0));
    for mut label in &mut labels {
        let heading = label
            .0
            .lines()
            .next()
            .unwrap_or(crate::crash_behavior::METHOD)
            .to_owned();
        let status = state.map_or_else(
            || "Mounted / recovery".to_owned(),
            |(_, b)| {
                format!(
                    "{:?} / support:{} / attempts:{} / effort estimate:{:.0} J",
                    b.reaction.mode, b.supported, b.reaction.attempts, b.reaction.work_left
                )
            },
        );
        label.0 =
            format!("{heading}\nF8 combat stagger | F9 frontal release | F10 next speed\n{status}");
    }
}
