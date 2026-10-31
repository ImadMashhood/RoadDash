//! Bounded actor-specific physical knockdowns; no bike/wreck is spawned for NPCs.
use crate::{
    combat::NpcHit,
    game::{Debris, Phase, Race, Rider, Traffic},
    recovery::{self, BodyKind, OwnedBody},
    world::{self, Art},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NpcKind {
    Cow,
    Pedestrian,
}
#[derive(Component)]
pub struct Npc {
    pub kind: NpcKind,
    pub hits: rr_core::HitChain,
    pub parts: Vec<Entity>,
    pub spawn: Transform,
    pub elapsed: f32,
    pub fresh: bool,
}
impl Npc {
    pub fn new(kind: NpcKind, spawn: Transform) -> Self {
        Self {
            kind,
            hits: default(),
            parts: vec![],
            spawn,
            elapsed: 0.0,
            fresh: false,
        }
    }
    pub fn active(&self) -> bool {
        self.parts.is_empty()
    }
}
fn cow_ragdoll(
    commands: &mut Commands,
    art: &Art,
    owner: Entity,
    t: &Transform,
    velocity: Vec3,
) -> Vec<Entity> {
    let mut specs = vec![
        (Vec3::Y, Vec3::new(2.1, 1.0, 0.9), None, 220.0),
        (
            Vec3::new(1.25, 1.25, 0.0),
            Vec3::new(0.65, 0.7, 0.65),
            Some((0, Vec3::new(0.95, 1.2, 0.0))),
            35.0,
        ),
    ];
    for x in [-0.7, 0.7] {
        for z in [-0.32, 0.32] {
            let parent = specs.len();
            specs.push((
                Vec3::new(x, 0.55, z),
                Vec3::new(0.22, 0.65, 0.22),
                Some((0, Vec3::new(x, 0.85, z))),
                18.0,
            ));
            specs.push((
                Vec3::new(x, 0.15, z),
                Vec3::new(0.20, 0.4, 0.20),
                Some((parent, Vec3::new(x, 0.3, z))),
                9.0,
            ));
        }
    }
    let mut bodies = vec![];
    for (i, (p, size, parent, mass)) in specs.iter().enumerate() {
        let e = commands
            .spawn((
                Transform::from_translation(t.translation + t.rotation * *p)
                    .with_rotation(t.rotation),
                Visibility::default(),
                RigidBody::Dynamic,
                Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
                ColliderMassProperties::Mass(*mass),
                Velocity::linear(velocity),
                Damping {
                    linear_damping: 0.35,
                    angular_damping: 1.8,
                },
                Ccd::enabled(),
                Friction::coefficient(0.75),
                Restitution::coefficient(0.1),
                OwnedBody {
                    owner,
                    kind: BodyKind::Ragdoll,
                },
                Debris(f32::INFINITY),
            ))
            .id();
        let mesh = world::part(
            commands,
            art.cube.clone(),
            if i == 0 {
                art.white.clone()
            } else {
                art.dark.clone()
            },
            Vec3::ZERO,
            *size,
            Quat::IDENTITY,
        );
        crate::runtime::attach_visuals(commands, e, &[mesh]);
        if let Some((parent, anchor)) = parent {
            let hinge = i >= 3 && i % 2 == 1;
            let mut j = GenericJointBuilder::new(if hinge {
                JointAxesMask::LIN_AXES | JointAxesMask::ANG_X | JointAxesMask::ANG_Y
            } else {
                JointAxesMask::LIN_AXES
            })
            .local_anchor1(*anchor - specs[*parent].0)
            .local_anchor2(*anchor - *p)
            .build();
            for axis in [JointAxis::AngX, JointAxis::AngY, JointAxis::AngZ] {
                if hinge && axis != JointAxis::AngZ {
                    continue;
                }
                j.set_limits(
                    axis,
                    if hinge {
                        if p.x > 0.0 { [-1.6, 0.1] } else { [-0.1, 1.6] }
                    } else {
                        [-0.6, 0.6]
                    },
                );
                j.set_motor_position(axis, 0.0, 5.0, 3.0);
                j.set_motor_max_force(axis, 20.0);
            }
            j.set_contacts_enabled(false);
            commands.entity(e).insert(ImpulseJoint::new(
                bodies[*parent],
                TypedJoint::GenericJoint(j),
            ));
        }
        bodies.push(e);
    }
    bodies
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn knockdowns(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    art: Res<Art>,
    mut hits: MessageReader<NpcHit>,
    mut events: MessageReader<CollisionEvent>,
    mut sweeps: MessageReader<recovery::SweptContact>,
    mut npcs: Query<(Entity, &mut Npc, &mut Transform, &mut Visibility)>,
    sources: Query<
        (
            Option<&Rider>,
            Option<&Traffic>,
            Option<&Velocity>,
            Option<&crate::environment::Patrol>,
            Option<&crate::vehicle::BikeState>,
            &Transform,
        ),
        Without<Npc>,
    >,
    bodies: Query<&Transform, (With<OwnedBody>, Without<Npc>)>,
    context: ReadRapierContext,
    owned: Query<&OwnedBody>,
    ground: Query<(), With<crate::world::Ground>>,
    mut audio: MessageWriter<crate::sound::AudioCue>,
) {
    if race.phase == Phase::Paused {
        hits.clear();
        events.clear();
        sweeps.clear();
        return;
    }
    let dt = time.delta_secs();
    let mut requests: Vec<_> = hits.read().map(|h| (h.0, h.1, h.2)).collect();
    for (a, b) in events
        .read()
        .filter_map(|e| match e {
            CollisionEvent::Started(a, b, _) => Some((*a, *b)),
            _ => None,
        })
        .chain(sweeps.read().map(|c| (c.0, c.1)))
    {
        for (target, source) in [(a, b), (b, a)] {
            if !npcs.contains(target) {
                continue;
            }
            if let Ok((rider, car, v, patrol, bike, t)) = sources.get(source) {
                let velocity = if let Some(b) = bike {
                    b.impact_velocity
                } else if let Some(r) = rider {
                    r.motion_velocity
                } else if let Some(v) = v {
                    v.linvel
                } else if let Some(car) = car {
                    t.rotation * Vec3::Z * car.speed.abs()
                } else {
                    patrol.map_or(Vec3::ZERO, |path| path.axis * path.speed)
                };
                if velocity.length() >= 5.0 {
                    requests.push((target, velocity, false));
                }
            }
        }
    }
    let mut active = npcs.iter().filter(|(_, n, _, _)| !n.active()).count();
    for (entity, velocity, direct) in requests {
        let Ok((_, mut npc, t, mut visible)) = npcs.get_mut(entity) else {
            continue;
        };
        if !npc.active() || race.phase != Phase::Racing {
            continue;
        }
        let knockout = !direct || npc.hits.direct_hit(race.elapsed);
        if knockout && active < 8 {
            npc.spawn = *t;
            npc.elapsed = 0.0;
            npc.fresh = true;
            npc.parts = match npc.kind {
                NpcKind::Cow => cow_ragdoll(&mut commands, &art, entity, &t, velocity * 0.55),
                NpcKind::Pedestrian => recovery::humanoid(
                    &mut commands,
                    &art,
                    entity,
                    4,
                    t.translation,
                    t.rotation,
                    velocity * 0.7,
                ),
            };
            audio.write(crate::sound::AudioCue::new(
                crate::sound::Cue::Crash,
                t.translation,
            ));
            *visible = Visibility::Hidden;
            commands.entity(entity).insert(ColliderDisabled);
            active += 1;
        }
    }
    for (entity, mut npc, mut t, mut visible) in &mut npcs {
        if npc.active() {
            npc.hits.expire(race.elapsed);
            continue;
        }
        if npc.fresh {
            npc.fresh = false;
            continue;
        }
        npc.elapsed += dt;
        if let Some(torso) = npc.parts.first().and_then(|e| bodies.get(*e).ok()) {
            t.translation = torso.translation;
        } else {
            npc.elapsed = 20.0;
        }
        if npc.elapsed > 18.0 || matches!(race.phase, Phase::Countdown | Phase::Garage) {
            if race.phase == Phase::Racing
                && let Ok(context) = context.single()
            {
                let shape = Collider::cuboid(
                    if npc.kind == NpcKind::Cow { 1.2 } else { 0.3 },
                    if npc.kind == NpcKind::Cow {
                        0.9
                    } else {
                        crate::human::HEIGHT * 0.5
                    },
                    if npc.kind == NpcKind::Cow { 0.6 } else { 0.3 },
                );
                let predicate = |other| {
                    !ground.contains(other)
                        && !owned.get(other).is_ok_and(|body| body.owner == entity)
                };
                let mut occupied = false;
                context.intersect_shape(
                    npc.spawn.translation
                        + Vec3::Y
                            * if npc.kind == NpcKind::Cow {
                                0.95
                            } else {
                                crate::human::HEIGHT * 0.5
                            },
                    npc.spawn.rotation,
                    shape.raw.as_ref(),
                    QueryFilter::default()
                        .exclude_rigid_body(entity)
                        .exclude_sensors()
                        .predicate(&predicate),
                    |_| {
                        occupied = true;
                        false
                    },
                );
                if occupied {
                    npc.elapsed = 17.5;
                    continue;
                }
            }
            for part in npc.parts.drain(..) {
                commands.entity(part).despawn();
            }
            *t = npc.spawn;
            *visible = Visibility::Visible;
            npc.hits = default();
            commands.entity(entity).remove::<ColliderDisabled>();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(kind: NpcKind) -> (App, Entity) {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let (p, q) = app
            .world()
            .resource::<crate::track::Track>()
            .sample(-8.0, -3.7);
        let t = Transform::from_translation(p).with_rotation(q);
        let e = app
            .world_mut()
            .spawn((
                Npc::new(kind, t),
                t,
                Visibility::default(),
                RigidBody::Fixed,
                Collider::compound(vec![(
                    Vec3::Y,
                    Quat::IDENTITY,
                    Collider::cuboid(0.45, 0.9, 0.4),
                )]),
            ))
            .id();
        (app, e)
    }
    #[test]
    fn direct_melee_spawns_human_and_quadruped_topologies_without_bikes() {
        for (kind, parts, joints) in [(NpcKind::Pedestrian, 11, 10), (NpcKind::Cow, 10, 9)] {
            let (mut app, e) = fixture(kind);
            for count in 1..=3 {
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .release(KeyCode::KeyJ);
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .press(KeyCode::KeyJ);
                {
                    let mut q = app.world_mut().query::<&mut Rider>();
                    q.single_mut(app.world_mut()).unwrap().attack = 0.0;
                }
                app.update();
                if count < 3 {
                    assert_eq!(app.world().get::<Npc>(e).unwrap().hits.count, count);
                    for _ in 0..20 {
                        app.update();
                    }
                }
            }
            assert_eq!(app.world().get::<Npc>(e).unwrap().parts.len(), parts);
            assert!(app.world().get::<ColliderDisabled>(e).is_some());
            assert_eq!(
                app.world_mut()
                    .query::<&ImpulseJoint>()
                    .iter(app.world())
                    .count(),
                joints
            );
            assert!(
                app.world_mut()
                    .query::<&OwnedBody>()
                    .iter(app.world())
                    .all(|b| b.kind == BodyKind::Ragdoll)
            );
            for _ in 0..120 {
                app.update();
            }
            for t in app
                .world_mut()
                .query_filtered::<&Transform, With<OwnedBody>>()
                .iter(app.world())
            {
                assert!(t.translation.is_finite() && t.translation.y > -0.5);
            }
            app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
            app.update();
            assert!(app.world().get::<Npc>(e).unwrap().active());
            assert!(app.world().get::<ColliderDisabled>(e).is_none());
        }
    }
    #[test]
    fn actor_ragdolls_are_bounded_and_reset_cleans_every_part() {
        let (mut app, _) = fixture(NpcKind::Cow);
        let mut actors = vec![];
        for i in 0..10 {
            let t = Transform::from_xyz(i as f32 * 4.0, 0.0, 25.0);
            let e = app
                .world_mut()
                .spawn((Npc::new(NpcKind::Cow, t), t, Visibility::default()))
                .id();
            actors.push(e);
            app.world_mut()
                .resource_mut::<Messages<NpcHit>>()
                .write(NpcHit(e, Vec3::Z * 15.0, false));
        }
        app.update();
        assert_eq!(
            actors
                .iter()
                .filter(|e| !app.world().get::<Npc>(**e).unwrap().active())
                .count(),
            8
        );
        app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
        app.update();
        assert_eq!(
            app.world_mut()
                .query::<&OwnedBody>()
                .iter(app.world())
                .count(),
            0
        );
    }
    #[test]
    fn standing_respawn_waits_until_the_original_spot_is_clear() {
        let (mut app, e) = fixture(NpcKind::Pedestrian);
        let pose = app.world().get::<Transform>(e).unwrap().translation;
        app.world_mut()
            .resource_mut::<Messages<NpcHit>>()
            .write(NpcHit(e, Vec3::Z * 8.0, false));
        for _ in 0..3 {
            app.update();
        }
        let blocker = app
            .world_mut()
            .spawn((
                RigidBody::Fixed,
                Collider::cuboid(1.0, 1.5, 1.0),
                Transform::from_translation(pose + Vec3::Y),
            ))
            .id();
        app.update();
        app.world_mut().get_mut::<Npc>(e).unwrap().elapsed = 19.0;
        app.update();
        assert!(!app.world().get::<Npc>(e).unwrap().active());
        app.world_mut().entity_mut(blocker).despawn();
        for _ in 0..40 {
            app.update();
        }
        assert!(app.world().get::<Npc>(e).unwrap().active());
    }
}
