//! Resident, material-matched distant facade proxies baked from actual storeys.
//! Physical buildings and close authored geometry are untouched.
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    mesh::{Indices, PrimitiveTopology},
    render::render_resource::{Face, TextureFormat},
};
use std::collections::BTreeMap;
pub const JSON: &str = include_str!("../../../art/buildings/mesh_assets/far_facades/MANIFEST.json");
#[derive(Resource)]
pub struct Identity {
    pub material: Handle<StandardMaterial>,
    pub tiles: BTreeMap<(String, usize), rr_content::facade_atlas::Tile>,
    sources: Vec<(String, u32, Handle<Image>)>,
    ready: bool,
    pub meshes: usize,
    pub quads: usize,
    pub bytes: usize,
}
impl Identity {
    #[cfg(test)]
    pub(super) fn for_test(atlas: rr_content::facade_atlas::Atlas) -> Self {
        Self {
            material: Handle::default(),
            tiles: atlas
                .records
                .into_iter()
                .map(|t| ((t.design.clone(), t.band), t))
                .collect(),
            sources: vec![],
            ready: false,
            meshes: 0,
            quads: 0,
            bytes: 0,
        }
    }

    pub fn load(
        plan: &Architecture,
        server: &AssetServer,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let atlas = rr_content::facade_atlas::Atlas::from_json(JSON, &plan.catalog)
            .expect("verified facade atlas");
        let sources = atlas
            .images
            .iter()
            .map(|m| {
                let srgb = m.kind == "albedo";
                let image = server
                    .load_builder()
                    .with_settings(move |s: &mut bevy::image::ImageLoaderSettings| {
                        s.is_srgb = srgb;
                    })
                    .load(format!("architecture://far_facades/{}", m.path));
                (m.kind.clone(), m.level, image)
            })
            .collect();
        Self {
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                alpha_mode: AlphaMode::Mask(0.25),
                metallic: 1.0,
                perceptual_roughness: 1.0,
                cull_mode: Some(Face::Back),
                ..default()
            }),
            tiles: atlas
                .records
                .into_iter()
                .map(|t| ((t.design.clone(), t.band), t))
                .collect(),
            sources,
            ready: false,
            meshes: 0,
            quads: 0,
            bytes: 0,
        }
    }
    pub fn ready(&self) -> bool {
        self.ready
    }
    pub fn mesh(&mut self, building: &Assembly, design: &Design, face: u8) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tangents = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        for panel in building.panels(design, 1 << face) {
            let Some(tile) = self.tiles.get(&(design.id.clone(), panel.band)) else {
                continue;
            };
            if tile.covered_pixels == 0 {
                continue;
            }
            let lo = Vec3::from_array(tile.bounds.min);
            let hi = Vec3::from_array(tile.bounds.max);
            let front = design
                .lods
                .iter()
                .map(|l| l.bounds[panel.band].max[2])
                .fold(0.0f32, f32::max)
                + 0.015 / panel.transform.scale.z;
            let base = positions.len() as u32;
            for p in [
                Vec3::new(lo.x, lo.y, front),
                Vec3::new(hi.x, lo.y, front),
                Vec3::new(hi.x, hi.y, front),
                Vec3::new(lo.x, hi.y, front),
            ] {
                positions.push(panel.transform.transform_point(p).to_array());
            }
            normals.extend([panel.transform.rotation * Vec3::Z; 4].map(|v| v.to_array()));
            tangents.extend([(panel.transform.rotation * Vec3::X).extend(-1.0).to_array(); 4]);
            uvs.extend([
                [tile.uv_min[0], tile.uv_max[1]],
                tile.uv_max,
                [tile.uv_max[0], tile.uv_min[1]],
                tile.uv_min,
            ]);
            indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
            self.quads += 1;
        }
        self.meshes += 1;
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
    }
}
/// Join offline gamma-correct/normal-renormalized mip images only during loading.
/// No mip work, source file IO or mesh rebuilding occurs during racing.
pub fn prepare(
    identity: Option<ResMut<Identity>>,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(mut identity) = identity else {
        return;
    };
    if identity.ready {
        return;
    }
    for (_, _, h) in &identity.sources {
        if let Some(bevy::asset::LoadState::Failed(e)) = server.get_load_state(h.id()) {
            panic!("Required facade atlas failed: {e}");
        }
        if !server.is_loaded_with_dependencies(h.id()) || images.get(h).is_none() {
            return;
        }
    }
    let mut handles = Vec::new();
    for kind in ["albedo", "normal", "packed"] {
        let chain: Vec<_> = identity
            .sources
            .iter()
            .filter(|(k, _, _)| k == kind)
            .collect();
        let mut image = images.get(&chain[0].2).unwrap().clone();
        let mut bytes = Vec::new();
        for (_, level, h) in chain {
            let mip = images.get(h).unwrap();
            assert_eq!(mip.width(), 2048 >> level);
            assert_eq!(mip.height(), 4096 >> level);
            let data = mip.data.as_ref().expect("CPU atlas data");
            assert_eq!(data.len(), mip.width() as usize * mip.height() as usize * 4);
            bytes.extend_from_slice(data);
        }
        identity.bytes += bytes.len();
        image.data = Some(bytes);
        image.texture_descriptor.mip_level_count = 5;
        image.texture_descriptor.format = if kind == "albedo" {
            TextureFormat::Rgba8UnormSrgb
        } else {
            TextureFormat::Rgba8Unorm
        };
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            lod_max_clamp: 4.0,
            ..ImageSamplerDescriptor::linear()
        });
        handles.push(images.add(image));
    }
    let mut material = materials
        .get_mut(&identity.material)
        .expect("facade material");
    material.base_color_texture = Some(handles[0].clone());
    material.normal_map_texture = Some(handles[1].clone());
    material.metallic_roughness_texture = Some(handles[2].clone());
    assert!(
        identity.bytes <= 128 * 1024 * 1024,
        "facade atlas exceeded resident byte cap"
    );
    identity.sources.clear();
    identity.ready = true;
}
