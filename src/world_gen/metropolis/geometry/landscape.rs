//! Original, render-only pastoral dressing. No physical surface is replaced.
use super::*;
use bevy::{camera::visibility::VisibilityRange, light::NotShadowCaster};
#[derive(Component)]
pub(crate) struct GrassBatch {
    pub triangles: usize,
}
const CELL: f32 = 32.;
const MAX_TUFTS: usize = 120_000;
const MAX_BATCHES: usize = 600;
const RADIUS: f32 = 0.45;

fn noise(x: i32, z: i32, salt: u32) -> f32 {
    let mut n = (x as u32).wrapping_mul(0x9e3779b9) ^ (z as u32).wrapping_mul(0x85ebca6b) ^ salt;
    n ^= n >> 16;
    n = n.wrapping_mul(0x7feb352d);
    n ^= n >> 15;
    (n & 0xffff) as f32 / 65535.
}
fn meadow(p: Vec2) -> f32 {
    0.5 + 0.25 * (p.x * 0.071 + (p.y * 0.023).sin()).sin() + 0.25 * (p.y * 0.11 + p.x * 0.019).cos()
}
/// Coherent ground tint persists beyond the grass range, without extra draws.
pub(super) fn tint(batch: &mut Batch, material: usize, track: &Track) {
    if ![27, 30, 31].contains(&material) {
        return;
    }
    for (p, color) in batch.p.iter().zip(&mut batch.colors) {
        let c = ground_tint(Vec2::new(p[0], p[2]), track);
        *color = [c.x, c.y, c.z, 1.];
    }
}
fn ground_tint(p: Vec2, track: &Track) -> Vec3 {
    let variation = 0.74 + meadow(p) * 0.42;
    let (station, lane) = track.nearby_station(p.x, p.y);
    let edge = track.half_width(station) + 0.7 + (station * 0.17).sin() * 0.4;
    let soil = if station < 3000. {
        (1. - ((lane.abs() - edge) / 4.).max(0.)).clamp(0., 1.)
    } else {
        0.
    };
    Vec3::new(
        variation * (1. + soil * 0.20),
        variation * (1. - soil * 0.16),
        variation * (1. - soil * 0.15),
    )
}
pub(crate) fn ground_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let mut pixels = Vec::with_capacity(64 * 64 * 4);
    for z in 0..64 {
        for x in 0..64 {
            let n = noise(x, z, 201);
            let v = (220. + n * 35.) as u8;
            pixels.extend([v, v, v, 255]);
        }
    }
    images.add(crate::road_detail::image(pixels, 64, false))
}
pub(crate) fn ground_material(color: Color, texture: &Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        base_color_texture: Some(texture.clone()),
        perceptual_roughness: 1.,
        reflectance: 0.18,
        ..default()
    }
}

