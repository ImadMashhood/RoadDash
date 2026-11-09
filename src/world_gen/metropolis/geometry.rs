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
    let raw = include_bytes!("../../../assets/metropolis/facades.rgba");
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
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.data = Some(pixels);
    image.texture_descriptor.mip_level_count = 5;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        lod_max_clamp: 4.,
        ..ImageSamplerDescriptor::linear()
    });
    let handle = facades.add(super::material::FacadeMaterial {
        base: StandardMaterial {
            perceptual_roughness: 0.84,
            ..default()
        },
        extension: super::material::FacadeAtlas {
            image: images.add(image),
        },
    });
    let empty = materials.add(StandardMaterial::default());
    (vec![empty; 24], handle)
}

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    facades: &mut Assets<super::material::FacadeMaterial>,
    m: &Metropolis,
    track: &Track,
) -> Census {
    let (mut mats, facade) = textures(images, materials, facades);
    mats.push(materials.add(Color::srgb(0.23, 0.24, 0.24)));
    mats.push(materials.add(Color::srgb(0.12, 0.13, 0.14)));
    mats.push(materials.add(super::streetscape::paving(images)));
    mats.push(materials.add(Color::srgb(0.3, 0.44, 0.23)));
    mats.push(materials.add(Color::srgb(0.84, 0.78, 0.62)));
    mats.push(materials.add(Color::srgb(0.30, 0.23, 0.14)));
    mats.push(materials.add(Color::srgb(0.57, 0.52, 0.28)));
    mats.push(materials.add(Color::srgb(0.40, 0.36, 0.25)));
    mats.push(materials.add(Color::srgb(0.43, 0.43, 0.39)));
    mats.push(materials.add(Color::srgb(0.88, 0.87, 0.82)));
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
            .shape(&f.shape, |x, z| track.terrain_height(x, z) - 0.12, true);
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
                (divider - 0.11, 28),
                (divider + 0.11, 28),
                (-half, 33),
                (half, 33),
            ];
            if (s as i32).rem_euclid(12) < 6 {
                for i in 1..profile.outbound {
                    stripes.push((divider - i as f32 * width, 33));
                }
                for i in 1..profile.inbound {
                    stripes.push((divider + i as f32 * width, 33));
                }
            }
            for (lane, material) in stripes {
                let (p, q) = track.street_sample(route, s, 0.);
                let (a, _) = track.street_sample(route, (s + 2.).min(12000.), 0.);
                let side = q * Vec3::X;
                let p = p - side * lane + Vec3::Y * 0.05;
                let a = a - side * lane + Vec3::Y * 0.05;
                batches.entry(key(p.xz(), material)).or_default().quad(
                    [
                        p - side * 0.04,
                        a - side * 0.04,
                        a + side * 0.04,
                        p + side * 0.04,
                    ],
                    [[0., 0.], [0., 1.], [1., 1.], [1., 0.]],
                );
            }
        }
    }
    let mut census = Census {
        buildings: m.buildings.len(),
        physical_routes: 2,
        source_layout_sha256: m.source_layout_sha256.clone(),
        ..default()
    };
    let mut collision_batches: BTreeMap<(i32, i32, usize), CollisionBatch> = BTreeMap::new();
    for ((x, z, mat), batch) in batches {
        if batch.ix.is_empty() {
            continue;
        }
        let origin = Vec3::new(x as f32 * 256., 0., z as f32 * 256.);
        census.render_triangles += batch.ix.len();
        census.chunks += 1;
        let mut entity = commands.spawn((
            Name::new(format!("Metro chunk {x} {z} / {mat}")),
            MetroChunk,
            Mesh3d(meshes.add(batch.mesh(origin))),
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
    census
}

#[cfg(test)]
mod tests {
    use super::*;
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
