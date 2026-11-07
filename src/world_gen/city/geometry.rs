//! Shared preview/racing geometry; UVs map original authored material tiles.
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use rr_content::city::CityChunk;
use std::collections::HashMap;
pub fn chunk_mesh(chunk: &CityChunk) -> Mesh {
    chunk_mesh_filtered(chunk, false)
}
fn chunk_mesh_filtered(chunk: &CityChunk, props: bool) -> Mesh {
    const FACES: [([usize; 4], [f32; 3]); 6] = [
        ([0, 3, 2, 1], [0., 0., -1.]),
        ([4, 5, 6, 7], [0., 0., 1.]),
        ([0, 1, 5, 4], [0., -1., 0.]),
        ([3, 7, 6, 2], [0., 1., 0.]),
        ([0, 4, 7, 3], [-1., 0., 0.]),
        ([1, 2, 6, 5], [1., 0., 0.]),
    ];
    let mut positions = Vec::with_capacity(chunk.boxes.len() * 24);
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut indices = Vec::with_capacity(chunk.boxes.len() * 36);
    for (name, center, size, color) in &chunk.boxes {
        if props && matches!(name.as_str(), "trunk" | "canopy" | "lamp") {
            continue;
        }
        let c = Vec3::from_array(*center);
        let half = Vec3::from_array(*size) * 0.5;
        let points = [
            (-1., -1., -1.),
            (1., -1., -1.),
            (1., 1., -1.),
            (-1., 1., -1.),
            (-1., -1., 1.),
            (1., -1., 1.),
            (1., 1., 1.),
            (-1., 1., 1.),
        ]
        .map(|(x, y, z)| (c + half * Vec3::new(x, y, z)).to_array());
        let rgba = Color::srgb_u8(color[0], color[1], color[2])
            .to_linear()
            .to_f32_array();
        for (face, normal) in FACES {
            let base = positions.len() as u32;
            for vertex in face {
                let point = points[vertex];
                uvs.push(if normal[1] != 0.0 {
                    [point[0] / 1.6, point[2] / 1.6]
                } else if normal[0] != 0.0 {
                    [point[2] / 1.6, point[1] / 1.6]
                } else {
                    [point[0] / 1.6, point[1] / 1.6]
                });
                positions.push(point);
                normals.push(normal);
                colors.push(rgba);
            }
            indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    server: Option<&AssetServer>,
    chunks: &[CityChunk],
) {
    spawn_dressed(commands, meshes, materials, server, chunks, false);
}
pub fn spawn_dressed(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    server: Option<&AssetServer>,
    chunks: &[CityChunk],
    props: bool,
) {
    let mut handles = HashMap::<String, Handle<StandardMaterial>>::new();
    for chunk in chunks {
        if props
            && chunk
                .boxes
                .iter()
                .all(|b| matches!(b.0.as_str(), "trunk" | "canopy" | "lamp"))
        {
            continue;
        }
        let key = chunk
            .material
            .as_deref()
            .unwrap_or("vertex-colors")
            .to_owned();
        let material = handles
            .entry(key.clone())
            .or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color_texture: chunk.material.as_ref().and(server).map(|s| {
                        s.load_with_settings(
                            format!("architecture://materials/{key}_albedo.png"),
                            |settings: &mut bevy::image::ImageLoaderSettings| {
                                settings.sampler = bevy::image::ImageSampler::Descriptor(
                                    bevy::image::ImageSamplerDescriptor {
                                        address_mode_u: bevy::image::ImageAddressMode::Repeat,
                                        address_mode_v: bevy::image::ImageAddressMode::Repeat,
                                        ..default()
                                    },
                                );
                            },
                        )
                    }),
                    perceptual_roughness: 0.85,
                    ..default()
                })
            })
            .clone();
        let mut entity = commands.spawn((
            Name::new(chunk.name.clone()),
            Mesh3d(meshes.add(chunk_mesh_filtered(chunk, props))),
            MeshMaterial3d(material),
            Transform::default(),
        ));
        if chunk.name == "roads" {
            entity.insert(crate::environment::RoadChunk);
        }
    }
}
