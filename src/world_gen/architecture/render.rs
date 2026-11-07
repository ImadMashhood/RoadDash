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
}
struct Prepared {
    key: (usize, u8),
    children: Vec<Entity>,
}
impl Slot {
    pub fn prepared_roots(&self) -> &[Entity] {
        self.prepared
            .as_ref()
            .map_or(&[], |p| p.children.as_slice())
    }
    pub fn scene_roots(&self) -> &[Entity] {
        &self.children
    }

    pub fn new(index: usize) -> Self {
        Self {
            index,
            active: None,
            children: vec![],
            prepared: None,
            blend: 0.0,
            target: 0.0,
        }
    }
}
#[derive(Component)]
pub struct Fallback {
    pub index: usize,
    pub wall: bool,
    pub size: Vec3,
    pub range: bevy::camera::visibility::VisibilityRange,
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
    let mut nearest: Vec<_> = slots
        .iter()
        .map(|(e, s)| {
            (
                e,
                s.index,
                plan.buildings[s.index]
                    .position
                    .distance(camera.translation),
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
        let mask = b.face_mask(camera.translation);
        let panels = b.panels(design, mask);
        if reserved + panels.len() > plan.rules.max_active_panels {
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
        while lod < 2 && triangles + count > plan.rules.max_active_triangles {
            lod += 1;
            count = panels
                .iter()
                .map(|p| design.lods[lod].triangles[p.band])
                .sum::<usize>()
                .max(displayed);
        }
        if triangles + count > plan.rules.max_active_triangles {
            continue;
        }
        considered += 1;
        reserved += panels.len();
        triangles += count;
        let tick = stream.tick;
        let scenes = if lod == 2 {
            stream.coarse[&b.design].scenes.clone()
        } else {
            let cached = stream
                .cache
                .entry((b.design, lod))
                .or_insert_with(|| Cached {
                    scenes: std::array::from_fn(|band| {
                        server.load(format!(
                            "architecture://storey_kit/{}_lod{lod}.glb#Scene{band}",
                            design.id
                        ))
                    }),
                    used: tick,
                });
            cached.used = tick;
            requested.insert((b.design, lod));
            cached.scenes.clone()
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
            sum.1 + s.children.len(),
            sum.2 + cost(s.index, s.active),
        )
    });
    for (entity, mut slot) in &mut slots {
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
            })
        });
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
            stream.active_panels += slot.children.len();
            let b = &plan.buildings[slot.index];
            let design = &plan.catalog.records[b.design];
            stream.active_triangles += b
                .panels(design, mask)
                .iter()
                .map(|p| design.lods[lod].triangles[p.band])
                .sum::<usize>();
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
            commands.entity(entity).insert(FadePart { slot: root.slot });
            if root.coarse {
                commands.entity(entity).insert(bevy::light::NotShadowCaster);
            }
        }
    }
}
/// Encode time opacity in Bevy's existing dithered-LOD shader, without cloning materials.
fn fade_range(distance: f32, weight: f32) -> bevy::camera::visibility::VisibilityRange {
    let width = (distance * 0.5).clamp(0.01, 5.0);
    let alpha = weight.clamp(0.0, 1.0);
    bevy::camera::visibility::VisibilityRange {
        start_margin: 0.0..0.0,
        end_margin: (distance - (1.0 - alpha) * width)..(distance + alpha * width),
        use_aabb: false,
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
            &Fallback,
            &GlobalTransform,
            &mut Transform,
            &mut Visibility,
            &mut bevy::camera::visibility::VisibilityRange,
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
            &InheritedVisibility,
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
    }
    let mut values = vec![0.0; plan.buildings.len()];
    for (_, slot) in &slots {
        values[slot.index] = slot.blend;
    }
    for (f, global, mut transform, mut visibility, mut range) in &mut fallback {
        let weight = values[f.index];
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
                fade_range(global.translation().distance(camera), 1.0 - weight)
            };
            if *range != next_range {
                *range = next_range;
            }
        }
    }
    for (entity, part, inherited, global, range) in &parts {
        // Prepared scenes are hidden: avoid per-mesh commands and render extraction
        // until their parent becomes visible.
        if !inherited.get() {
            continue;
        }
        let weight = slots.get(part.slot).map_or(0.0, |(_, s)| s.blend);
        if weight >= 0.999 {
            if range.is_some() {
                commands
                    .entity(entity)
                    .remove::<bevy::camera::visibility::VisibilityRange>();
            }
        } else {
            commands
                .entity(entity)
                .insert(fade_range(global.translation().distance(camera), weight));
        }
    }
}
#[cfg(test)]
mod fade_tests {
    use super::*;
    #[test]
    fn dither_weights_match_across_different_band_origins() {
        for d in [0.1, 2.0, 30.0, 180.0, 400.0] {
            for a in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let r = fade_range(d, a);
                let actual = (r.end_margin.end - d) / (r.end_margin.end - r.end_margin.start);
                assert!((actual - a).abs() < 0.001);
            }
        }
    }
}
