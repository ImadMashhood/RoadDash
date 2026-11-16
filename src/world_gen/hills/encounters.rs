//! Short, local street scenes using the existing solid pedestrian rigs.
use super::{Track, geometry, props::Model};
use crate::{
    environment::{Patrol, PedestrianLimb, spawn_pedestrian_patrol},
    game::{Phase, Race, Rider},
    human::{HumanSegment, SPECS},
    npc::Npc,
    runtime::VisualRoot,
    sound::{AudioCue, Cue},
    world::{self, Art},
};
use bevy::prelude::*;

#[derive(Clone, Copy)]
enum Scene {
    Brawl,
    Shootout,
}

const SITES: [(f32, f32, Scene); 5] = [
    (7525., 9., Scene::Brawl),
    (7710., -10., Scene::Shootout),
    (7880., 0., Scene::Brawl),
    (8080., -9., Scene::Brawl),
    (8310., 10., Scene::Shootout),
];

#[derive(Component)]
pub(crate) struct Participant {
    scene: Scene,
    partner: Entity,
    turn: f32,
    origin: Vec3,
}

#[derive(Component)]
pub(crate) struct Flash;

#[derive(Component)]
pub(crate) struct SceneLimb(Entity);

pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
) {
    let mut gun = Model::default();
    gun.block(
        Vec3::new(0., -0.13, 0.),
        Vec3::new(0.13, 0.37, 0.15),
        [0.15; 3],
    );
    gun.block(
        Vec3::new(0., 0., -0.09),
        Vec3::new(0.12, 0.16, 0.2),
        [0.22, 0.13, 0.07],
    );
    let gun = meshes.add(gun.mesh());
    let metal = materials.add(StandardMaterial::default());
    let flash = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0.65, 0.12),
        unlit: true,
        ..default()
    });
    for (index, (station, lane, scene)) in SITES.into_iter().enumerate() {
        let separation = match scene {
            Scene::Brawl => 1.65,
            Scene::Shootout => 8.,
        };
        let poses = [-0.5, 0.5].map(|side| {
            let s = station + side * separation;
            let (p, _) = track.sample_route(0, s, lane);
            p.with_y(geometry::crossing_support(track, 0, p, s) + 0.02)
        });
        let actors = [0, 1].map(|i| {
            let forward = poses[1 - i] - poses[i];
            let q = Quat::from_rotation_y(forward.x.atan2(forward.z));
            let actor = spawn_pedestrian_patrol(
                commands,
                art,
                poses[i],
                q,
                (index * 14 + i * 7) as f32,
                1.,
            );
            commands
                .entity(actor)
                .remove::<Patrol>()
                .insert(Name::new(format!(
                    "Western {} {} actor {}",
                    match scene {
                        Scene::Brawl => "brawl",
                        Scene::Shootout => "shootout",
                    },
                    index + 1,
                    i + 1,
                )));
            actor
        });
        for i in [0, 1] {
            commands.entity(actors[i]).insert(Participant {
                scene,
                partner: actors[1 - i],
                turn: i as f32,
                origin: poses[i],
            });
            let pistol = matches!(scene, Scene::Shootout).then(|| {
                let pistol = world::part(
                    commands,
                    gun.clone(),
                    metal.clone(),
                    Vec3::NEG_Y * 0.26,
                    Vec3::ONE,
                    Quat::IDENTITY,
                );
                let muzzle = world::part(
                    commands,
                    art.sphere.clone(),
                    flash.clone(),
                    Vec3::NEG_Y * 0.24,
                    Vec3::splat(0.24),
                    Quat::IDENTITY,
                );
                commands.entity(muzzle).insert((Flash, Visibility::Hidden));
                commands.entity(pistol).add_child(muzzle);
                pistol
            });
            let actor = actors[i];
            commands.queue(move |world: &mut World| {
                let Some(visual) = world.get::<Children>(actor).and_then(|children| {
                    children
                        .iter()
                        .find(|&child| world.get::<VisualRoot>(child).is_some())
                }) else {
                    return;
                };
                let segments: Vec<_> = world
                    .get::<Children>(visual)
                    .into_iter()
                    .flatten()
                    .filter_map(|&child| {
                        world
                            .get::<HumanSegment>(child)
                            .map(|segment| (child, segment.0))
                    })
                    .collect();
                for (child, index) in segments {
                    if world.get::<PedestrianLimb>(child).is_some() {
                        world.entity_mut(child).insert(SceneLimb(actor));
                    }
                    if index == 6
                        && let Some(pistol) = pistol
                    {
                        world.entity_mut(child).add_child(pistol);
                    }
                }
            });
        }
    }
}

