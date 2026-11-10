//! Shared moving actors and persistent prop damage, keyed by authored origins/names.
use super::replication::{Pose, put_pose};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn viewers(world: &mut World) -> Vec<[f32; 3]> {
    world
        .query::<(&crate::game::Rider, &crate::recovery::Recovery, &Transform)>()
        .iter(world)
        .filter(|(r, _, _)| r.id == 0 || r.remote_human)
        .map(|(_, rec, transform)| {
            rec.runner
                .or(rec.torso)
                .and_then(|e| world.get::<Transform>(e))
                .unwrap_or(transform)
                .translation
                .to_array()
        })
        .collect()
}
pub(super) fn interested(position: Vec3, viewers: &[[f32; 3]], radius: f32) -> bool {
    rr_core::online::replication_interest(position.to_array(), viewers, radius)
}
fn key(v: Vec3) -> [u32; 3] {
    v.to_array().map(f32::to_bits)
}
#[derive(Serialize, Deserialize)]
struct Actor {
    key: [u32; 3],
    pose: Pose,
    parts: Vec<Pose>,
    elapsed: f32,
}
#[derive(Serialize, Deserialize)]
struct Prop {
    name: String,
    pose: Pose,
    spent: f32,
    revision: u32,
    broken: bool,
}
#[derive(Default, Serialize, Deserialize)]
pub(super) struct State {
    actors: Vec<Actor>,
    crossings: Vec<([u32; 3], Pose)>,
    props: Vec<Prop>,
}
impl State {
    pub(super) fn valid(&self) -> bool {
        self.actors.len() <= 4096
            && self.crossings.len() <= 2048
            && self.props.len() <= 12000
            && self.actors.iter().all(|a| {
                a.pose.valid()
                    && a.elapsed.is_finite()
                    && a.parts.len() <= 16
                    && a.parts.iter().all(Pose::valid)
            })
            && self.crossings.iter().all(|(_, p)| p.valid())
            && self.props.iter().all(|p| {
                p.name.len() <= 256 && p.pose.valid() && p.spent.is_finite() && p.spent >= 0.
            })
    }
}
pub(super) fn capture(world: &mut World, viewers: &[[f32; 3]]) -> State {
    let actors = world
        .query::<(&crate::npc::Npc, &Transform)>()
        .iter(world)
        .filter(|(npc, t)| {
            interested(t.translation, viewers, 750.)
                || npc.parts.iter().any(|e| {
                    world
                        .get::<Transform>(*e)
                        .is_some_and(|t| interested(t.translation, viewers, 750.))
                })
        })
        .map(|(npc, t)| Actor {
            key: key(npc.spawn.translation),
            pose: Pose::from(t),
            parts: npc
                .parts
                .iter()
                .filter_map(|e| world.get::<Transform>(*e).map(Pose::from))
                .collect(),
            elapsed: npc.elapsed,
        })
        .collect();
    let crossings=world.query_filtered::<(&crate::environment::Patrol,&Transform),With<crate::environment::CrossTraffic>>().iter(world)
        .filter(|(_,t)| interested(t.translation, viewers, 750.))
        .map(|(p,t)|(key(p.center),Pose::from(t))).collect();
    let props = world
        .query::<(&Name, &crate::destruction::Breakable, &Transform)>()
        .iter(world)
        .filter(|(_, b, _)| b.state.revision > 0)
        .map(|(name, b, t)| Prop {
            name: name.to_string(),
            pose: Pose::from(t),
            spent: b.state.spent_j,
            revision: b.state.revision,
            broken: b.state.broken,
        })
        .collect();
    State {
        actors,
        crossings,
        props,
    }
}
pub(super) fn apply(world: &mut World, state: State) {
    let actors: BTreeMap<_, _> = world
        .query::<(Entity, &crate::npc::Npc)>()
        .iter(world)
        .map(|(e, n)| (key(n.spawn.translation), e))
        .collect();
    let present: BTreeSet<_> = state.actors.iter().map(|a| a.key).collect();
    for (key, &entity) in &actors {
        if !present.contains(key) {
            let parts = world.get::<crate::npc::Npc>(entity).unwrap().parts.clone();
            world.entity_mut(entity).insert(Visibility::Hidden);
            for part in parts {
                if let Ok(mut e) = world.get_entity_mut(part) {
                    e.insert(Visibility::Hidden);
                }
            }
        }
    }
    for actor in state.actors {
        let Some(&entity) = actors.get(&actor.key) else {
            continue;
        };
        let mut npc = world.entity_mut(entity).take::<crate::npc::Npc>().unwrap();
        if actor.parts.is_empty() {
            for e in npc.parts.drain(..) {
                world.despawn(e);
            }
        } else if npc.parts.is_empty() {
            let art = world.resource::<crate::world::Art>();
            let appearance = world
                .get::<crate::props::ActorAppearance>(entity)
                .map(|a| a.0.as_str());
            let mut queue = bevy::ecs::world::CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            npc.parts = match npc.kind {
                crate::npc::NpcKind::Cow => crate::npc::cow_ragdoll(
                    &mut commands,
                    art,
                    entity,
                    &actor.pose.transform(),
                    Vec3::ZERO,
                    appearance,
                ),
                crate::npc::NpcKind::Pedestrian => crate::recovery::humanoid_with_prop(
                    &mut commands,
                    art,
                    entity,
                    11,
                    Vec3::ZERO,
                    Quat::IDENTITY,
                    Vec3::ZERO,
                    appearance,
                ),
            };
            queue.apply(world);
        }
        for (e, pose) in npc.parts.iter().zip(&actor.parts) {
            put_pose(world, *e, pose);
            world.entity_mut(*e).insert(Visibility::Visible);
        }
        npc.elapsed = actor.elapsed;
        let visible = npc.parts.is_empty();
        world.entity_mut(entity).insert((
            npc,
            if visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
        ));
        put_pose(world, entity, &actor.pose);
    }
    let crossings:BTreeMap<_,_>=world.query_filtered::<(Entity,&crate::environment::Patrol),With<crate::environment::CrossTraffic>>().iter(world).map(|(e,p)|(key(p.center),e)).collect();
    let present: BTreeSet<_> = state.crossings.iter().map(|(key, _)| *key).collect();
    for (key, &entity) in &crossings {
        world.entity_mut(entity).insert(if present.contains(key) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        });
    }
    for (key, pose) in state.crossings {
        if let Some(e) = crossings.get(&key) {
            put_pose(world, *e, &pose);
        }
    }
    let props: BTreeMap<_, _> = world
        .query_filtered::<(Entity, &Name), With<crate::destruction::Breakable>>()
        .iter(world)
        .map(|(e, n)| (n.to_string(), e))
        .collect();
    for prop in state.props {
        if let Some(&e) = props.get(&prop.name) {
            let mut b = world.get_mut::<crate::destruction::Breakable>(e).unwrap();
            b.state = rr_core::destruction::Damage {
                spent_j: prop.spent,
                revision: prop.revision,
                broken: prop.broken,
            };
            put_pose(world, e, &prop.pose);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interest_hides_stale_parts_reenters_and_restores_offline_visibility() {
        let mut world = World::new();
        let transform = Transform::from_xyz(100., 0., 0.);
        let part = world.spawn((transform, Visibility::Visible)).id();
        let mut npc = crate::npc::Npc::new(crate::npc::NpcKind::Cow, transform);
        npc.parts.push(part);
        let entity = world.spawn((npc, transform, Visibility::Visible)).id();
        apply(&mut world, State::default());
        assert_eq!(
            *world.get::<Visibility>(entity).unwrap(),
            Visibility::Hidden
        );
        assert_eq!(*world.get::<Visibility>(part).unwrap(), Visibility::Hidden);
        assert_eq!(
            world.get::<Transform>(entity).unwrap().translation,
            transform.translation
        );
        apply(
            &mut world,
            State {
                actors: vec![Actor {
                    key: key(transform.translation),
                    pose: Pose::from(&transform),
                    parts: vec![],
                    elapsed: 0.,
                }],
                ..Default::default()
            },
        );
        assert_eq!(
            *world.get::<Visibility>(entity).unwrap(),
            Visibility::Visible
        );
        assert!(world.get_entity(part).is_err());
        apply(&mut world, State::default());
        super::super::replication::clear_remote(&mut world);
        assert_eq!(
            *world.get::<Visibility>(entity).unwrap(),
            Visibility::Visible
        );
    }
    #[test]
    fn capture_omits_far_actors_without_changing_authoritative_entities() {
        let mut world = World::new();
        for x in [100., 900., 2050.] {
            let t = Transform::from_xyz(x, 0., 0.);
            world.spawn((
                crate::npc::Npc::new(crate::npc::NpcKind::Cow, t),
                t,
                Visibility::Visible,
            ));
        }
        let state = capture(&mut world, &[[0.; 3], [2000., 0., 0.]]);
        assert_eq!(state.actors.len(), 2);
        assert_eq!(world.query::<&crate::npc::Npc>().iter(&world).count(), 3);
        assert!(
            world
                .query::<&Visibility>()
                .iter(&world)
                .all(|v| *v == Visibility::Visible)
        );
    }
    #[test]
    fn on_foot_interest_tracks_runner_instead_of_abandoned_bike() {
        let mut world = World::new();
        let runner = world.spawn(Transform::from_xyz(2000., 0., 0.)).id();
        let rec = crate::recovery::Recovery {
            runner: Some(runner),
            ..default()
        };
        world.spawn((crate::game::Rider::new(0), rec, Transform::IDENTITY));
        assert_eq!(viewers(&mut world), vec![[2000., 0., 0.]]);
    }
}
