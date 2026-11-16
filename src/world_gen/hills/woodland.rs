//! Original low-poly distant woodland. Spatial mesh batches share one material;
//! resident trunk compounds are independent of render distance and visibility.
use super::{HillsSolid, countryside, geometry, props::Model};
use crate::{track::Track, world::Obstacle};
use bevy::math::Vec3A;
use bevy::{
    camera::{
        primitives::{Aabb, MeshAabb},
        visibility::VisibilityRange,
    },
    light::NotShadowCaster,
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use std::collections::{BTreeMap, HashMap};
pub(super) mod tree_lods;

#[derive(Component)]
pub struct WoodlandBatch;
#[derive(Resource, Default, serde::Serialize)]
pub struct WoodlandCensus {
    pub trees: usize,
    pub batches: usize,
    pub triangles: usize,
    pub backdrop_trees: usize,
    pub backdrop_batches: usize,
    pub middle_batches: usize,
    pub middle_triangles: usize,
    pub infill_trees: usize,
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
#[derive(Default)]
pub(super) struct TreeSpacing {
    occupied: HashMap<IVec2, Vec<(Vec2, f32)>>,
    individual_meshes: HashMap<(&'static str, usize), Handle<Mesh>>,
}

impl TreeSpacing {
    pub(super) fn plant(&mut self, track: &Track, point: Vec3, radius: f32) -> Option<Vec3> {
        let cell = (point.xz() / 8.).floor().as_ivec2();
        let overlaps = (-2..=2).any(|x| {
            (-2..=2).any(|z| {
                self.occupied
                    .get(&(cell + IVec2::new(x, z)))
                    .is_some_and(|trees| {
                        trees.iter().any(|(center, other)| {
                            point.xz().distance_squared(*center) < (radius + other).powi(2)
                        })
                    })
            })
        });
        if overlaps {
            return None;
        }
        let point = rooted(track, point)?;
        self.occupied
            .entry(cell)
            .or_default()
            .push((point.xz(), radius));
        Some(point)
    }
}

fn canopy_radius(tree: Tree) -> f32 {
    tree.height
        * if tree.seed.is_multiple_of(3) {
            0.25
        } else {
            0.375
        }
}

fn plan(track: &Track, spacing: &mut TreeSpacing) -> Vec<Vec<Tree>> {
    let farms = countryside::farm_sites(track);
    let mut batches = vec![];
    for start in (1600..5200).step_by(128) {
        for side in [-1., 1.] {
            let mut trees = vec![];
            for i in 0..12_u32 {
                for row in 0..18_u32 {
                    let seed = start as u32 * 179 + i * 73 + row * 919 + u32::from(side > 0.);
                    // Correlated clearings break up the rows; jitter breaks the grid.
                    let s = start as f32 + (i as f32 + noise(seed) * 0.81) * (128. / 12.);
                    if s > 5200.
                        || noise(seed + 1) < 0.21 + 0.19 * (s * 0.019 + row as f32 * 0.4).sin()
                    {
                        continue;
                    }
                    let lane = side * (110. + (row as f32 + noise(seed + 2) * 0.81) * 10.5);
                    let Some((p, _)) = super::props::site(track, 0, s, lane, 4.) else {
                        continue;
                    };
                    if super::bridge::reserved(track, p, 5.)
                        || super::mounted::reserved(track, p, 5.)
                        || !countryside::field_clear(p, &farms)
                    {
                        continue;
                    }
                    let mut tree = Tree {
                        p,
                        height: 7. + noise(seed + 7) * 8.,
                        seed,
                    };
                    if let Some(base) = spacing.plant(track, tree.p, canopy_radius(tree)) {
                        tree.p = base;
                        trees.push(tree);
                    }
                }
            }
            if !trees.is_empty() {
                batches.push(trees);
            }
        }
    }
    batches
}
fn rooted(track: &Track, p: Vec3) -> Option<Vec3> {
    const RUN: f32 = 2.;
    const MAX_GRADE: f32 = 0.62;
    let height = |q: Vec3| geometry::ground_height(track, q.x, q.z);
    let base = height(p);
    let grade = [Vec3::X, Vec3::Z].map(|axis| {
        let before = height(p - axis * RUN);
        let after = height(p + axis * RUN);
        (after - base).abs().max((before - base).abs()) / RUN
    });
    (Vec2::from_array(grade).length() <= MAX_GRADE).then_some(p.with_y(base - 0.12))
}

// Three stands trace the marked slopes around the mountain farm and bridge approach.
const INFILL_STANDS: [(Vec2, Vec2); 3] = [
    (Vec2::new(-156., 3340.), Vec2::new(98., 182.)),
    (Vec2::new(-282., 3118.), Vec2::new(181., 115.)),
    (Vec2::new(-804., 3515.), Vec2::new(132., 160.)),
];

fn slope_infill(track: &Track, spacing: &mut TreeSpacing) -> Vec<Vec<Tree>> {
    const SPACING: f32 = 8.;
    let cell = |p: Vec2| (p / SPACING).floor().as_ivec2();
    let farms = countryside::farm_sites(track);
    let mut batches = BTreeMap::<(i32, i32), Vec<Tree>>::new();
    for (stand, (center, radii)) in INFILL_STANDS.into_iter().enumerate() {
        let min = cell(center - radii);
        let max = cell(center + radii);
        for x in min.x..=max.x {
            for z in min.y..=max.y {
                let seed = (stand as u32 * 433_505)
                    .wrapping_add((x as u32).wrapping_mul(23_981))
                    .wrapping_add((z as u32).wrapping_mul(7_919));
                let p = Vec2::new(x as f32 + noise(seed), z as f32 + noise(seed + 1)) * SPACING;
                let edge = ((p - center) / radii).length_squared();
                if edge > 1. || noise(seed + 2) < 0.13 + edge * 0.32 {
                    continue;
                }
                let point = Vec3::new(p.x, 0., p.y);
                // Blend into the middle woodland. The separate backdrop
                // lattice already owns the region beyond 325 m.
                let (_, _, distance) = track.project(point, 4300.);
                if !(102. ..325.).contains(&distance)
                    || !track.scenery_fits(point, 5., 70.)
                    || super::bridge::reserved(track, point, 5.)
                    || super::mounted::reserved(track, point, 5.)
                    || super::wildlife::reserved(track, point, 5.)
                    || !countryside::field_clear(point, &farms)
                {
                    continue;
                }
                let mut tree = Tree {
                    p: point,
                    height: 7. + noise(seed + 7) * 8.,
                    seed,
                };
                if let Some(base) = spacing.plant(track, tree.p, canopy_radius(tree)) {
                    tree.p = base;
                    batches
                        .entry((x.div_euclid(12), z.div_euclid(12)))
                        .or_default()
                        .push(tree);
                }
            }
        }
    }
    batches.into_values().collect()
}

const BACKDROP_TILE: f32 = 160.;
const BACKDROP_ROWS: u32 = 21;

fn backdrop(track: &Track, clearance: &mut TreeSpacing) -> Vec<(Vec3, Vec<Tree>)> {
    let min = track.bounds.0.xz() - Vec2::splat(320.);
    let max = track.bounds.1.xz() + Vec2::splat(320.);
    let size = ((max - min) / BACKDROP_TILE).ceil().as_uvec2();
    let spacing = BACKDROP_TILE / BACKDROP_ROWS as f32;
    let farms = countryside::farm_sites(track);
    let mut batches = Vec::new();
    for x in 0..size.x {
        for z in 0..size.y {
            let base = min + Vec2::new(x as f32, z as f32) * BACKDROP_TILE;
            let mut trees = Vec::new();
            for i in 0..BACKDROP_ROWS {
                for j in 0..BACKDROP_ROWS {
                    let seed = (x * 1259 + z * 6739 + i * 73 + j * 919).wrapping_add(7919);
                    let offset = Vec2::new(i as f32 + noise(seed), j as f32 + noise(seed + 1));
                    let surface = base + offset * spacing;
                    if noise(seed + 2)
                        < 0.24 + 0.20 * (surface.x * 0.021 + (surface.y * 0.012).sin()).sin()
                    {
                        continue;
                    }
                    let p = Vec3::new(surface.x, 0., surface.y);
                    let (s, _, distance) = track.project(p, 0.);
                    if !(1250. ..5300.).contains(&s)
                        || !(325. ..820.).contains(&distance)
                        || track
                            .hills_branch
                            .as_ref()
                            .is_some_and(|branch| !branch.scenery_fits(p, 5., 140.))
                    {
                        continue;
                    }
                    if super::bridge::reserved(track, p, 5.) || !countryside::field_clear(p, &farms)
                    {
                        continue;
                    }
                    let mut tree = Tree {
                        p,
                        height: 8. + noise(seed + 7) * 9.,
                        seed,
                    };
                    if let Some(base) = clearance.plant(track, tree.p, canopy_radius(tree)) {
                        tree.p = base;
                        trees.push(tree);
                    }
                }
            }
            if !trees.is_empty() {
                let center = base + Vec2::splat(BACKDROP_TILE * 0.5);
                batches.push((Vec3::new(center.x, trees[0].p.y, center.y), trees));
            }
        }
    }
    batches
}

fn shared_bounds(meshes: &[Mesh]) -> Aabb {
    let (min, max) = meshes.iter().filter_map(MeshAabb::get_aabb).fold(
        (Vec3A::splat(f32::INFINITY), Vec3A::splat(f32::NEG_INFINITY)),
        |(min, max), bounds| (min.min(bounds.min()), max.max(bounds.max())),
    );
    // Share the full extents across crossfading tiers. Pad for numerical
    // tolerance at the edge of the view.
    Aabb::from_min_max((min - Vec3A::ONE).into(), (max + Vec3A::ONE).into())
}

fn outer_fade(bounds: Aabb) -> std::ops::Range<f32> {
    let radius = bounds.min().abs().max(bounds.max().abs()).length();
    // Bevy fades using the entity origin. Include the furthest possible
    // extent so some trees can't disappear just because their origin is distant.
    (1250. + radius)..(1550. + radius)
}

// The original far mesh stays resident. Only nearby tiles allocate higher tiers.
#[derive(Component)]
pub(crate) struct BackdropLod {
    far: Handle<Mesh>,
    trees: Vec<Tree>,
    tier: usize,
}

fn backdrop_tier(distance: f32, current: usize) -> usize {
    if distance < if current == 0 { 90. } else { 64. } {
        0
    } else if distance < if current <= 1 { 200. } else { 170. } {
        1
    } else {
        2
    }
}

pub(crate) fn update_backdrop_lods(
    cameras: Query<&GlobalTransform, With<crate::game::ChaseCamera>>,
    mut tiles: Query<(Entity, &Transform, &Aabb, &mut Mesh3d, &mut BackdropLod)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Some(camera) = cameras.iter().next() else {
        return;
    };
    let mut detail = Vec::new();
    for (entity, origin, bounds, mut mesh, mut lod) in &mut tiles {
        let local = camera.translation() - origin.translation;
        let distance = local.distance(local.clamp(bounds.min().into(), bounds.max().into()));
        let tier = backdrop_tier(distance, lod.tier);
        if tier == lod.tier {
            continue;
        }
        if tier == 2 {
            mesh.0 = lod.far.clone();
            lod.tier = tier;
        } else {
            detail.push((distance, entity, tier));
        }
    }
    if let Some((_, entity, tier)) = detail.into_iter().min_by(|a, b| a.0.total_cmp(&b.0))
        && let Ok((_, origin, _, mut mesh, mut lod)) = tiles.get_mut(entity)
    {
        let mut model = Model::default();
        for tree in &lod.trees {
            let (templates, transform) =
                tree_lods::fitted(tree.seed, tree.height, tree.p - origin.translation);
            let template = &templates[tier];
            model.append(
                &template.positions,
                &template.colors,
                &template.triangles,
                transform,
            );
        }
        mesh.0 = meshes.add(model.mesh());
        lod.tier = tier;
    }
}

pub(super) fn single_tree_lods(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    spacing: &mut TreeSpacing,
    parent: Entity,
    family: &'static str,
    size: Vec3,
) {
    let templates = tree_lods::named(family);
    for (tier, (start_margin, end_margin)) in [
        (0. ..0., 27. ..35.),
        (27. ..35., 70. ..85.),
        (70. ..85., 420. ..480.),
    ]
    .into_iter()
    .enumerate()
    {
        let mesh = spacing
            .individual_meshes
            .entry((family, tier))
            .or_insert_with(|| {
                let template = &templates[tier];
                let mut model = Model::default();
                model.append(
                    &template.positions,
                    &template.colors,
                    &template.triangles,
                    Transform::IDENTITY,
                );
                meshes.add(model.mesh())
            });
        let child = commands
            .spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_scale(size),
                VisibilityRange {
                    start_margin,
                    end_margin,
                    use_aabb: false,
                },
            ))
            .id();
        commands.entity(parent).add_child(child);
    }
}

pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
    spacing: &mut TreeSpacing,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.68,
        metallic: 0.08,
        ..default()
    });
    let middle = plan(track, spacing);
    let infill = slope_infill(track, spacing);
    let mut census = WoodlandCensus {
        infill_trees: infill.iter().map(Vec::len).sum(),
        ..default()
    };
    for trees in middle.into_iter().chain(infill) {
        let origin = trees[trees.len() / 2].p;
        let mut model = Model::default();
        let mut middle = Model::default();
        let mut far = Model::default();
        let mut trunks = vec![];
        for tree in trees {
            let p = tree.p - origin;
            let (templates, transform) = tree_lods::fitted(tree.seed, tree.height, p);
            for (mesh, template) in [&mut model, &mut middle, &mut far]
                .into_iter()
                .zip(templates)
            {
                mesh.append(
                    &template.positions,
                    &template.colors,
                    &template.triangles,
                    transform,
                );
                census.triangles += template.triangles.len();
            }
            census.middle_triangles += templates[1].triangles.len();
            trunks.push((
                p + Vec3::Y * tree.height * 0.4,
                Quat::IDENTITY,
                Collider::cylinder(tree.height * 0.4, 0.28),
            ));
            census.trees += 1;
        }
        commands.spawn((
            Transform::from_translation(origin),
            Collider::compound(trunks),
            Obstacle,
            HillsSolid,
        ));
        let meshes_for_batch = [model.mesh(), middle.mesh(), far.mesh()];
        let bounds = shared_bounds(&meshes_for_batch);
        let end = outer_fade(bounds);
        for (mesh, (start_margin, end_margin)) in meshes_for_batch.into_iter().zip([
            (0. ..0., 155. ..210.),
            (155. ..210., 380. ..470.),
            (380. ..470., end),
        ]) {
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(origin),
                bounds,
                WoodlandBatch,
                NotShadowCaster,
                VisibilityRange {
                    start_margin,
                    end_margin,
                    use_aabb: false,
                },
            ));
        }
        census.batches += 3;
        census.middle_batches += 1;
    }
    for (origin, trees) in backdrop(track, spacing) {
        let mut model = Model::default();
        census.backdrop_trees += trees.len();
        census.backdrop_batches += 1;
        census.trees += trees.len();
        census.batches += 1;
        for tree in &trees {
            let (templates, transform) = tree_lods::fitted(tree.seed, tree.height, tree.p - origin);
            let far = &templates[2];
            model.append(&far.positions, &far.colors, &far.triangles, transform);
            census.triangles += far.triangles.len();
        }
        let mesh = model.mesh();
        let far_bounds = shared_bounds(std::slice::from_ref(&mesh));
        // The detailed crowns can extend slightly beyond their far proxies.
        let bounds = Aabb::from_min_max(
            (far_bounds.min() - Vec3A::splat(7.)).into(),
            (far_bounds.max() + Vec3A::splat(7.)).into(),
        );
        let far = meshes.add(mesh);
        commands.spawn((
            Mesh3d(far.clone()),
            BackdropLod {
                far,
                trees,
                tier: 2,
            },
            MeshMaterial3d(material.clone()),
            Transform::from_translation(origin),
            bounds,
            WoodlandBatch,
            NotShadowCaster,
            VisibilityRange {
                start_margin: 0. ..0.,
                end_margin: outer_fade(bounds),
                use_aabb: false,
            },
        ));
    }
    commands.insert_resource(census);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backdrop_places_thousands_of_grounded_trees_with_bounded_geometry() {
        let track = Track::hills().unwrap();
        let batches = backdrop(&track, &mut TreeSpacing::default());
        let count: usize = batches.iter().map(|(_, trees)| trees.len()).sum();
        let triangles: usize = batches
            .iter()
            .flat_map(|(_, trees)| trees)
            .map(|tree| {
                tree_lods::fitted(tree.seed, tree.height, tree.p).0[2]
                    .triangles
                    .len()
            })
            .sum();
        println!(
            "Hills backdrop: {count} trees, {} batches, {} triangles",
            batches.len(),
            triangles
        );
        assert!((10_000..=28_000).contains(&count));
        assert!(batches.len() <= 160);
        assert!(triangles <= 1_300_000);
        for (_, trees) in &batches {
            assert!(trees.len() <= (BACKDROP_ROWS * BACKDROP_ROWS) as usize);
            for tree in trees {
                assert!(track.scenery_fits(tree.p, 5., 140.));
                assert!(
                    (geometry::ground_height(&track, tree.p.x, tree.p.z) - tree.p.y - 0.12).abs()
                        < 0.001
                );
            }
        }
    }

    #[test]
    fn distant_woodland_has_supported_clear_placed_geometry_and_bounded_batches() {
        let track = Track::hills().unwrap();
        let batches = plan(&track, &mut TreeSpacing::default());
        let count: usize = batches.iter().map(Vec::len).sum();
        println!(
            "Hills middle woodland: {count} trees in {} batches",
            batches.len()
        );
        assert!((3_000..=12_000).contains(&count), "trees: {count}");
        assert!(batches.len() <= 58);
        for tree in batches.iter().flatten() {
            assert!((tree.p.y - geometry::ground_height(&track, tree.p.x, tree.p.z)).abs() < 2.);
            for route in 0..2 {
                let (s, _, distance) = track.project_route(route, tree.p, 0.);
                assert!(distance > track.half_width(s) + 7.);
            }
        }
        let repeat = plan(&track, &mut TreeSpacing::default());
        assert!(
            batches
                .iter()
                .flatten()
                .zip(repeat.iter().flatten())
                .all(|(a, b)| a.p == b.p && a.height == b.height)
        );
    }
    #[test]
    fn targeted_stands_add_grounded_trees_away_from_farms_and_bridge() {
        let track = Track::hills().unwrap();
        let mut spacing = TreeSpacing::default();
        let _ = plan(&track, &mut spacing);
        let batches = slope_infill(&track, &mut spacing);
        let farms = countryside::farm_sites(&track);
        let total = batches.iter().map(Vec::len).sum::<usize>();
        println!(
            "Hills slope infill: {total} trees in {} batches",
            batches.len()
        );
        assert!((350..=1500).contains(&total));
        for (center, radii) in INFILL_STANDS {
            assert!(
                batches
                    .iter()
                    .flatten()
                    .filter(|t| ((t.p.xz() - center) / radii).length() < 1.)
                    .count()
                    > 40
            );
        }
        for tree in batches.iter().flatten() {
            assert!(track.scenery_fits(tree.p, 5., 70.));
            assert!(countryside::field_clear(tree.p, &farms));
            assert!(!super::super::bridge::reserved(&track, tree.p, 5.));
        }
    }

    #[test]
    fn range_keeps_a_batch_whose_near_edge_is_still_in_range() {
        let bounds = Aabb::from_min_max(Vec3::new(-90., -5., -6.), Vec3::new(10., 18., 6.));
        assert!(outer_fade(bounds).start >= 1340.);
        assert!(outer_fade(bounds).end >= 1640.);
    }
    #[test]
    fn tree_spacing_rejects_overlaps_and_backdrop_supports_all_tiers() {
        let track = Track::hills().unwrap();
        let mut spacing = TreeSpacing::default();
        let tree = plan(&track, &mut spacing)[0][0];
        assert!(spacing.plant(&track, tree.p + Vec3::X * 0.5, 3.).is_none());
        assert_eq!(backdrop_tier(40., 2), 0);
        assert_eq!(backdrop_tier(80., 0), 0);
        assert_eq!(backdrop_tier(100., 0), 1);
        assert_eq!(backdrop_tier(300., 1), 2);
    }
}
