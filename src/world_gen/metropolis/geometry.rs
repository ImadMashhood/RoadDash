pub(crate) mod crops;
pub(crate) mod grass_material;
pub(crate) mod landscape;
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    mesh::{Indices, PrimitiveTopology},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use rr_content::metropolis::{Metropolis, Shape};
use std::collections::BTreeMap;
#[derive(Default)]
struct Batch {
    p: Vec<[f32; 3]>,
    n: Vec<[f32; 3]>,
    uv: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    ix: Vec<[u32; 3]>,
}
impl Batch {
    fn quad(&mut self, p: [Vec3; 4], uv: [[f32; 2]; 4]) {
        let k = self.p.len() as u32;
        let n = (p[1] - p[0])
            .cross(p[2] - p[0])
            .normalize_or_zero()
            .to_array();
        self.p.extend(p.map(|p| p.to_array()));
        self.n.extend([n; 4]);
        self.uv.extend(uv);
        self.colors.extend([[1.; 4]; 4]);
        self.ix.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
    }
    fn cuboid(&mut self, center: Vec3, size: Vec3, q: Quat) {
        let p = |x, y, z| center + q * (Vec3::new(x, y, z) * size * 0.5);
        for v in [
            [
                p(-1., -1., -1.),
                p(-1., 1., -1.),
                p(1., 1., -1.),
                p(1., -1., -1.),
            ],
            [
                p(1., -1., 1.),
                p(1., 1., 1.),
                p(-1., 1., 1.),
                p(-1., -1., 1.),
            ],
            [
                p(-1., -1., 1.),
                p(-1., 1., 1.),
                p(-1., 1., -1.),
                p(-1., -1., -1.),
            ],
            [
                p(1., -1., -1.),
                p(1., 1., -1.),
                p(1., 1., 1.),
                p(1., -1., 1.),
            ],
            [
                p(-1., 1., -1.),
                p(-1., 1., 1.),
                p(1., 1., 1.),
                p(1., 1., -1.),
            ],
            [
                p(-1., -1., 1.),
                p(-1., -1., -1.),
                p(1., -1., -1.),
                p(1., -1., 1.),
            ],
        ] {
            self.quad(v, [[0., 0.], [0., 1.], [1., 1.], [1., 0.]]);
        }
    }
    fn shape(&mut self, s: &Shape, y: impl Fn(f32, f32) -> f32, up: bool) {
        let k = self.p.len() as u32;
        for [x, z] in s.vertices() {
            self.p.push([x, y(x, z), z]);
            self.n.push([0., if up { 1. } else { -1. }, 0.]);
            self.uv.push([x * 0.25, z * 0.25]);
            self.colors.push([1.; 4]);
        }
        for t in &s.triangles {
            self.ix.push(if up {
                [k + t[0], k + t[2], k + t[1]]
            } else {
                [k + t[0], k + t[1], k + t[2]]
            });
        }
    }
    fn surface_uvs(&mut self, mat: usize) {
        if [25, 27, 28, 30, 31, 33].contains(&mat) {
            let scale = if mat == 25 {
                crate::road_detail::TILE_METERS
            } else if [27, 30, 31].contains(&mat) {
                4.
            } else {
                0.5
            };
            // Before origin rebasing: adjacent chunks share the same world phase.
            self.uv = self
                .p
                .iter()
                .map(|p| [p[0] / scale, p[2] / scale])
                .collect();
        }
    }
    fn mesh(&self, origin: Vec3) -> Mesh {
        let mut m = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        m.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.p
                .iter()
                .map(|p| (Vec3::from_array(*p) - origin).to_array())
                .collect::<Vec<_>>(),
        );
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.n.clone());
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uv.clone());
        m.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors.clone());
        m.insert_attribute(
            Mesh::ATTRIBUTE_TANGENT,
            vec![[1., 0., 0., 1.]; self.p.len()],
        );
        m.insert_indices(Indices::U32(self.ix.iter().flatten().copied().collect()));
        m
    }
}
/// Project paint onto the very same triangles used by pavement rendering and
/// collision. Sampling the analytic height at the road centre loses crossfall
/// and the tile triangulation, and can bury stripes on cross-sloped streets.
#[derive(Default)]
struct PavementHeights {
    cells: BTreeMap<(i32, i32), Vec<[Vec3; 3]>>,
}
impl PavementHeights {
    fn add(&mut self, triangle: [Vec3; 3]) {
        let lo = triangle
            .iter()
            .fold(Vec2::splat(f32::INFINITY), |a, p| a.min(p.xz()));
        let hi = triangle
            .iter()
            .fold(Vec2::splat(f32::NEG_INFINITY), |a, p| a.max(p.xz()));
        for x in (lo.x / 32.).floor() as i32..=(hi.x / 32.).floor() as i32 {
            for z in (lo.y / 32.).floor() as i32..=(hi.y / 32.).floor() as i32 {
                self.cells.entry((x, z)).or_default().push(triangle);
            }
        }
    }
    fn height(&self, p: Vec2) -> Option<f32> {
        self.cells
            .get(&((p.x / 32.).floor() as i32, (p.y / 32.).floor() as i32))?
            .iter()
            .filter_map(|[a, b, c]| {
                let ab = b.xz() - a.xz();
                let ac = c.xz() - a.xz();
                let ap = p - a.xz();
                let det = ab.perp_dot(ac);
                if det.abs() < 1e-8 {
                    return None;
                }
                let u = ap.perp_dot(ac) / det;
                let v = ab.perp_dot(ap) / det;
                (u >= -1e-4 && v >= -1e-4 && u + v <= 1.0001)
                    .then_some(a.y + u * (b.y - a.y) + v * (c.y - a.y))
            })
            .max_by(f32::total_cmp)
    }
}
/// Collision batches are keyed by gameplay semantics, independently of render
/// materials. Keep every triangle and its winding; do not weld or simplify.
#[derive(Default)]
struct CollisionBatch {
    points: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
}
impl CollisionBatch {
    fn append(&mut self, batch: &Batch, origin: Vec3) {
        let offset = self.points.len() as u32;
        self.points
            .extend(batch.p.iter().map(|p| Vec3::from_array(*p) - origin));
        self.triangles
            .extend(batch.ix.iter().map(|t| t.map(|i| i + offset)));
    }
}
fn collision_role(material: usize) -> usize {
    if material < 25 { 0 } else { material }
}
fn key(p: Vec2, material: usize) -> (i32, i32, usize) {
    (
        (p.x / 256.).floor() as i32,
        (p.y / 256.).floor() as i32,
        material,
    )
}
fn textures(
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
    facades: &mut Assets<super::material::FacadeMaterial>,
) -> (
    Vec<Handle<StandardMaterial>>,
    Handle<super::material::FacadeMaterial>,
) {
    let image = facade_image(
        include_bytes!("../../../assets/metropolis/facades.rgba"),
        TextureFormat::Rgba8UnormSrgb,
    );
    let packed = facade_image(
        include_bytes!("../../../assets/metropolis/facades-packed.rgba"),
        TextureFormat::Rgba8Unorm,
    );
    let handle = facades.add(super::material::FacadeMaterial {
        base: StandardMaterial {
            perceptual_roughness: 1.0,
            metallic: 1.0,
            ..default()
        },
        extension: super::material::FacadeAtlas {
            image: images.add(image),
            packed: images.add(packed),
        },
    });
    let empty = materials.add(StandardMaterial::default());
    (vec![empty; 24], handle)
}

