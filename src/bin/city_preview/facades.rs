//! Actual facade artwork on UV-mapped relief surfaces, shared by source sheet.
use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageSampler, ImageSamplerDescriptor, ImageType},
    mesh::Indices,
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FacadeBatch {
    pub sheet: String,
    pub quads: Vec<FacadeQuad>,
}
#[derive(Deserialize)]
pub struct FacadeQuad {
    pub positions: [[f32; 3]; 4],
    pub uvs: [[f32; 2]; 4],
}

pub fn spawn(
    commands: &mut Commands,
    batches: Vec<FacadeBatch>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> (usize, usize) {
    assert!(
        !batches.is_empty(),
        "No facade assets: regenerate the city scene"
    );
    let sheets = batches.len();
    let quads = batches.iter().map(|b| b.quads.len()).sum();
    for batch in batches {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(&batch.sheet);
        let bytes = std::fs::read(&path)
            .unwrap_or_else(|e| panic!("Missing ACTUAL facade artwork {}: {e}", path.display()));
        let mut sampler = ImageSamplerDescriptor::linear();
        sampler.anisotropy_clamp = 8;
        let image = Image::from_buffer(
            &bytes,
            ImageType::Extension("png"),
            CompressedImageFormats::NONE,
            true,
            ImageSampler::Descriptor(sampler),
            RenderAssetUsages::RENDER_WORLD,
        )
        .expect("Decode original new facade PNG");
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(image)),
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            // Painted light/shadow stays recognizable; shell/relief provide depth.
            unlit: true,
            cull_mode: None,
            ..default()
        });
        let name = format!("Actual facade texture: {}", batch.sheet);
        commands.spawn((
            Name::new(name),
            Mesh3d(meshes.add(mesh(&batch.quads))),
            MeshMaterial3d(material),
            Transform::default(),
        ));
    }
    (sheets, quads)
}

pub fn mesh(quads: &[FacadeQuad]) -> Mesh {
    let mut positions = Vec::with_capacity(quads.len() * 4);
    let mut normals = Vec::with_capacity(quads.len() * 4);
    let mut uvs = Vec::with_capacity(quads.len() * 4);
    let mut indices = Vec::with_capacity(quads.len() * 6);
    for quad in quads {
        let p = quad.positions.map(Vec3::from_array);
        let normal = (p[1] - p[0]).cross(p[2] - p[0]).normalize_or_zero();
        assert!(
            normal.is_finite() && normal.length_squared() > 0.5,
            "Degenerate facade geometry"
        );
        assert!(
            quad.uvs
                .iter()
                .flatten()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
            "Invalid facade UV"
        );
        let base = positions.len() as u32;
        positions.extend(quad.positions);
        normals.extend([normal.to_array(); 4]);
        uvs.extend(quad.uvs);
        indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn painted_facades_retain_uvs_and_reject_empty_geometry() {
        let q = FacadeQuad {
            positions: [[0., 0., 0.], [0., 2., 0.], [1., 2., 0.], [1., 0., 0.]],
            uvs: [[0.1, 0.9], [0.1, 0.2], [0.3, 0.2], [0.3, 0.9]],
        };
        let m = mesh(&[q]);
        assert_eq!(m.count_vertices(), 4);
        assert!(m.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
        let degenerate = FacadeQuad {
            positions: [[0.; 3]; 4],
            uvs: [[0.; 2]; 4],
        };
        assert!(std::panic::catch_unwind(|| mesh(&[degenerate])).is_err());
    }
}
