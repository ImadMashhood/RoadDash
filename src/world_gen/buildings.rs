//! Original structural kit. Near/far meshes share their owner's exact local origin.
use super::{WorldPlan, layout::WorldLayout};
use crate::world::{Art, Obstacle};
use bevy::{camera::visibility::VisibilityRange, mesh::VertexAttributeValues, prelude::*};
use bevy_rapier3d::prelude::*;
use rr_content::world::{Family, Roof};
use std::collections::HashMap;
#[derive(Component)]
pub struct GeneratedBuilding {
    pub id: String,
    pub archetype_id: String,
    pub section_id: String,
}
#[derive(Component)]
pub struct BuildingLod {
    pub near: bool,
}
#[derive(Component)]
pub struct BuildingChunk(pub u32);
fn cube(height: f32, bottom: f32) -> Mesh {
    let mut mesh = Mesh::from(Cuboid::new(1.0, height, 1.0));
    if let Some(VertexAttributeValues::Float32x3(vertices)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for p in vertices {
            p[1] += bottom + height * 0.5;
        }
    }
    mesh
}
fn roof_mesh(style: Roof, base: f32) -> Mesh {
    if matches!(style, Roof::Flat) {
        return cube(1.0 - base, base);
    }
    let teeth = if matches!(style, Roof::Sawtooth) {
        3
    } else {
        1
    };
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for i in 0..teeth {
        let left = -0.5 + i as f32 / teeth as f32;
        let right = left + 1.0 / teeth as f32;
        let peak = if teeth == 1 {
            (left + right) * 0.5
        } else {
            right
        };
        let n = vertices.len() as u32;
        vertices.extend([
            Vec3::new(left, base, -0.5),
            Vec3::new(right, base, -0.5),
            Vec3::new(peak, 1.0, -0.5),
            Vec3::new(left, base, 0.5),
            Vec3::new(right, base, 0.5),
            Vec3::new(peak, 1.0, 0.5),
        ]);
        for t in [
            [0, 2, 1],
            [3, 4, 5],
            [0, 3, 5],
            [0, 5, 2],
            [1, 2, 5],
            [1, 5, 4],
            [0, 1, 4],
            [0, 4, 3],
        ] {
            indices.push(t.map(|v| v + n));
        }
    }
    crate::environment::mesh(&vertices, &indices)
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    plan: &WorldPlan,
    layout: &WorldLayout,
) {
    let proxy_mesh = meshes.add(cube(1.0, 0.0));
    let mut shapes = HashMap::<(u8, u32), (Handle<Mesh>, Handle<Mesh>)>::new();
    let mut palette = Vec::new();
    for (i, a) in plan.archetypes.archetypes.iter().enumerate() {
        let base = match a.family {
            Family::Farm => Vec3::new(0.58, 0.27, 0.19),
            Family::Residential => Vec3::new(0.68, 0.60, 0.43),
            Family::Commercial => Vec3::new(0.44, 0.48, 0.50),
            Family::Urban => Vec3::new(0.40, 0.48, 0.55),
            Family::Hill => Vec3::new(0.40, 0.30, 0.20),
        };
        let base = base * (0.82 + (i % 4) as f32 * 0.08);
        palette.push(materials.add(StandardMaterial {
            base_color: Color::srgb(base.x, base.y, base.z),
            perceptual_roughness: 0.88,
            ..default()
        }));
    }
    for b in &layout.buildings {
        let index = plan
            .archetypes
            .archetypes
            .iter()
            .position(|a| a.id == b.archetype_id)
            .unwrap();
        let a = &plan.archetypes.archetypes[index];
        let size = Vec3::from_array(b.size);
        let (kind, roof_height) = match a.roof {
            Roof::Flat => (0, 0.3),
            Roof::Pitched => (1, 1.3),
            Roof::Sawtooth => (2, 1.0),
        };
        let wall_height = size.y - roof_height;
        let (wall_mesh, roof_mesh) = shapes
            .entry((kind, size.y.to_bits()))
            .or_insert_with(|| {
                let base = wall_height / size.y;
                (
                    meshes.add(cube(base, 0.0)),
                    meshes.add(roof_mesh(a.roof, base)),
                )
            })
            .clone();
        let root = commands
            .spawn((
                Name::new(b.id.clone()),
                GeneratedBuilding {
                    id: b.id.clone(),
                    archetype_id: b.archetype_id.clone(),
                    section_id: b.section_id.clone(),
                },
                BuildingChunk((b.station_m / 160.0) as u32),
                crate::environment::ZoneProp(
                    match plan.definition.sections[b.section_index].district {
                        rr_content::world::District::Farms
                        | rr_content::world::District::RuralEdge => {
                            rr_content::track::Sector::Farms
                        }
                        rr_content::world::District::Hills
                        | rr_content::world::District::CityToHills => {
                            rr_content::track::Sector::Hills
                        }
                        _ => rr_content::track::Sector::City,
                    },
                ),
                Transform::from_translation(Vec3::from_array(b.position))
                    .with_rotation(Quat::from_rotation_y(b.yaw)),
                Visibility::default(),
                RigidBody::Fixed,
                Collider::compound(vec![
                    (
                        Vec3::Y * (-b.raw_relief_m.max(0.4) * 0.25),
                        Quat::IDENTITY,
                        Collider::cuboid(
                            size.x * 0.5,
                            b.raw_relief_m.max(0.4) * 0.25,
                            size.z * 0.5,
                        ),
                    ),
                    (
                        Vec3::Y * wall_height * 0.5,
                        Quat::IDENTITY,
                        Collider::cuboid(size.x * 0.5, wall_height * 0.5, size.z * 0.5),
                    ),
                    (
                        Vec3::Y * (wall_height + roof_height * 0.5),
                        Quat::IDENTITY,
                        Collider::cuboid(size.x * 0.5, roof_height * 0.5, size.z * 0.5),
                    ),
                ]),
                Obstacle,
                Friction::coefficient(0.75),
            ))
            .id();
        let near = VisibilityRange {
            start_margin: 0.0..0.0,
            end_margin: layout.settings.render_near_m..layout.settings.render_near_m + 30.0,
            use_aabb: false,
        };
        let far = VisibilityRange {
            start_margin: near.end_margin.clone(),
            end_margin: layout.settings.render_far_m..layout.settings.render_far_m + 50.0,
            use_aabb: false,
        };
        let wall = commands
            .spawn((
                Mesh3d(wall_mesh),
                MeshMaterial3d(palette[index].clone()),
                Transform::from_scale(size),
                near.clone(),
                BuildingLod { near: true },
            ))
            .id();
        let roof = commands
            .spawn((
                Mesh3d(roof_mesh),
                MeshMaterial3d(art.dark.clone()),
                Transform::from_scale(Vec3::new(size.x + 0.3, size.y, size.z + 0.3)),
                near,
                BuildingLod { near: true },
            ))
            .id();
        let proxy = commands
            .spawn((
                Mesh3d(proxy_mesh.clone()),
                MeshMaterial3d(palette[index].clone()),
                Transform::from_scale(size),
                far,
                BuildingLod { near: false },
            ))
            .id();
        commands.entity(root).add_children(&[wall, roof, proxy]);
    }
}
