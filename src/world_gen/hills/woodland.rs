//! Original low-poly distant woodland. Spatial mesh batches share one material;
//! resident trunk compounds are independent of render distance and visibility.
use super::{HillsSolid, countryside, geometry, props::Model};
use crate::{track::Track, world::Obstacle};
use bevy::{camera::visibility::VisibilityRange, light::NotShadowCaster, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct WoodlandBatch;
#[derive(Resource, Default, serde::Serialize)]
pub struct WoodlandCensus {
    pub trees: usize,
    pub batches: usize,
    pub triangles: usize,
}
#[derive(Clone, Copy)]
struct Tree {
    p: Vec3,
    height: f32,
    seed: u32,
}
fn noise(seed: u32) -> f32 {
    let h = seed.wrapping_mul(747796405).wrapping_add(2891336453);
    let h = ((h >> ((h >> 28) + 4)) ^ h).wrapping_mul(277803737);
    ((h >> 22) ^ h) as f32 / u32::MAX as f32
}
fn plan(track: &Track) -> Vec<Vec<Tree>> {
    let farms = countryside::farm_sites(track);
    let mut batches = vec![];
    for start in (1600..5200).step_by(128) {
        for side in [-1., 1.] {
            let mut trees = vec![];
            for i in 0..8_u32 {
                for row in 0..9_u32 {
                    let seed = start as u32 * 179 + i * 73 + row * 919 + u32::from(side > 0.);
                    // Correlated clearings break up the rows; jitter breaks the grid.
                    let s = start as f32 + i as f32 * 16. + noise(seed) * 13.;
                    if s > 5200.
                        || noise(seed + 1) < 0.28 + 0.19 * (s * 0.019 + row as f32 * 0.8).sin()
                    {
                        continue;
                    }
                    let lane = side * (110. + row as f32 * 21. + noise(seed + 2) * 17.);
                    let Some((mut p, _)) = super::props::site(track, 0, s, lane, 4.) else {
                        continue;
                    };
                    if super::bridge::reserved(track, p, 5.)
                        || super::mounted::reserved(track, p, 5.)
                        || !countryside::field_clear(p, &farms)
                    {
                        continue;
                    }
                    let mut low = p.y;
                    let mut high = p.y;
                    for offset in [Vec3::X, -Vec3::X, Vec3::Z, -Vec3::Z] {
                        let q = p + offset * 0.5;
                        let y = geometry::ground_height(track, q.x, q.z);
                        low = low.min(y);
                        high = high.max(y);
                    }
                    if high - low > 1.8 {
                        continue;
                    }
                    p.y = low - 0.15;
                    trees.push(Tree {
                        p,
                        height: 7. + noise(seed + 7) * 8.,
                        seed,
                    });
                }
            }
            if !trees.is_empty() {
                batches.push(trees);
            }
        }
    }
    batches
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 1.,
        ..default()
    });
    let mut census = WoodlandCensus::default();
    for trees in plan(track) {
        let origin = trees[trees.len() / 2].p;
        let mut model = Model::default();
        let mut trunks = vec![];
        for tree in trees {
            let p = tree.p - origin;
            model.block(
                p + Vec3::Y * tree.height * 0.4,
                Vec3::new(0.55, tree.height * 0.8, 0.55),
                [0.27, 0.20, 0.12],
            );
            let tint = noise(tree.seed + 5);
            for tier in 0..3 {
                let base = p + Vec3::Y * tree.height * (0.18 + tier as f32 * 0.22);
                model.cone(
                    base,
                    tree.height * (0.25 - tier as f32 * 0.045),
                    tree.height * 0.42,
                    [0.12 + tint * 0.05, 0.22 + tint * 0.09, 0.09 + tint * 0.04],
                );
            }
            trunks.push((
                p + Vec3::Y * tree.height * 0.4,
                Quat::IDENTITY,
                Collider::cylinder(tree.height * 0.4, 0.28),
            ));
            census.trees += 1;
            census.triangles += 60; // 12 trunk + 3 x 16 closed eight-sided crowns.
        }
        commands.spawn((
            Transform::from_translation(origin),
            Collider::compound(trunks),
            Obstacle,
            HillsSolid,
        ));
        commands.spawn((
            Mesh3d(meshes.add(model.mesh())),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(origin),
            WoodlandBatch,
            NotShadowCaster,
            VisibilityRange {
                start_margin: 0. ..0.,
                end_margin: 850. ..1100.,
                use_aabb: true,
            },
        ));
        census.batches += 1;
    }
    commands.insert_resource(census);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distant_woodland_has_supported_clear_placed_geometry_and_bounded_batches() {
        let track = Track::hills().unwrap();
        let batches = plan(&track);
        let count: usize = batches.iter().map(Vec::len).sum();
        assert!((1200..4000).contains(&count), "trees: {count}");
        assert!(batches.len() <= 58);
        for tree in batches.iter().flatten() {
            assert!((tree.p.y - geometry::ground_height(&track, tree.p.x, tree.p.z)).abs() < 2.);
            for route in 0..2 {
                let (s, _, distance) = track.project_route(route, tree.p, 0.);
                assert!(distance > track.half_width(s) + 7.);
            }
        }
        let repeat = plan(&track);
        assert!(
            batches
                .iter()
                .flatten()
                .zip(repeat.iter().flatten())
                .all(|(a, b)| a.p == b.p && a.height == b.height)
        );
    }
}
