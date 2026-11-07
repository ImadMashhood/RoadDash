//! Original deterministic asphalt PBR, applied once to render meshes only.
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    mesh::VertexAttributeValues,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
const SIDE: usize = 512;
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
            let coarse = grain(x / 32, y / 32);
            let tar = (y as f32 - (270.0 + 8.0 * (x as f32 * 0.021).sin())).abs() < 1.3
                && x > 80
                && x < 415;
            let patch = x > 275 && x < 485 && y > 55 && y < 200;
            let tire = (x as f32 / SIDE as f32 - 0.28).abs() < 0.045
                || (x as f32 / SIDE as f32 - 0.72).abs() < 0.045;
            let value = (49.0 + grit * 25.0 + coarse * 6.0
                - if tar { 18.0 } else { 0.0 }
                - if patch { 7.0 } else { 0.0 }
                - if tire { 3.0 } else { 0.0 }) as u8;
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
    let [albedo, normal, rough] = pixels();
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(image(albedo, SIDE, true))),
        normal_map_texture: Some(images.add(image(normal, SIDE, false))),
        metallic_roughness_texture: Some(images.add(image(rough, SIDE, false))),
        perceptual_roughness: 1.0,
        reflectance: 0.18,
        ..default()
    });
    for (entity, handle, road, surface) in &roads {
        if road.is_none() && !surface.is_some_and(|s| matches!(s.0, "junction" | "grid_road")) {
            continue;
        }
        if let Some(mesh) = meshes.get_mut(&handle.0) {
            if let Some(VertexAttributeValues::Float32x3(p)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            {
                let uv = p
                    .iter()
                    .map(|v| [v[0] / 6.0, v[2] / 6.0])
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
