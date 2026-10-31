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
    pub stage: RecoveryStage,
    pub elapsed: f32,
    pub torso: Option<Entity>,
    pub parts: Vec<Entity>,
    pub wreck: Option<Entity>,
    pub runner: Option<Entity>,
    pub pose: Option<Entity>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BodyKind {
    Ragdoll,
    Wreck,
    Runner,
}
#[derive(Component, Clone, Copy)]
pub struct OwnedBody {
    pub owner: Entity,
    pub kind: BodyKind,
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
    let specs = crate::human::SPECS;
    let mut bodies = Vec::new();
    for (i, (p, size, parent)) in specs.iter().enumerate() {
        let e = commands
            .spawn((
                Visibility::default(),
                Transform::from_translation(origin + rotation * *p).with_rotation(rotation),
                RigidBody::Dynamic,
                Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
                Velocity {
                    linvel: velocity,
                    angvel: rotation * Vec3::Z * -0.5,
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
        // Child geometry keeps the physics transform unscaled.
        let visual = crate::human::segment(commands, art, color, i, Vec3::ZERO, Quat::IDENTITY);
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
    for part in recovery.parts.drain(..) {
        commands.entity(part).despawn();
    }
    let old_runner = recovery.runner.take();
    if let Some(runner) = old_runner {
        commands.entity(runner).despawn();
    }
    recovery.pose = None;
    let forward = (rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
    let rotation = Quat::from_rotation_y(forward.x.atan2(forward.z));
    let momentum =
        if rider.motion_velocity.is_finite() && rider.motion_velocity.length_squared() > 0.001 {
            rider.motion_velocity
        } else {
            forward * rider.speed + Vec3::Y * rider.hop_velocity
        };
    let velocity = momentum + kick;
    let rider_origin = detached_origin(context, origin, rotation, owner, old_runner);
    let separation_side = if (rider_origin - origin).dot(rotation * Vec3::X) < 0.0 {
        -1.0
    } else {
        1.0
    };
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
            Transform::from_translation(origin).with_rotation(rotation),
            RigidBody::Dynamic,
            Collider::compound(vec![(
                Vec3::new(0.0, 0.85, 0.0),
                Quat::IDENTITY,
                Collider::cuboid(0.38, 0.8, 1.12),
            )]),
            ColliderMassProperties::Mass(rider.bike_mass),
            Velocity {
                linvel: momentum,
                angvel: forward * 0.5 * separation_side,
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
    race: Res<Race>,
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
    context: ReadRapierContext,
    mut audio: MessageWriter<AudioCue>,
) {
    if race.phase != Phase::Racing {
        return;
    }
    let dt = time.delta_secs();
    for (owner, mut rider, mut recovery, mut root, mut visibility) in &mut riders {
        if recovery.stage == RecoveryStage::Riding {
            continue;
        }
        *visibility = Visibility::Hidden;
        recovery.elapsed += dt;
        let Some(wreck) = recovery.wreck else {
            continue;
        };
        let Ok((wreck_transform, wreck_velocity, _, _)) = bodies.get_mut(wreck) else {
            continue;
        };
        let wreck_pose = *wreck_transform;
        let wreck_pos = wreck_pose.translation;
        let wreck_speed = wreck_velocity.map_or(0.0, |v| v.linvel.length());
        let (_, road_rotation) = track.sample(rider.progress, rider.lane);
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
                    },
                    OwnedBody {
                        owner,
                        kind: BodyKind::Runner,
                    },
                    Debris(f32::INFINITY),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::all(),
                ));
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
                    t.scale.y = 0.4 + 0.6 * fraction;
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
                let Ok((mut transform, _, controller, _)) = bodies.get_mut(runner) else {
                    continue;
                };
                let contact = bike_contact_point(&wreck_pose, transform.translation);
                let center = wreck_pose.translation + wreck_pose.rotation * Vec3::Y * 0.85;
                let delta = (center - transform.translation).with_y(0.0);
                let direction = delta.normalize_or_zero();
                let reach = contact - transform.translation;
                if reach.with_y(0.0).length() <= 0.8 && reach.y.abs() <= 1.2 && wreck_speed <= 2.0 {
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
                    let mut movement = direction;
                    if let Ok(context) = context.single() {
                        let predicate = |e| e != wreck;
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
                    transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
                    if let Some(mut controller) = controller {
                        controller.translation = Some((movement * 6.0 - Vec3::Y * 5.0) * dt);
                    }
                    if rider.id == 0
                        && ((recovery.elapsed - dt) * 3.0) as u32 != (recovery.elapsed * 3.0) as u32
                    {
                        audio.write(AudioCue::new(Cue::Footstep, transform.translation));
                    }
                }
            }
            RecoveryStage::Mounting => {
                let fraction = (recovery.elapsed / 0.75).min(1.0);
                if let Ok((mut t, _, _, _)) = bodies.get_mut(wreck) {
                    let (_, rotation) = track.sample(rider.progress, 0.0);
                    t.rotation = t.rotation.slerp(rotation, (dt * 8.0).min(1.0));
                    t.translation.y += (track.terrain_height(t.translation.x, t.translation.z)
                        + 0.13
                        - t.translation.y)
                        * (dt * 8.0).min(1.0);
                }
                if let Some(pose) = recovery.pose
                    && let Ok((mut t, _, _, _)) = bodies.get_mut(pose)
                {
                    t.rotation = Quat::from_rotation_z(-fraction * 0.4);
                    t.translation.y = -0.95 + fraction * 0.35;
                }
                if fraction >= 1.0 {
                    let (progress, lane, _) = track.project(wreck_pos, rider.progress);
                    rider.progress = progress;
                    rider.lane = lane;
                    rider.health = 100.0;
                    rider.motion_velocity = Vec3::ZERO;
                    rider.side_velocity = 0.0;
                    rider.crashed = 0.0;
                    rider.immunity = 1.0;
                    rider.impact_cooldown = 0.6;
                    let (p, q) = track.sample(progress, lane);
                    *root = Transform::from_translation(p).with_rotation(q);
                    *visibility = Visibility::Visible;
                    commands.entity(owner).remove::<ColliderDisabled>();
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

/// Contacts found by the riding controller's continuous motion sweep.
#[derive(Message, Clone, Copy)]
pub struct SweptContact(pub Entity, pub Entity);

#[derive(Clone, Copy)]
enum Impact {
    Crash(Entity, Vec3),
    Hop(Entity, f32),
    Kick(Entity, Vec3),
    Nudge(Entity, Vec3),
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
    transforms: Query<&Transform>,
    owned: Query<&OwnedBody>,
    (cars, cows, cross_cars, npcs): (
        Query<&Traffic>,
        Query<(), With<crate::environment::Cow>>,
        Query<(), With<crate::environment::CrossTraffic>>,
        Query<&crate::npc::Npc>,
    ),
    (velocities, bike_states): (Query<&Velocity>, Query<&crate::vehicle::BikeState>),
    (obstacles, ground): (
        Query<(), With<Obstacle>>,
        Query<(), With<crate::world::Ground>>,
    ),
    controllers: Query<(Entity, &KinematicCharacterControllerOutput)>,
    mut audio: MessageWriter<AudioCue>,
    context: ReadRapierContext,
) {
    let mut pairs: Vec<_> = events
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(a, b, _) => Some((*a, *b)),
            _ => None,
        })
        .collect();
    pairs.extend(swept.read().map(|c| (c.0, c.1)));
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
                let closing = (velocity - other_velocity).dot(normal).max(0.0);
                let forward = (a.rotation * Vec3::Z).with_y(0.0).normalize_or_zero();
                let longitudinal = normal.dot(forward).abs() > 0.7;
                if rr_core::bike_contact_severity(closing, longitudinal)
                    == rr_core::ContactSeverity::Wipeout
                {
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
                    actions.push(Impact::Crash(victim, Vec3::ZERO));
                } else if closing > 0.05 {
                    let transfer = normal * (closing * 0.35).min(2.5);
                    actions.push(Impact::Nudge(bike, -transfer));
                    actions.push(Impact::Nudge(other, transfer));
                }
            } else if let Ok(body) = owned.get(other) {
                if body.owner == bike {
                    continue;
                }
                if speed > 0.75 {
                    actions.push(Impact::Hop(
                        bike,
                        if body.kind == BodyKind::Wreck {
                            1.5
                        } else {
                            crate::human::HEIGHT
                        },
                    ));
                    if body.kind != BodyKind::Runner {
                        actions.push(Impact::Kick(
                            other,
                            velocity
                                * if body.kind == BodyKind::Wreck {
                                    45.0
                                } else {
                                    3.0
                                },
                        ));
                    }
                    if body.kind == BodyKind::Runner {
                        actions.push(Impact::Crash(body.owner, velocity * 0.55));
                    }
                }
            } else if cows.contains(other) && speed > 0.75 {
                actions.push(Impact::Hop(bike, 1.7));
            } else if let Ok(npc) = npcs.get(other) {
                if npc.active() && speed > 0.75 {
                    actions.push(Impact::Hop(
                        bike,
                        if npc.kind == crate::npc::NpcKind::Cow {
                            1.7
                        } else {
                            crate::human::HEIGHT
                        },
                    ));
                }
            } else if cross_cars.contains(other)
                || (!ground.contains(other) && obstacles.contains(other))
                || cars
                    .get(other)
                    .is_ok_and(|car| speed + car.speed.abs() > 0.75)
            {
                actions.push(Impact::Crash(bike, Vec3::ZERO));
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
                if is_car || is_other_wreck {
                    let kick = transforms
                        .get(object)
                        .map_or(Vec3::ZERO, |t| t.rotation * Vec3::Z * 9.0);
                    actions.push(Impact::Crash(body.owner, kick));
                }
            }
        }
    }
    for action in actions {
        match action {
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
            Impact::Kick(body, impulse) => {
                commands.entity(body).insert((
                    ExternalImpulse {
                        impulse,
                        torque_impulse: Vec3::new(2.0, 1.0, 0.0),
                    },
                    Sleeping::disabled(),
                ));
            }
            Impact::Hop(owner, height) => {
                if let Ok((mut rider, recovery, _)) = riders.get_mut(owner)
                    && recovery.stage == RecoveryStage::Riding
                    && rider.impact_cooldown <= 0.0
                {
                    // Explicit animal/pedestrian clearance is an arcade jump, separate
                    // from ordinary ragdoll crashes which still inherit momentum only.
                    let launch = (2.0 * 9.81 * (height + 1.5)).sqrt().clamp(7.5, 10.0);
                    if let Ok(current) = velocities.get(owner) {
                        let incoming = bike_states
                            .get(owner)
                            .map_or(rider.motion_velocity, |s| s.impact_velocity);
                        let target = incoming.with_y(current.linvel.y.max(launch));
                        commands.entity(owner).insert(ExternalImpulse {
                            impulse: (target - current.linvel) * (rider.bike_mass + 70.0),
                            torque_impulse: Vec3::ZERO,
                        });
                    }
                    rider.hop_velocity = launch;
                    rider.hop_height = rider.hop_height.max(0.02);
                    rider.impact_cooldown = 0.8;
                    audio.write(AudioCue::new(
                        Cue::Crash,
                        transforms.get(owner).map_or(Vec3::ZERO, |t| t.translation),
                    ));
                    if rider.id == 0 {
                        race.shake = 0.7;
                    }
                }
            }
            Impact::Crash(owner, kick) => {
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
                        kick,
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
