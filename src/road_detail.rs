//! Original deterministic asphalt PBR, applied once to render meshes only.
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    mesh::VertexAttributeValues,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
const SIDE: usize = 512;
pub(crate) const TILE_METERS: f32 = 8.0;
fn hash(x: u32, y: u32) -> u32 {
    let mut h = x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^ (h >> 16)
}
fn grain(x: usize, y: usize) -> f32 {
    (hash((x % SIDE) as u32, (y % SIDE) as u32) & 65535) as f32 / 65535.0
}
fn height(x: usize, y: usize) -> f32 {
    grain(x, y) * 0.65 + grain(x / 4, y / 4) * 0.35
}
fn pixels() -> [Vec<u8>; 3] {
    let mut data = std::array::from_fn(|_| Vec::with_capacity(SIDE * SIDE * 4));
    for y in 0..SIDE {
        for x in 0..SIDE {
            let grit = height(x, y);
            // Quiet, metre-scale repairs; no axis-aligned tire bands repeated across lanes.
            let u = x as f32 / SIDE as f32;
            let v = y as f32 / SIDE as f32;
            let macro_tone =
                (u * std::f32::consts::TAU).sin() * (v * std::f32::consts::TAU).cos() * 2.5;
            let patch = (u > 0.12 && u < 0.28 && v > 0.18 && v < 0.43)
                || (u > 0.62 && u < 0.85 && v > 0.67 && v < 0.82);
            let tar = (y as f32 - (270.0 + 8.0 * (x as f32 * 0.021).sin())).abs() < 0.8
                && x > 80
                && x < 215;
            let value = (79.0 + grit * 17.0 + macro_tone
                - if tar { 10.0 } else { 0.0 }
                - if patch { 5.0 } else { 0.0 }) as u8;
            data[0].extend([value, value.saturating_add(2), value.saturating_add(3), 255]);
            let dx = height((x + 1) % SIDE, y) - height((x + SIDE - 1) % SIDE, y);
            let dy = height(x, (y + 1) % SIDE) - height(x, (y + SIDE - 1) % SIDE);
            let n = Vec3::new(-dx * 0.22, -dy * 0.22, 1.0).normalize();
            data[1].extend([
                ((n.x * 0.5 + 0.5) * 255.0) as u8,
                ((n.y * 0.5 + 0.5) * 255.0) as u8,
                ((n.z * 0.5 + 0.5) * 255.0) as u8,
                255,
            ]);
            data[2].extend([
                255,
                (if tar { 170.0 } else { 220.0 } + grit * 25.0) as u8,
                0,
                255,
            ]);
        }
    }
    data
}
pub(crate) fn image(base: Vec<u8>, size: usize, srgb: bool) -> Image {
    let mut result = Image::new(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        base.clone(),
        if srgb {
            TextureFormat::Rgba8UnormSrgb
        } else {
            TextureFormat::Rgba8Unorm
        },
        RenderAssetUsages::default(),
    );
    let mut all = base.clone();
    let mut previous = base;
    let mut side = size;
    let mut levels = 1;
    while side > 1 {
        let next = side / 2;
        let mut mip = vec![0; next * next * 4];
        for y in 0..next {
            for x in 0..next {
                for c in 0..4 {
                    let sum = [(0, 0), (1, 0), (0, 1), (1, 1)]
                        .into_iter()
                        .map(|(dx, dy)| {
                            u32::from(previous[((y * 2 + dy) * side + x * 2 + dx) * 4 + c])
                        })
                        .sum::<u32>();
                    mip[(y * next + x) * 4 + c] = (sum / 4) as u8;
                }
            }
        }
        all.extend(&mip);
        previous = mip;
        side = next;
        levels += 1;
    }
    result.data = Some(all);
    result.texture_descriptor.mip_level_count = levels;
    result.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    result
}
/// Shared source for legacy and Metropolis roads; UVs are measured in world metres.
pub(crate) fn asphalt(images: &mut Assets<Image>) -> StandardMaterial {
    let [albedo, normal, rough] = pixels();
    StandardMaterial {
        base_color_texture: Some(images.add(image(albedo, SIDE, true))),
        normal_map_texture: Some(images.add(image(normal, SIDE, false))),
        metallic_roughness_texture: Some(images.add(image(rough, SIDE, false))),
        perceptual_roughness: 1.0,
        reflectance: 0.18,
        ..default()
    }
}
/// Opaque faded paint retains continuous silhouettes and does not require blending.
pub(crate) fn paint(images: &mut Assets<Image>, color: Color) -> StandardMaterial {
    let mut pixels = Vec::with_capacity(64 * 64 * 4);
    for y in 0..64 {
        for x in 0..64 {
            let value = 215 + (hash(x, y) % 41) as u8;
            pixels.extend([value, value, value, 255]);
        }
    }
    StandardMaterial {
        base_color: color,
        base_color_texture: Some(images.add(image(pixels, 64, true))),
        perceptual_roughness: 0.96,
        reflectance: 0.1,
        ..default()
    }
}
#[allow(clippy::type_complexity)]
pub fn install(
    mut commands: Commands,
    images: Option<ResMut<Assets<Image>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    roads: Query<(
        Entity,
        &Mesh3d,
        Option<&crate::environment::RoadChunk>,
        Option<&crate::world_gen::surfaces::StructuralSurface>,
    )>,
) {
    let Some(mut images) = images else { return };
    // Metropolis owns its render materials; its RoadChunk components are collider-only.
    // Do not allocate an unused second PBR image set there.
    if !roads.iter().any(|(_, _, road, surface)| {
        road.is_some() || surface.is_some_and(|s| matches!(s.0, "junction" | "grid_road"))
    }) {
        return;
    }
    let material = materials.add(asphalt(&mut images));
    for (entity, handle, road, surface) in &roads {
        if road.is_none() && !surface.is_some_and(|s| matches!(s.0, "junction" | "grid_road")) {
            continue;
        }
        if let Some(mut mesh) = meshes.get_mut(&handle.0) {
            if let Some(VertexAttributeValues::Float32x3(p)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            {
                let uv = p
                    .iter()
                    .map(|v| [v[0] / TILE_METERS, v[2] / TILE_METERS])
                    .collect::<Vec<_>>();
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
            }
            mesh.remove_attribute(Mesh::ATTRIBUTE_COLOR);
            mesh.generate_tangents()
                .expect("road triangles have valid world-space UVs");
        }
        commands
            .entity(entity)
            .insert(MeshMaterial3d(material.clone()));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_installer_does_not_allocate_without_matching_render_surfaces() {
        let mut app = App::new();
        app.init_resource::<Assets<Image>>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Update, install);
        app.world_mut().spawn(crate::environment::RoadChunk);
        app.update();
        assert_eq!(app.world().resource::<Assets<Image>>().len(), 0);
        assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 0);
    }
    #[test]
    fn paint_stays_opaque_and_readable_through_all_distance_mips() {
        let mut images = Assets::<Image>::default();
        let material = paint(&mut images, Color::WHITE);
        assert_eq!(material.alpha_mode, AlphaMode::Opaque);
        let image = images
            .get(material.base_color_texture.as_ref().unwrap())
            .unwrap();
        assert_eq!(image.texture_descriptor.mip_level_count, 7);
        for pixel in image.data.as_ref().unwrap().chunks_exact(4) {
            assert!(pixel[0] >= 215);
            assert_eq!(pixel[3], 255);
        }
    }
    #[test]
    fn asphalt_has_repeat_sampling_full_mips_and_bounded_pbr_channels() {
        let [a, n, r] = pixels();
        assert_eq!(a.len(), SIDE * SIDE * 4);
        assert!(a.chunks_exact(4).all(|p| p[0] >= 20 && p[0] <= 100));
        assert!(n.chunks_exact(4).all(|p| p[2] >= 250));
        assert!(r.chunks_exact(4).all(|p| p[1] >= 160 && p[2] == 0));
        assert_eq!(grain(0, 0), grain(SIDE, SIDE));
        let im = image(a, SIDE, true);
        assert_eq!(im.texture_descriptor.mip_level_count, 10);
        assert_eq!(
            im.data.unwrap().len(),
            (0..10).map(|i| (SIDE >> i).pow(2) * 4).sum::<usize>()
        );
    }
}
