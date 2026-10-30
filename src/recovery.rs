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
    let specs = [
        (Vec3::new(0.0, 1.55, 0.0), Vec3::new(0.5, 0.58, 0.32), None),
        (
            Vec3::new(0.0, 2.10, 0.0),
            Vec3::new(0.40, 0.42, 0.38),
            Some((0, Vec3::new(0.0, 1.87, 0.0))),
        ),
        (
            Vec3::new(0.0, 1.10, 0.0),
            Vec3::new(0.44, 0.28, 0.30),
            Some((0, Vec3::new(0.0, 1.26, 0.0))),
        ),
        (
            Vec3::new(-0.45, 1.55, 0.0),
            Vec3::new(0.24, 0.40, 0.24),
            Some((0, Vec3::new(-0.29, 1.69, 0.0))),
        ),
        (
            Vec3::new(-0.49, 1.14, 0.0),
            Vec3::new(0.21, 0.38, 0.21),
            Some((3, Vec3::new(-0.47, 1.34, 0.0))),
        ),
        (
            Vec3::new(0.45, 1.55, 0.0),
            Vec3::new(0.24, 0.40, 0.24),
            Some((0, Vec3::new(0.29, 1.69, 0.0))),
        ),
        (
            Vec3::new(0.49, 1.14, 0.0),
            Vec3::new(0.21, 0.38, 0.21),
            Some((5, Vec3::new(0.47, 1.34, 0.0))),
        ),
        (
            Vec3::new(-0.17, 0.73, 0.0),
            Vec3::new(0.25, 0.44, 0.25),
            Some((2, Vec3::new(-0.17, 0.96, 0.0))),
        ),
        (
            Vec3::new(-0.17, 0.25, 0.0),
            Vec3::new(0.22, 0.44, 0.22),
            Some((7, Vec3::new(-0.17, 0.49, 0.0))),
        ),
        (
            Vec3::new(0.17, 0.73, 0.0),
            Vec3::new(0.25, 0.44, 0.25),
            Some((2, Vec3::new(0.17, 0.96, 0.0))),
        ),
        (
            Vec3::new(0.17, 0.25, 0.0),
            Vec3::new(0.22, 0.44, 0.22),
            Some((9, Vec3::new(0.17, 0.49, 0.0))),
        ),
    ];
    let mut bodies = Vec::new();
    for (i, (p, size, parent)) in specs.iter().enumerate() {
        let e = commands
            .spawn((
                Visibility::default(),
                Transform::from_translation(rider_origin + rotation * *p).with_rotation(rotation),
                RigidBody::Dynamic,
                Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
                Velocity {
                    linvel: velocity,
                    angvel: -forward * 0.5 * separation_side,
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
        let visual = world::part(
            commands,
            art.cube.clone(),
            if i == 1 {
                art.white.clone()
            } else if i >= 7 || i == 2 {
                art.dark.clone()
            } else {
                art.colors[rider.id].clone()
            },
            Vec3::ZERO,
            *size,
            Quat::IDENTITY,
        );
        crate::runtime::attach_visuals(commands, e, &[visual]);
        if let Some((parent, anchor)) = parent {
            let mut joint = SphericalJointBuilder::new()
                .local_anchor1(*anchor - specs[*parent].0)
                .local_anchor2(*anchor - *p)
                .limits(JointAxis::AngX, [-1.4, 1.4])
                .limits(JointAxis::AngY, [-0.7, 0.7])
                .limits(JointAxis::AngZ, [-1.1, 1.1])
                .build();
            joint.set_contacts_enabled(false);
            if matches!(i, 4 | 6 | 8 | 10) {
                joint.set_limits(JointAxis::AngX, [-0.1, 2.2]);
                joint.set_limits(JointAxis::AngY, [-0.05, 0.05]);
                joint.set_limits(JointAxis::AngZ, [-0.05, 0.05]);
            }
            commands
                .entity(e)
                .insert(ImpulseJoint::new(bodies[*parent], joint));
        }
        bodies.push(e);
    }
    let wreck = if let Some(wreck) = recovery.wreck {
        commands.entity(wreck).insert(RigidBody::Dynamic);
        wreck
    } else {
        let wreck = world::bike(commands, art, rider.id, false);
        commands.entity(wreck).insert((
            Transform::from_translation(origin).with_rotation(rotation),
            RigidBody::Dynamic,
            Collider::compound(vec![(
                Vec3::new(0.0, 0.85, 0.0),
                Quat::IDENTITY,
                Collider::cuboid(0.38, 0.8, 1.12),
            )]),
            ColliderMassProperties::Mass(170.0),
            Velocity {
                linvel: momentum,
                angvel: forward * 0.5 * separation_side,
            },
            Ccd::enabled(),
            Restitution::coefficient(0.25),
            Friction::coefficient(0.8),
            Damping {
                linear_damping: 0.4,
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
                let (runner, pose) = world::runner(&mut commands, &art, rider.id);
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
                    t.translation.y += (0.13 - t.translation.y) * (dt * 8.0).min(1.0);
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
    Hop(Entity),
    Kick(Entity, Vec3),
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
    cars: Query<&Traffic>,
    velocities: Query<&Velocity>,
    obstacles: Query<(), With<Obstacle>>,
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
    let mut actions = Vec::new();
    for (a, b) in pairs {
        for (bike, other) in [(a, b), (b, a)] {
            let Ok((rider, recovery, _)) = riders.get(bike) else {
                continue;
            };
            if recovery.stage != RecoveryStage::Riding {
                continue;
            }
            let velocity = rider.motion_velocity;
            let speed = velocity.length();
            if let Ok((other_rider, other_recovery, _)) = riders.get(other) {
                if other_recovery.stage == RecoveryStage::Riding
                    && (speed > other_rider.motion_velocity.length()
                        || (speed == other_rider.motion_velocity.length()
                            && bike.to_bits() < other.to_bits()))
                    && speed > 0.75
                {
                    actions.push(Impact::Crash(other, velocity * 0.45));
                    actions.push(Impact::Hop(bike));
                }
            } else if let Ok(body) = owned.get(other) {
                if body.owner == bike {
                    continue;
                }
                if speed > 0.75 {
                    actions.push(Impact::Hop(bike));
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
            } else if obstacles.contains(other)
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
                let is_car = cars.contains(object);
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
            Impact::Kick(body, impulse) => {
                commands.entity(body).insert((
                    ExternalImpulse {
                        impulse,
                        torque_impulse: Vec3::new(2.0, 1.0, 0.0),
                    },
                    Sleeping::disabled(),
                ));
            }
            Impact::Hop(owner) => {
                if let Ok((mut rider, recovery, _)) = riders.get_mut(owner)
                    && recovery.stage == RecoveryStage::Riding
                    && rider.impact_cooldown <= 0.0
                {
                    rider.hop_velocity = 5.2;
                    rider.hop_height = rider.hop_height.max(0.02);
                    rider.speed *= 0.78;
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
