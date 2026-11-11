//! Shared original micro-normal/roughness maps, generated off the gameplay thread.
use bevy::{
    prelude::*,
    render::render_resource::TextureFormat,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};
use rr_content::material_detail::{Config, Kind, Profile};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
const JSON: &str = include_str!("../content/world/material_profiles.json");
#[derive(Clone)]
struct Maps {
    normal: Handle<Image>,
    rough: Handle<Image>,
}
struct Derived {
    size: usize,
    normal: Vec<u8>,
    rough: Vec<u8>,
}
struct ResultMap {
    fingerprint: Option<u64>,
    derived: Option<Derived>,
}
struct Job {
    id: AssetId<Image>,
    task: Task<ResultMap>,
}
#[derive(Resource)]
pub struct Queue {
    profiles: Arc<HashMap<u64, Profile>>,
    pending: HashSet<AssetId<StandardMaterial>>,
    done: HashSet<AssetId<StandardMaterial>>,
    sources: HashMap<AssetId<Image>, Option<u64>>,
    ready: HashMap<u64, Maps>,
    jobs: Vec<Job>,
    inflight: HashSet<AssetId<Image>>,
    stable: usize,
    pub applied: usize,
}
impl Default for Queue {
    fn default() -> Self {
        let profiles = Config::from_json(JSON).expect("validated material provenance");
        Self {
            profiles: Arc::new(
                profiles
                    .profiles
                    .into_iter()
                    .map(|p| (u64::from_str_radix(&p.pixel_fnv64, 16).unwrap(), p))
                    .collect(),
            ),
            pending: default(),
            done: default(),
            sources: default(),
            ready: default(),
            jobs: default(),
            inflight: default(),
            stable: 0,
            applied: 0,
        }
    }
}
impl Queue {
    pub fn material_ready(&self, id: AssetId<StandardMaterial>) -> bool {
        self.done.contains(&id)
    }
    pub(crate) fn loading_evidence(&self) -> serde_json::Value {
        serde_json::json!({"pending":self.pending.len(),"jobs":self.jobs.len(),"done":self.done.len()})
    }
    pub fn settled(&self) -> bool {
        self.jobs.is_empty() && self.pending.is_empty() && self.stable >= 2
    }
    pub fn unique_maps(&self) -> usize {
        self.ready.len()
    }
}
fn fingerprint(pixels: &[u8]) -> u64 {
    pixels.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(1099511628211)
    })
}
fn derive(pixels: &[u8], size: usize, profile: &Profile) -> Derived {
    debug_assert!(size.is_power_of_two());
    // Each source texel previously converted from RGB to height four times,
    // once for every neighbouring normal. Cache that exact conversion locally
    // for this job; the authored source image and resulting maps are unchanged.
    let heights: Vec<f32> = pixels
        .chunks_exact(4)
        .map(|p| {
            let l = (f32::from(p[0]) + f32::from(p[1]) + f32::from(p[2])) / (255.0 * 3.0);
            match profile.kind {
                Kind::Brick => 1.0 - l,
                Kind::Wood => l * 0.7,
                Kind::Stone => l,
                Kind::Metal | Kind::Plastic => l * 0.08,
                Kind::Plaster => l * 0.2,
            }
        })
        .collect();
    let wrap = size - 1;
    let height = |x: usize, y: usize| heights[((y & wrap) * size) + (x & wrap)];
    let mut normal = Vec::with_capacity(size * size * 4);
    let mut rough = Vec::with_capacity(normal.capacity());
    for y in 0..size {
        for x in 0..size {
            let dx = (height(x + 1, y) - height(x + size - 1, y)) * profile.normal_strength;
            let dy = (height(x, y + 1) - height(x, y + size - 1)) * profile.normal_strength;
            let n = Vec3::new(-dx, -dy, 1.0).normalize();
            normal.extend([
                ((n.x * 0.5 + 0.5) * 255.0) as u8,
                ((n.y * 0.5 + 0.5) * 255.0) as u8,
                ((n.z * 0.5 + 0.5) * 255.0) as u8,
                255,
            ]);
            let micro =
                ((x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263)) & 255;
            let r = (profile.roughness + (micro as f32 / 255.0 - 0.5) * 0.08).clamp(0.1, 1.0);
            // Metallic B=1 preserves the original material's metallic multiplier.
            rough.extend([255, (r * 255.0) as u8, 255, 255]);
        }
    }
    Derived {
        size,
        normal,
        rough,
    }
}
pub fn update(
    mut queue: ResMut<Queue>,
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    mut image_events: MessageReader<AssetEvent<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    screen: Option<Res<State<crate::runtime::AppScreen>>>,
) {
    // Measured loading-only admission; background worker count is unchanged.
    // Retain the two-job gameplay budget and cap requested load admission at eight.
    let job_limit = if screen.as_ref().is_some_and(|s| {
        matches!(
            s.get(),
            crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
        )
    }) {
        std::env::var("ROADDASH_LOAD_JOBS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(8)
            .clamp(2, 8)
    } else {
        2
    };
    for event in image_events.read() {
        if let AssetEvent::Removed { id } = event {
            queue.sources.remove(id);
        }
    }
    for event in events.read() {
        if let AssetEvent::Removed { id } = event {
            queue.done.remove(id);
            queue.pending.remove(id);
            continue;
        }

        if let AssetEvent::Added { id } | AssetEvent::Modified { id } = event
            && !queue.done.contains(id)
        {
            queue.pending.insert(*id);
        }
    }
    let mut completed = Vec::new();
    let mut i = 0;
    while i < queue.jobs.len() {
        if let Some(result) = block_on(future::poll_once(&mut queue.jobs[i].task)) {
            let job = queue.jobs.swap_remove(i);
            queue.inflight.remove(&job.id);
            completed.push((job.id, result));
        } else {
            i += 1;
        }
    }
    for (id, result) in completed {
        if !images.contains(id) {
            continue;
        }
        queue.sources.insert(id, result.fingerprint);
        if let (Some(key), Some(derived)) = (result.fingerprint, result.derived) {
            queue.ready.entry(key).or_insert_with(|| Maps {
                normal: images.add(crate::road_detail::image(
                    derived.normal,
                    derived.size,
                    false,
                )),
                rough: images.add(crate::road_detail::image(
                    derived.rough,
                    derived.size,
                    false,
                )),
            });
        }
    }
    let pending: Vec<_> = queue.pending.iter().copied().collect();
    for id in pending {
        let Some(material) = materials.get(id) else {
            queue.pending.remove(&id);
            continue;
        };
        let Some(albedo) = material.base_color_texture.as_ref().map(Handle::id) else {
            queue.done.insert(id);
            queue.pending.remove(&id);
            continue;
        };
        if material.normal_map_texture.is_some() && material.metallic_roughness_texture.is_some() {
            queue.done.insert(id);
            queue.pending.remove(&id);
            continue;
        }
        if let Some(source) = queue.sources.get(&albedo) {
            let maps = source.and_then(|key| queue.ready.get(&key)).cloned();
            if let Some(maps) = maps
                && let Some(material) = materials.get_mut(id)
            {
                if material.normal_map_texture.is_none() {
                    material.normal_map_texture = Some(maps.normal);
                }
                if material.metallic_roughness_texture.is_none() {
                    material.metallic_roughness_texture = Some(maps.rough);
                    material.perceptual_roughness = 1.0;
                }
                queue.applied += 1;
            }
            queue.done.insert(id);
            queue.pending.remove(&id);
            continue;
        }
        if queue.jobs.len() >= job_limit || queue.inflight.contains(&albedo) {
            continue;
        }
        let Some(image) = images.get(albedo) else {
            continue;
        };
        let size = image.texture_descriptor.size.width as usize;
        if size > 1024
            || size != image.texture_descriptor.size.height as usize
            || !size.is_power_of_two()
            || !matches!(
                image.texture_descriptor.format,
                TextureFormat::Rgba8UnormSrgb | TextureFormat::Rgba8Unorm
            )
        {
            queue.sources.insert(albedo, None);
            continue;
        }
        let Some(pixels) = image.data.as_ref() else {
            continue;
        };
        if pixels.len() != size * size * 4 {
            queue.sources.insert(albedo, None);
            continue;
        }
        let pixels = pixels.clone();
        let profiles = queue.profiles.clone();
        let cached: HashSet<_> = queue.ready.keys().copied().collect();
        let task = AsyncComputeTaskPool::get().spawn(async move {
            let hash = fingerprint(&pixels);
            let Some(profile) = profiles.get(&hash) else {
                return ResultMap {
                    fingerprint: None,
                    derived: None,
                };
            };
            let derived = (!cached.contains(&hash)).then(|| derive(&pixels, size, profile));
            ResultMap {
                fingerprint: Some(hash),
                derived,
            }
        });
        queue.jobs.push(Job { id: albedo, task });
        queue.inflight.insert(albedo);
    }
    if queue.pending.is_empty() && queue.jobs.is_empty() {
        queue.stable += 1;
    } else {
        queue.stable = 0;
    }
}
/// Reuse the original concrete tile on the actual walkable surfaces, with metre UVs.
pub fn paving(
    server: Option<Res<AssetServer>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    walks: Query<(
        &crate::world_gen::surfaces::StructuralSurface,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    let Some(server) = server else { return };
    let texture = server.load_with_settings(
        "architecture://materials/concrete_albedo.png",
        |settings: &mut bevy::image::ImageLoaderSettings| {
            settings.sampler =
                bevy::image::ImageSampler::Descriptor(bevy::image::ImageSamplerDescriptor {
                    address_mode_u: bevy::image::ImageAddressMode::Repeat,
                    address_mode_v: bevy::image::ImageAddressMode::Repeat,
                    ..bevy::image::ImageSamplerDescriptor::linear()
                });
        },
    );
    for (surface, handle) in &walks {
        if matches!(surface.0, "sidewalk" | "grid_walk")
            && let Some(material) = materials.get_mut(&handle.0)
        {
            material.base_color = Color::srgb(0.88, 0.87, 0.84);
            material.base_color_texture = Some(texture.clone());
            material.uv_transform = bevy::math::Affine2::from_scale(Vec2::splat(5.0));
            material.perceptual_roughness = 0.95;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn reference_derive(pixels: &[u8], size: usize, profile: &Profile) -> Derived {
        let height = |x: usize, y: usize| {
            let p = &pixels[((y % size) * size + x % size) * 4..];
            let l = (f32::from(p[0]) + f32::from(p[1]) + f32::from(p[2])) / (255.0 * 3.0);
            match profile.kind {
                Kind::Brick => 1.0 - l,
                Kind::Wood => l * 0.7,
                Kind::Stone => l,
                Kind::Metal | Kind::Plastic => l * 0.08,
                Kind::Plaster => l * 0.2,
            }
        };
        let mut normal = Vec::with_capacity(size * size * 4);
        let mut rough = Vec::with_capacity(size * size * 4);
        for y in 0..size {
            for x in 0..size {
                let dx = (height(x + 1, y) - height(x + size - 1, y)) * profile.normal_strength;
                let dy = (height(x, y + 1) - height(x, y + size - 1)) * profile.normal_strength;
                let n = Vec3::new(-dx, -dy, 1.0).normalize();
                normal.extend([
                    ((n.x * 0.5 + 0.5) * 255.0) as u8,
                    ((n.y * 0.5 + 0.5) * 255.0) as u8,
                    ((n.z * 0.5 + 0.5) * 255.0) as u8,
                    255,
                ]);
                let micro =
                    ((x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263)) & 255;
                let r = (profile.roughness + (micro as f32 / 255.0 - 0.5) * 0.08).clamp(0.1, 1.0);
                rough.extend([255, (r * 255.0) as u8, 255, 255]);
            }
        }
        Derived {
            size,
            normal,
            rough,
        }
    }
    #[test]
    fn cached_heights_match_original_derived_bytes_for_every_profile() {
        let config = Config::from_json(JSON).unwrap();
        for size in [2, 8, 32] {
            let pixels: Vec<_> = (0..size * size)
                .flat_map(|i| {
                    [
                        ((i * 37 + 19) & 255) as u8,
                        ((i * 89 + 5) & 255) as u8,
                        ((i * 13 + 211) & 255) as u8,
                        255,
                    ]
                })
                .collect();
            for profile in &config.profiles {
                let old = reference_derive(&pixels, size, profile);
                let new = derive(&pixels, size, profile);
                assert_eq!(
                    new.normal, old.normal,
                    "normal map {} at {size}",
                    profile.source
                );
                assert_eq!(
                    new.rough, old.rough,
                    "rough map {} at {size}",
                    profile.source
                );
            }
        }
    }
    #[test]
    fn provenance_profiles_are_bounded_and_maps_preserve_metallic() {
        let c = Config::from_json(JSON).unwrap();
        assert_eq!(c.profiles.len(), 34);
        let pixels = [
            vec![30, 30, 30, 255],
            vec![180, 180, 180, 255],
            vec![30, 30, 30, 255],
            vec![180, 180, 180, 255],
        ]
        .concat();
        let d = derive(&pixels, 2, &c.profiles[0]);
        assert!(d.normal.chunks_exact(4).all(|p| p[2] >= 220 && p[3] == 255));
        assert!(d.rough.chunks_exact(4).all(|p| p[2] == 255 && p[1] >= 20));
        assert_eq!(fingerprint(b"hello"), 0xa430d84680aabd0b);
        let bad = JSON.replacen("0.32", "3.2", 1);
        assert!(Config::from_json(&bad).is_err());
    }
    #[test]
    fn facade_baker_roughness_fixture_matches_actual_runtime_maps() {
        let fixtures: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/testing/polish-12/pass07/roughness-fixture.json"
        ))
        .unwrap();
        let c = Config::from_json(JSON).unwrap();
        for row in fixtures.as_array().unwrap() {
            let roughness = row["roughness"].as_f64().unwrap() as f32;
            let profile = c
                .profiles
                .iter()
                .find(|p| p.roughness == roughness)
                .unwrap();
            let derived = derive(&vec![128; 512 * 512 * 4], 512, profile);
            for (index, (x, y)) in [(0, 0), (1, 0), (0, 1), (37, 91), (511, 511)]
                .into_iter()
                .enumerate()
            {
                assert_eq!(
                    u64::from(derived.rough[(y * 512 + x) * 4 + 1]),
                    row["values"][index].as_u64().unwrap()
                );
            }
        }
    }
    #[test]
    fn native_png_decoder_matches_every_original_palette_profile() {
        let c = Config::from_json(JSON).unwrap();
        for p in c.profiles {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(&p.source);
            let bytes = std::fs::read(path).unwrap();
            let image = Image::from_buffer(
                &bytes,
                bevy::image::ImageType::Extension("png"),
                bevy::image::CompressedImageFormats::NONE,
                true,
                bevy::image::ImageSampler::default(),
                bevy::asset::RenderAssetUsages::default(),
            )
            .unwrap();
            assert_eq!(
                format!("{:016x}", fingerprint(image.data.as_ref().unwrap())),
                p.pixel_fnv64,
                "{}",
                p.source
            );
        }
    }
}
