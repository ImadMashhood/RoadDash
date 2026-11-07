//! Bounded, on-demand authored storeys. All geometry stays cosmetic and nested under lot roots.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Component)]
pub struct Slot {
    pub index: usize,
    active: Option<(usize, u8)>,
    children: Vec<Entity>,
    prepared: Option<Prepared>,
    blend: f32,
    target: f32,
    retiring: Option<Prepared>,
    transition: f32,
}
struct Prepared {
    key: (usize, u8),
    children: Vec<Entity>,
}
impl Slot {
    fn face_coverage(&self, face: usize) -> (f32, bool) {
        let active = self.active.is_some_and(|(_, mask)| mask & (1 << face) != 0);
        let retired = self
            .retiring
            .as_ref()
            .is_some_and(|p| p.key.1 & (1 << face) != 0);
        (
            self.blend
                * (f32::from(active) * self.transition
                    + f32::from(retired) * (1.0 - self.transition)),
            retired && !active,
        )
    }
    pub fn prepared_roots(&self) -> &[Entity] {
        self.prepared
            .as_ref()
            .map_or(&[], |p| p.children.as_slice())
    }
    pub fn scene_roots(&self) -> Vec<Entity> {
        self.children
            .iter()
            .chain(self.retiring.iter().flat_map(|p| &p.children))
            .copied()
            .collect()
    }