pub(crate) struct Footprint<'a> {
    shape: &'a Shape,
    lo: Vec2,
    hi: Vec2,
}
impl<'a> Footprint<'a> {
    pub(crate) fn new(shape: &'a Shape) -> Self {
        let (mut lo, mut hi) = (Vec2::splat(f32::INFINITY), Vec2::splat(f32::NEG_INFINITY));
        for v in shape.vertices().map(Vec2::from_array) {
            lo = lo.min(v);
            hi = hi.max(v);
        }
        Self { shape, lo, hi }
    }
    pub(crate) fn contains(&self, p: Vec2) -> bool {
        if p.cmplt(self.lo).any() || p.cmpgt(self.hi).any() {
            return false;
        }
        self.shape.rings.iter().fold(false, |inside, ring| {
            let hit = ring
                .iter()
                .zip(ring.iter().cycle().skip(1))
                .take(ring.len())
                .fold(false, |hit, (a, b)| {
                    let (a, b) = (Vec2::from_array(*a), Vec2::from_array(*b));
                    hit ^ ((a.y > p.y) != (b.y > p.y)
                        && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x)
                });
            inside ^ hit
        })
    }
    pub(crate) fn boundary_distance(&self, p: Vec2) -> f32 {
        self.shape
            .rings
            .iter()
            .flat_map(|r| r.iter().zip(r.iter().cycle().skip(1)).take(r.len()))
            .map(|(a, b)| {
                let (a, b) = (Vec2::from_array(*a), Vec2::from_array(*b));
                let edge = b - a;
                p.distance(
                    a + edge * ((p - a).dot(edge) / edge.length_squared().max(1e-8)).clamp(0., 1.),
                )
            })
            .fold(f32::INFINITY, f32::min)
    }
    fn intersects_quad(&self, q: [Vec2; 4]) -> bool {
        let lo = q
            .iter()
            .copied()
            .fold(Vec2::splat(f32::INFINITY), Vec2::min);
        let hi = q
            .iter()
            .copied()
            .fold(Vec2::splat(f32::NEG_INFINITY), Vec2::max);
        if hi.cmplt(self.lo).any() || lo.cmpgt(self.hi).any() {
            return false;
        }
        if q.iter().any(|p| self.contains(*p)) {
            return true;
        }
        let in_quad = |p: Vec2| {
            let cross: Vec<_> = q
                .iter()
                .zip(q.iter().cycle().skip(1))
                .take(4)
                .map(|(a, b)| (*b - *a).perp_dot(p - *a))
                .collect();
            cross.iter().all(|v| *v >= 0.) || cross.iter().all(|v| *v <= 0.)
        };
        self.shape.rings.iter().any(|ring| {
            ring.iter().any(|v| in_quad(Vec2::from_array(*v)))
                || ring
                    .iter()
                    .zip(ring.iter().cycle().skip(1))
                    .take(ring.len())
                    .any(|(a, b)| {
                        let (a, b) = (Vec2::from_array(*a), Vec2::from_array(*b));
                        q.iter()
                            .zip(q.iter().cycle().skip(1))
                            .take(4)
                            .any(|(c, d)| {
                                (b - a).perp_dot(*c - a) * (b - a).perp_dot(*d - a) < 0.
                                    && (*d - *c).perp_dot(a - *c) * (*d - *c).perp_dot(b - *c) < 0.
                            })
                    })
        })
    }
    pub(crate) fn overlaps(&self, p: Vec2, r: f32) -> bool {
        if (p + Vec2::splat(r)).cmplt(self.lo).any() || (p - Vec2::splat(r)).cmpgt(self.hi).any() {
            return false;
        }
        self.contains(p) || self.boundary_distance(p) < r
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Tuft {
    p: Vec3,
    seed: f32,
    crop: usize,
}
fn farm_solids(track: &Track) -> Vec<Footprint<'_>> {
    let m = track.metropolis.as_ref().unwrap();
    // Broad-phase once: farm-only surfaces/buildings, including access paths.
    let solids: Vec<_> = m
        .surfaces
        .iter()
        .map(|s| &s.shape)
        .chain(m.buildings.iter().map(|b| &b.shape))
        .filter(|s| {
            s.vertices()
                .any(|[x, z]| track.nearby_station(x, z).0 < 3050.)
        })
        .map(Footprint::new)
        .collect();
    solids
}
fn placements(track: &Track) -> Vec<Tuft> {
    let m = track.metropolis.as_ref().unwrap();
    let fields: Vec<_> = m
        .fields
        .iter()
        .map(|f| {
            (
                Footprint::new(&f.shape),
                if f.crop == "grain" {
                    1
                } else if f.crop == "fallow" {
                    2
                } else {
                    0
                },
            )
        })
        .collect();
    let solids = farm_solids(track);
    let barns: Vec<_> = [420., 980., 1560., 2220., 2810.]
        .into_iter()
        .enumerate()
        .map(|(i, s)| track.sample(s, if i % 2 == 0 { -65. } else { 65. }).0.xz())
        .collect();
    let farm_props = super::super::farms::plan(track);
    let mut result = Vec::new();
    for spacing in [1.2f32, 0.5] {
        let extent = if spacing < 1. { 12. } else { 34. };
        let cols = (extent / spacing) as i32;
        for row in ((-336. / spacing) as i32)..((3000. / spacing) as i32) {
            let s = row as f32 * spacing;
            for col in -cols..=cols {
                let lane = col as f32 * spacing;
                if lane.abs() < track.half_width(s) + 3.5 || (spacing > 1. && lane.abs() <= 12.) {
                    continue;
                }
                let seed = noise(row, col, 73);
                let (p, _) = track.sample(
                    s + (noise(row, col, 19) - 0.5) * spacing * 0.9,
                    lane + (noise(row, col, 31) - 0.5) * spacing * 0.9,
                );
                let xy = p.xz();
                let patch = meadow(xy);
                // Denser near the road, clustered openings farther into the fields.
                if seed
                    > (if spacing < 1. {
                        0.84
                    } else if lane.abs() < 18. {
                        0.96
                    } else {
                        0.30 + 0.40 * patch
                    })
                {
                    continue;
                }
                let crop = if let Some((field, crop)) = fields.iter().find(|(f, _)| f.contains(xy))
                {
                    if field.boundary_distance(xy) < 0.75 {
                        continue;
                    }
                    *crop
                } else {
                    // An unplanted road verge is allowed, but a hole in a parcel is not.
                    let in_parcel_bounds = fields
                        .iter()
                        .any(|(f, _)| !xy.cmplt(f.lo).any() && !xy.cmpgt(f.hi).any());
                    if lane.abs() > 12. || in_parcel_bounds {
                        continue;
                    }
                    2
                };
                if solids.iter().any(|f| f.overlaps(xy, RADIUS + 0.2))
                    || barns.iter().any(|b| b.distance(xy) < 15.)
                    || farm_props
                        .iter()
                        .any(|p| p.position.xz().distance(xy) < p.radius + RADIUS + 0.5)
                {
                    continue;
                }
                // Low fallow weeds and mown verges, taller pasture/grain behind them.
                result.push(Tuft {
                    p: p.with_y(track.terrain_height(p.x, p.z) - 0.20),
                    seed: noise(row, col, 137),
                    crop,
                });
                assert!(result.len() <= MAX_TUFTS, "grass placement budget exceeded");
            }
        }
    }
    // Civic meadow patches share the existing distance-culled grass batches.
    // Turf stays continuous; blades are sparse and avoid every paved patch.
    for site in m.civic_sites.iter().filter(|s| s.kind != "cafe") {
        let lawns: Vec<_> = site.grounds.iter().map(Footprint::new).collect();
        let paving: Vec<_> = site.paving.iter().map(Footprint::new).collect();
        let lo = lawns
            .iter()
            .fold(Vec2::splat(f32::INFINITY), |a, f| a.min(f.lo));
        let hi = lawns
            .iter()
            .fold(Vec2::splat(f32::NEG_INFINITY), |a, f| a.max(f.hi));
        for x in (lo.x.floor() as i32..hi.x.ceil() as i32).step_by(3) {
            for z in (lo.y.floor() as i32..hi.y.ceil() as i32).step_by(3) {
                let p = Vec2::new(x as f32 + noise(x, z, 21), z as f32 + noise(x, z, 42));
                if noise(x, z, 9) > 0.65
                    || !lawns.iter().any(|f| f.contains(p))
                    || paving.iter().any(|f| f.overlaps(p, 0.8))
                    || ![Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y]
                        .iter()
                        .all(|d| lawns.iter().any(|f| f.contains(p + *d * 0.8)))
                    || (site.kind == "park"
                        && !super::super::civic::park_planting_clear(
                            p - Vec2::from_array(site.position),
                            0.8,
                        ))
                {
                    continue;
                }
                result.push(Tuft {
                    p: Vec3::new(p.x, track.terrain_height(p.x, p.y) + 0.17, p.y),
                    seed: noise(x, z, 137),
                    crop: 2,
                });
            }
        }
    }
    assert!(result.len() <= MAX_TUFTS);
    result
}
fn shoulder_geometry(track: &Track) -> BTreeMap<i32, Batch> {
    let solids = farm_solids(track);
    let mut batches: BTreeMap<i32, Batch> = BTreeMap::new();
    for station in (-334i32..2998).step_by(2) {
        let s = station as f32;
        for side in [-1., 1.] {
            for band in 0..3 {
                let point = |s: f32, band: usize, clipping: bool| {
                    let width = match band {
                        // Asphalt is 6.8 m from the route centre. Cover its edge
                        // with a small underlap instead of exposing an 8 cm grass seam.
                        0 => {
                            if clipping {
                                0.08
                            } else {
                                -0.10
                            }
                        }
                        1 => 1.5 + 0.12 * (s * 0.19).sin(),
                        2 => 2.4 + 0.25 * (s * 0.19).sin(),
                        _ => 4.6 + 0.65 * (s * 0.13).sin() + 0.3 * (s * 0.7).sin(),
                    };
                    let p = track
                        .sample(s, side * (track.half_width(s) + 0.8 + width))
                        .0;
                    p.with_y(
                        track.terrain_height(p.x, p.z)
                            + match band {
                                0 => 0.02,
                                1 => -0.07,
                                2 => -0.14,
                                _ => -0.175,
                            },
                    )
                };
                let points = [
                    point(s, band, false),
                    point(s + 2., band, false),
                    point(s + 2., band + 1, false),
                    point(s, band + 1, false),
                ];
                let points = if side < 0. {
                    points
                } else {
                    [points[3], points[2], points[1], points[0]]
                };
                // Retain the original road/access exclusion footprint. Only the
                // inner render edge extends 18 cm toward the asphalt beneath it.
                let clipping = [
                    point(s, band, true),
                    point(s + 2., band, true),
                    point(s + 2., band + 1, true),
                    point(s, band + 1, true),
                ];
                if solids
                    .iter()
                    .any(|f| f.intersects_quad(clipping.map(|p| p.xz())))
                {
                    continue;
                }
                let batch = batches.entry(station.div_euclid(128)).or_default();
                batch.quad(points, points.map(|p| [p.x * 0.25, p.z * 0.25]));
                let start = batch.colors.len() - 4;
                for (i, p) in points.iter().enumerate() {
                    let outer = if side < 0. { i >= 2 } else { i < 2 };
                    let base = Vec3::new(0.07323896, 0.1626472, 0.04323832);
                    let ground = ground_tint(p.xz(), track);
                    let soil = Vec3::new(0.30, 0.25, 0.17) / base;
                    let weight = match band + usize::from(outer) {
                        0 => 0.95,
                        1 => 0.90,
                        2 => 0.32,
                        _ => 0.,
                    };
                    let color = ground.lerp(soil, weight);
                    batch.colors[start + i] = [color.x, color.y, color.z, 1.];
                }
            }
        }
    }
    batches
}
fn shoulders(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    track: &Track,
) -> usize {
    let batches = shoulder_geometry(track);
    let mut triangles = 0;
    for (station, batch) in batches {
        triangles += batch.ix.len();
        let origin = track.sample(station as f32 * 128. + 64., 0.).0;
        commands.spawn((
            Name::new("Irregular soil shoulder (visual only)"),
            Mesh3d(meshes.add(batch.mesh(origin))),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(origin),
            NotShadowCaster,
        ));
    }
    triangles
}
fn tuft_height(seed: f32, crop: usize) -> f32 {
    (0.38 + seed * 0.48)
        * match crop {
            1 => 0.22,
            2 => 0.65,
            _ => 1.,
        }
}
pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<super::grass_material::GrassMaterial>,
    track: &Track,
    census: &mut Census,
    ground_material: &Handle<StandardMaterial>,
) {
    let started = std::time::Instant::now();
    let tufts = placements(track);
    let mut batches: BTreeMap<(i32, i32), Batch> = BTreeMap::new();
    for t in &tufts {
        let batch = batches
            .entry(((t.p.x / CELL).floor() as i32, (t.p.z / CELL).floor() as i32))
            .or_default();
        // Grain parcels now carry separate persistent wheat. Keep low undergrowth
        // below the flattened canopy so crash and walking trails stay readable.
        let height = tuft_height(t.seed, t.crop);
        for blade in 0..6 {
            let angle = t.seed * std::f32::consts::TAU + blade as f32 * 2.399;
            let axis = Vec3::new(angle.cos(), 0., angle.sin());
            let across = Vec3::new(-axis.z, 0., axis.x);
            let root = t.p + axis * 0.12;
            let h = height * (0.72 + 0.28 * (angle * 2.7).sin().abs());
            let mid = root + Vec3::Y * h * 0.55 + axis * 0.045;
            let tip = root + Vec3::Y * h + axis * 0.16;
            let k = batch.p.len() as u32;
            batch.p.extend(
                [
                    root - across * 0.014,
                    root + across * 0.014,
                    mid - across * 0.010,
                    mid + across * 0.010,
                    tip,
                ]
                .map(|p| p.to_array()),
            );
            let n = (Vec3::Y * 0.75 + axis * 0.25).normalize().to_array();
            batch.n.extend([n; 5]);
            batch
                .uv
                .extend([[0., 0.], [1., 0.], [0., 0.55], [1., 0.55], [0.5, 1.]]);
            let c = if t.crop == 1 || (blade + (t.seed * 19.) as i32) % 7 == 0 {
                Vec3::new(0.28, 0.25, 0.075)
            } else {
                Vec3::new(0.07, 0.20, 0.035)
            };
            for factor in [0.45, 0.45, 0.88, 0.88, 1.1 + t.seed * 0.35] {
                let c = c * factor;
                batch.colors.push([c.x, c.y, c.z, 1.]);
            }
            batch.ix.extend([
                [k, k + 1, k + 2],
                [k + 1, k + 3, k + 2],
                [k + 2, k + 3, k + 4],
            ]);
        }
    }
    assert!(batches.len() <= MAX_BATCHES, "grass batch budget exceeded");
    census.grass_tufts = tufts.len();
    census.grass_batches = batches.len();
    census.grass_triangles = tufts.len() * 18;
    census.render_triangles += census.grass_triangles;
    let material = materials.add(super::grass_material::GrassMaterial {
        base: StandardMaterial {
            base_color: Color::WHITE,
            cull_mode: None,
            double_sided: false,
            perceptual_roughness: 1.,
            ..default()
        },
        extension: super::grass_material::GrassWind {
            settings: Vec4::new(0.07, 1.3, 0., 0.),
        },
    });
    census.shoulder_triangles = shoulders(commands, meshes, ground_material, track);
    census.render_triangles += census.shoulder_triangles;
    for ((x, z), batch) in batches {
        let origin = Vec3::new((x as f32 + 0.5) * CELL, 0., (z as f32 + 0.5) * CELL);
        // Include the maximum horizontal shader sway in culling bounds.
        let (lo, hi) = batch.p.iter().map(|p| Vec3::from_array(*p)).fold(
            (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
            |(lo, hi), p| (lo.min(p), hi.max(p)),
        );
        let bounds = bevy::camera::primitives::Aabb {
            center: ((lo + hi) * 0.5 - origin).into(),
            half_extents: ((hi - lo) * 0.5 + Vec3::splat(0.08)).into(),
        };
        commands.spawn((
            bounds,
            Name::new("Pastoral grass (visual only)"),
            GrassBatch {
                triangles: batch.ix.len(),
            },
            Mesh3d(meshes.add(batch.mesh(origin))),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(origin),
            NotShadowCaster,
            VisibilityRange {
                start_margin: 0. ..0.,
                end_margin: 65. ..110.,
                use_aabb: true,
            },
        ));
    }
    info!(
        "Landscape geometry generated in {:.3}ms",
        started.elapsed().as_secs_f64() * 1000.
    );
    info!(
        "Landscape: {} tufts / {} batches / {} triangles; caps {MAX_TUFTS}/{MAX_BATCHES}",
        census.grass_tufts, census.grass_batches, census.grass_triangles
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crop_undergrowth_stays_below_flattened_wheat_without_shortening_other_grass() {
        for seed in [0., 0.25, 0.5, 0.75, 1.] {
            assert!(tuft_height(seed, 1) < 0.2);
            assert_eq!(tuft_height(seed, 0), 0.38 + seed * 0.48);
            assert_eq!(tuft_height(seed, 2), (0.38 + seed * 0.48) * 0.65);
        }
    }
    #[test]
    fn soil_ribbons_remain_farm_only_and_outside_drivable_road() {
        let track = Track::metropolis().unwrap();
        let batches = shoulder_geometry(&track);
        assert!(batches.len() <= 28);
        let triangles = batches.values().map(|b| b.ix.len()).sum::<usize>();
        assert!((18000..20000).contains(&triangles));
        println!("Actual clipped shoulder triangles: {triangles}");
        let m = track.metropolis.as_ref().unwrap();
        let solids: Vec<_> = m
            .surfaces
            .iter()
            .filter(|s| s.kind != 0)
            .map(|s| &s.shape)
            .chain(m.buildings.iter().map(|b| &b.shape))
            .map(Footprint::new)
            .collect();
        for batch in batches.values() {
            assert!(
                batch.n.iter().all(|n| n[1] > 0.85),
                "soil must face the sky"
            );
            for quad in batch.p.chunks_exact(4) {
                let q = std::array::from_fn(|i| Vec2::new(quad[i][0], quad[i][2]));
                assert!(!solids.iter().any(|f| f.intersects_quad(q)));
            }
            for p in &batch.p {
                let (s, lane) = track.nearby_station(p[0], p[2]);
                assert!(s < 3000.1);
                assert!(lane.abs() > track.half_width(s), "{s} {lane}");
                let offset = p[1] - track.terrain_height(p[0], p[2]);
                assert!((-0.176..=0.021).contains(&offset));
            }
        }
    }
    #[test]
    fn soil_meets_the_farm_asphalt_without_an_exposed_grass_seam() {
        let track = Track::metropolis().unwrap();
        let batches = shoulder_geometry(&track);
        let mut soil = PavementHeights::default();
        for b in batches.values() {
            for indices in &b.ix {
                soil.add(indices.map(|i| Vec3::from_array(b.p[i as usize])));
            }
        }
        let mut checked = 0;
        for station in (0..2990).step_by(5) {
            for side in [-1., 1.] {
                let p = track.sample(station as f32, side * 6.81).0;
                let outer = track.sample(station as f32, side * 7.2).0;
                if soil.height(outer.xz()).is_none() {
                    continue;
                }
                assert!(
                    soil.height(p.xz()).is_some(),
                    "uncovered asphalt edge at {station}/{side}"
                );
                checked += 1;
            }
        }
        assert!(checked > 500);
    }
    #[test]
    fn field_holes_and_full_blade_footprints_are_excluded() {
        let shape = Shape {
            rings: vec![
                vec![[0., 0.], [10., 0.], [10., 10.], [0., 10.]],
                vec![[4., 4.], [6., 4.], [6., 6.], [4., 6.]],
            ],
            triangles: vec![],
        };
        let f = Footprint::new(&shape);
        assert!(f.contains(Vec2::new(2., 2.)));
        assert!(!f.contains(Vec2::new(5., 5.)));
        assert!(f.overlaps(Vec2::new(-0.2, 5.), RADIUS));
        assert!(!f.overlaps(Vec2::new(-1., 5.), RADIUS));
    }
    #[test]
    fn civic_meadows_are_populated_and_keep_paved_access_clear() {
        let track = Track::metropolis().unwrap();
        let tufts = placements(&track);
        for site in track
            .metropolis
            .as_ref()
            .unwrap()
            .civic_sites
            .iter()
            .filter(|s| s.kind != "cafe")
        {
            let lawns: Vec<_> = site.grounds.iter().map(Footprint::new).collect();
            let paths: Vec<_> = site.paving.iter().map(Footprint::new).collect();
            let planted: Vec<_> = tufts
                .iter()
                .filter(|t| lawns.iter().any(|f| f.contains(t.p.xz())))
                .collect();
            assert!(planted.len() > 500, "{}", site.id);
            assert!(
                planted
                    .iter()
                    .all(|t| !paths.iter().any(|f| f.overlaps(t.p.xz(), 0.8)))
            );
            if site.kind == "park" {
                let trees = super::super::super::civic::park_tree_positions(site);
                assert!(trees.len() > 26);
                println!("Park trees: {}", trees.len());
            }
        }
    }
    #[test]
    fn actual_farm_placements_are_repeatable_bounded_and_clear_of_pavement() {
        let track = Track::metropolis().unwrap();
        let a = placements(&track);
        assert_eq!(a, placements(&track));
        assert!(a.len() > 20_000 && a.len() <= MAX_TUFTS, "{}", a.len());
        let m = track.metropolis.as_ref().unwrap();
        let roads: Vec<_> = m
            .surfaces
            .iter()
            .filter(|s| s.shape.vertices().any(|[_, z]| z < 3000.))
            .map(|s| Footprint::new(&s.shape))
            .collect();
        for t in &a {
            assert!(t.p.is_finite());
            assert!(!roads.iter().any(|f| f.overlaps(t.p.xz(), RADIUS)));
        }
        println!(
            "Deterministic placed grass: {} tufts / {} triangles",
            a.len(),
            a.len() * 18
        );
    }
}