fn beat(actor: &Participant, elapsed: f32) -> f32 {
    let interval = match actor.scene {
        Scene::Brawl => 1.15,
        Scene::Shootout => 1.8,
    };
    (elapsed + actor.turn * interval).rem_euclid(2. * interval)
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn animate(
    race: Res<Race>,
    riders: Query<(&Rider, &Transform)>,
    actors: Query<(&Participant, &Npc, &Transform), Without<SceneLimb>>,
    visuals: Query<&VisualRoot>,
    parents: Query<&ChildOf>,
    mut limbs: Query<(&HumanSegment, &PedestrianLimb, &SceneLimb, &mut Transform), Without<Rider>>,
    mut flashes: Query<(&ChildOf, &mut Visibility), With<Flash>>,
    mut audio: MessageWriter<AudioCue>,
    mut previous: Local<Option<f32>>,
) {
    let prior = previous.replace(race.elapsed).unwrap_or(race.elapsed);
    let racing = race.phase == Phase::Racing;
    let playing = |actor: &Participant| {
        racing
            && actors
                .get(actor.partner)
                .is_ok_and(|(_, npc, _)| npc.active())
            && riders.iter().any(|(rider, pose)| {
                rider.human_controlled()
                    && pose.translation.distance_squared(actor.origin) < 90. * 90.
            })
    };
    for (segment, limb, owner, mut transform) in &mut limbs {
        let Ok((actor, npc, _)) = actors.get(owner.0) else {
            continue;
        };
        let phase = beat(actor, race.elapsed);
        let active = npc.active() && playing(actor);
        let swing = ((phase - 0.15) / 0.36 * std::f32::consts::PI)
            .clamp(0., std::f32::consts::PI)
            .sin();
        let angle = if active {
            match (actor.scene, segment.0) {
                (Scene::Brawl, 5) => -0.65 - 0.95 * swing,
                (Scene::Brawl, 6) => -1.55 - 0.12 * swing,
                (Scene::Brawl, 3 | 4) => -1.05,
                (Scene::Shootout, 5 | 6) => -1.55 - 0.24 * swing,
                _ => 0.,
            }
        } else {
            0.
        };
        let turn = Quat::from_rotation_x(angle);
        let pivot = if matches!(segment.0, 4 | 6) {
            let shoulder = SPECS[segment.0 - 1].2.unwrap().1;
            let upper = match (actor.scene, segment.0) {
                (Scene::Brawl, 6) => Quat::from_rotation_x(-0.65 - 0.95 * swing),
                _ => turn,
            };
            shoulder + upper * (limb.pivot - shoulder)
        } else {
            limb.pivot
        };
        transform.translation = if active {
            pivot + turn * (limb.rest.translation - limb.pivot)
        } else {
            limb.rest.translation
        };
        transform.rotation = turn * limb.rest.rotation;
    }
    for (parent, mut visibility) in &mut flashes {
        let actor = parents
            .get(parent.parent())
            .ok()
            .and_then(|arm| parents.get(arm.parent()).ok())
            .and_then(|visual| visuals.get(visual.parent()).ok())
            .and_then(|visual| actors.get(visual.owner).ok());
        *visibility = if actor.is_some_and(|(actor, npc, _)| {
            npc.active() && playing(actor) && (0.33..0.42).contains(&beat(actor, race.elapsed))
        }) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    if !racing || race.elapsed <= prior {
        return;
    }
    for (actor, npc, pose) in &actors {
        if npc.active()
            && playing(actor)
            && beat(actor, prior) < 0.33
            && beat(actor, race.elapsed) >= 0.33
        {
            let (cue, offset) = match actor.scene {
                Scene::Brawl => (Cue::TownPunch, 0.82),
                Scene::Shootout => (Cue::TownShot, 0.85),
            };
            audio.write(AudioCue::new(
                cue,
                pose.translation + Vec3::Y * 1.65 + pose.rotation * Vec3::Z * offset,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn western_scenes_spawn_pairs_and_emit_only_nearby_beats() {
        let mut app = crate::game::tests::simulation();
        let track = Track::hills().unwrap();
        app.world_mut()
            .run_system_once(move |mut commands: Commands, art: Res<Art>| {
                spawn(
                    &mut commands,
                    &mut Assets::default(),
                    &mut Assets::default(),
                    &art,
                    &track,
                );
            })
            .unwrap();
        let world = app.world_mut();
        let actors: Vec<_> = world
            .query::<(Entity, &Participant)>()
            .iter(world)
            .map(|(entity, actor)| (entity, actor.partner, actor.scene, actor.origin))
            .collect();
        assert_eq!(actors.len(), 10);
        assert_eq!(world.query::<&Flash>().iter(world).count(), 4);
        for (actor, partner, _, _) in &actors {
            assert_eq!(world.get::<Participant>(*partner).unwrap().partner, *actor);
            assert!(world.get::<Patrol>(*actor).is_none());
        }

        let track = Track::hills().unwrap();
        let animation = world.register_system(animate);
        let mut cursor = world.resource::<Messages<AudioCue>>().get_cursor_current();
        for (station, lane, scene) in SITES {
            let (center, _) = track.sample_route(0, station, lane);
            let position = actors
                .iter()
                .map(|actor| actor.3)
                .min_by(|a, b| {
                    a.distance_squared(center)
                        .total_cmp(&b.distance_squared(center))
                })
                .unwrap();
            let (_, _, lateral) = track.project_route(0, position, station);
            if lane == 0. {
                assert!(lateral < 0.3);
            } else {
                assert!(lateral > track.half_width(station) + 1.);
            }
            let (mut player, _) = world
                .query::<(&mut Transform, &Rider)>()
                .iter_mut(world)
                .find(|(_, rider)| rider.id == 0)
                .unwrap();
            player.translation = position;
            let mut race = world.resource_mut::<Race>();
            race.phase = Phase::Racing;
            race.elapsed = 0.30;
            world.run_system(animation).unwrap();
            world.resource_mut::<Race>().elapsed = 0.36;
            world.run_system(animation).unwrap();
            let cues: Vec<_> = cursor
                .read(world.resource::<Messages<AudioCue>>())
                .collect();
            assert_eq!(cues.len(), 1, "{station}");
            assert!(matches!(
                (scene, cues[0].kind),
                (Scene::Brawl, Cue::TownPunch) | (Scene::Shootout, Cue::TownShot)
            ));
            assert!(actors.iter().any(|actor| {
                actor.3.distance_squared(center) < 30. && cues[0].position.distance(actor.3) < 3.
            }));
        }
    }
}