    pub fn new(index: usize) -> Self {
        Self {
            index,
            active: None,
            children: vec![],
            prepared: None,
            blend: 0.0,
            target: 0.0,
            retiring: None,
            transition: 1.0,
        }
    }
}
#[derive(Component)]
pub struct Fallback {
    pub face: Option<u8>,
    pub index: usize,
    pub wall: bool,
    pub size: Vec3,
    pub range: bevy::camera::visibility::VisibilityRange,
    last_weight: f32,
    last_incoming: bool,
}
impl Fallback {
    pub fn new(
        index: usize,
        wall: bool,
        size: Vec3,
        range: bevy::camera::visibility::VisibilityRange,
        face: Option<u8>,
    ) -> Self {
        Self {
            index,
            wall,
            size,
            range,
            face,
            last_weight: f32::NAN,
            last_incoming: false,
        }
    }
}
struct Cached {
    scenes: [Handle<Scene>; 3],
    used: u64,
}
#[derive(Resource, Default)]
pub struct Streaming {
    cache: BTreeMap<(usize, usize), Cached>,
    coarse: BTreeMap<usize, Cached>,
    masonry: Vec<Handle<Image>>,
    tick: u64,
    elapsed: f32,
    pub active_buildings: usize,
    pub active_panels: usize,
    pub active_triangles: usize,
    pub switches: u64,
    pub direct_switches: u64,
    pub budget_holds: u64,
}
impl Streaming {
    pub fn preload(plan: &Architecture, server: &AssetServer) -> Self {
        let mut s = Self::default();
        for design in plan
            .buildings
            .iter()
            .map(|b| b.design)
            .collect::<BTreeSet<_>>()
        {
            s.coarse.insert(
                design,
                Cached {
                    scenes: std::array::from_fn(|band| {
                        server.load(format!(
                            "architecture://storey_kit/{}_lod2.glb#Scene{band}",
                            plan.catalog.records[design].id
                        ))
                    }),
                    used: 0,
                },
            );
        }
        s.masonry = plan
            .catalog
            .records
            .iter()
            .map(|d| d.masonry.as_str())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|m| server.load(format!("architecture://materials/{m}_albedo.png")))
            .collect();
        s
    }

    fn request(
        &mut self,
        plan: &Architecture,
        server: &AssetServer,
        design: usize,
        lod: usize,
        pinned: &BTreeSet<(usize, usize)>,
    ) -> Option<[Handle<Scene>; 3]> {
        let key = (design, lod);
        if !self.cache.contains_key(&key) && self.cache.len() >= 48 {
            let victim = self
                .cache
                .iter()
                .filter(|(k, _)| !pinned.contains(k))
                .min_by_key(|(_, v)| v.used)
                .map(|(k, _)| *k)?;
            self.cache.remove(&victim);
        }
        let cached = self.cache.entry(key).or_insert_with(|| Cached {
            scenes: std::array::from_fn(|band| {
                server.load(format!(
                    "architecture://storey_kit/{}_lod{lod}.glb#Scene{band}",
                    plan.catalog.records[design].id
                ))
            }),
            used: self.tick,
        });
        cached.used = self.tick;
        Some(cached.scenes.clone())
    }
    pub fn ready(&self, server: &AssetServer) -> bool {
        self.masonry
            .iter()
            .all(|h| server.is_loaded_with_dependencies(h.id()))
            && self.cache.values().chain(self.coarse.values()).all(|c| {
                c.scenes.iter().all(|h| {
                    if let Some(bevy::asset::LoadState::Failed(e)) = server.get_load_state(h.id()) {
                        panic!("Required architecture kit failed: {e}");
                    }
                    server.is_loaded_with_dependencies(h.id())
                })
            })
    }
    pub fn resident_coarse_lods(&self) -> usize {
        self.coarse.len()
    }
    pub fn cached_lods(&self) -> usize {
        self.cache.len()
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    plan: Res<Architecture>,
    server: Res<AssetServer>,
    cameras: Query<&Transform, With<Camera3d>>,
    mut stream: ResMut<Streaming>,
    instances: Query<&bevy::scene::SceneInstance>,
    spawner: Res<bevy::scene::SceneSpawner>,
    descendants: Query<&Children>,
    render_meshes: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>,
    materials: Res<Assets<StandardMaterial>>,
    meshes: Res<Assets<Mesh>>,
    detail: Res<crate::material_detail::Queue>,
    mut slots: Query<(Entity, &mut Slot), Without<Fallback>>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    stream.tick += 1;
    // Re-evaluate at 10Hz, not every render frame; visual-only scene changes belong in Update.
    stream.elapsed += time.delta_secs();
    if stream.elapsed < 0.1 {
        return;
    }
    stream.elapsed = 0.0;
    for (_, mut slot) in &mut slots {
        if slot.transition >= 1.0
            && let Some(retiring) = slot.retiring.take()
        {
            for child in retiring.children {
                commands.entity(child).despawn();
            }
        }
    }
    let mut nearest: Vec<_> = slots
        .iter()
        .map(|(e, s)| {
            (
                e,
                s.index,
                plan.buildings[s.index].distance_to(camera.translation),
            )
        })
        .filter(|(e, _, d)| {
            *d < plan.rules.detail_distance_m
                + if slots.get(*e).is_ok_and(|(_, s)| s.active.is_some()) {
                    25.0
                } else {
                    0.0
                }
        })
        .collect();
    nearest.sort_by(|a, b| {
        let priority = |v: &(Entity, usize, f32)| {
            v.2 - if slots.get(v.0).is_ok_and(|(_, s)| s.active.is_some()) {
                25.0
            } else {
                0.0
            }
        };
        priority(a)
            .total_cmp(&priority(b))
            .then_with(|| a.1.cmp(&b.1))
    });
    let mut pinned: BTreeSet<_> = slots
        .iter()
        .flat_map(|(_, s)| {
            s.active
                .into_iter()
                .chain(s.retiring.as_ref().map(|p| p.key))
                .chain(s.prepared.as_ref().map(|p| p.key))
                .filter(|key| key.0 < 2)
                .map(|key| (plan.buildings[s.index].design, key.0))
                .collect::<Vec<_>>()
        })
        .collect();
    for (_, index, distance) in nearest.iter().take(plan.rules.max_active_buildings) {
        if *distance > 130.0 {
            continue;
        }
        let b = &plan.buildings[*index];
        for lod in 0..2 {
            if stream
                .request(&plan, &server, b.design, lod, &pinned)
                .is_some()
            {
                pinned.insert((b.design, lod));
            }
        }
    }
    let mut selected = BTreeMap::new();
    let mut reserved = 0;
    let mut considered = 0;
    let mut triangles = 0;
    let mut requested = BTreeSet::new();
    for (entity, index, distance) in nearest {
        if considered >= plan.rules.max_active_buildings {
            break;
        }
        let b = &plan.buildings[index];
        let design = &plan.catalog.records[b.design];
        let desired_mask = b.face_mask(camera.translation);
        let local = b.rotation.inverse() * (camera.translation - b.position);
        let previous_mask = slots
            .get(entity)
            .ok()
            .and_then(|(_, s)| s.active.map(|k| k.1));
        let mask = previous_mask.map_or(desired_mask, |old| {
            let x = if local.x.abs() < 4.0 {
                old & 3
            } else {
                desired_mask & 3
            };
            let z = if local.z.abs() < 4.0 {
                old & 12
            } else {
                desired_mask & 12
            };
            x | z
        });
        let panels = b.panels(design, mask);
        if reserved + panels.len() > plan.rules.max_active_panels * 3 / 4 {
            continue;
        }
        let mut lod = if distance < 18.0 {
            0
        } else if distance < 55.0 {
            1
        } else {
            2
        };
        let previous = slots
            .get(entity)
            .ok()
            .and_then(|(_, s)| s.active.map(|v| v.0));
        if let Some(old) = previous
            && ((old == 0 && lod == 1 && distance < 21.0)
                || (old == 1 && lod == 0 && distance > 15.0)
                || (old == 1 && lod == 2 && distance < 60.0)
                || (old == 2 && lod == 1 && distance > 50.0))
        {
            lod = old;
        }
        let displayed = previous.map_or(0, |lod| {
            panels
                .iter()
                .map(|p| design.lods[lod].triangles[p.band])
                .sum::<usize>()
        });
        let mut count = panels
            .iter()
            .map(|p| design.lods[lod].triangles[p.band])
            .sum::<usize>()
            .max(displayed);
        while lod < 2 && triangles + count > plan.rules.max_active_triangles * 3 / 4 {
            lod += 1;
            count = panels
                .iter()
                .map(|p| design.lods[lod].triangles[p.band])
                .sum::<usize>()
                .max(displayed);
        }
        if triangles + count > plan.rules.max_active_triangles * 3 / 4 {
            continue;
        }
        considered += 1;
        reserved += panels.len();
        triangles += count;
        let scenes = if lod == 2 {
            stream.coarse[&b.design].scenes.clone()
        } else if let Some(scenes) = stream.request(&plan, &server, b.design, lod, &pinned) {
            requested.insert((b.design, lod));
            pinned.insert((b.design, lod));
            scenes
        } else {
            // Fully pinned finite cache: retain the current group without thrashing.
            if let Some((old, old_mask)) = slots.get(entity).ok().and_then(|(_, s)| s.active) {
                let held = if old == 2 {
                    Some(stream.coarse[&b.design].scenes.clone())
                } else {
                    stream.cache.get(&(b.design, old)).map(|c| c.scenes.clone())
                };
                if let Some(held) = held {
                    selected.insert(
                        entity,
                        (index, old, old_mask, b.panels(design, old_mask), held),
                    );
                }
            }
            continue;
        };
        if !scenes
            .iter()
            .all(|h| server.is_loaded_with_dependencies(h.id()))
        {
            // A requested high LOD must never replace a ready scene with a blank shell.
            // Keep the old ready LOD, or the permanently resident coarse facade.
            let old = previous.unwrap_or(2);
            let held = if old == 2 {
                Some(stream.coarse[&b.design].scenes.clone())
            } else {
                stream.cache.get(&(b.design, old)).map(|c| c.scenes.clone())
            };
            if let Some(held) = held {
                let held_count = panels
                    .iter()
                    .map(|p| design.lods[old].triangles[p.band])
                    .sum::<usize>();
                if triangles - count + held_count <= plan.rules.max_active_triangles {
                    triangles = triangles - count + held_count;
                    if old != 2 {
                        requested.insert((b.design, old));
                    }
                    selected.insert(entity, (index, old, mask, panels, held));
                }
            }
            continue;
        }
        selected.insert(entity, (index, lod, mask, panels, scenes));
    }
    let cost = |index: usize, key: Option<(usize, u8)>| {
        key.map_or(0, |(lod, mask)| {
            let b = &plan.buildings[index];
            let d = &plan.catalog.records[b.design];
            b.panels(d, mask)
                .iter()
                .map(|p| d.lods[lod].triangles[p.band])
                .sum::<usize>()
        })
    };
    let mut usage = slots.iter().fold((0usize, 0usize, 0usize), |sum, (_, s)| {
        (
            sum.0 + usize::from(s.active.is_some()),
            sum.1 + s.children.len() + s.retiring.as_ref().map_or(0, |p| p.children.len()),
            sum.2 + cost(s.index, s.active) + cost(s.index, s.retiring.as_ref().map(|p| p.key)),
        )
    });
    for (entity, mut slot) in &mut slots {
        if slot.retiring.is_some() {
            continue;
        }
        let next = selected.remove(&entity);
        let key = next.as_ref().map(|(_, lod, mask, _, _)| (*lod, *mask));
        if slot.prepared.as_ref().is_some_and(|p| Some(p.key) != key) {
            for child in slot.prepared.take().unwrap().children {
                commands.entity(child).despawn();
            }
        }
        if slot.active == key {
            slot.target = f32::from(key.is_some());
            continue;
        }
        let ready = slot.prepared.as_ref().is_some_and(|p| {
            p.children.iter().all(|e| {
                instances
                    .get(*e)
                    .is_ok_and(|i| spawner.instance_is_ready(**i))
                    && descendants.iter_descendants(*e).all(|entity| {
                        render_meshes.get(entity).map_or(true, |(mesh, mat)| {
                            detail.material_ready(mat.0.id())
                                && materials.get(&mat.0).is_some_and(|m| {
                                    m.normal_map_texture.is_none()
                                        || meshes.get(&mesh.0).is_some_and(|m| {
                                            m.contains_attribute(Mesh::ATTRIBUTE_TANGENT)
                                        })
                                })
                        })
                    })
            })
        });
        if ready && key.is_some() && slot.active.is_some() {
            if slot.blend < 0.999 {
                continue;
            }
            let prepared = slot.prepared.as_ref().unwrap();
            let new_panels = prepared.children.len();
            let new_cost = cost(slot.index, key);
            // Retain the old opaque facade if overlap would exceed the same
            // displayed panel/triangle caps. Never flash back to a generic shell.
            if usage.1 + new_panels > plan.rules.max_active_panels
                || usage.2 + new_cost > plan.rules.max_active_triangles
            {
                stream.budget_holds += 1;
                continue;
            }
            usage.1 += new_panels;
            usage.2 += new_cost;
            let prepared = slot.prepared.take().unwrap();
            for child in &prepared.children {
                commands.entity(*child).insert(Visibility::Inherited);
            }
            let old_children = std::mem::replace(&mut slot.children, prepared.children);
            slot.retiring = Some(Prepared {
                key: slot.active.unwrap(),
                children: old_children,
            });
            slot.active = key;
            slot.transition = 0.0;
            slot.target = 1.0;
            stream.direct_switches += 1;
            stream.switches += 1;
            continue;
        }
        if key.is_none() || ready {
            slot.target = 0.0;
            if slot.blend > 0.001 {
                continue;
            }
            let old = (
                usize::from(slot.active.is_some()),
                slot.children.len(),
                cost(slot.index, slot.active),
            );
            let new = (
                usize::from(key.is_some()),
                slot.prepared.as_ref().map_or(0, |p| p.children.len()),
                cost(slot.index, key),
            );
            if usage.0 - old.0 + new.0 > plan.rules.max_active_buildings
                || usage.1 - old.1 + new.1 > plan.rules.max_active_panels
                || usage.2 - old.2 + new.2 > plan.rules.max_active_triangles
            {
                continue;
            }
            usage = (
                usage.0 - old.0 + new.0,
                usage.1 - old.1 + new.1,
                usage.2 - old.2 + new.2,
            );
            for child in slot.children.drain(..) {
                commands.entity(child).despawn();
            }
            if let Some(prepared) = slot.prepared.take() {
                for child in &prepared.children {
                    commands.entity(*child).insert(Visibility::Inherited);
                }
                slot.children = prepared.children;
            }
            slot.active = key;
            slot.target = f32::from(key.is_some());
            stream.switches += 1;
        } else if slot.prepared.is_none()
            && let Some((_, lod, mask, panels, scenes)) = next
        {
            let mut children = Vec::with_capacity(panels.len());
            for panel in panels {
                let child = commands
                    .spawn((
                        SceneRoot(scenes[panel.band].clone()),
                        panel.transform,
                        Visibility::Hidden,
                        FacadeRoot {
                            slot: entity,
                            coarse: lod == 2,
                        },
                    ))
                    .observe(scene_ready)
                    .id();
                commands.entity(entity).add_child(child);
                children.push(child);
            }
            slot.prepared = Some(Prepared {
                key: (lod, mask),
                children,
            });
        }
    }
    // Count the displayed group, not pending load requests or prepared hidden scenes.
    stream.active_buildings = 0;
    stream.active_panels = 0;
    stream.active_triangles = 0;
    for (_, slot) in &slots {
        if let Some((lod, mask)) = slot.active {
            stream.active_buildings += 1;
            stream.active_panels +=
                slot.children.len() + slot.retiring.as_ref().map_or(0, |p| p.children.len());
            stream.active_triangles += cost(slot.index, slot.retiring.as_ref().map(|p| p.key));
            let b = &plan.buildings[slot.index];
            let design = &plan.catalog.records[b.design];
            stream.active_triangles += b
                .panels(design, mask)
                .iter()
                .map(|p| design.lods[lod].triangles[p.band])
                .sum::<usize>();
        }
    }
    for (_, slot) in &slots {
        for key in slot
            .active
            .into_iter()
            .chain(slot.retiring.as_ref().map(|p| p.key))
            .chain(slot.prepared.as_ref().map(|p| p.key))
        {
            if key.0 < 2 {
                requested.insert((plan.buildings[slot.index].design, key.0));
            }
        }
    }
    if stream.cache.len() > 48 {
        let mut old: Vec<_> = stream
            .cache
            .iter()
            .filter(|(k, _)| !requested.contains(k))
            .map(|(k, v)| (*k, v.used))
            .collect();
        old.sort_by_key(|v| v.1);
        let remove = stream.cache.len() - 48;
        for (key, _) in old.into_iter().take(remove) {
            stream.cache.remove(&key);
        }
    }
}

