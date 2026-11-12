//! Optional City Run: keep old ready geometry until its prepared replacement is
//! ready, then crossfade. Cosmetic scene state never changes collision or lots.
use super::*;
use bevy::camera::visibility::VisibilityRange;
use std::collections::{BTreeMap, BTreeSet};
const MAX_TRANSITIONS: usize = 8;
const CACHE: usize = 48;
#[derive(Component)]
struct Group {
    owner: Entity,
}
#[derive(Component)]
pub struct Part {
    owner: Entity,
    group: Entity,
}
#[derive(Component)]
pub struct Instance {
    active: Entity,
    pending: Option<(usize, Entity)>,
    retiring: Option<(usize, Entity)>,
    blend: f32,
}
#[derive(Default)]
pub(super) struct Cache {
    used: BTreeMap<(bool, usize, usize), u64>,
    tick: u64,
}
fn root(
    commands: &mut Commands,
    owner: Entity,
    handle: Handle<WorldAsset>,
    hidden: bool,
) -> Entity {
    let e = commands
        .spawn((
            WorldAssetRoot(handle),
            Transform::IDENTITY,
            if hidden {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            },
            Group { owner },
        ))
        .observe(scene_ready)
        .id();
    commands.entity(owner).add_child(e);
    e
}
pub(super) fn spawn(
    commands: &mut Commands,
    name: String,
    b: BuildingInstance,
    scene: Handle<WorldAsset>,
    transform: Transform,
) {
    let owner = commands
        .spawn((Name::new(name), b, transform, Visibility::Inherited))
        .id();
    let active = root(commands, owner, scene, false);
    commands.entity(owner).insert(Instance {
        active,
        pending: None,
        retiring: None,
        blend: 1.0,
    });
}
fn scene_ready(
    event: On<bevy::world_serialization::WorldInstanceReady>,
    groups: Query<&Group>,
    children: Query<&Children>,
    meshes: Query<(), With<Mesh3d>>,
    mut commands: Commands,
) {
    let Ok(group) = groups.get(event.entity) else {
        return;
    };
    for e in children.iter_descendants(event.entity) {
        if meshes.contains(e) {
            commands.entity(e).insert((
                Part {
                    owner: group.owner,
                    group: event.entity,
                },
                VisibilityRange::abrupt(0.0, 0.0),
            ));
        }
    }
}
impl ModelCatalog {
    fn request(
        &mut self,
        server: &AssetServer,
        key: (bool, usize, usize),
        pinned: &BTreeSet<(bool, usize, usize)>,
    ) -> Option<Handle<WorldAsset>> {
        let (proof, kind, lod) = key;
        if lod < 2 && !self.cache.used.contains_key(&key) {
            if self.cache.used.len() >= CACHE {
                let victim = self
                    .cache
                    .used
                    .iter()
                    .filter(|(k, _)| !pinned.contains(k))
                    .min_by_key(|(_, t)| *t)
                    .map(|(k, _)| *k)?;
                self.cache.used.remove(&victim);
                if victim.0 {
                    self.scenes[victim.1][victim.2] = None;
                } else {
                    self.front_scenes[victim.1][victim.2] = None;
                }
            }
            self.cache.used.insert(key, self.cache.tick);
        }
        if lod < 2 {
            self.cache.used.insert(key, self.cache.tick);
        }
        let collection = if proof {
            &mut self.scenes
        } else {
            &mut self.front_scenes
        };
        Some(
            collection[kind][lod]
                .get_or_insert_with(|| {
                    server.load(format!(
                        "architecture://{}/{}_lod{lod}.glb#Scene0",
                        if proof { "models" } else { "front_modules" },
                        self.ids[kind]
                    ))
                })
                .clone(),
        )
    }
    pub fn cached_high_lods(&self) -> usize {
        self.cache.used.len()
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn update_lod(
    mut commands: Commands,
    time: Res<Time>,
    mut catalog: ResMut<ModelCatalog>,
    server: Res<AssetServer>,
    cameras: Query<&Transform, With<Camera3d>>,
    mut buildings: Query<(Entity, &Transform, &mut BuildingInstance, &mut Instance)>,
    instances: Query<&bevy::world_serialization::WorldInstance>,
    spawner: Res<bevy::world_serialization::WorldInstanceSpawner>,
    children: Query<&Children>,
    parts: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>,
    materials: Res<Assets<StandardMaterial>>,
    meshes: Res<Assets<Mesh>>,
    detail: Option<Res<crate::material_detail::Queue>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    catalog.cache.tick += 1;
    for (_, _, _, mut instance) in &mut buildings {
        instance.blend = (instance.blend + time.delta_secs() / 0.3).min(1.0);
        if instance.blend >= 1.0
            && let Some((_, old)) = instance.retiring.take()
        {
            commands.entity(old).despawn();
        }
    }
    let mut pinned: BTreeSet<_> = buildings
        .iter()
        .flat_map(|(_, _, b, i)| {
            std::iter::once(b.lod)
                .chain(i.pending.map(|p| p.0))
                .chain(i.retiring.map(|p| p.0))
                .filter(|lod| *lod < 2)
                .map(|lod| (b.proof, b.kind, lod))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut in_flight = buildings
        .iter()
        .filter(|(_, _, _, i)| i.pending.is_some() || i.retiring.is_some())
        .count();
    let mut order: Vec<_> = buildings
        .iter()
        .map(|(e, t, _, _)| (e, t.translation.distance(camera.translation)))
        .collect();
    order.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    for (entity, distance) in order {
        let Ok((_, _, mut b, mut i)) = buildings.get_mut(entity) else {
            continue;
        };
        if i.retiring.is_some() {
            continue;
        }
        let desired = if b.proof && distance < 45.0 {
            0
        } else {
            lod_for_distance(distance)
        };
        let desired = if (distance - 28.0).abs() <= 3.0 || (distance - 75.0).abs() <= 5.0 {
            b.lod
        } else {
            desired
        };
        if i.pending.is_some_and(|p| p.0 != desired) {
            commands.entity(i.pending.take().unwrap().1).despawn();
            in_flight -= 1;
        }
        // Cache early inside130m; cap actual concurrent prepared/retiring groups.
        if distance < 130.0 {
            for lod in 0..2 {
                if catalog
                    .request(&server, (b.proof, b.kind, lod), &pinned)
                    .is_some()
                {
                    pinned.insert((b.proof, b.kind, lod));
                }
            }
        }
        if desired == b.lod {
            continue;
        }
        if let Some((lod, pending)) = i.pending {
            let ready = instances
                .get(pending)
                .is_ok_and(|s| spawner.instance_is_ready(**s))
                && children.iter_descendants(pending).all(|e| {
                    parts.get(e).map_or(true, |(mesh, mat)| {
                        detail.as_ref().is_none_or(|d| d.material_ready(mat.0.id()))
                            && materials.get(&mat.0).is_some_and(|m| {
                                m.normal_map_texture.is_none()
                                    || meshes.get(&mesh.0).is_some_and(|m| {
                                        m.contains_attribute(Mesh::ATTRIBUTE_TANGENT)
                                    })
                            })
                    })
                });
            if ready {
                let old = i.active;
                i.active = pending;
                i.pending = None;
                i.retiring = Some((b.lod, old));
                i.blend = 0.0;
                b.lod = lod;
                commands.entity(pending).insert(Visibility::Inherited);
                catalog.direct_crossfades += 1;
            }
        } else if in_flight < MAX_TRANSITIONS
            && let Some(handle) = catalog.request(&server, (b.proof, b.kind, desired), &pinned)
            && server.is_loaded_with_dependencies(handle.id())
        {
            i.pending = Some((desired, root(&mut commands, entity, handle, true)));
            in_flight += 1;
            pinned.insert((b.proof, b.kind, desired));
        }
    }
    catalog.transition_groups = in_flight;
    if !catalog.initial_instances_ready {
        catalog.initial_instances_ready = buildings.iter().all(|(_, _, _, i)| {
            instances
                .get(i.active)
                .is_ok_and(|s| spawner.instance_is_ready(**s))
        });
    }
}
#[allow(clippy::type_complexity)]
pub fn fade(
    mut commands: Commands,
    cameras: Query<&Transform, With<Camera3d>>,
    owners: Query<&Instance>,
    parts: Query<(
        Entity,
        &Part,
        Option<&crate::world_gen::architecture::render::TemporalFade>,
        Option<&VisibilityRange>,
    )>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    let end = if camera.translation.y > 150.0 {
        19000.0..20000.0
    } else {
        650.0..750.0
    };
    for (entity, part, temporal, range) in &parts {
        let Ok(owner) = owners.get(part.owner) else {
            continue;
        };
        let retiring = owner.retiring.is_some_and(|(_, g)| g == part.group);
        let weight = if owner.active == part.group {
            owner.blend
        } else if retiring {
            1.0 - owner.blend
        } else {
            continue;
        };
        if weight >= 0.999 {
            let normal = VisibilityRange {
                end_margin: end.clone(),
                ..default()
            };
            if temporal.is_some() || range != Some(&normal) {
                commands
                    .entity(entity)
                    .remove::<crate::world_gen::architecture::render::TemporalFade>()
                    .insert(normal);
            }
        } else {
            commands
                .entity(entity)
                .insert(crate::world_gen::architecture::render::TemporalFade {
                    weight,
                    incoming: !retiring,
                });
        }
    }
}
