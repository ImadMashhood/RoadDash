//! Solid party-wall infill closes spacing between neighboring city buildings.
use super::{
    layout::{Building, WorldLayout},
    spatial::Rect,
};
use crate::{track::Track, world::Obstacle};
use bevy::{camera::visibility::VisibilityRange, prelude::*};
use bevy_rapier3d::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Clone, Debug, Serialize)]
pub struct Join {
    pub owners: [String; 2],
    pub corners: [[f32; 3]; 4],
    pub bounds: Rect,
    pub bottom: f32,
    pub top: f32,
}
#[derive(Component)]
pub struct PartyWallInfill;
fn end(b: &Building, forward: bool) -> [Vec3; 2] {
    let p = Vec3::from_array(b.position);
    let q = Quat::from_rotation_y(b.yaw);
    let z = b.size[2] * 0.5 * if forward { 1.0 } else { -1.0 };
    [-1.0, 1.0].map(|s| p + q * Vec3::new(s * b.size[0] * 0.5, 0.0, z))
}
pub fn generate(layout: &WorldLayout, track: &Track) -> Vec<Join> {
    let mut rows: BTreeMap<(i32, i32), Vec<&Building>> = BTreeMap::new();
    for b in layout
        .all_buildings()
        .filter(|b| (3150.0..4900.0).contains(&b.station_m))
    {
        let row = if b.lane_m.abs() < 40.0 {
            0
        } else {
            (b.lane_m.abs() / 10.0).round() as i32
        };
        rows.entry((b.lane_m.signum() as i32, row))
            .or_default()
            .push(b);
    }
    let streets: Vec<_> = layout
        .grid_streets
        .iter()
        .map(|s| s.reservation())
        .collect();
    let mut joins = Vec::new();
    for row in rows.values_mut() {
        row.sort_by(|a, b| a.station_m.total_cmp(&b.station_m));
        for pair in row.windows(2) {
            let [a, b] = [pair[0], pair[1]];
            let ae = end(a, true);
            let be = end(b, false);
            let ca = (ae[0] + ae[1]) * 0.5;
            let cb = (be[0] + be[1]) * 0.5;
            let delta = (cb - ca).with_y(0.0);
            if !(0.2..90.0).contains(&delta.length())
                || delta.dot(Quat::from_rotation_y(a.yaw) * Vec3::Z) <= 0.0
            {
                continue;
            }
            let q = Quat::from_rotation_y(delta.x.atan2(delta.z));
            let corners = [ae[0], ae[1], be[1], be[0]];
            let center = (ca + cb) * 0.5;
            let local = corners.map(|p| q.inverse() * (p - center));
            let half = local
                .iter()
                .fold(Vec2::ZERO, |h, p| h.max(Vec2::new(p.x.abs(), p.z.abs())));
            let bounds = Rect::new(center, q, half);
            if layout
                .road_reservations
                .iter()
                .chain(&layout.junction_rects)
                .chain(&streets)
                .any(|r| r.overlaps(&bounds))
            {
                continue;
            }
            if layout.all_buildings().any(|other| {
                other.id != a.id && other.id != b.id && other.footprint.overlaps(&bounds)
            }) {
                continue;
            }
            let bottom = corners
                .iter()
                .map(|p| track.base_terrain_height(p.x, p.z))
                .fold(a.foundation_m.min(b.foundation_m), f32::min)
                - 1.0;
            let top = (a.foundation_m + a.size[1]).min(b.foundation_m + b.size[1]);
            joins.push(Join {
                owners: [a.id.clone(), b.id.clone()],
                corners: corners.map(|p| p.to_array()),
                bounds,
                bottom,
                top,
            });
        }
    }
    joins
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    layout: &WorldLayout,
    plan: &super::WorldPlan,
    server: Option<&AssetServer>,
) {
    let architecture =
        super::architecture::Architecture::new(plan, layout).expect("validated infill styles");
    let mut surfaces = BTreeMap::new();
    let trim = materials.add(Color::srgb(0.58, 0.56, 0.51));
    let wall = materials.add(StandardMaterial {
        base_color: Color::srgb(0.38, 0.43, 0.47),
        perceptual_roughness: 0.88,
        ..default()
    });
    let glass = materials.add(Color::srgb(0.15, 0.26, 0.30));
    for join in &layout.frontages {
        let origin = Vec3::new(join.bounds.center[0], join.bottom, join.bounds.center[1]);
        let mut vertices = Vec::new();
        for y in [join.bottom, join.top] {
            for p in join.corners {
                vertices.push(Vec3::new(p[0], y, p[2]) - origin);
            }
        }
        let mut ix = vec![[4, 5, 6], [4, 6, 7], [0, 2, 1], [0, 3, 2]];
        for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
            ix.extend([[a, b, b + 4], [a, b + 4, a + 4]]);
        }
        // Corners follow local +X then +Z; reverse to face outward.
        for triangle in &mut ix {
            triangle.swap(1, 2);
        }
        let collider = Collider::convex_hull(&vertices).expect("convex frontage prism");
        let root = commands
            .spawn((
                Name::new(format!(
                    "Party wall {} / {}",
                    join.owners[0], join.owners[1]
                )),
                PartyWallInfill,
                Transform::from_translation(origin),
                Visibility::default(),
                RigidBody::Fixed,
                collider,
                Obstacle,
                Friction::coefficient(0.75),
            ))
            .id();
        let style = architecture
            .buildings
            .iter()
            .find(|b| b.id == join.owners[0])
            .map(|b| architecture.catalog.records[b.design].masonry.as_str())
            .unwrap_or("concrete");
        let facing = surfaces
            .entry(style.to_string())
            .or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: server.map(|s| {
                        s.load_builder()
                            .with_settings(|setting: &mut bevy::image::ImageLoaderSettings| {
                                setting.sampler = bevy::image::ImageSampler::Descriptor(
                                    bevy::image::ImageSamplerDescriptor {
                                        address_mode_u: bevy::image::ImageAddressMode::Repeat,
                                        address_mode_v: bevy::image::ImageAddressMode::Repeat,
                                        ..default()
                                    },
                                );
                            })
                            .load(format!("architecture://materials/{style}_albedo.png"))
                    }),
                    perceptual_roughness: 0.88,
                    ..default()
                })
            })
            .clone();
        let mut solid_mesh = crate::environment::mesh(&vertices, &ix);
        solid_mesh.duplicate_vertices();
        solid_mesh.compute_flat_normals();
        if let Some(bevy::mesh::VertexAttributeValues::Float32x3(p)) =
            solid_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let uv = p
                .iter()
                .map(|v| [(v[0] + v[2]) * 0.5, v[1] * 0.5])
                .collect::<Vec<_>>();
            solid_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        }
        let mesh = meshes.add(solid_mesh);
        let body = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(if server.is_some() {
                    facing
                } else {
                    wall.clone()
                }),
                Transform::default(),
                VisibilityRange {
                    start_margin: 0.0..0.0,
                    end_margin: layout.settings.render_far_m..layout.settings.render_far_m + 50.0,
                    use_aabb: false,
                },
            ))
            .id();
        commands.entity(root).add_child(body);
        let mut detail = super::elevation::Elevation::default();
        let base = join
            .owners
            .iter()
            .filter_map(|id| {
                layout
                    .all_buildings()
                    .find(|b| &b.id == id)
                    .map(|b| b.foundation_m)
            })
            .fold(join.bottom, f32::max)
            - origin.y;
        for (a, b) in [(0, 3), (2, 1)] {
            let a = Vec3::from_array(join.corners[a]) - origin;
            let b = Vec3::from_array(join.corners[b]) - origin;
            let outward = ((a + b) * 0.5).with_y(0.0).normalize_or_zero();
            detail.wall(a, b, base, join.top - origin.y, outward);
        }
        if !detail.glass_indices.is_empty() {
            let (glass_mesh, frame_mesh) = detail.meshes();
            for (mesh, material) in [(glass_mesh, glass.clone()), (frame_mesh, trim.clone())] {
                let e = commands
                    .spawn((
                        bevy::light::NotShadowCaster,
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(material),
                        Transform::default(),
                        VisibilityRange {
                            start_margin: 0.0..0.0,
                            end_margin: layout.settings.render_far_m
                                ..layout.settings.render_far_m + 50.0,
                            use_aabb: false,
                        },
                    ))
                    .id();
                commands.entity(root).add_child(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn party_walls_touch_both_owners_and_preserve_streets() {
        let track = Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
        let plan = super::super::WorldPlan::from_json(
            super::super::WORLD_JSON,
            super::super::ARCHETYPES_JSON,
            &track,
        )
        .unwrap();
        let layout = WorldLayout::generate(
            &plan,
            &track,
            super::super::layout::Construction::from_json(
                super::super::layout::CONSTRUCTION_JSON,
                &track,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            layout.frontages.len() > 150,
            "only {} infills",
            layout.frontages.len()
        );
        for j in &layout.frontages {
            let a = layout
                .all_buildings()
                .find(|b| b.id == j.owners[0])
                .unwrap();
            let b = layout
                .all_buildings()
                .find(|b| b.id == j.owners[1])
                .unwrap();
            let ae = end(a, true);
            let be = end(b, false);
            assert_eq!(
                j.corners,
                [ae[0], ae[1], be[1], be[0]].map(|p| p.to_array())
            );
            assert!(j.top > j.bottom);
            assert!(
                layout
                    .road_reservations
                    .iter()
                    .chain(&layout.junction_rects)
                    .all(|r| !r.overlaps(&j.bounds))
            );
            assert!(
                layout
                    .grid_streets
                    .iter()
                    .all(|s| !s.reservation().overlaps(&j.bounds))
            );
        }
    }
}
