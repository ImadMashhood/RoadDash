//! Physical crashes, on-foot recovery, and solid-object impact rules.
use crate::{
    game::{Debris, Phase, Race, Rider, Traffic},
    sound::{AudioCue, Cue},
    track::Track,
    world::{self, Art, Obstacle},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum RecoveryStage {
    #[default]
    Riding,
    Falling,
    GettingUp,
    Running,
    Mounting,
}
#[derive(Component, Default)]
pub struct Recovery {
    pub active_crash_behavior: bool,
    pub stagger: Option<rr_core::crash_behavior::Stagger>,
    pub defer_allowed: bool,
    pub deferred: Option<crate::offscreen_crash::Deferred>,
    pub stage: RecoveryStage,
    pub elapsed: f32,
    pub torso: Option<Entity>,
    pub parts: Vec<Entity>,
    pub wreck: Option<Entity>,
    pub runner: Option<Entity>,
    pub pose: Option<Entity>,
    pub walk_path: Vec<Vec3>,
    pub walk_goal: Option<Vec3>,
    pub path_refresh: f32,
    pub walk_watch: rr_core::escape::WalkWatch,
    pub walk_live_retry: f32,
    pub rescues: u32,
    pub last_rescue_stationary: f32,
    pub rescue_penalty_total: f32,
    pub foot: rr_core::recovery::FootControl,
    pub previous_foot: Option<Vec3>,
    pub walk_distance: f32,
    pub walk_velocity: Vec3,
    pub landing: Option<Vec<Transform>>,
}
#[derive(Resource)]
pub struct RecoveryTuning(pub rr_content::recovery::Config);
impl Default for RecoveryTuning {
    fn default() -> Self {
        let mut config =
            rr_content::recovery::Config::from_json(include_str!("../content/recovery.json"))
                .expect("validated recovery tuning");
        if std::env::args().any(|a| a == "--manual-recovery") {
            config.auto_seek = false;
        }
        Self(config)
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BodyKind {
    Ragdoll,
    Wreck,
    Runner,
}
#[derive(Component, Clone, Copy)]
#[require(PreviousMotion)]
pub struct OwnedBody {
    pub owner: Entity,
    pub kind: BodyKind,
}

/// Share the unchanged capsule step/slope settings with physical regressions.
pub(crate) fn runner_controller() -> KinematicCharacterController {
    KinematicCharacterController {
        translation: Some(Vec3::ZERO),
        custom_mass: Some(70.0),
        apply_impulse_to_dynamic_bodies: false,
        offset: CharacterLength::Absolute(0.02),
        snap_to_ground: Some(CharacterLength::Absolute(0.3)),
        autostep: Some(CharacterAutostep {
            max_height: CharacterLength::Absolute(0.3),
            min_width: CharacterLength::Absolute(0.15),
            include_dynamic_bodies: false,
        }),
        ..default()
    }
}

pub fn bike_body() -> impl Bundle {
    (
        RigidBody::KinematicPositionBased,
        Collider::compound(vec![(
            Vec3::new(0.0, 1.0, 0.0),
            Quat::IDENTITY,
            Collider::cuboid(0.38, 0.96, 1.12),
        )]),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
        Friction::coefficient(0.7),
    )
}

/// Closest reachable point on the actual rotated bike collision box, not its
/// displaced model pivot (which can be high above an upside-down bike).
pub fn bike_contact_point(bike: &Transform, person: Vec3) -> Vec3 {
    let center = Vec3::new(0.0, 0.85, 0.0);
    let half = Vec3::new(0.38, 0.8, 1.12);
    let local = bike.rotation.inverse() * (person - bike.translation);
    bike.translation + bike.rotation * (center + (local - center).clamp(-half, half))
}

fn detached_origin(
    context: &ReadRapierContext,
    origin: Vec3,
    rotation: Quat,
    owner: Entity,
    old_runner: Option<Entity>,
) -> Vec3 {
    let Ok(context) = context.single() else {
        return origin + rotation * Vec3::X * 1.25;
    };
    let shape = Collider::cuboid(0.65, 1.1, 0.45);
    let predicate = |e| Some(e) != old_runner;
    let filter = QueryFilter::default()
        .exclude_rigid_body(owner)
        .exclude_sensors()
        .predicate(&predicate);
    for offset in [
        Vec3::X * 1.25,
        -Vec3::X * 1.25,
        Vec3::Z * 2.2,
        -Vec3::Z * 2.2,
        Vec3::new(1.5, 0.0, 1.5),
        Vec3::new(-1.5, 0.0, 1.5),
        Vec3::new(1.5, 0.0, -1.5),
        Vec3::new(-1.5, 0.0, -1.5),
        Vec3::X * 2.5,
        -Vec3::X * 2.5,
    ] {
        let point = origin + rotation * offset;
        let mut blocked = false;
        context.intersect_shape(
            point + Vec3::Y * 1.25,
            rotation,
            shape.raw.as_ref(),
            filter,
            |_| {
                blocked = true;
                false
            },
        );
        if !blocked {
            return point;
        }
    }
    // Confined collisions still use momentum and gravity; never inject a launch.
    origin
}

/// Passive tissue approximation: anatomical stops, low-strength restorative
/// torque and velocity damping. Hinge axes cannot twist like ball joints.
fn anatomical_joint(part: usize, anchor1: Vec3, anchor2: Vec3) -> GenericJoint {
    let hinge = matches!(part, 4 | 6 | 8 | 10);
    let mut joint = GenericJointBuilder::new(if hinge {
        JointAxesMask::LOCKED_REVOLUTE_AXES
    } else {
        JointAxesMask::LIN_AXES
    })
    .local_anchor1(anchor1)
    .local_anchor2(anchor2)
    .build();
    let limits = match part {
        1 => [[-0.6, 0.7], [-0.75, 0.75], [-0.45, 0.45]],
        2 => [[-0.45, 0.6], [-0.3, 0.3], [-0.3, 0.3]],
        3 => [[-1.5, 1.8], [-0.5, 0.5], [-2.2, 0.35]],
        5 => [[-1.5, 1.8], [-0.5, 0.5], [-0.35, 2.2]],
        4 | 6 => [[-2.35, 0.03], [0.0, 0.0], [0.0, 0.0]],
        8 | 10 => [[-0.03, 2.45], [0.0, 0.0], [0.0, 0.0]],
        _ => [[-1.6, 0.5], [-0.35, 0.35], [-0.5, 0.5]],
    };
    for (axis, range) in [JointAxis::AngX, JointAxis::AngY, JointAxis::AngZ]
        .into_iter()
        .zip(limits)
    {
        if hinge && axis != JointAxis::AngX {
            continue;
        }
        joint.set_limits(axis, range);
        let rest = if hinge && part < 7 {
            -0.15
        } else if hinge {
            0.15
        } else {
            0.0
        };
        joint.set_motor_model(axis, MotorModel::ForceBased);
        joint.set_motor_position(axis, rest, if hinge { 3.0 } else { 5.0 }, 1.8);
        joint.set_motor_max_force(axis, if hinge { 8.0 } else { 12.0 });
    }
    joint.set_contacts_enabled(false);
    joint
}
pub fn humanoid(
    commands: &mut Commands,
    art: &Art,
    owner: Entity,
    color: usize,
    origin: Vec3,
    rotation: Quat,
    velocity: Vec3,
) -> Vec<Entity> {
    humanoid_with_prop(
        commands, art, owner, color, origin, rotation, velocity, None,
    )
}

/// Optional civilian appearance uses the same eleven authoritative bodies.
#[allow(clippy::too_many_arguments)]
pub fn humanoid_with_prop(
    commands: &mut Commands,
    art: &Art,
    owner: Entity,
    color: usize,
    origin: Vec3,
    rotation: Quat,
    velocity: Vec3,
    appearance: Option<&str>,
) -> Vec<Entity> {
    let specs = crate::human::SPECS;
    let mut bodies = Vec::new();
    let mut visual_joints = Vec::new();
    for (i, (p, size, parent)) in specs.iter().enumerate() {
        let e = commands
            .spawn((
                Visibility::default(),
                Transform::from_translation(origin + rotation * *p).with_rotation(rotation),
                RigidBody::Dynamic,
                Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
                Velocity {
                    linvel: velocity,
                    angvel: Vec3::ZERO,
                },
                ColliderMassProperties::Mass(match i {
                    0 => 14.0,
                    1 => 4.5,
                    2 => 9.0,
                    3 | 5 => 2.0,
                    4 | 6 => 1.5,
                    7 | 9 => 7.0,
                    _ => 4.0,
                }),
                Damping {
                    linear_damping: 0.2,
                    angular_damping: 1.5,
                },
                Ccd::enabled(),
                Friction::coefficient(0.8),
                Debris(f32::INFINITY),
                OwnedBody {
                    owner,
                    kind: BodyKind::Ragdoll,
                },
                ActiveEvents::COLLISION_EVENTS,
                Restitution::coefficient(0.12),
            ))
            .id();
        commands.entity(e).insert(crate::biomechanics::Candidate(i));
        // Child geometry keeps the physics transform unscaled.
        let visual = if appearance.is_some() && art.props.is_some() {
            commands
                .spawn((
                    crate::human::HumanSegment(i),
                    Transform::IDENTITY,
                    Visibility::default(),
                ))
                .id()
        } else {
            crate::human::segment(commands, art, color, i, Vec3::ZERO, Quat::IDENTITY)
        };
        visual_joints.push(visual);
        crate::runtime::attach_visuals(commands, e, &[visual]);
        if let Some((parent, anchor)) = parent {
            let joint = anatomical_joint(i, *anchor - specs[*parent].0, *anchor - *p);
            commands.entity(e).insert(ImpulseJoint::new(
                bodies[*parent],
                TypedJoint::GenericJoint(joint),
            ));
        }
        bodies.push(e);
    }
    if let (Some(props), Some(name)) = (&art.props, appearance) {
        let skin = props.skin(commands, props.named(name), &visual_joints);
        commands.entity(bodies[0]).add_child(skin);
    } else {
        crate::models::ragdoll(commands, art, owner, color, &visual_joints);
    }
    bodies
}

/// A real articulated rigid-body ragdoll. Adjacent body parts cannot collide;
/// ball joints constrain translation while allowing the limbs to tumble.
#[allow(clippy::too_many_arguments)]
pub fn crash(
    commands: &mut Commands,
    art: &Art,
    owner: Entity,
    rider: &mut Rider,
    recovery: &mut Recovery,
    origin: Vec3,
    rotation: Quat,
    kick: Vec3,
    context: &ReadRapierContext,
) {
    crash_with_response(
        commands, art, owner, rider, recovery, origin, rotation, kick, context, None,
    );
}

#[allow(clippy::too_many_arguments)]
fn crash_with_response(
    commands: &mut Commands,
    art: &Art,
    owner: Entity,
    rider: &mut Rider,
    recovery: &mut Recovery,
    origin: Vec3,
    rotation: Quat,
    kick: Vec3,
    context: &ReadRapierContext,
    original: Option<(Vec3, Vec3)>,
) {
    if recovery.deferred.is_some()
        || crate::offscreen_crash::start(commands, owner, rider, recovery, origin)
    {
        return;
    }
    for part in recovery.parts.drain(..) {
        commands.entity(part).despawn();
    }
    let old_runner = recovery.runner.take();
    commands
        .entity(owner)
        .insert(crate::biomechanics::EjectionAngular(
            if old_runner.is_some() {
                Vec3::ZERO
            } else {
                rider.motion_angular_velocity
            },
            old_runner.is_none(),
        ));
    if let Some(runner) = old_runner {
        commands.entity(runner).despawn();
    }
    recovery.pose = None;
    recovery.foot = default();
    recovery.walk_distance = 0.0;
    recovery.walk_velocity = Vec3::ZERO;
    recovery.previous_foot = None;
    recovery.landing = None;
    recovery.walk_path.clear();
    recovery.walk_goal = None;
    recovery.path_refresh = 0.0;
    recovery.walk_watch = default();
    recovery.walk_live_retry = 0.;
    let forward = (rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
    let rotation = Quat::from_rotation_y(forward.x.atan2(forward.z));
    let momentum = original.map_or_else(
        || {
            if rider.motion_velocity.is_finite() && rider.motion_velocity.length_squared() > 0.001 {
                rider.motion_velocity
            } else {
                forward * rider.speed + Vec3::Y * rider.hop_velocity
            }
        },
        |(_, bike)| bike,
    );
    rider.steering = default();
    rider.raw_steer = 0.0;
    let velocity = original.map_or(momentum + kick, |(rider, _)| rider);
    let continuous = recovery.active_crash_behavior && old_runner.is_none();
    recovery.stagger = None;
    let rider_origin = if continuous {
        origin
    } else {
        detached_origin(context, origin, rotation, owner, old_runner)
    };
    let separation_side = if (rider_origin - origin).dot(rotation * Vec3::X) < 0.0 {
        -1.0
    } else {
        1.0
    };
    // Release into an unstable lean, with zero spin: gravity topples the wreck.
    // Tilting lowers its centre of mass instead of adding an angular impulse.
    let wreck_rotation = rotation * Quat::from_rotation_z(0.45 * separation_side);
    let bodies = humanoid(
        commands,
        art,
        owner,
        rider.palette_index,
        rider_origin,
        rotation,
        velocity,
    );
    let wreck = if let Some(wreck) = recovery.wreck {
        commands.entity(wreck).insert(RigidBody::Dynamic);
        wreck
    } else {
        let wreck = world::bike(commands, art, rider.palette_index, false);
        commands.entity(wreck).insert((
            Transform::from_translation(origin).with_rotation(wreck_rotation),
            RigidBody::Dynamic,
            Collider::compound(vec![(
                Vec3::new(0.0, 0.85, 0.0),
                Quat::IDENTITY,
                Collider::cuboid(0.22, 0.65, 1.12),
            )]),
            ColliderMassProperties::Mass(rider.bike_mass),
            Velocity {
                linvel: momentum,
                angvel: Vec3::ZERO,
            },
            Ccd::enabled(),
            Restitution::coefficient(0.25),
            Friction {
                coefficient: 0.8,
                combine_rule: CoefficientCombineRule::Min,
            },
            Sleeping::disabled(),
            Damping {
                linear_damping: 0.25,
                angular_damping: 0.8,
            },
            Debris(f32::INFINITY),
            OwnedBody {
                owner,
                kind: BodyKind::Wreck,
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
        wreck
    };
    if continuous {
        // ColliderDisabled does not exclude the hidden chassis from KCC casts.
        // The new solid wreck replaces this mounted-only collision proxy.
        commands.queue(move |world: &mut World| {
            if let Some(collider) = world.entity_mut(owner).take::<Collider>() {
                world
                    .entity_mut(owner)
                    .insert(crate::crash_behavior::DormantChassis(collider));
            }
        });
        commands
            .entity(owner)
            .insert(crate::crash_behavior::ReleaseGrace {
                wreck,
                remaining: 0.65,
            });
        commands
            .entity(wreck)
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS);
    }
    // Collision entry has no artificial angular kick. Physical contacts may
    // subsequently tumble the bodies within their original joint constraints.
    for (i, body) in bodies.iter().enumerate() {
        commands.entity(*body).insert((
            Velocity::linear(velocity),
            ReadMassProperties::default(),
            CrashEnvelope::new(
                velocity,
                rider_origin + rotation * crate::human::SPECS[i].0,
                rotation,
                forward,
            )
            .with_original_direction(original.is_some()),
        ));
    }
    commands.entity(wreck).insert((
        ReadMassProperties::default(),
        CrashEnvelope::new(momentum, origin, wreck_rotation, forward)
            .with_original_direction(original.is_some()),
    ));
    if original.is_some() {
        commands.entity(wreck).insert(Velocity {
            linvel: momentum,
            angvel: Vec3::ZERO,
        });
    }
    recovery.torso = Some(bodies[0]);
    recovery.parts = bodies;
    recovery.wreck = Some(wreck);
    recovery.stage = RecoveryStage::Falling;
    recovery.elapsed = 0.0;
    rider.crashed = 1.0;
    rider.health = 0.0;
    rider.speed = 0.0;
    rider.hop_height = 0.0;
    rider.hop_velocity = 0.0;
    commands.entity(owner).insert(ColliderDisabled);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn recover(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut race: ResMut<Race>,
    mut controls: ResMut<crate::controls::Controls>,
    (tuning, adapter, police, network_inputs): (
        Res<RecoveryTuning>,
        Res<crate::vehicle::VehiclePhysicsAdapter>,
        Res<crate::police::Police>,
        Option<Res<crate::online::AuthorityInputs>>,
    ),
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut profile: ResMut<crate::game::Profile>,
    persistence: Res<crate::game::Persistence>,
    track: Res<Track>,
    art: Res<Art>,
    mut riders: Query<(
        Entity,
        &mut Rider,
        &mut Recovery,
        &mut Transform,
        &mut Visibility,
    )>,
    mut bodies: Query<
        (
            &mut Transform,
            Option<&mut Velocity>,
            Option<&mut KinematicCharacterController>,
            Option<&mut RigidBody>,
        ),
        Without<Rider>,
    >,
    (context, controller_outputs, nav_ground, nav_owned): (
        ReadRapierContext,
        Query<&KinematicCharacterControllerOutput>,
        Query<(), With<crate::world::Ground>>,
        Query<&OwnedBody>,
    ),
    mut audio: MessageWriter<AudioCue>,
    layout: Option<Res<crate::world_gen::layout::WorldLayout>>,
    (city, metro_navigation): (
        Option<Res<crate::world_gen::city::GeneratedCity>>,
        Option<Res<crate::world_gen::metropolis::navigation::Navigation>>,
    ),
) {
    if race.phase != Phase::Racing {
        return;
    }
    let dt = time.delta_secs();
    let config = &tuning.0;
    let input = controls.foot.clamp_length_max(1.0);
    let interact = std::mem::take(&mut controls.interact);
    let finishers = riders
        .iter()
        .filter(|(_, r, _, _, _)| !r.police && r.id != 0 && r.finished.is_some())
        .count();
    let pursuit_body = riders
        .iter()
        .find(|(_, r, _, _, _)| r.id == 0)
        .map(|(e, _, rec, _, _)| rec.runner.unwrap_or(e));
    let mut walk_plan_budget = 1usize;
    for (owner, mut rider, mut recovery, mut root, mut visibility) in &mut riders {
        let remote_input = network_inputs.as_ref().and_then(|n| n.get(rider.id));
        let input = remote_input.map_or(input, |i| Vec2::from_array(i.foot).clamp_length_max(1.));
        let interact = remote_input.map_or(interact, |i| i.interact);
        if recovery.stage == RecoveryStage::Riding || rider.finished.is_some() {
            continue;
        }
        *visibility = Visibility::Hidden;
        recovery.elapsed += dt;
        if crate::offscreen_crash::finish(
            &mut commands,
            owner,
            &mut rider,
            &mut recovery,
            &mut root,
            &mut visibility,
            &track,
        ) {
            continue;
        }
        let Some(wreck) = recovery.wreck else {
            continue;
        };
        let Ok((wreck_transform, wreck_velocity, _, _)) = bodies.get_mut(wreck) else {
            continue;
        };
        let wreck_pose = *wreck_transform;
        let wreck_pos = wreck_pose.translation;
        let wreck_speed = wreck_velocity.map_or(0.0, |v| v.linvel.length());
        let (_, road_rotation) = track.sample_route(rider.route, rider.progress, rider.lane);
        match recovery.stage {
            RecoveryStage::Falling => {
                let Some(torso) = recovery.torso else {
                    continue;
                };
                let Ok((torso_transform, velocity, _, _)) = bodies.get_mut(torso) else {
                    continue;
                };
                let torso_pos = torso_transform.translation;
                let settled = velocity.is_some_and(|v| v.linvel.length() < 3.0);
                if recovery.elapsed < 1.8 || (!settled && recovery.elapsed < 7.0) {
                    continue;
                }
                // Find the actual supporting surface, including a tree or a car roof.
                let floor = context
                    .single()
                    .ok()
                    .and_then(|c| {
                        c.cast_ray(
                            torso_pos + Vec3::Y * 0.2,
                            -Vec3::Y,
                            4.0,
                            true,
                            QueryFilter::from(
                                QueryFilterFlags::EXCLUDE_DYNAMIC
                                    | QueryFilterFlags::EXCLUDE_SENSORS,
                            ),
                        )
                    })
                    .map(|(_, t)| torso_pos.y + 0.2 - t);
                let Some(floor) = floor else {
                    continue;
                };
                if torso_pos.y - floor > 1.6 {
                    continue;
                }
                let foot = Vec3::new(torso_pos.x, floor + 0.03, torso_pos.z);
                let (runner, pose) = world::runner(&mut commands, &art, rider.palette_index);
                commands.entity(runner).insert((
                    Transform::from_translation(foot + Vec3::Y * 0.95).with_rotation(road_rotation),
                    RigidBody::KinematicPositionBased,
                    Collider::capsule_y(0.65, 0.28),
                    runner_controller(),
                    OwnedBody {
                        owner,
                        kind: BodyKind::Runner,
                    },
                    Debris(f32::INFINITY),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::all(),
                ));
                let mut landing = Vec::with_capacity(recovery.parts.len());
                for part in &recovery.parts {
                    if let Ok((t, _, _, _)) = bodies.get_mut(*part) {
                        landing.push(*t);
                    }
                }
                recovery.landing = Some(landing);
                recovery.previous_foot = Some(foot + Vec3::Y * 0.95);
                for part in recovery.parts.drain(..) {
                    commands.entity(part).despawn();
                }
                recovery.torso = None;
                recovery.runner = Some(runner);
                recovery.pose = Some(pose);
                recovery.stage = RecoveryStage::GettingUp;
                recovery.elapsed = 0.0;
            }
            RecoveryStage::GettingUp => {
                if let Some(pose) = recovery.pose
                    && let Ok((mut t, _, _, _)) = bodies.get_mut(pose)
                {
                    let fraction = (recovery.elapsed / 0.9).min(1.0);
                    t.scale.y = if art.models.is_some() {
                        1.0
                    } else {
                        0.4 + 0.6 * fraction
                    };
                    t.translation.y = -0.95;
                }
                if recovery.elapsed >= 0.9 {
                    recovery.stage = RecoveryStage::Running;
                    recovery.elapsed = 0.0;
                }
            }
            RecoveryStage::Running => {
                let Some(runner) = recovery.runner else {
                    continue;
                };
                let Ok((mut transform, _, mut controller, _)) = bodies.get_mut(runner) else {
                    continue;
                };
                let previous = recovery.previous_foot.unwrap_or(transform.translation);
                let traveled = previous.xz().distance(transform.translation.xz());
                recovery.walk_velocity = (transform.translation - previous) / dt;
                let previous_step = (recovery.walk_distance / (config.stride_length * 0.5)) as u32;
                recovery.walk_distance += traveled;
                recovery.previous_foot = Some(transform.translation);
                rider.route = track.choose_route(
                    rider.route,
                    rider.progress,
                    transform.translation,
                    rider.human_controlled(),
                );
                let (before, _, _) = track.project_route(rider.route, previous, rider.progress);
                let (progress, lane, _) =
                    track.project_route(rider.route, transform.translation, rider.progress);
                let route = rider.route;
                rider.gates.observe_route(
                    &track,
                    route,
                    previous - Vec3::Y * 0.95,
                    transform.translation - Vec3::Y * 0.95,
                    before,
                    progress,
                );
                rider.progress = progress;
                rider.lane = lane;
                if crate::game::finish_rider(
                    &mut rider,
                    &mut race,
                    &mut profile,
                    persistence.0,
                    1 + finishers,
                    true,
                ) {
                    recovery.foot.mode = rr_core::recovery::FootMode::Finished;
                    if let Some(mut c) = controller {
                        c.translation = Some(Vec3::ZERO);
                    }
                    commands.entity(runner).insert(ColliderDisabled);
                    continue;
                }
                let manual = rider.human_controlled()
                    && recovery.foot.step(
                        input.length_squared() > 0.01,
                        config.auto_seek,
                        dt,
                        config.neutral_delay,
                    ) == rr_core::recovery::FootMode::Manual;
                let hills_seek_rescue = if track.is_hills() && rider.human_controlled() {
                    recovery.walk_watch.seek_rescue_due(
                        dt,
                        transform.translation.xz().to_array(),
                        !manual && !rider.police,
                        config.rescue_after,
                    )
                } else {
                    false
                };
                if manual
                    && transform.translation.y
                        < track.terrain_height(transform.translation.x, transform.translation.z)
                            - 8.0
                {
                    let distance = rider
                        .progress
                        .clamp(0.0, (track.race_interval().finish - 6.0).max(0.0));
                    let (safe, rotation) = track.sample_route(rider.route, distance, 0.0);
                    transform.translation = safe + Vec3::Y * 0.98;
                    transform.rotation = rotation;
                    recovery.previous_foot = Some(transform.translation);
                    if let Some(mut c) = controller {
                        c.translation = Some(Vec3::ZERO);
                    }
                    race.elapsed += config.rescue_penalty;
                    recovery.rescues += 1;
                    recovery.last_rescue_stationary = 0.;
                    recovery.rescue_penalty_total += config.rescue_penalty;
                    race.message = "ON-FOOT RESCUE".into();
                    race.message_time = 3.0;
                    continue;
                }
                let contact = bike_contact_point(&wreck_pose, transform.translation);
                let pursuit = rider.police
                    && police.intentional_foot
                    && police
                        .target
                        .is_some_and(|(t, _)| t.downed || t.speed <= police.rules.arrest_speed)
                    && police.foot_goal.is_some();
                let center = if pursuit {
                    police.foot_goal.unwrap() + Vec3::Y * 0.95
                } else {
                    wreck_pose.translation + wreck_pose.rotation * Vec3::Y * 0.85
                };
                let delta = (center - transform.translation).with_y(0.0);
                let mut direction = delta.normalize_or_zero();
                let mut live_hold = false;
                if !manual
                    && !rider.human_controlled()
                    && let Ok(ctx) = context.single()
                {
                    recovery.walk_live_retry = (recovery.walk_live_retry - dt).max(0.);
                    let p = transform.translation.xz();
                    let stalled = recovery.walk_watch.stalled(dt, p.to_array());
                    let pursuit_owner = pursuit_body
                        .and_then(|e| nav_owned.get(e).ok().map(|o| o.owner))
                        .or(pursuit_body);
                    let clear = |a: [f32; 2], b: [f32; 2]| {
                        let from = Vec3::new(a[0], track.terrain_height(a[0], a[1]) + 0.98, a[1]);
                        let to = Vec3::new(b[0], track.terrain_height(b[0], b[1]) + 0.98, b[1]);
                        let delta = to - from;
                        let predicate = |e| {
                            e != runner
                                && e != wreck
                                && !(pursuit
                                    && (Some(e) == pursuit_body
                                        || nav_owned
                                            .get(e)
                                            .is_ok_and(|o| Some(o.owner) == pursuit_owner)))
                                && !nav_ground.contains(e)
                                && nav_owned.get(e).map_or(true, |o| o.owner != owner)
                        };
                        ctx.cast_shape(
                            from,
                            Quat::IDENTITY,
                            delta.normalize_or_zero(),
                            Collider::capsule_y(0.55, 0.38).raw.as_ref(),
                            ShapeCastOptions {
                                max_time_of_impact: delta.length(),
                                stop_at_penetration: false,
                                ..default()
                            },
                            QueryFilter::default()
                                .exclude_collider(runner)
                                .predicate(&predicate),
                        )
                        .is_none()
                    };
                    let next = recovery.walk_path.first().copied().unwrap_or(center).xz();
                    let blocked = !clear(
                        p.to_array(),
                        (p + (next - p).clamp_length_max(2.)).to_array(),
                    );
                    if (stalled || blocked)
                        && recovery.walk_live_retry == 0.
                        && walk_plan_budget > 0
                    {
                        walk_plan_budget -= 1;
                        recovery.walk_live_retry = 1.5;
                        let goal = p + (center.xz() - p).clamp_length_max(65.);
                        recovery.walk_path =
                            rr_core::escape::route(p.to_array(), goal.to_array(), clear)
                                .unwrap_or_default()
                                .into_iter()
                                .map(|[x, z]| Vec3::new(x, track.terrain_height(x, z) + 0.98, z))
                                .collect();
                        recovery.walk_goal = Some(center);
                        recovery.path_refresh = 2.;
                    }
                    live_hold = blocked && recovery.walk_path.is_empty();
                }
                if !manual && (layout.is_some() || city.is_some() || metro_navigation.is_some()) {
                    recovery.path_refresh -= dt;
                    if recovery.path_refresh <= 0.0
                        && (recovery.walk_goal.is_none_or(|p| p.distance(center) > 2.0)
                            || recovery.walk_path.is_empty())
                    {
                        recovery.walk_path = if let Some(ref metro) = metro_navigation {
                            metro.route(&track, transform.translation, center)
                        } else if let Some(ref layout) = layout {
                            crate::world_gen::navigation::route(
                                layout,
                                &track,
                                transform.translation,
                                center,
                            )
                        } else {
                            crate::world_gen::navigation::route_rects(
                                &city.as_ref().expect("city navigation").blockers,
                                &track,
                                transform.translation,
                                center,
                            )
                        }
                        .unwrap_or_default();
                        recovery.walk_goal = Some(center);
                        recovery.path_refresh = 2.0;
                    }
                    if let Some(next) = recovery.walk_path.first().copied() {
                        if next.xz().distance(transform.translation.xz()) < 0.9 {
                            recovery.walk_path.remove(0);
                        } else {
                            direction = (next - transform.translation)
                                .with_y(0.0)
                                .normalize_or_zero();
                        }
                    }
                }
                // Existing urban exceptional rescue retains its elapsed/below
                // policy. Hills adds only a full configured wait inside one
                // half-metre region: continuous progress and manual movement
                // cannot be mistaken for a trapped automatic recovery.
                let below = transform.translation.y
                    < track.terrain_height(transform.translation.x, transform.translation.z) - 8.;
                let authored_navigation =
                    layout.is_some() || city.is_some() || metro_navigation.is_some();
                let exceptional = (authored_navigation
                    && (recovery.elapsed > config.rescue_after || below))
                    || hills_seek_rescue
                    || (track.is_hills() && below);
                if !manual && !rider.police && exceptional {
                    let (d, _, _) = track.project_route(rider.route, wreck_pos, rider.progress);
                    let d = d.min(track.race_interval().finish - 6.0);
                    let (p, q) = track.sample_route(rider.route, d, 0.0);
                    transform.translation = p + Vec3::Y * 0.98 - q * Vec3::X * 2.0;
                    if let Some(ref mut c) = controller {
                        c.translation = Some(Vec3::ZERO);
                    }
                    recovery.path_refresh = 0.0;
                    recovery.previous_foot = Some(transform.translation);
                    if let Ok((mut t, velocity, _, _)) = bodies.get_mut(wreck) {
                        *t = Transform::from_translation(p).with_rotation(q);
                        if let Some(mut v) = velocity {
                            *v = Velocity::zero();
                        }
                    }
                    recovery.walk_path.clear();
                    recovery.walk_goal = None;
                    recovery.rescues += 1;
                    recovery.last_rescue_stationary = recovery.walk_watch.stationary_seconds();
                    recovery.rescue_penalty_total += config.rescue_penalty;
                    recovery.walk_watch = default();
                    recovery.elapsed = 0.0;
                    race.elapsed += config.rescue_penalty;
                    race.message = "RECOVERY RESCUE / +5 SEC".into();
                    race.message_time = 4.0;
                    continue;
                }
                if !manual && !rider.human_controlled() {
                    while recovery
                        .walk_path
                        .first()
                        .is_some_and(|p| p.xz().distance(transform.translation.xz()) < 0.8)
                    {
                        recovery.walk_path.remove(0);
                    }
                    if let Some(next) = recovery.walk_path.first() {
                        direction = (*next - transform.translation)
                            .with_y(0.)
                            .normalize_or_zero();
                    }
                }
                let reach = contact - transform.translation;
                let facing = reach.with_y(0.0).length() < 0.1
                    || (transform.rotation * Vec3::Z).dot(direction) >= config.mount_facing;
                if !pursuit
                    && (!manual || interact)
                    && facing
                    && reach.with_y(0.0).length() <= config.mount_radius
                    && reach.y.abs() <= config.mount_height
                    && wreck_speed <= config.mount_speed
                {
                    recovery.stage = RecoveryStage::Mounting;
                    recovery.elapsed = 0.0;
                    if let Some(mut controller) = controller {
                        controller.translation = Some(Vec3::ZERO);
                    }
                    // Right the bike as the rider lifts it into a riding position.
                    if let Ok((_, velocity, _, body)) = bodies.get_mut(wreck) {
                        if let Some(mut body) = body {
                            *body = RigidBody::KinematicPositionBased;
                        }
                        if let Some(mut velocity) = velocity {
                            *velocity = Velocity::zero();
                        }
                    }
                } else {
                    let mut movement = if pursuit
                        && delta.length() <= (police.rules.arrest_radius - 0.1).max(0.5)
                    {
                        Vec3::ZERO
                    } else {
                        direction
                    };
                    if live_hold {
                        movement = Vec3::ZERO;
                    }
                    if !manual
                        && movement.length_squared() > 0.001
                        && let Ok(context) = context.single()
                    {
                        let predicate =
                            |e| (!pursuit || Some(e) != pursuit_body) && (pursuit || e != wreck);
                        let filter = QueryFilter::default()
                            .exclude_collider(runner)
                            .predicate(&predicate);
                        if let Some((_, hit)) = context.cast_ray_and_get_normal(
                            transform.translation,
                            direction,
                            2.0,
                            true,
                            filter,
                        ) {
                            // Go around obstacles while the controller preserves solid contacts.
                            let normal = hit.normal.with_y(0.0).normalize_or_zero();
                            let tangent = Vec3::Y.cross(normal);
                            let side = if tangent.dot(direction) >= 0.0 {
                                tangent
                            } else {
                                -tangent
                            };
                            movement = (direction + normal * 1.5 + side * 1.3).normalize_or_zero();
                        }
                    }
                    if manual {
                        let forward = cameras
                            .iter()
                            .next()
                            .map_or(road_rotation * Vec3::Z, |c| *c.forward())
                            .with_y(0.0)
                            .normalize_or_zero();
                        let forward = remote_input.map_or(forward, |i| {
                            Vec3::new(i.foot_forward[0], 0., i.foot_forward[1]).normalize_or_zero()
                        });
                        movement = forward.cross(Vec3::Y) * input.x + forward * input.y;
                    }
                    if movement.length_squared() > 0.001 {
                        transform.rotation = transform.rotation.slerp(
                            Quat::from_rotation_y(movement.x.atan2(movement.z)),
                            (dt * 16.0).min(1.0),
                        );
                    }
                    if let Some(mut controller) = controller {
                        let grounded = controller_outputs
                            .get(runner)
                            .is_ok_and(|output| output.grounded);
                        let down = rr_core::recovery::foot_downward_speed(grounded);
                        controller.translation =
                            Some((movement * config.walk_speed - Vec3::Y * down) * dt);
                    }
                    if rider.human_controlled()
                        && (recovery.walk_distance / (config.stride_length * 0.5)) as u32
                            != previous_step
                    {
                        audio.write(AudioCue::new(Cue::Footstep, transform.translation));
                    }
                }
            }
            RecoveryStage::Mounting => {
                let fraction = (recovery.elapsed / 0.75).min(1.0);
                if let Ok((mut t, _, _, _)) = bodies.get_mut(wreck) {
                    let (_, rotation) = track.sample_route(rider.route, rider.progress, 0.0);
                    t.rotation = t.rotation.slerp(rotation, (dt * 8.0).min(1.0));
                    // Keep the pickup's horizontal position and existing support
                    // height. Do not pull a bike on a roof/curb down to terrain.
                    let support = |e| {
                        e != owner && Some(e) != recovery.runner && !recovery.parts.contains(&e)
                    };
                    let floor = context
                        .single()
                        .ok()
                        .and_then(|c| {
                            c.cast_ray(
                                t.translation + Vec3::Y * 2.0,
                                -Vec3::Y,
                                5.0,
                                true,
                                QueryFilter::default()
                                    .exclude_rigid_body(wreck)
                                    .exclude_sensors()
                                    .predicate(&support),
                            )
                        })
                        .map(|(_, distance)| t.translation.y + 2.0 - distance);
                    if let Some(floor) = floor {
                        t.translation.y += (floor + 0.13 - t.translation.y) * (dt * 8.0).min(1.0);
                    }
                }
                if let Some(pose) = recovery.pose
                    && let Ok((mut t, _, _, _)) = bodies.get_mut(pose)
                {
                    if art.models.is_none() {
                        t.rotation = Quat::from_rotation_z(-fraction * 0.4);
                        t.translation.y = -0.95 + fraction * 0.35;
                    } else {
                        t.rotation = Quat::IDENTITY;
                        t.translation.y = -0.95;
                    }
                }
                if fraction >= 1.0 {
                    let (progress, lane, _) =
                        track.project_route(rider.route, wreck_pos, rider.progress);
                    rider.progress = progress;
                    rider.lane = lane;
                    rider.health = 100.0;
                    rider.original_condition = crate::original_crash::initial_condition();
                    rider.motion_velocity = Vec3::ZERO;
                    rider.side_velocity = 0.0;
                    rider.crashed = 0.0;
                    rider.immunity = 1.0;
                    rider.impact_cooldown = 0.6;
                    // Projection updates race bookkeeping only. The live wreck is
                    // the authoritative pickup location, including elevated supports.
                    let q = crate::animation::bike_anchor(wreck_pose).rotation;
                    *root = Transform::from_translation(wreck_pos).with_rotation(q);
                    rider.speed = 0.0;
                    let (road_point, _) = track.sample_route(rider.route, progress, lane);
                    rider.hop_height =
                        if *adapter == crate::vehicle::VehiclePhysicsAdapter::Assisted {
                            (wreck_pos.y - road_point.y).max(0.0)
                        } else {
                            0.0
                        };
                    rider.hop_velocity = 0.0;
                    rider.motion_angular_velocity = Vec3::ZERO;
                    crate::vehicle::remount_at(&mut commands, owner, *root);
                    *visibility = Visibility::Visible;
                    commands.entity(owner).remove::<ColliderDisabled>();
                    if rider.police {
                        commands.entity(owner).remove::<RigidBodyDisabled>();
                    }
                    crate::crash_behavior::restore_chassis(&mut commands, owner);
                    if let Some(runner) = recovery.runner {
                        commands.entity(runner).despawn();
                    }
                    commands.entity(wreck).despawn();
                    *recovery = Recovery::default();
                }
            }
            RecoveryStage::Riding => {}
        }
    }
}

/// World bounds of the actual Rapier-scaled shape; no rendering bounds or double scaling.
pub(crate) fn collider_bounds(t: &Transform, c: &Collider) -> (Vec3, Vec3) {
    let local = c.raw.compute_local_aabb();
    let mut lo = Vec3::splat(f32::INFINITY);
    let mut hi = Vec3::splat(f32::NEG_INFINITY);
    for x in [local.mins.x, local.maxs.x] {
        for y in [local.mins.y, local.maxs.y] {
            for z in [local.mins.z, local.maxs.z] {
                let p = t.translation + t.rotation * Vec3::new(x, y, z);
                lo = lo.min(p);
                hi = hi.max(p);
            }
        }
    }
    (lo, hi)
}
fn contact_class(
    target: &Transform,
    shape: Option<&Collider>,
    rider: &Transform,
    low: bool,
    tall: bool,
) -> rr_core::contact::Class {
    let left = (rider.rotation * Vec3::X).with_y(0.0).normalize_or_zero();
    let (lo, hi) = shape.map_or(
        (
            target.translation - Vec3::splat(0.5),
            target.translation + Vec3::splat(0.5),
        ),
        |c| collider_bounds(target, c),
    );
    let centre = (lo + hi) * 0.5;
    let half = (hi - lo) * 0.5;
    let span = left.abs().dot(half);
    let lateral = (centre - rider.translation).dot(left);
    rr_core::contact::classify(low, lateral.abs() + 0.18 < span, tall)
}

/// Contacts found by the riding controller's continuous motion sweep.
#[derive(Message, Clone, Copy)]
pub struct SweptContact(pub Entity, pub Entity);

#[derive(Clone, Copy)]
struct SoftHit {
    owner: Entity,
    target: Entity,
    incoming: Vec3,
    target_before: Vec3,
    target_after: Vec3,
    closing: f32,
    target_mass: f32,
}
#[derive(Clone, Copy)]
enum Impact {
    Crash(Entity, Option<Vec3>),
    Nudge(Entity, Vec3),
    Rear(Entity, Entity, Vec3, Vec3),
    RideOver(Entity, Vec3, f32, Option<(Entity, Vec3)>),
    Pedestrian(SoftHit),
    Original(Entity, rr_core::original_crash::Contact, Quat),
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn impacts(
    time: Res<Time<Fixed>>,
    mut events: MessageReader<CollisionEvent>,
    mut swept: MessageReader<SweptContact>,
    mut commands: Commands,
    art: Res<Art>,
    mut race: ResMut<Race>,
    mut riders: Query<(&mut Rider, &mut Recovery, &mut Visibility)>,
    (transforms, shapes, track, clock, mut observations): (
        Query<&Transform>,
        Query<&Collider>,
        Res<crate::track::Track>,
        Res<crate::runtime::SimulationClock>,
        ResMut<crate::contact_telemetry::VehicleContacts>,
    ),
    owned: Query<&OwnedBody>,
    (cars, cows, cross_cars, npcs): (
        Query<&Traffic>,
        Query<(), With<crate::environment::Cow>>,
        Query<Option<&crate::environment::Patrol>, With<crate::environment::CrossTraffic>>,
        Query<&crate::npc::Npc>,
    ),
    (velocities, bike_states, previous): (
        Query<&Velocity>,
        Query<&crate::vehicle::BikeState>,
        Query<&PreviousMotion>,
    ),
    (obstacles, ground, breakable): (
        Query<(), With<Obstacle>>,
        Query<(), With<crate::world::Ground>>,
        Query<&crate::destruction::Breakable>,
    ),
    controllers: Query<(Entity, &KinematicCharacterControllerOutput)>,
    (mut audio, mut npc_hits): (
        MessageWriter<AudioCue>,
        MessageWriter<crate::combat::NpcHit>,
    ),
    context: ReadRapierContext,
    original: Option<Res<crate::original_crash::OriginalCrashes>>,
) {
    let mut pairs: Vec<_> = events
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(a, b, _) => Some((*a, *b)),
            _ => None,
        })
        .collect();
    pairs.extend(swept.read().map(|c| (c.0, c.1)));
    // A gentle initial touch can become a hard impact without a second Started
    // event. Re-evaluate active rider/vehicle contacts using current pre-solver motion.
    if let Ok(ctx) = context.single() {
        for pair in ctx
            .simulation
            .contact_pairs(ctx.colliders, ctx.rigidbody_set)
        {
            if pair.has_any_active_contact()
                && let (Some(a), Some(b)) = (pair.collider1(), pair.collider2())
            {
                let vehicle = |e| cars.contains(e) || cross_cars.contains(e);
                if (riders.contains(a) && vehicle(b)) || (riders.contains(b) && vehicle(a)) {
                    pairs.push((a, b));
                }
            }
        }
    }

    if race.phase != Phase::Racing {
        return;
    }
    for (runner, output) in &controllers {
        for contact in &output.collisions {
            pairs.push((runner, contact.entity));
        }
    }
    for pair in &mut pairs {
        if pair.0.to_bits() > pair.1.to_bits() {
            *pair = (pair.1, pair.0);
        }
    }
    pairs.sort_unstable_by_key(|(a, b)| (a.to_bits(), b.to_bits()));
    pairs.dedup();
    let mut actions = Vec::new();
    for (a, b) in pairs {
        for (bike, other) in [(a, b), (b, a)] {
            if breakable
                .get(other)
                .is_ok_and(|p| p.state.broken && p.profile.soft)
            {
                continue;
            }
            let Ok((rider, recovery, _)) = riders.get(bike) else {
                continue;
            };
            if recovery.stage != RecoveryStage::Riding {
                continue;
            }
            let velocity = bike_states
                .get(bike)
                .map_or(rider.motion_velocity, |s| s.impact_velocity);
            let speed = velocity.length();
            if let Ok((other_rider, other_recovery, _)) = riders.get(other) {
                if other_recovery.stage != RecoveryStage::Riding || bike.to_bits() > other.to_bits()
                {
                    continue;
                }
                let (Ok(a), Ok(b)) = (transforms.get(bike), transforms.get(other)) else {
                    continue;
                };
                let normal = (b.translation - a.translation)
                    .with_y(0.0)
                    .normalize_or_zero();
                let other_velocity = bike_states
                    .get(other)
                    .map_or(other_rider.motion_velocity, |s| s.impact_velocity);
                let forward_a = (a.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
                let forward_b = (b.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
                let closing = (velocity - other_velocity).dot(normal).max(0.0);
                let along = normal.dot(forward_a);
                if forward_a.dot(forward_b) > 0.7 && along.abs() > 0.7 {
                    if closing <= 0.05 {
                        continue;
                    }
                    let shared = rr_core::contact::shared_normal_speed(
                        velocity.dot(normal),
                        other_velocity.dot(normal),
                        rider.bike_mass + 70.0,
                        other_rider.bike_mass + 70.0,
                    );
                    let va = velocity + normal * (shared - velocity.dot(normal));
                    let vb = other_velocity + normal * (shared - other_velocity.dot(normal));
                    actions.push(Impact::Rear(bike, other, va, vb));
                    if closing >= rr_core::contact::REAR_KNOCKOFF_MPS {
                        let (victim, response) = if along > 0.0 { (other, vb) } else { (bike, va) };
                        actions.push(Impact::Crash(victim, Some(response)));
                    }
                    continue;
                }
                if original.as_ref().is_some_and(|p| p.0) {
                    let (response, frame) = crate::original_crash::contact_plan(
                        velocity,
                        other_velocity,
                        a.rotation,
                        b.translation - a.translation,
                        false,
                        rider.original_condition,
                    );
                    actions.push(Impact::Original(bike, response, frame));
                    let (response, frame) = crate::original_crash::contact_plan(
                        other_velocity,
                        velocity,
                        b.rotation,
                        a.translation - b.translation,
                        false,
                        other_rider.original_condition,
                    );
                    actions.push(Impact::Original(other, response, frame));
                    continue;
                }
                let closing = (velocity - other_velocity).dot(normal).max(0.0);
                let forward = (a.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
                let longitudinal = normal.dot(forward).abs() > 0.7;
                let class = contact_class(b, shapes.get(other).ok(), a, false, false);
                if rr_core::contact::wipeout(class, closing) {
                    let victim = if longitudinal && forward.dot(b.rotation * Vec3::Z) > 0.5 {
                        if (b.translation - a.translation).dot(forward) > 0.0 {
                            other
                        } else {
                            bike
                        }
                    } else if velocity.dot(normal).abs() >= other_velocity.dot(normal).abs() {
                        other
                    } else {
                        bike
                    };
                    actions.push(Impact::Crash(victim, None));
                } else if closing > 0.05 {
                    let transfer = normal * (closing * 0.35).min(2.5);
                    actions.push(Impact::Nudge(bike, -transfer));
                    actions.push(Impact::Nudge(other, transfer));
                }
            } else if let Ok(body) = owned.get(other) {
                if body.owner == bike {
                    continue;
                }
                // Original class 0: a detached rider or wreck is not a fresh
                // mounted-bike crash. Rapier still resolves the solid contact.
                if matches!(body.kind, BodyKind::Ragdoll | BodyKind::Wreck) {
                    if let (Ok(a), Ok(b)) = (transforms.get(bike), transforms.get(other)) {
                        let direction = (b.translation - a.translation)
                            .with_y(0.0)
                            .normalize_or_zero();
                        // Both sides of the threshold must use the same pre-solver
                        // sample. A solver push/bounce can otherwise hide a real
                        // approach or turn a gentle touch into a scripted lift.
                        let target_velocity = previous.get(other).map_or_else(
                            |_| velocities.get(other).map_or(Vec3::ZERO, |v| v.linvel),
                            |v| v.0,
                        );
                        let closing = (velocity - target_velocity).dot(direction).max(0.0);
                        if rr_core::contact::ride_over_velocity(closing).is_some()
                            && b.translation.y < a.translation.y + 1.5
                        {
                            actions.push(Impact::RideOver(
                                bike,
                                velocity,
                                closing,
                                Some((other, target_velocity)),
                            ));
                        }
                    }
                    continue;
                }
                if let (Ok(a), Ok(b)) = (transforms.get(bike), transforms.get(other)) {
                    let normal = (b.translation - a.translation)
                        .with_y(0.0)
                        .normalize_or_zero();
                    let other_velocity = controllers.get(other).map_or_else(
                        |_| velocities.get(other).map_or(Vec3::ZERO, |v| v.linvel),
                        |(_, out)| out.effective_translation / time.delta_secs(),
                    );
                    let closing = (velocity - other_velocity).dot(normal).max(0.0);
                    let shared = rr_core::contact::shared_normal_speed(
                        velocity.dot(normal),
                        other_velocity.dot(normal),
                        rider.bike_mass + 70.0,
                        70.0,
                    );
                    let source = velocity + normal * (shared - velocity.dot(normal));
                    let target = other_velocity + normal * (shared - other_velocity.dot(normal));
                    if closing > 0.75 {
                        actions.push(Impact::Crash(body.owner, Some(target)));
                    }
                    if rr_core::contact::ride_over_velocity(closing).is_some() {
                        actions.push(Impact::RideOver(bike, source, closing, None));
                    }
                }
            } else if cows.contains(other) || npcs.get(other).is_ok_and(|npc| npc.active()) {
                if speed > 0.75 {
                    let target_mass = npcs.get(other).map_or(363.0, |npc| {
                        if npc.kind == crate::npc::NpcKind::Cow {
                            363.0
                        } else {
                            56.5
                        }
                    });
                    let fraction =
                        rr_core::crash::shared_speed_fraction(rider.bike_mass + 70.0, target_mass);
                    if npcs
                        .get(other)
                        .is_ok_and(|npc| npc.kind == crate::npc::NpcKind::Pedestrian)
                    {
                        let normal = transforms
                            .get(bike)
                            .ok()
                            .zip(transforms.get(other).ok())
                            .map_or(velocity.with_y(0.0).normalize_or_zero(), |(a, b)| {
                                (b.translation - a.translation)
                                    .with_y(0.0)
                                    .normalize_or_zero()
                            });
                        let target_velocity =
                            velocities.get(other).map_or(Vec3::ZERO, |v| v.linvel);
                        let closing = (velocity - target_velocity).dot(normal).max(0.0);
                        let shared = rr_core::contact::shared_normal_speed(
                            velocity.dot(normal),
                            target_velocity.dot(normal),
                            rider.bike_mass + 70.0,
                            target_mass,
                        );
                        actions.push(Impact::Pedestrian(SoftHit {
                            owner: bike,
                            target: other,
                            incoming: velocity,
                            target_before: target_velocity,
                            target_after: target_velocity
                                + normal * (shared - target_velocity.dot(normal)),
                            closing,
                            target_mass,
                        }));
                    } else {
                        actions.push(Impact::Crash(bike, Some(velocity * fraction)));
                    }
                }
            } else if cross_cars.contains(other)
                || (!ground.contains(other) && obstacles.contains(other))
                || cars
                    .get(other)
                    .is_ok_and(|car| speed + car.speed.abs() > 0.75)
            {
                let object_velocity = transforms.get(other).map_or(Vec3::ZERO, |t| {
                    if let Ok(car) = cars.get(other) {
                        t.rotation * Vec3::Z * car.speed.abs()
                    } else if let Ok(Some(path)) = cross_cars.get(other) {
                        t.rotation * Vec3::Z * path.speed
                    } else {
                        velocities.get(other).map_or(Vec3::ZERO, |v| v.linvel)
                    }
                });
                let Ok(b) = transforms.get(other) else {
                    continue;
                };
                let low = shapes.get(other).is_ok_and(|shape| {
                    let (lo, hi) = collider_bounds(b, shape);
                    rr_core::contact::low_geometry(
                        lo.y,
                        hi.y,
                        track
                            .sample_route(rider.route, rider.progress, rider.lane)
                            .0
                            .y,
                    )
                });
                if low && !cars.contains(other) && !cross_cars.contains(other) {
                    continue;
                }
                // Use closing motion into the contacted surface, not total bike
                // speed. A parking-speed touch or fast parallel scrape must not
                // enter the crash path. Swept contacts may precede a manifold.
                let mut normal = context
                    .single()
                    .ok()
                    .and_then(|ctx| {
                        let pair = ctx.contact_pair(bike, other)?;
                        pair.manifolds().next().map(|m| {
                            m.normal()
                                * if pair.collider1() == Some(bike) {
                                    1.0
                                } else {
                                    -1.0
                                }
                        })
                    })
                    .or_else(|| {
                        let a = transforms.get(bike).ok()?;
                        let sample = a.translation + Vec3::Y;
                        let point = shapes.get(other).map_or(b.translation, |shape| {
                            shape
                                .project_point(b.translation, b.rotation, sample, false)
                                .point
                        });
                        (point - sample).with_y(0.).try_normalize()
                    })
                    .unwrap_or(Vec3::ZERO);
                let vehicle = cars.contains(other) || cross_cars.contains(other);
                if vehicle && let Ok(a) = transforms.get(bike) {
                    let direction = (b.translation - a.translation)
                        .with_y(0.)
                        .normalize_or_zero();
                    normal = normal.with_y(0.).normalize_or_zero();
                    if normal == Vec3::ZERO {
                        normal = direction;
                    }
                    if normal == Vec3::ZERO {
                        normal = (velocity - object_velocity).with_y(0.).normalize_or_zero();
                    }
                    if normal.dot(direction) < 0. {
                        normal = -normal;
                    }
                }
                let (approach, measured_closing) = rr_core::contact::vehicle_approach(
                    velocity.to_array(),
                    object_velocity.to_array(),
                    normal.to_array(),
                );
                // Keep wall normals (including vertical components) authoritative;
                // vehicle normals above are horizontal and measured in rr_core.
                let closing = if vehicle {
                    measured_closing
                } else {
                    (velocity - object_velocity).dot(normal).max(0.0)
                };
                let interrupted_stagger =
                    recovery.stagger.is_some() && rr_core::contact::interrupts_stagger(closing);
                let crash = interrupted_stagger
                    || if vehicle {
                        rr_core::contact::vehicle_wipeout(closing)
                    } else {
                        rr_core::contact::wipeout(rr_core::contact::Class::Tall, closing)
                    };
                if vehicle {
                    let sample = crate::contact_telemetry::VehicleContact {
                        tick: clock.ticks,
                        rider: bike,
                        object: other,
                        incoming: velocity,
                        object_velocity,
                        normal,
                        closing_mps: closing,
                        approach,
                        object_height_m: shapes.get(other).ok().map(|shape| {
                            let (lo, hi) = collider_bounds(b, shape);
                            hi.y - lo.y
                        }),
                        interrupted_stagger,
                        crash,
                    };
                    if std::env::var_os("ROADDASH_TRACE_CONTACTS").is_some() {
                        eprintln!("vehicle-contact {sample:?}");
                    }
                    observations.record(sample);
                }
                if !crash {
                    continue;
                }
                if original.as_ref().is_some_and(|p| p.0)
                    && (cars.contains(other) || cross_cars.contains(other))
                    && let (Ok(a), Ok(b)) = (transforms.get(bike), transforms.get(other))
                {
                    let (response, frame) = crate::original_crash::contact_plan(
                        velocity,
                        object_velocity,
                        a.rotation,
                        b.translation - a.translation,
                        true,
                        rider.original_condition,
                    );
                    actions.push(Impact::Original(bike, response, frame));
                    continue;
                }
                let incoming = velocity.with_y(0.0);
                let retained = rr_core::crash::retained_speed(
                    incoming.length(),
                    object_velocity.with_y(0.0).length(),
                );
                let response =
                    (incoming.normalize_or_zero() * retained).with_y(velocity.y.min(0.0));
                actions.push(Impact::Crash(bike, Some(response)));
            }
        }
        // A traffic car or moving wreck can knock an on-foot rider down again.
        for (person, object) in [(a, b), (b, a)] {
            if let Ok(body) = owned.get(person)
                && body.kind == BodyKind::Runner
            {
                let is_car = cars.contains(object) || cross_cars.contains(object);
                let is_other_wreck = owned.get(object).is_ok_and(|o| {
                    o.kind == BodyKind::Wreck
                        && o.owner != body.owner
                        && velocities
                            .get(object)
                            .is_ok_and(|v| v.linvel.length() > 3.0)
                });
                let object_velocity = velocities.get(object).map_or_else(
                    |_| {
                        transforms.get(object).map_or(Vec3::ZERO, |t| {
                            if let Ok(car) = cars.get(object) {
                                t.rotation * Vec3::Z * car.speed.abs()
                            } else if let Ok(Some(path)) = cross_cars.get(object) {
                                path.axis * path.speed
                            } else {
                                Vec3::ZERO
                            }
                        })
                    },
                    |v| v.linvel,
                );
                if (is_car && object_velocity.length() > 3.0) || is_other_wreck {
                    // A solid moving object may knock the runner down; do not add
                    // a fixed 9 m/s impulse to the runner's existing movement.
                    actions.push(Impact::Crash(body.owner, None));
                }
            }
        }
    }
    for action in actions {
        match action {
            Impact::Original(owner, response, frame) => {
                if let Ok((mut rider, mut recovery, mut visibility)) = riders.get_mut(owner)
                    && recovery.stage == RecoveryStage::Riding
                {
                    let response =
                        rr_core::original_crash::contact(rr_core::original_crash::Contact {
                            condition: rider.original_condition,
                            ..response
                        });
                    rider.original_condition = response.condition;
                    let bike_velocity = crate::original_crash::velocity(response.bike, frame);
                    if let Some(rider_motion) = response.rider {
                        let Ok(t) = transforms.get(owner) else {
                            continue;
                        };
                        let rider_velocity = crate::original_crash::velocity(rider_motion, frame);
                        if (rider.motion_velocity - rider_velocity).dot(t.rotation * Vec3::Z) > 3.0
                        {
                            commands
                                .entity(owner)
                                .insert(crate::biomechanics::reactions::FrontalRelease);
                        }
                        crash_with_response(
                            &mut commands,
                            &art,
                            owner,
                            &mut rider,
                            &mut recovery,
                            t.translation,
                            t.rotation,
                            Vec3::ZERO,
                            &context,
                            Some((rider_velocity, bike_velocity)),
                        );
                        *visibility = Visibility::Hidden;
                        audio.write(AudioCue::new(Cue::Crash, t.translation));
                        if rider.id == 0 {
                            race.shake = 1.0;
                        }
                    } else {
                        rider.speed = (response.bike.forward as f32
                            * rr_core::original_crash::FORWARD_MPS_PER_UNIT)
                            .max(0.0);
                        rider.side_velocity = -(response.bike.lateral as f32)
                            * rr_core::original_crash::VERTICAL_MPS_PER_UNIT;
                        rider.motion_velocity = bike_velocity;
                    }
                }
            }
            Impact::Pedestrian(hit) => {
                if let Ok((mut rider, recovery, _)) = riders.get_mut(hit.owner)
                    && recovery.stage == RecoveryStage::Riding
                {
                    npc_hits.write(crate::combat::NpcHit(hit.target, hit.target_after, false));
                    let hop = if rider.impact_cooldown <= 0.0 {
                        rr_core::contact::ride_over_velocity(hit.closing)
                    } else {
                        None
                    };
                    let resolved = Vec3::from_array(
                        rr_core::contact::ride_over_response(
                            hit.incoming.to_array(),
                            hit.target_before.to_array(),
                            hit.target_after.to_array(),
                            rider.bike_mass + 70.0,
                            hit.target_mass,
                            hop.unwrap_or(0.0),
                        )
                        .unwrap_or(hit.incoming.to_array()),
                    );
                    rider.motion_velocity = resolved;
                    rider.speed = resolved.with_y(0.0).length();
                    if hop.is_some() {
                        rider.hop_velocity = rider.hop_velocity.max(resolved.y);
                        rider.impact_cooldown = 0.35;
                    }
                    commands.queue(move |world: &mut World| {
                        if let Some(mut velocity) = world.get_mut::<Velocity>(hit.owner) {
                            velocity.linvel = resolved;
                        }
                    });
                }
            }
            Impact::RideOver(owner, incoming, closing, target) => {
                if let Ok((mut rider, recovery, _)) = riders.get_mut(owner)
                    && recovery.stage == RecoveryStage::Riding
                    && rider.impact_cooldown <= 0.0
                    && let Some(up) = rr_core::contact::ride_over_velocity(closing)
                {
                    let own_mass = rider.bike_mass + 70.0;
                    rider.hop_velocity = rider.hop_velocity.max(up);
                    rider.motion_velocity = incoming.with_y(incoming.y.max(up));
                    rider.impact_cooldown = 0.35;
                    commands.queue(move |world: &mut World| {
                        let mut before = Vec3::ZERO;
                        let mut after = Vec3::ZERO;
                        let mut mass = 0.0;
                        if let Some((target, target_before)) = target {
                            before = target_before;
                            mass = world
                                .get::<ReadMassProperties>(target)
                                .map_or(20.0, |m| m.mass.max(0.25));
                            let delta =
                                incoming.with_y(0.0).normalize_or_zero() * (closing * 0.2).min(3.0);
                            if let Some(mut velocity) = world.get_mut::<Velocity>(target) {
                                velocity.linvel += delta;
                                after = velocity.linvel;
                            }
                        }
                        let adjusted = Vec3::from_array(
                            rr_core::contact::ride_over_response(
                                incoming.to_array(),
                                before.to_array(),
                                after.to_array(),
                                own_mass,
                                mass,
                                up,
                            )
                            .unwrap_or(incoming.to_array()),
                        );
                        if let Some(mut velocity) = world.get_mut::<Velocity>(owner) {
                            velocity.linvel = adjusted;
                        }
                        if let Some(mut rider) = world.get_mut::<Rider>(owner) {
                            rider.speed = adjusted.with_y(0.0).length();
                            rider.motion_velocity = adjusted;
                            rider.hop_velocity = adjusted.y.max(0.0);
                        }
                    });
                }
            }
            Impact::Rear(a, b, va, vb) => {
                for (owner, velocity) in [(a, va), (b, vb)] {
                    if let Ok((mut r, rec, _)) = riders.get_mut(owner)
                        && rec.stage == RecoveryStage::Riding
                        && let Ok(t) = transforms.get(owner)
                    {
                        let forward = t.rotation * Vec3::Z;
                        r.speed = velocity.dot(forward).max(0.0);
                        r.side_velocity = velocity.dot(forward.cross(Vec3::Y));
                        r.motion_velocity = velocity;
                        r.impact_cooldown = 0.15;
                        // Override the solver's contact bounce using the shared pre-impact normal speed.
                        commands.entity(owner).insert(Velocity::linear(velocity));
                    }
                }
            }
            Impact::Nudge(owner, delta) => {
                if let Ok((mut r, state, _)) = riders.get_mut(owner)
                    && state.stage == RecoveryStage::Riding
                    && r.impact_cooldown <= 0.0
                    && let Ok(t) = transforms.get(owner)
                {
                    let forward = t.rotation * Vec3::Z;
                    let right = forward.cross(Vec3::Y);
                    r.side_velocity += delta.dot(right);
                    r.speed = (r.speed + delta.dot(forward)).max(0.0);
                    r.motion_velocity += delta;
                    r.impact_cooldown = 0.15;
                }
            }
            Impact::Crash(owner, response) => {
                if let Ok((mut rider, mut recovery, mut visibility)) = riders.get_mut(owner)
                    && recovery.stage != RecoveryStage::Falling
                {
                    let actor = recovery.runner.unwrap_or(owner);
                    let Ok(t) = transforms.get(actor) else {
                        continue;
                    };
                    if recovery.runner.is_some() {
                        // A repeat knockdown inherits the runner's current movement,
                        // never the hidden bike's stale pre-crash velocity.
                        rider.motion_velocity =
                            controllers.get(actor).map_or(Vec3::ZERO, |(_, output)| {
                                output.effective_translation / time.delta_secs()
                            });
                        rider.speed = 0.0;
                        rider.hop_velocity = 0.0;
                    } else if let Ok(state) = bike_states.get(owner) {
                        // Readback contains the solver's bounce. Detach using the same
                        // pre-contact world velocity that triggered this collision.
                        rider.motion_velocity = state.impact_velocity;
                    }
                    if let Some(response) = response {
                        if recovery.runner.is_none()
                            && (rider.motion_velocity - response).dot(t.rotation * Vec3::Z) > 3.0
                        {
                            commands
                                .entity(owner)
                                .insert(crate::biomechanics::reactions::FrontalRelease);
                        }
                        let severity = (rider.motion_velocity - response).length() / 25.0;
                        commands
                            .entity(owner)
                            .insert(crate::biomechanics::ImpactSeverity(
                                severity.clamp(0.0, 1.0),
                            ));
                        rider.motion_velocity = response;
                        rider.speed = 0.0;
                        rider.hop_velocity = 0.0;
                    }
                    let origin = t.translation
                        - if recovery.runner.is_some() {
                            Vec3::Y * 0.95
                        } else {
                            Vec3::ZERO
                        };
                    crash(
                        &mut commands,
                        &art,
                        owner,
                        &mut rider,
                        &mut recovery,
                        origin,
                        t.rotation,
                        Vec3::ZERO,
                        &context,
                    );
                    *visibility = Visibility::Hidden;
                    audio.write(AudioCue::new(Cue::Crash, origin));
                    if rider.id == 0 {
                        race.shake = 1.0;
                    }
                }
            }
        }
    }
}

/// Preserve useful stopping friction on flat roads while allowing fallen bikes
/// to slide downhill instead of sticking to steep terrain.
pub fn slope_friction(
    race: Res<Race>,
    context: ReadRapierContext,
    mut wrecks: Query<(&OwnedBody, &Transform, &mut Friction)>,
) {
    if race.phase != Phase::Racing {
        return;
    }
    let Ok(context) = context.single() else {
        return;
    };
    for (body, t, mut friction) in &mut wrecks {
        if body.kind != BodyKind::Wreck {
            continue;
        }
        let center = t.translation + t.rotation * Vec3::Y * 0.85;
        let slope = context
            .cast_ray_and_get_normal(
                center + Vec3::Y * 1.5,
                -Vec3::Y,
                4.0,
                true,
                QueryFilter::exclude_dynamic().exclude_sensors(),
            )
            .is_some_and(|(_, hit)| hit.normal.y > 0.1 && hit.normal.y < 0.98);
        friction.coefficient = if slope { 0.08 } else { 0.8 };
    }
}

#[cfg(test)]
mod articulation_tests {
    use super::*;
    #[test]
    fn hinges_lock_twist_and_passive_tissue_has_finite_limited_torque() {
        for i in 1..11 {
            let j = anatomical_joint(i, Vec3::Y, Vec3::NEG_Y);
            assert!(!j.contacts_enabled());
            let motor = j.motor(JointAxis::AngX).unwrap();
            assert!(motor.stiffness > 0.0 && motor.damping > 0.0 && motor.max_force <= 12.0);
            let limits = j.limits(JointAxis::AngX).unwrap();
            if matches!(i, 4 | 6 | 8 | 10) {
                assert!(
                    j.locked_axes()
                        .contains(JointAxesMask::ANG_Y | JointAxesMask::ANG_Z)
                );
                if i < 7 {
                    assert!(limits.max <= 0.04 && limits.min < -2.0);
                } else {
                    assert!(limits.min >= -0.04 && limits.max > 2.0);
                }
            }
        }
    }
}

/// Dissipative envelope for one crash's rider and wreck. It permits gravity and
/// redistribution into tumble, but a kinematic car cannot act as an unlimited
/// energy source or reverse the whole crash against its approach direction.
#[derive(Component, Clone, Copy)]
pub(crate) struct CrashEnvelope {
    initial_speed_squared: f32,
    initial_angular_velocity: Vec3,
    initial_position: Vec3,
    initial_rotation: Quat,
    forward: Vec3,
    allow_reverse: bool,
}
impl CrashEnvelope {
    pub(crate) fn new(
        velocity: Vec3,
        initial_position: Vec3,
        initial_rotation: Quat,
        forward: Vec3,
    ) -> Self {
        Self {
            initial_speed_squared: velocity.length_squared(),
            initial_angular_velocity: Vec3::ZERO,
            initial_position,
            initial_rotation,
            forward,
            allow_reverse: false,
        }
    }
    pub(crate) fn with_angular_velocity(mut self, velocity: Vec3) -> Self {
        self.initial_angular_velocity = velocity;
        self
    }
    pub(crate) fn allows_reverse(self) -> bool {
        self.allow_reverse
    }
    pub(crate) fn with_original_direction(mut self, enabled: bool) -> Self {
        self.allow_reverse = enabled;
        self
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn constrain_crash_energy(
    mut bodies: Query<(
        &OwnedBody,
        &Transform,
        &ReadMassProperties,
        &CrashEnvelope,
        Option<&crate::biomechanics::Segment>,
        &mut Velocity,
    )>,
) {
    #[derive(Default)]
    struct Budget {
        mass: f64,
        momentum: bevy::math::DVec3,
        kinetic: f64,
        available: f64,
        forward: Vec3,
        allow_reverse: bool,
    }
    let mut groups = std::collections::HashMap::<Entity, Budget>::new();
    for (owner, t, mass, envelope, _, velocity) in &bodies {
        if mass.mass <= 0.0 {
            continue;
        }
        let group = groups.entry(owner.owner).or_default();
        group.mass += mass.mass as f64;
        group.momentum += velocity.linvel.as_dvec3() * mass.mass as f64;
        group.forward = envelope.forward;
        group.allow_reverse |= envelope.allow_reverse;
        let angular = (t.rotation * mass.principal_inertia_local_frame).inverse() * velocity.angvel;
        group.kinetic += mass.mass as f64 * velocity.linvel.as_dvec3().length_squared()
            + (mass.principal_inertia.as_dvec3() * angular.as_dvec3() * angular.as_dvec3())
                .element_sum();
        let initial_height =
            (envelope.initial_position + envelope.initial_rotation * mass.local_center_of_mass).y;
        let current_height = t.transform_point(mass.local_center_of_mass).y;
        let initial_angular = (envelope.initial_rotation * mass.principal_inertia_local_frame)
            .inverse()
            * envelope.initial_angular_velocity;
        group.available += (mass.principal_inertia.as_dvec3()
            * initial_angular.as_dvec3()
            * initial_angular.as_dvec3())
        .element_sum();
        group.available += mass.mass as f64
            * (envelope.initial_speed_squared as f64
                + 2.0 * 9.81 * (initial_height as f64 - current_height as f64));
    }
    for (owner, _, _, _, biomechanical, mut velocity) in &mut bodies {
        let Some(group) = groups.get(&owner.owner) else {
            continue;
        };
        let backward = if group.allow_reverse {
            0.0
        } else {
            (group.momentum / group.mass)
                .dot(group.forward.as_dvec3())
                .min(0.0)
        };
        // Remove only backwards centre-of-mass motion; individual limbs can still
        // swing backwards relative to their connected body.
        if biomechanical.is_none() || backward < -0.05 {
            velocity.linvel -= group.forward * backward as f32;
        }
        let kinetic = (group.kinetic - group.mass * backward * backward).max(0.0);
        let scale = if kinetic > group.available.max(0.0) {
            (group.available.max(0.0) / kinetic).sqrt()
        } else {
            1.0
        };
        if biomechanical.is_none() || scale < 0.999999 {
            velocity.linvel *= scale as f32;
            velocity.angvel *= scale as f32;
        }
    }
}

#[cfg(test)]
mod tests;

/// Snapshot after gameplay response, before the physics step that may stop a bike.
#[derive(Component, Default)]
pub struct PreviousMotion(pub Vec3);
pub fn remember_owned_motion(mut bodies: Query<(&Velocity, &mut PreviousMotion), With<OwnedBody>>) {
    for (v, mut before) in &mut bodies {
        before.0 = v.linvel;
    }
}