fn facade_image(raw: &[u8], format: TextureFormat) -> Image {
    let per: usize = (0..5).map(|l| ((96 >> l) * (96 >> l) * 4) as usize).sum();
    assert_eq!(
        raw.len(),
        per * rr_content::metropolis::FACADE_FAMILIES.len() * 3
    );
    let mut pixels = vec![];
    for level in 0..5 {
        let width = 1024 >> level;
        let height = 1152 >> level;
        let cell = 128 >> level;
        let gutter = 16 >> level;
        let side = 96 >> level;
        let offset: usize = (0..level)
            .map(|l| ((96 >> l) * (96 >> l) * 4) as usize)
            .sum();
        let mut layer = vec![0; width * height * 4];
        for tile in 0..rr_content::metropolis::FACADE_FAMILIES.len() * 3 {
            for y in 0..cell {
                for x in 0..cell {
                    let sx = (x as i32 - gutter).rem_euclid(side as i32) as usize;
                    let sy = (y as i32 - gutter).rem_euclid(side as i32) as usize;
                    let src = tile * per + offset + (sy * side + sx) * 4;
                    let dest = ((tile / 8 * cell + y) * width + tile % 8 * cell + x) * 4;
                    layer[dest..dest + 4].copy_from_slice(&raw[src..src + 4]);
                }
            }
        }
        pixels.extend(layer);
    }
    let mut image = Image::new(
        Extent3d {
            width: 1024,
            height: 1152,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels[..1024 * 1152 * 4].to_vec(),
        format,
        RenderAssetUsages::default(),
    );
    image.data = Some(pixels);
    image.texture_descriptor.mip_level_count = 5;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        lod_max_clamp: 4.,
        ..ImageSamplerDescriptor::linear()
    });
    image
}

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    facades: &mut Assets<super::material::FacadeMaterial>,
    grass: &mut Assets<grass_material::GrassMaterial>,
    m: &Metropolis,
    track: &Track,
) -> Census {
    let (mut mats, facade) = textures(images, materials, facades);
    let ground_texture = landscape::ground_texture(images);
    mats.push(materials.add(Color::srgb(0.23, 0.24, 0.24)));
    mats.push(materials.add(crate::road_detail::asphalt(images)));
    mats.push(materials.add(super::streetscape::paving(images)));
    mats.push(materials.add(landscape::ground_material(
        Color::srgb(0.3, 0.44, 0.23),
        &ground_texture,
    )));
    mats.push(materials.add(crate::road_detail::paint(
        images,
        Color::srgb(0.84, 0.78, 0.62),
    )));
    mats.push(materials.add(Color::srgb(0.30, 0.23, 0.14)));
    mats.push(materials.add(landscape::ground_material(
        Color::srgb(0.57, 0.52, 0.28),
        &ground_texture,
    )));
    mats.push(materials.add(landscape::ground_material(
        Color::srgb(0.40, 0.36, 0.25),
        &ground_texture,
    )));
    mats.push(materials.add(Color::srgb(0.43, 0.43, 0.39)));
    mats.push(materials.add(crate::road_detail::paint(
        images,
        Color::srgb(0.88, 0.87, 0.82),
    )));
    let mut batches: BTreeMap<(i32, i32, usize), Batch> = BTreeMap::new();
    let mut details: BTreeMap<(i32, i32, usize), Batch> = BTreeMap::new();
    let catalog =
        rr_content::architecture::Catalog::from_json(crate::world_gen::architecture::CATALOG)
            .unwrap();
    for b in &m.buildings {
        let vv: Vec<_> = b.shape.vertices().collect();
        let center = vv.iter().map(|p| Vec2::from_array(*p)).sum::<Vec2>() / vv.len() as f32;
        let base = vv
            .iter()
            .map(|p| track.terrain_height(p[0], p[1]))
            .fold(f32::NEG_INFINITY, f32::max)
            + 0.12;
        let top = base + b.height;
        let design = catalog
            .records
            .iter()
            .find(|d| d.id == rr_content::metropolis::FACADE_FAMILIES[b.style])
            .unwrap();
        for ring in &b.shape.rings {
            for (a, z) in ring
                .iter()
                .zip(ring.iter().cycle().skip(1))
                .take(ring.len())
            {
                let a = Vec3::new(a[0], track.terrain_height(a[0], a[1]) - 0.8, a[1]);
                let z = Vec3::new(z[0], track.terrain_height(z[0], z[1]) - 0.8, z[1]);
                let width = a.xz().distance(z.xz());
                let (bays, ground, cap, upper_count) =
                    super::facades::dimensions(width, b.height, design);
                let bays = bays as f32;
                let upper_count = upper_count as f32;
                let mid = base + ground;
                for (band, lo, hi, repeats) in [
                    (0, a.y.min(z.y), mid, (mid - a.y.min(z.y)) / ground),
                    (1, mid, top - cap, upper_count),
                    (2, top - cap, top, 1.),
                ] {
                    let batch = batches.entry(key(center, 0)).or_default();
                    batch.quad(
                        [a.with_y(lo), a.with_y(hi), z.with_y(hi), z.with_y(lo)],
                        [
                            [0., 1.],
                            [0., 1. - repeats],
                            [bays, 1. - repeats],
                            [bays, 1.],
                        ],
                    );
                    let end = batch.colors.len();
                    for c in &mut batch.colors[end - 4..] {
                        *c = [(b.style * 3 + band) as f32, 0., 0., 1.];
                    }
                }
                if b.tier <= 1 {
                    let edge = (z - a).with_y(0.).normalize();
                    let out = Vec3::new(edge.z, 0., -edge.x) * 0.18;
                    let ds = details.entry(key(center, 28)).or_default();
                    for floor in 1..=b.floors {
                        let y = base + floor as f32 * 3.2;
                        ds.quad(
                            [
                                a.with_y(y) + out,
                                a.with_y(y + 0.12) + out,
                                z.with_y(y + 0.12) + out,
                                z.with_y(y) + out,
                            ],
                            [[0., 0.], [0., 1.], [1., 1.], [1., 0.]],
                        );
                    }
                }
            }
        }
        batches
            .entry(key(center, 24))
            .or_default()
            .shape(&b.shape, |_, _| top, true);
        batches.entry(key(center, 24)).or_default().shape(
            &b.shape,
            |x, z| track.terrain_height(x, z) - 0.8,
            false,
        );
    }
    for surface in &m.surfaces {
        let p = surface.shape.rings[0][0];
        let mat = match surface.kind {
            0 => 25,
            1 => 26,
            _ => 32,
        };
        batches
            .entry(key(Vec2::from_array(p), mat))
            .or_default()
            .shape(
                &surface.shape,
                |x, z| track.terrain_height(x, z) + if surface.kind == 1 { 0.14 } else { 0.025 },
                true,
            );
    }
    for f in &m.fields {
        let p = f.shape.rings[0][0];
        for patch in &f.patches {
            batches
                .entry(key(
                    Vec2::from_array(p),
                    if f.crop == "grain" {
                        30
                    } else if f.crop == "fallow" {
                        31
                    } else {
                        27
                    },
                ))
                .or_default()
                .shape(patch, |x, z| track.terrain_height(x, z) - 0.12, true);
        }
    }
    let mut fence_posts = std::collections::HashSet::new();
    for field in &m.fields {
        for ring in &field.shape.rings {
            for (a, b) in ring
                .iter()
                .zip(ring.iter().cycle().skip(1))
                .take(ring.len())
            {
                let a = Vec2::from_array(*a);
                let b = Vec2::from_array(*b);
                let n = (a.distance(b) / 6.).ceil() as usize;
                for i in 0..n {
                    if i % 13 == 0 {
                        continue;
                    }
                    let p = a.lerp(b, i as f32 / n as f32);
                    let end = a.lerp(b, (i + 1) as f32 / n as f32);
                    let middle = (p + end) * 0.5;
                    // Keep field boundaries out of the road and urban-transition apron.
                    let (s, lane) = track.nearby_station(middle.x, middle.y);
                    if s > 3000. || lane.abs() < 12. {
                        continue;
                    }
                    let q = Quat::from_rotation_y((end.x - p.x).atan2(end.y - p.y));
                    let ground = track.terrain_height(middle.x, middle.y);
                    let batch = batches.entry(key(middle, 29)).or_default();
                    for point in [p, end] {
                        let id = (
                            (point.x * 1000.).round() as i32,
                            (point.y * 1000.).round() as i32,
                        );
                        if fence_posts.insert(id) {
                            batch.cuboid(
                                Vec3::new(
                                    point.x,
                                    track.terrain_height(point.x, point.y) + 0.6,
                                    point.y,
                                ),
                                Vec3::new(0.13, 1.2, 0.13),
                                q,
                            );
                        }
                    }
                    for y in [0.45, 0.95] {
                        batch.cuboid(
                            Vec3::new(middle.x, ground + y, middle.y),
                            Vec3::new(0.1, 0.12, p.distance(end)),
                            q,
                        );
                    }
                }
            }
        }
    }
    // Ground is continuous under the entire city, fields, shoulders and starting grid.
    let [xmin, zmin, xmax, zmax] = m.bounds;
    for x in ((xmin / 256.).floor() as i32)..=((xmax / 256.).ceil() as i32) {
        for z in ((zmin / 256.).floor() as i32)..=((zmax / 256.).ceil() as i32) {
            let batch = batches.entry((x, z, 27)).or_default();
            for i in 0..32 {
                for j in 0..32 {
                    let xx = x as f32 * 256. + i as f32 * 8.;
                    let zz = z as f32 * 256. + j as f32 * 8.;
                    let v = |x, z| Vec3::new(x, track.terrain_height(x, z) - 0.2, z);
                    batch.quad(
                        [
                            v(xx, zz),
                            v(xx, zz + 8.),
                            v(xx + 8., zz + 8.),
                            v(xx + 8., zz),
                        ],
                        [
                            [xx / 8., zz / 8.],
                            [xx / 8., zz / 8. + 1.],
                            [xx / 8. + 1., zz / 8. + 1.],
                            [xx / 8. + 1., zz / 8.],
                        ],
                    );
                }
            }
        }
    }
    let mut pavement = PavementHeights::default();
    for ((_, _, material), batch) in &batches {
        if *material == 25 {
            for triangle in &batch.ix {
                pavement.add(triangle.map(|i| Vec3::from_array(batch.p[i as usize])));
            }
        }
    }
    // Road paint follows both actual alternatives; it never draws a third spine between them.
    for (route, path) in m.paths.iter().enumerate() {
        for s in (-340..12000).step_by(4) {
            let s = s as f32;
            if route == 1 && !(3350. ..5362.5).contains(&s) {
                continue;
            }
            let _ = path;
            let sections = &m.street_life.lane_sections;
            let profile = rr_core::streets::section(sections, route, s);
            let (divider, width, half) = rr_core::streets::markings(sections, route, s);
            let mut stripes = vec![
                (divider - 0.11, 28, 4.),
                (divider + 0.11, 28, 4.),
                // Keep the complete 14cm edge stripe inside the carriageway,
                // including the small chords across reserved street bends.
                (-half + 0.25, 33, 4.),
                (half - 0.25, 33, 4.),
            ];
            if (s as i32).rem_euclid(12) < 6 {
                for i in 1..profile.outbound {
                    stripes.push((
                        divider - i as f32 * width,
                        33,
                        (6 - (s as i32).rem_euclid(12)).min(4) as f32,
                    ));
                }
                for i in 1..profile.inbound {
                    stripes.push((
                        divider + i as f32 * width,
                        33,
                        (6 - (s as i32).rem_euclid(12)).min(4) as f32,
                    ));
                }
            }
            for (lane, material, length) in stripes {
                let (p, q) = track.street_sample(route, s, 0.);
                let end_s = (s + length).min(12000.);
                let (end_divider, end_width, end_half) =
                    rr_core::streets::markings(sections, route, end_s);
                let end_lane = if material == 33 && (lane.abs() - (half - 0.25)).abs() < 0.001 {
                    lane.signum() * (end_half - 0.25)
                } else {
                    end_divider + (lane - divider) / width * end_width
                };
                let (a, end_q) = track.street_sample(route, end_s, end_lane);
                let side = q * Vec3::X;
                let end_side = end_q * Vec3::X;
                let p = p - side * lane + Vec3::Y * 0.05;
                let a = a + Vec3::Y * 0.05;
                batches.entry(key(p.xz(), material)).or_default().quad(
                    [
                        p - side * 0.07,
                        a - end_side * 0.07,
                        a + end_side * 0.07,
                        p + side * 0.07,
                    ]
                    .map(|vertex| {
                        vertex.with_y(
                            pavement.height(vertex.xz()).unwrap_or_else(|| {
                                track.terrain_height(vertex.x, vertex.z) + 0.025
                            }) + 0.018,
                        )
                    }),
                    [[0., 0.], [0., 1.], [1., 1.], [1., 0.]],
                );
            }
        }
    }
    // Marked pedestrian crossings on both actual carriageways, projected onto
    // the same pavement triangles as lane paint; never stripe a sidewalk.
    for junction in &m.junctions {
        let point = Vec3::new(junction.point[0], 0., junction.point[1]);
        let route =
            if track.project_route(0, point, 4000.).2 < track.project_route(1, point, 4000.).2 {
                0
            } else {
                1
            };
        let station = track.project_route(route, point, 4000.).0 - 17.;
        let (center, q) = track.street_sample(route, station, 0.);
        let half = rr_core::streets::markings(&m.street_life.lane_sections, route, station).2;
        for i in -12..=12 {
            let lane = i as f32 * 0.8;
            if lane.abs() > half - 0.5 {
                continue;
            }
            let center = center - q * Vec3::X * lane;
            let points = [
                Vec3::new(-0.23, 0., -1.8),
                Vec3::new(-0.23, 0., 1.8),
                Vec3::new(0.23, 0., 1.8),
                Vec3::new(0.23, 0., -1.8),
            ]
            .map(|p| center + q * p);
            if points.iter().any(|p| pavement.height(p.xz()).is_none()) {
                continue;
            }
            batches.entry(key(center.xz(), 33)).or_default().quad(
                points.map(|p| p.with_y(pavement.height(p.xz()).unwrap() + 0.022)),
                [[0., 0.], [0., 1.], [1., 1.], [1., 0.]],
            );
        }
    }
    let mut census = Census {
        buildings: m.buildings.len(),
        physical_routes: 2,
        source_layout_sha256: m.source_layout_sha256.clone(),
        ..default()
    };
    let mut collision_batches: BTreeMap<(i32, i32, usize), CollisionBatch> = BTreeMap::new();
    for ((x, z, mat), mut batch) in batches {
        batch.surface_uvs(mat);
        landscape::tint(&mut batch, mat, track);
        if batch.ix.is_empty() {
            continue;
        }
        let origin = Vec3::new(x as f32 * 256., 0., z as f32 * 256.);
        census.render_triangles += batch.ix.len();
        census.chunks += 1;
        let mut entity = commands.spawn((
            Name::new(format!("Metro chunk {x} {z} / {mat}")),
            MetroChunk,
            Mesh3d(meshes.add({
                let mut mesh = batch.mesh(origin);
                if mat == 25 {
                    mesh.generate_tangents().expect("world-space road UVs");
                }
                mesh
            })),
            Transform::from_translation(origin),
        ));
        if mat == 0 {
            entity.insert(MeshMaterial3d(facade.clone()));
        } else {
            entity.insert(MeshMaterial3d(mats[mat].clone()));
        }
        if mat != 28 && mat != 33 {
            collision_batches
                .entry((x, z, collision_role(mat)))
                .or_default()
                .append(&batch, origin);
            census.source_collision_batches += 1;
            census.collision_triangles += batch.ix.len();
        }
    }
    census.static_collision_batches = collision_batches.len();
    for ((x, z, role), batch) in collision_batches {
        let origin = Vec3::new(x as f32 * 256., 0., z as f32 * 256.);
        let mut entity = commands.spawn((
            Name::new(format!("Metro solid {x} {z} / role {role}")),
            // No body/writeback state for immutable world geometry.
            Collider::trimesh(batch.points, batch.triangles)
                .expect("validated metropolis triangles"),
            Transform::from_translation(origin),
            Friction::coefficient(0.8),
        ));
        if role == 0 || role == 29 {
            entity.insert(Obstacle);
        } else {
            entity.insert(Ground);
        }
        if role == 25 {
            entity.insert(crate::environment::RoadChunk);
        }
        if role == 26 {
            entity.insert(crate::vehicle::RideableStep);
        }
    }
    for ((x, z, mat), batch) in details {
        let origin = Vec3::new(x as f32 * 256., 0., z as f32 * 256.);
        census.render_triangles += batch.ix.len();
        commands.spawn((
            Name::new("Metro additive cornices"),
            Mesh3d(meshes.add(batch.mesh(origin))),
            MeshMaterial3d(mats[mat].clone()),
            Transform::from_translation(origin),
            bevy::camera::visibility::VisibilityRange {
                start_margin: 0. ..0.,
                end_margin: 180. ..240.,
                use_aabb: false,
            },
        ));
    }
    landscape::spawn(commands, meshes, grass, track, &mut census, &mats[27]);
    census
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn landscape_finishes_preserve_actual_field_collision_geometry() {
        let track = Track::metropolis().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        let mut checked = 0;
        for field in &m.fields {
            let material = if field.crop == "grain" {
                30
            } else if field.crop == "fallow" {
                31
            } else {
                27
            };
            for patch in &field.patches {
                let mut batch = Batch::default();
                batch.shape(patch, |x, z| track.terrain_height(x, z) - 0.12, true);
                let mut before = CollisionBatch::default();
                before.append(&batch, Vec3::ZERO);
                batch.surface_uvs(material);
                landscape::tint(&mut batch, material, &track);
                let mut after = CollisionBatch::default();
                after.append(&batch, Vec3::ZERO);
                assert_eq!(before.points, after.points);
                assert_eq!(before.triangles, after.triangles);
                checked += after.triangles.len();
            }
        }
        assert!(checked > 1000);
        println!("Exact field collision positions/winding preserved: {checked} triangles");
    }
    #[test]
    fn packed_facade_image_preserves_linear_channels_and_wrapped_mip_gutters() {
        let raw = include_bytes!("../../../assets/metropolis/facades-packed.rgba");
        let image = facade_image(raw, TextureFormat::Rgba8Unorm);
        assert_eq!(image.texture_descriptor.format, TextureFormat::Rgba8Unorm);
        assert_eq!(image.texture_descriptor.mip_level_count, 5);
        let data = image.data.unwrap();
        let per: usize = (0..5).map(|l| (96 >> l) * (96 >> l) * 4).sum();
        let mut layer_offset = 0;
        let mut source_offset = 0;
        for level in 0..5 {
            let width = 1024 >> level;
            let height = 1152 >> level;
            let cell = 128 >> level;
            let gutter = 16 >> level;
            let side = 96 >> level;
            for tile in 0..72 {
                for y in [0, gutter, cell - 1] {
                    for x in [0, gutter, cell - 1] {
                        let sx = (x as i32 - gutter as i32).rem_euclid(side as i32) as usize;
                        let sy = (y as i32 - gutter as i32).rem_euclid(side as i32) as usize;
                        let src = tile * per + source_offset + (sy * side + sx) * 4;
                        let dest = layer_offset
                            + ((tile / 8 * cell + y) * width + tile % 8 * cell + x) * 4;
                        assert_eq!(&data[dest..dest + 4], &raw[src..src + 4]);
                    }
                }
            }
            layer_offset += width * height * 4;
            source_offset += side * side * 4;
        }
        assert_eq!(data.len(), layer_offset);
        assert_eq!(data.len(), 6_285_312);
    }

    #[test]
    fn road_surface_uvs_survive_chunk_rebasing_without_changing_geometry() {
        let mut batch = Batch::default();
        batch.quad(
            [
                Vec3::new(254., 3., -2.),
                Vec3::new(254., 3.1, 2.),
                Vec3::new(258., 3.1, 2.),
                Vec3::new(258., 3., -2.),
            ],
            [[0.; 2]; 4],
        );
        let points = batch.p.clone();
        let indices = batch.ix.clone();
        batch.surface_uvs(25);
        assert_eq!(batch.p, points);
        assert_eq!(batch.ix, indices);
        let mut a = batch.mesh(Vec3::ZERO);
        let mut b = batch.mesh(Vec3::new(256., 0., 0.));
        a.generate_tangents().unwrap();
        b.generate_tangents().unwrap();
        assert_eq!(
            a.attribute(Mesh::ATTRIBUTE_UV_0),
            b.attribute(Mesh::ATTRIBUTE_UV_0)
        );
        assert_eq!(
            a.attribute(Mesh::ATTRIBUTE_TANGENT),
            b.attribute(Mesh::ATTRIBUTE_TANGENT)
        );
        assert_eq!(batch.uv[0], [31.75, -0.25]);
        batch.surface_uvs(33);
        assert_eq!(batch.uv[0], [508., -4.]);
        assert_eq!(batch.p, points);
        assert_eq!(batch.ix, indices);
    }
    #[test]
    fn paint_follows_paved_crossfall_instead_of_burying_outer_lanes() {
        let mut heights = PavementHeights::default();
        heights.add([
            Vec3::new(-8., 0., 0.),
            Vec3::new(8., 0.32, 0.),
            Vec3::new(-8., 0., 32.),
        ]);
        heights.add([
            Vec3::new(8., 0.32, 0.),
            Vec3::new(8., 0.32, 32.),
            Vec3::new(-8., 0., 32.),
        ]);
        let centre = heights.height(Vec2::new(0., 10.)).unwrap();
        let outer = heights.height(Vec2::new(6., 10.)).unwrap();
        assert!(
            outer > centre + 0.05,
            "old centre-height paint was occluded"
        );
        assert!((outer - 0.28).abs() < 1e-6);
        assert!(heights.height(Vec2::new(9., 10.)).is_none());
    }
    #[test]
    fn both_routes_and_grid_follow_actual_pavement_triangles() {
        let track = Track::metropolis().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        let mut support = PavementHeights::default();
        for s in m.surfaces.iter().filter(|s| s.kind == 0) {
            let v: Vec<_> = s
                .shape
                .vertices()
                .map(|[x, z]| Vec3::new(x, track.terrain_height(x, z) + 0.025, z))
                .collect();
            for t in &s.shape.triangles {
                support.add(t.map(|i| v[i as usize]));
            }
        }
        let mut worst = 0f32;
        for route in 0..2 {
            for station in (-300..=12000).step_by(5) {
                let s = station as f32;
                let mut points = vec![track.sample_route(route, s, 0.).0];
                if station < 0 {
                    points.extend([-4.5, -1.5].map(|lane| track.sample_route(route, s, lane).0));
                } else {
                    for outgoing in [true, false] {
                        for slot in 0..2 {
                            let lane = rr_core::streets::lane_center(
                                &m.street_life.lane_sections,
                                route,
                                s,
                                outgoing,
                                slot,
                            );
                            points.push(track.street_sample(route, s, lane).0);
                        }
                    }
                }
                for (lane, p) in points.into_iter().enumerate() {
                    let road = support.height(p.xz()).unwrap_or_else(||panic!("missing pavement route={route} station={station} sample={lane} point={p:?}"));
                    worst = worst.max((p.y - road).abs());
                }
            }
        }
        println!("Both routes/grid maximum sampled pavement support deviation: {worst}m");
        assert!(worst < 0.015, "{worst}");
    }
    #[test]
    fn entry_car_bodies_and_paint_stay_on_visible_carriageway() {
        let track = Track::metropolis().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        let mut road = PavementHeights::default();
        let mut walk = PavementHeights::default();
        for surface in m.surfaces.iter().filter(|s| s.kind <= 1) {
            let vertices: Vec<_> = surface
                .shape
                .vertices()
                .map(|[x, z]| Vec3::new(x, 0., z))
                .collect();
            let support = if surface.kind == 0 {
                &mut road
            } else {
                &mut walk
            };
            for triangle in &surface.shape.triangles {
                support.add(triangle.map(|i| vertices[i as usize]));
            }
        }
        for route in 0..2 {
            for station in (2700..3500).chain(5260..5480).step_by(2) {
                let s = station as f32;
                let mut points = Vec::new();
                for outgoing in [true, false] {
                    for slot in 0..2 {
                        let lane = rr_core::streets::lane_center(
                            &m.street_life.lane_sections,
                            route,
                            s,
                            outgoing,
                            slot,
                        );
                        let (p, q) = track.street_sample(route, s, lane);
                        for x in [-0.95, 0., 0.95] {
                            for z in [-1.95, 0., 1.95] {
                                points.push((p + q * Vec3::new(x, 0., z)).xz());
                            }
                        }
                    }
                }
                let (divider, _, half) =
                    rr_core::streets::markings(&m.street_life.lane_sections, route, s);
                for lane in [divider - 0.11, divider + 0.11, -half + 0.25, half - 0.25] {
                    for edge in [-0.07, 0.07] {
                        points.push(track.street_sample(route, s, lane + edge).0.xz());
                    }
                }
                for point in points {
                    assert!(
                        road.height(point).is_some(),
                        "missing road route={route} s={s} p={point:?}"
                    );
                    assert!(
                        walk.height(point).is_none(),
                        "raised sidewalk over traffic/paint route={route} s={s} p={point:?}"
                    );
                }
            }
        }
    }
    fn crossing_trace(
        route: usize,
        station: f32,
        side: f32,
        start_offset: f32,
        wall: bool,
        legacy_down: bool,
    ) -> Vec3 {
        let mut track = Track::metropolis().unwrap();
        if legacy_down {
            // Freeze the whole historical support context, not just its triangles.
            // The live road centre/elevation can change without invalidating this
            // deliberately broken-controller negative control.
            #[derive(serde::Deserialize)]
            struct Frozen {
                elevation_profile: rr_content::metropolis::ElevationProfile,
                traffic_path: rr_content::metropolis::Path,
            }
            let frozen: Frozen = serde_json::from_str(include_str!(
                "../../../docs/testing/living-city-ai/fixtures/old-farm-rim-context.json"
            ))
            .unwrap();
            let map = std::sync::Arc::make_mut(track.metropolis.as_mut().unwrap());
            map.elevation_profile = frozen.elevation_profile;
            map.traffic_paths[0] = frozen.traffic_path;
        }
        let m = track.metropolis.as_ref().unwrap();
        let (centre, rotation) = track.street_sample(route, station, 0.);
        let lateral = rotation * Vec3::X * side;
        let start = centre + lateral * start_offset;
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1. / 60.,
            substeps: 3,
        });
        // The deliberately broken old controller needs its frozen pre-remodel rim.
        // Current controller crossing tests always use the current baked map.
        let historical: Vec<rr_content::metropolis::Surface> = if legacy_down {
            serde_json::from_str(include_str!(
                "../../../docs/testing/living-city-ai/fixtures/old-farm-rim-surfaces.json"
            ))
            .unwrap()
        } else {
            vec![]
        };
        let surfaces = if legacy_down {
            &historical
        } else {
            &m.surfaces
        };
        // Use actual source pavement/sidewalk triangles, at their production heights.
        for surface in surfaces {
            if !surface
                .shape
                .vertices()
                .any(|[x, z]| Vec2::new(x, z).distance(centre.xz()) < 50.)
            {
                continue;
            }
            let mut batch = Batch::default();
            batch.shape(
                &surface.shape,
                |x, z| track.terrain_height(x, z) + if surface.kind == 1 { 0.14 } else { 0.025 },
                true,
            );
            app.world_mut().spawn((
                Collider::trimesh(
                    batch.p.iter().map(|p| Vec3::from_array(*p)).collect(),
                    batch.ix,
                )
                .unwrap(),
                Transform::IDENTITY,
            ));
        }
        // Identical underlying terrain grid in the local crossing window.
        let mut terrain = Batch::default();
        let cx = (centre.x / 8.).floor() as i32;
        let cz = (centre.z / 8.).floor() as i32;
        for x in cx - 6..=cx + 6 {
            for z in cz - 6..=cz + 6 {
                let point = |x: i32, z: i32| {
                    Vec3::new(
                        x as f32 * 8.,
                        track.terrain_height(x as f32 * 8., z as f32 * 8.) - 0.2,
                        z as f32 * 8.,
                    )
                };
                terrain.quad(
                    [
                        point(x, z),
                        point(x, z + 1),
                        point(x + 1, z + 1),
                        point(x + 1, z),
                    ],
                    [[0.; 2]; 4],
                );
            }
        }
        app.world_mut().spawn((
            Collider::trimesh(
                terrain.p.iter().map(|p| Vec3::from_array(*p)).collect(),
                terrain.ix,
            )
            .unwrap(),
            Transform::IDENTITY,
        ));
        if wall {
            app.world_mut().spawn((
                Collider::cuboid(0.2, 0.5, 20.),
                Transform::from_translation(centre + Vec3::Y * 0.5).with_rotation(rotation),
            ));
        }
        let controller = crate::recovery::runner_controller();
        let runner = app
            .world_mut()
            .spawn((
                RigidBody::KinematicPositionBased,
                Collider::capsule_y(0.65, 0.28),
                Transform::from_xyz(
                    start.x,
                    track.terrain_height(start.x, start.z) + 1.1,
                    start.z,
                ),
                controller,
            ))
            .id();
        let mut best = 0f32;
        let mut stalled = 0u32;
        let mut worst_stall = 0u32;
        let mut reached_frame = None;
        for frame in 0..600 {
            let progress =
                (app.world().get::<Transform>(runner).unwrap().translation - start).dot(-lateral);
            if progress > best + 0.01 {
                best = progress;
                stalled = 0;
            } else if frame >= 30 && reached_frame.is_none() {
                stalled += 1;
                worst_stall = worst_stall.max(stalled);
            }
            if progress > 18. && reached_frame.is_none() {
                reached_frame = Some(frame);
            }
            let reached = (app.world().get::<Transform>(runner).unwrap().translation - start)
                .dot(-lateral)
                > 18.;
            let movement = if frame < 30 || reached {
                Vec3::ZERO
            } else {
                -lateral * 0.065
            };
            let down = if legacy_down {
                5.
            } else {
                rr_core::recovery::foot_downward_speed(
                    app.world()
                        .get::<KinematicCharacterControllerOutput>(runner)
                        .is_some_and(|output| output.grounded),
                )
            };
            app.world_mut()
                .get_mut::<KinematicCharacterController>(runner)
                .unwrap()
                .translation = Some(movement - Vec3::Y * (down / 60.));
            app.update();
        }
        let end = app.world().get::<Transform>(runner).unwrap().translation;
        println!(
            "Crossing profile route={route} station={station} side={side} start={start_offset} wall={wall}: advance={:.3}m reached_frame={reached_frame:?} longest_stall={worst_stall}ticks",
            (end - start).dot(-lateral)
        );
        if legacy_down {
            assert!(
                (end - start).dot(-lateral) < 10. && worst_stall > 120,
                "old unconditional down-drive control must reproduce the rim trap"
            );
        } else if wall {
            assert!(
                (end - start).dot(-lateral) < start_offset - 0.2,
                "solid metre-high wall must remain blocking: {start:?} -> {end:?}"
            );
        } else {
            assert!(
                worst_stall < 30,
                "residual rim sticking: {worst_stall}ticks"
            );
            assert!(
                (end - start).dot(-lateral) > 17.5,
                "blocked route {route} station {station} side {side}: {start:?} -> {end:?}; output {:?}",
                app.world()
                    .get::<KinematicCharacterControllerOutput>(runner)
            );
        }
        println!(
            "Physical foot crossing route={route} station={station} side={side} start_offset={start_offset} wall={wall}: distance={:.3}m",
            (end - start).dot(-lateral)
        );
        assert!(end.y > track.terrain_height(end.x, end.z) + 0.65);
        end
    }
    #[test]
    fn real_metropolis_support_allows_foot_crossings_in_both_directions() {
        for (route, station) in [
            (0, 3000.),
            (0, 3250.),
            (0, 4100.),
            (1, 4100.),
            (0, 6500.),
            (0, 10000.),
        ] {
            for side in [-1., 1.] {
                for start in [8., 10.] {
                    crossing_trace(route, station, side, start, false, false);
                }
            }
        }
    }
    #[test]
    fn old_grounded_down_drive_reproduces_the_real_farm_rim_trap() {
        crossing_trace(0, 3000., -1., 8., false, true);
    }
    #[test]
    fn on_foot_steps_do_not_climb_solid_walls_above_the_cap() {
        crossing_trace(0, 6500., 1., 8., true, false);
    }
    #[test]
    fn semantic_collision_batching_preserves_exact_triangle_soup_and_support_roles() {
        let origin = Vec3::new(-256., 0., 512.);
        let mut a = Batch::default();
        a.cuboid(
            Vec3::new(-230., 4., 540.),
            Vec3::new(10., 8., 12.),
            Quat::IDENTITY,
        );
        let mut b = Batch::default();
        b.cuboid(
            Vec3::new(-210., 14., 580.),
            Vec3::new(8., 28., 10.),
            Quat::from_rotation_y(0.3),
        );
        let mut merged = CollisionBatch::default();
        merged.append(&a, origin);
        merged.append(&b, origin);
        let expected: Vec<_> = [&a, &b]
            .into_iter()
            .flat_map(|batch| {
                batch
                    .ix
                    .iter()
                    .map(|tri| tri.map(|i| Vec3::from_array(batch.p[i as usize]) - origin))
            })
            .collect();
        let actual: Vec<_> = merged
            .triangles
            .iter()
            .map(|tri| tri.map(|i| merged.points[i as usize]))
            .collect();
        assert_eq!(
            actual, expected,
            "world positions, winding and duplicate edges must survive batching"
        );
        assert_eq!(collision_role(0), collision_role(24));
        assert_eq!(
            collision_role(29),
            29,
            "farm fences keep their separate bounds"
        );
        for material in [25, 26, 27, 30, 31, 32] {
            assert_eq!(
                collision_role(material),
                material,
                "road/step/ground roles cannot merge"
            );
        }
    }
    fn static_contact_trace(explicit_bodies: bool) -> (Vec<(Transform, Velocity)>, usize) {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1. / 60.,
            substeps: 3,
        });
        let floor = Collider::trimesh(
            vec![
                Vec3::new(-20., 0., -20.),
                Vec3::new(20., 0., -20.),
                Vec3::new(20., 0., 20.),
                Vec3::new(-20., 0., 20.),
            ],
            vec![[0, 2, 1], [0, 3, 2]],
        )
        .unwrap();
        let wall = Collider::trimesh(
            vec![
                Vec3::new(3., 0., -10.),
                Vec3::new(3., 5., -10.),
                Vec3::new(3., 5., 10.),
                Vec3::new(3., 0., 10.),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        )
        .unwrap();
        let ground = app
            .world_mut()
            .spawn((
                floor,
                Transform::IDENTITY,
                Friction::coefficient(0.8),
                Ground,
            ))
            .id();
        let barrier = app
            .world_mut()
            .spawn((
                wall,
                Transform::IDENTITY,
                Friction::coefficient(0.8),
                Obstacle,
            ))
            .id();
        if explicit_bodies {
            for e in [ground, barrier] {
                app.world_mut().entity_mut(e).insert(RigidBody::Fixed);
            }
        }
        let rider = app
            .world_mut()
            .spawn((
                RigidBody::Dynamic,
                Collider::capsule_y(0.45, 0.2),
                Transform::from_xyz(0., 3., 0.),
                Velocity::linear(Vec3::new(7., 0., 1.)),
                Ccd::enabled(),
                ActiveEvents::COLLISION_EVENTS,
            ))
            .id();
        let mut trace = vec![];
        let mut contact = false;
        for _ in 0..240 {
            app.update();
            trace.push((
                *app.world().get::<Transform>(rider).unwrap(),
                *app.world().get::<Velocity>(rider).unwrap(),
            ));
            let mut events = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
            contact |= events
                .drain()
                .any(|e| matches!(e, CollisionEvent::Started(_, _, _)));
        }
        assert!(contact, "the actor must still hit immutable geometry");
        assert!(trace.last().unwrap().0.translation.y < 1.);
        let world = app.world_mut();
        let mut contexts = world.query::<(
            &RapierContextSimulation,
            &RapierContextColliders,
            &RapierContextJoints,
            &RapierRigidBodySet,
        )>();
        let (simulation, colliders, joints, rigidbody_set) = contexts.single(world).unwrap();
        let ctx = RapierContext {
            simulation,
            colliders,
            joints,
            rigidbody_set,
        };
        let (entity, toi) = ctx
            .cast_ray(
                Vec3::new(0., 5., 0.),
                Vec3::NEG_Y,
                10.,
                true,
                QueryFilter::exclude_dynamic(),
            )
            .expect("ground ray remains supported");
        assert_eq!(entity, ground);
        assert!((toi - 5.).abs() < 0.001);
        assert!(
            ctx.cast_ray(
                Vec3::new(0., 5., 0.),
                Vec3::NEG_Y,
                10.,
                true,
                QueryFilter {
                    flags: QueryFilterFlags::EXCLUDE_FIXED | QueryFilterFlags::EXCLUDE_DYNAMIC,
                    ..default()
                }
            )
            .is_none()
        );
        (trace, rigidbody_set.bodies.len())
    }
    #[test]
    fn standalone_static_chunks_keep_contact_ccd_and_support_trajectories() {
        let (control, with_fixed) = static_contact_trace(true);
        let (optimized, without_fixed) = static_contact_trace(false);
        assert_eq!((with_fixed, without_fixed), (3, 1));
        for ((a, av), (b, bv)) in control.iter().zip(optimized) {
            assert!(a.translation.abs_diff_eq(b.translation, 0.001));
            assert!(a.rotation.abs_diff_eq(b.rotation, 0.001));
            assert!(av.linvel.abs_diff_eq(bv.linvel, 0.001));
            assert!(av.angvel.abs_diff_eq(bv.angvel, 0.001));
        }
    }
}