#[derive(Component)]
struct FacadeRoot {
    slot: Entity,
    coarse: bool,
}
#[derive(Component)]
pub struct FadePart {
    slot: Entity,
    root: Entity,
}
#[derive(Component)]
pub struct TemporalFade {
    pub(crate) weight: f32,
    pub(crate) incoming: bool,
}
fn scene_ready(
    event: On<bevy::scene::SceneInstanceReady>,
    children: Query<&Children>,
    meshes: Query<(), With<Mesh3d>>,
    roots: Query<&FacadeRoot>,
    mut commands: Commands,
) {
    let Ok(root) = roots.get(event.entity) else {
        return;
    };
    for entity in children.iter_descendants(event.entity) {
        if meshes.contains(entity) {
            commands.entity(entity).insert((
                FadePart {
                    slot: root.slot,
                    root: event.entity,
                },
                TemporalFade {
                    weight: 0.0,
                    incoming: true,
                },
                fade_range(1.0, 0.0, true),
            ));
            if root.coarse {
                commands.entity(entity).insert(bevy::light::NotShadowCaster);
            }
        }
    }
}
/// Encode time opacity in Bevy's existing dithered-LOD shader, without cloning materials.
fn fade_range(
    distance: f32,
    weight: f32,
    incoming: bool,
) -> bevy::camera::visibility::VisibilityRange {
    let width = 8.0;
    let alpha = weight.clamp(0.0, 1.0);
    if incoming {
        bevy::camera::visibility::VisibilityRange {
            start_margin: (distance - alpha * width)..(distance + (1.0 - alpha) * width),
            end_margin: f32::MAX..f32::MAX,
            use_aabb: false,
        }
    } else {
        bevy::camera::visibility::VisibilityRange {
            start_margin: f32::MIN..f32::MIN,
            end_margin: (distance - (1.0 - alpha) * width)..(distance + alpha * width),
            use_aabb: false,
        }
    }
}
/// Recenter temporal dither against the FINAL render camera and propagated
/// mesh origins. Update-time distances lagged camera interpolation by a frame.
/// No geometry is spawned in PostUpdate.
pub fn synchronize_fades(
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut parts: Query<(
        &GlobalTransform,
        &TemporalFade,
        &mut bevy::camera::visibility::VisibilityRange,
    )>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    for (global, fade, mut range) in &mut parts {
        *range = fade_range(
            global.translation().distance(camera.translation()),
            fade.weight,
            fade.incoming,
        );
    }
}
#[allow(clippy::type_complexity)]
pub fn fade(
    mut commands: Commands,
    time: Res<Time>,
    plan: Res<Architecture>,
    cameras: Query<&Transform, With<Camera3d>>,
    mut slots: Query<(Entity, &mut Slot)>,
    mut fallback: Query<
        (
            Entity,
            &mut Fallback,
            &GlobalTransform,
            &mut Transform,
            &mut Visibility,
            &mut bevy::camera::visibility::VisibilityRange,
            Option<&TemporalFade>,
        ),
        (
            Without<Camera3d>,
            Without<bevy_rapier3d::prelude::RigidBody>,
        ),
    >,
    parts: Query<
        (
            Entity,
            &FadePart,
            &GlobalTransform,
            Option<&bevy::camera::visibility::VisibilityRange>,
        ),
        Without<Fallback>,
    >,
) {
    let Ok(camera) = cameras.single() else { return };
    let camera = camera.translation;
    for (_, mut slot) in &mut slots {
        let delta = time.delta_secs() / 0.30;
        slot.blend += (slot.target - slot.blend).clamp(-delta, delta);
        slot.transition = (slot.transition + delta).min(1.0);
    }
    let mut values = vec![(0.0, [(0.0, false); 4]); plan.buildings.len()];
    for (_, slot) in &slots {
        let faces = std::array::from_fn(|face| slot.face_coverage(face));
        values[slot.index] = (slot.blend, faces);
    }
    for (fallback_entity, mut f, global, mut transform, mut visibility, mut range, temporal) in
        &mut fallback
    {
        let (weight, proxy_incoming) = f.face.map_or((values[f.index].0, false), |face| {
            values[f.index].1[usize::from(face)]
        });
        // Settled proxies retain their static range. Camera synchronization
        // for partial fades runs separately; unchanged coverage needs no work.
        if f.last_weight == weight && f.last_incoming == proxy_incoming {
            continue;
        }
        f.last_weight = weight;
        f.last_incoming = proxy_incoming;
        if f.wall {
            let b = &plan.buildings[f.index];
            let inset = b.inset(&plan.catalog.records[b.design]) * weight;
            let scale = Vec3::new(
                (f.size.x - 2.0 * inset).max(1.0),
                f.size.y,
                (f.size.z - 2.0 * inset).max(1.0),
            );
            if transform.scale != scale {
                transform.scale = scale;
            }
        } else {
            let next_visibility = if weight >= 0.999 {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
            if *visibility != next_visibility {
                *visibility = next_visibility;
            }
            let next_range = if !(0.001..0.999).contains(&weight) {
                f.range.clone()
            } else {
                fade_range(
                    global.translation().distance(camera),
                    1.0 - weight,
                    proxy_incoming,
                )
            };
            if (0.001..0.999).contains(&weight) {
                commands.entity(fallback_entity).insert(TemporalFade {
                    weight: 1.0 - weight,
                    incoming: proxy_incoming,
                });
            } else if temporal.is_some() {
                commands.entity(fallback_entity).remove::<TemporalFade>();
            }
            if *range != next_range {
                *range = next_range;
            }
        }
    }
    for (entity, part, global, range) in &parts {
        // Prepared scenes are hidden: avoid per-mesh commands and render extraction
        // until their parent becomes visible.
        if !slots.get(part.slot).is_ok_and(|(_, s)| {
            s.children.contains(&part.root)
                || s.retiring
                    .as_ref()
                    .is_some_and(|p| p.children.contains(&part.root))
        }) {
            continue;
        }
        let (weight, incoming) = slots.get(part.slot).map_or((0.0, true), |(_, s)| {
            if s.retiring
                .as_ref()
                .is_some_and(|p| p.children.contains(&part.root))
            {
                (s.blend * (1.0 - s.transition), false)
            } else {
                (s.blend * s.transition, true)
            }
        });
        if weight >= 0.999 {
            if range.is_some() {
                commands
                    .entity(entity)
                    .remove::<(bevy::camera::visibility::VisibilityRange, TemporalFade)>();
            }
        } else {
            commands.entity(entity).insert((
                fade_range(global.translation().distance(camera), weight, incoming),
                TemporalFade { weight, incoming },
            ));
        }
    }
}
#[cfg(test)]
mod fade_tests {
    use super::*;
    fn level(r: &bevy::camera::visibility::VisibilityRange, d: f32) -> i32 {
        if d >= r.end_margin.start {
            ((d - r.end_margin.start) / (r.end_margin.end - r.end_margin.start) * 16.0)
                .round()
                .clamp(0.0, 16.0) as i32
        } else {
            -16 + ((d - r.start_margin.start) / (r.start_margin.end - r.start_margin.start) * 16.0)
                .round()
                .clamp(0.0, 16.0) as i32
        }
    }
    fn visible(level: i32, threshold: i32) -> bool {
        level == 0
            || (level > -16
                && level < 16
                && !((level >= 0 && level + threshold >= 16)
                    || (level < 0 && 1 + level + threshold <= 0)))
    }
    #[test]
    fn identity_proxy_covers_faces_missing_from_either_ready_group() {
        let mut slot = Slot::new(0);
        slot.blend = 1.0;
        for old in 0..16 {
            for new in 0..16 {
                for step in 0..=16 {
                    slot.active = Some((0, new));
                    slot.retiring = Some(Prepared {
                        key: (2, old),
                        children: vec![],
                    });
                    slot.transition = step as f32 / 16.0;
                    for face in 0..4 {
                        let (coverage, incoming) = slot.face_coverage(face);
                        let proxy = level(&fade_range(35.0, 1.0 - coverage, incoming), 35.0);
                        let a = level(&fade_range(2.0, slot.transition, true), 2.0);
                        let b = level(&fade_range(120.0, 1.0 - slot.transition, false), 120.0);
                        for pixel in 0..16 {
                            let count = usize::from(visible(proxy, pixel))
                                + usize::from(new & (1 << face) != 0 && visible(a, pixel))
                                + usize::from(old & (1 << face) != 0 && visible(b, pixel));
                            assert_eq!(
                                count, 1,
                                "old={old} new={new} face={face} step={step} pixel={pixel}"
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn temporal_ranges_follow_the_propagated_render_camera() {
        fn move_camera(mut cameras: Query<&mut Transform, With<Camera3d>>) {
            cameras.single_mut().unwrap().translation = Vec3::new(100.0, 0.0, 0.0);
        }
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::transform::TransformPlugin));
        app.add_systems(
            PostUpdate,
            move_camera.before(bevy::transform::TransformSystems::Propagate),
        );
        app.add_systems(
            PostUpdate,
            synchronize_fades.after(bevy::transform::TransformSystems::Propagate),
        );
        app.world_mut()
            .spawn((Camera3d::default(), Transform::IDENTITY));
        let mesh = app
            .world_mut()
            .spawn((
                Transform::from_xyz(1.0, 0.0, 0.0),
                GlobalTransform::default(),
                TemporalFade {
                    weight: 0.25,
                    incoming: true,
                },
                fade_range(1.0, 0.25, true),
            ))
            .id();
        app.update();
        let r = app
            .world()
            .get::<bevy::camera::visibility::VisibilityRange>(mesh)
            .unwrap();
        assert_eq!(level(r, 99.0), -12);
        assert_eq!(r.start_margin.start, 97.0);
    }
    #[test]
    fn incoming_and_outgoing_cover_every_dither_pixel_at_different_origins() {
        for step in 0..=16 {
            let a = step as f32 / 16.0;
            let i = level(&fade_range(0.1, a, true), 0.1);
            let o = level(&fade_range(400.0, 1.0 - a, false), 400.0);
            for threshold in 0..16 {
                assert_ne!(
                    visible(i, threshold),
                    visible(o, threshold),
                    "step={step} threshold={threshold} levels={i}/{o}"
                );
            }
        }
    }
}
