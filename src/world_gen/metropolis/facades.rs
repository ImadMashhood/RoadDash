//! Actual authored storey meshes on polygon edges; resident prototypes, bounded additive detail.
use super::*;
use bevy::gltf::{Gltf, GltfMesh};
use rr_content::architecture::{Catalog, Design};
use rr_content::metropolis::FACADE_FAMILIES as FAMILIES;
use std::collections::{BTreeMap, BTreeSet, HashMap};
const RADIUS: f32 = 175.;
const MAX_FACES: usize = 28;
const MAX_MESHES: usize = 6000;
const MAX_TRIANGLES: usize = 1_500_000;
#[derive(Clone)]
struct Primitive {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    triangles: usize,
}
struct Source {
    file: Handle<Gltf>,
    bands: Option<[Vec<Primitive>; 3]>,
}
#[derive(Clone)]
struct Face {
    style: usize,
    a: Vec3,
    b: Vec3,
    base: f32,
    height: f32,
}
impl Face {
    fn width(&self) -> f32 {
        self.a.distance(self.b)
    }
    fn normal(&self) -> Vec3 {
        let v = (self.b - self.a).normalize();
        Vec3::new(v.z, 0., -v.x)
    }
    fn distance(&self, p: Vec3) -> f32 {
        let v = self.b - self.a;
        let t = ((p - self.a).dot(v) / v.length_squared()).clamp(0., 1.);
        let q = self.a + v * t;
        Vec3::new(
            q.x - p.x,
            (p.y - self.base).min(0.).abs() + (p.y - self.base - self.height).max(0.),
            q.z - p.z,
        )
        .length()
    }
    // Retain visible detail across a small camera/face-plane deadband. A group
    // already retiring must qualify as new detail rather than inheriting the
    // radius/rank bonuses reserved for the currently selected faces.
    fn priority(&self, p: Vec3, retained: bool) -> Option<f32> {
        let distance = self.distance(p);
        let facing = self.normal().dot(p - (self.a + self.b) * 0.5);
        (distance < RADIUS + if retained { 25. } else { 0. }
            && facing > if retained { -2. } else { 0. })
        .then_some(distance - if retained { 20. } else { 0. })
    }
    fn panels(&self, designs: &[Design]) -> Vec<(usize, usize, usize, Transform)> {
        let d = &designs[self.style];
        let width = self.width();
        let (bays, ground, cap, count) = dimensions(width, self.height, d);
        let h = (self.height - ground - cap) / count as f32;
        let mut rows = vec![(0, 0., ground)];
        rows.extend((0..count).map(|i| (1, ground + i as f32 * h, h)));
        rows.push((2, self.height - cap, cap));
        let n = self.normal();
        let q = Quat::from_rotation_y(n.x.atan2(n.z));
        let midpoint = (self.a + self.b) * 0.5;
        let mut result = vec![];
        for (band, y, height) in rows {
            let lod = usize::from(y >= 10. || band == 2); // source index0=LOD1,1=LOD2
            for i in 0..bays {
                // The source BUSS4 upper contains a unique RADIO sign, not a repeated floor.
                let style = if self.style == 1 && band == 1 && (y > ground + 0.1 || i != bays / 2) {
                    3
                } else {
                    self.style
                };
                let source = &designs[style];
                let sx = width / (bays as f32 * source.width);
                let front = source.lods[lod + 1].bounds[band].max[2].max(0.01);
                let sz = sx.min(0.22 / front);
                let across = -width * 0.5 + (i as f32 + 0.5) * width / bays as f32;
                result.push((
                    style,
                    lod,
                    band,
                    Transform::from_translation(
                        midpoint.with_y(self.base + y) + n * 0.025 + q * Vec3::X * across,
                    )
                    .with_rotation(q)
                    .with_scale(Vec3::new(
                        sx,
                        height / source.heights[band].max(0.001),
                        sz,
                    )),
                ));
            }
        }
        result
    }
}
/// Shared full/coarse bay/storey fitting keeps source identity aligned at distance.
pub(super) fn dimensions(width: f32, height: f32, design: &Design) -> (usize, f32, f32, usize) {
    let bays = (width / design.width).round().max(1.) as usize;
    let ground = 3.2f32.min(height * 0.6);
    let cap = design.heights[2].min(1.2);
    let count = ((height - ground - cap) / 3.2).round().max(1.) as usize;
    (bays, ground, cap, count)
}
struct Group {
    entities: Vec<Entity>,
    weight: f32,
    wanted: bool,
    triangles: usize,
}
#[derive(Component)]
pub struct AuthoredMesh {
    face: usize,
}
#[derive(Resource)]
pub struct Kit {
    designs: Vec<Design>,
    sources: Vec<[Source; 2]>,
    faces: Vec<Face>,
    cells: HashMap<(i32, i32), Vec<usize>>,
    warm: Vec<Entity>,
    ready: bool,
    loaded_at: std::time::Instant,
    ready_seconds: f64,
    elapsed: f32,
    active: BTreeMap<usize, Group>,
    pub triangles: usize,
    pub mesh_instances: usize,
    pub peak_meshes: usize,
    pub peak_triangles: usize,
}
impl Kit {
    pub fn load(server: &AssetServer, track: &Track) -> Self {
        Self::load_buildings(server, &track.metropolis.as_ref().unwrap().buildings, track)
    }
    pub fn load_buildings(
        server: &AssetServer,
        buildings: &[rr_content::metropolis::Building],
        track: &Track,
    ) -> Self {
        let catalog = Catalog::from_json(crate::world_gen::architecture::CATALOG).unwrap();
        let designs: Vec<_> = FAMILIES
            .iter()
            .map(|id| {
                catalog
                    .records
                    .iter()
                    .find(|d| d.id == *id)
                    .unwrap()
                    .clone()
            })
            .collect();
        let sources = FAMILIES
            .iter()
            .map(|id| {
                std::array::from_fn(|lod| Source {
                    file: server.load(format!("architecture://storey_kit/{id}_lod{}.glb", lod + 1)),
                    bands: None,
                })
            })
            .collect();
        let mut faces = vec![];
        let mut cells: HashMap<_, Vec<usize>> = HashMap::new();
        for b in buildings {
            let base = b
                .shape
                .vertices()
                .map(|p| track.terrain_height(p[0], p[1]))
                .fold(f32::NEG_INFINITY, f32::max)
                + 0.12;
            for ring in &b.shape.rings {
                for (a, z) in ring
                    .iter()
                    .zip(ring.iter().cycle().skip(1))
                    .take(ring.len())
                {
                    let a = Vec3::new(a[0], 0., a[1]);
                    let z = Vec3::new(z[0], 0., z[1]);
                    if a.distance(z) < 2.4 {
                        continue;
                    }
                    let i = faces.len();
                    let lo = a.min(z);
                    let hi = a.max(z);
                    for x in (lo.x / 64.).floor() as i32..=(hi.x / 64.).floor() as i32 {
                        for y in (lo.z / 64.).floor() as i32..=(hi.z / 64.).floor() as i32 {
                            cells.entry((x, y)).or_default().push(i);
                        }
                    }
                    faces.push(Face {
                        style: b.style,
                        a,
                        b: z,
                        base,
                        height: b.height,
                    });
                }
            }
        }
        Self {
            designs,
            sources,
            faces,
            cells,
            warm: vec![],
            ready: false,
            loaded_at: std::time::Instant::now(),
            ready_seconds: 0.,
            elapsed: 0.,
            active: default(),
            triangles: 0,
            mesh_instances: 0,
            peak_meshes: 0,
            peak_triangles: 0,
        }
    }
    pub fn ready(&self) -> bool {
        self.ready
    }
    pub fn audit<'a>(
        &self,
        draws: impl Iterator<Item = (&'a Mesh3d, Option<&'a Collider>)>,
        meshes: &Assets<Mesh>,
    ) -> serde_json::Value {
        let allowed: BTreeSet<_> = self
            .sources
            .iter()
            .flatten()
            .flat_map(|s| {
                s.bands
                    .as_ref()
                    .unwrap()
                    .iter()
                    .flatten()
                    .map(|p| p.mesh.id())
            })
            .collect();
        let resident_source_triangles: usize = allowed
            .iter()
            .map(|id| {
                let m = meshes.get(*id).expect("resident source primitive");
                m.indices().map_or(m.count_vertices() / 3, |i| i.len() / 3)
            })
            .sum();
        let resident_vertex_bytes: usize = allowed
            .iter()
            .map(|id| {
                meshes
                    .get(*id)
                    .unwrap()
                    .attributes()
                    .map(|(_, v)| v.get_bytes().len())
                    .sum::<usize>()
            })
            .sum();
        let mut count = 0;
        let mut triangles = 0;
        for (mesh, collider) in draws {
            assert!(collider.is_none());
            assert!(
                allowed.contains(&mesh.0.id()),
                "draw is not an original GLB primitive"
            );
            let m = meshes.get(&mesh.0).expect("resident original mesh");
            count += 1;
            triangles += m.indices().map_or(m.count_vertices() / 3, |i| i.len() / 3);
        }
        assert!(count > 0 && count <= MAX_MESHES && triangles <= MAX_TRIANGLES);
        serde_json::json!({"resident_source_primitive_meshes":allowed.len(),"resident_source_triangles":resident_source_triangles,"resident_source_vertex_attribute_bytes":resident_vertex_bytes,"queried_original_mesh_instances":count,"queried_actual_triangles":triangles,"colliders_in_cosmetic_geometry":0})
    }
    pub fn evidence(&self) -> serde_json::Value {
        let mut active_families = BTreeMap::<&str, usize>::new();
        for i in self.active.keys() {
            *active_families
                .entry(FAMILIES[self.faces[*i].style])
                .or_default() += 1;
        }
        serde_json::json!({"load_to_material_ready_seconds":self.ready_seconds,"active_family_faces":active_families,"source_families":FAMILIES,"source_lods":[1,2],"ready":self.ready,"mapped_polygon_faces":self.faces.len(),"active_faces":self.active.len(),"actual_mesh_instances":self.mesh_instances,"actual_triangles":self.triangles,"peak_meshes":self.peak_meshes,"peak_triangles":self.peak_triangles,"mesh_cap":MAX_MESHES,"triangle_cap":MAX_TRIANGLES,"physical_envelopes":"unchanged; always resident","method":"shared original GLB primitive handles, no scene/LOD file replacement"})
    }
}
#[allow(clippy::too_many_arguments)]
pub fn prepare(
    mut commands: Commands,
    mut kit: ResMut<Kit>,
    server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    detail: Res<crate::material_detail::Queue>,
) {
    if kit.ready {
        return;
    }
    for style in 0..kit.sources.len() {
        for lod in 0..2 {
            let source = &kit.sources[style][lod];
            if source.bands.is_some() {
                continue;
            }
            if let Some(bevy::asset::LoadState::Failed(e)) = server.get_load_state(source.file.id())
            {
                panic!("Required authored facade failed: {e}");
            }
            if !server.is_loaded_with_dependencies(source.file.id()) {
                return;
            }
            let Some(g) = gltfs.get(&source.file) else {
                return;
            };
            let mut bands: [Vec<Primitive>; 3] = std::array::from_fn(|_| vec![]);
            assert_eq!(g.meshes.len(), 3, "authored kit ground/upper/crown mapping");
            for (band, h) in g.meshes.iter().enumerate() {
                let Some(gm) = gltf_meshes.get(h) else {
                    return;
                };
                for p in &gm.primitives {
                    let Some(m) = meshes.get(&p.mesh) else {
                        return;
                    };
                    let triangles = m.indices().map_or(m.count_vertices() / 3, |i| i.len() / 3);
                    if triangles == 0 {
                        continue;
                    }
                    let primitive = Primitive {
                        mesh: p.mesh.clone(),
                        material: p.material.clone().expect("authored kit material"),
                        triangles,
                    };
                    let e = commands
                        .spawn((
                            Mesh3d(primitive.mesh.clone()),
                            MeshMaterial3d(primitive.material.clone()),
                            Transform::default(),
                            Visibility::Hidden,
                        ))
                        .id();
                    kit.warm.push(e);
                    bands[band].push(primitive);
                }
            }
            kit.sources[style][lod].bands = Some(bands);
        }
    }
    for source in kit.sources.iter().flatten() {
        for p in source.bands.as_ref().unwrap().iter().flatten() {
            if !detail.material_ready(p.material.id()) {
                return;
            }
            let Some(mat) = materials.get(&p.material) else {
                return;
            };
            if mat.normal_map_texture.is_some()
                && !meshes
                    .get(&p.mesh)
                    .is_some_and(|m| m.contains_attribute(Mesh::ATTRIBUTE_TANGENT))
            {
                return;
            }
        }
    }
    for e in kit.warm.drain(..) {
        commands.entity(e).despawn();
    }
    kit.ready_seconds = kit.loaded_at.elapsed().as_secs_f64();
    kit.ready = true;
}
#[allow(clippy::too_many_arguments)]
pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    cameras: Query<&Transform, With<Camera3d>>,
    mut kit: ResMut<Kit>,
) {
    if !kit.ready {
        return;
    }
    let Ok(camera) = cameras.single() else {
        return;
    };
    let dt = time.delta_secs();
    for g in kit.active.values_mut() {
        g.weight = (g.weight + if g.wanted { dt / 0.45 } else { -dt / 0.45 }).clamp(0., 1.);
    }
    let dead: Vec<_> = kit
        .active
        .iter()
        .filter(|(_, g)| !g.wanted && g.weight == 0.)
        .map(|(i, _)| *i)
        .collect();
    for i in dead {
        let g = kit.active.remove(&i).unwrap();
        for e in g.entities {
            commands.entity(e).despawn();
        }
    }
    kit.elapsed += dt;
    if kit.elapsed < 0.1 {
        return;
    }
    kit.elapsed = 0.;
    let p = camera.translation;
    let mut ids: BTreeSet<usize> = BTreeSet::new();
    let r = RADIUS + 30.;
    for x in ((p.x - r) / 64.).floor() as i32..=((p.x + r) / 64.).floor() as i32 {
        for z in ((p.z - r) / 64.).floor() as i32..=((p.z + r) / 64.).floor() as i32 {
            if let Some(v) = kit.cells.get(&(x, z)) {
                ids.extend(v);
            }
        }
    }
    let mut nearest: Vec<_> = ids
        .into_iter()
        .filter_map(|i| {
            kit.faces[i]
                .priority(p, kit.active.get(&i).is_some_and(|g| g.wanted))
                .map(|priority| (i, priority))
        })
        .collect();
    nearest.sort_by(|a, b| a.1.total_cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    for g in kit.active.values_mut() {
        g.wanted = false;
    }
    let mut used_meshes: usize = kit.active.values().map(|g| g.entities.len()).sum();
    let mut used_triangles: usize = kit.active.values().map(|g| g.triangles).sum();
    let mut added = 0;
    for (i, _) in nearest.into_iter().take(MAX_FACES) {
        if let Some(g) = kit.active.get_mut(&i) {
            g.wanted = true;
            continue;
        }
        if added >= 4 || kit.active.len() >= MAX_FACES + 8 {
            continue;
        }
        let f = &kit.faces[i];
        let panels = f.panels(&kit.designs);
        let mut cost = 0;
        let mut count = 0;
        for (style, lod, band, _) in &panels {
            let primitives = &kit.sources[*style][*lod].bands.as_ref().unwrap()[*band];
            cost += primitives.iter().map(|p| p.triangles).sum::<usize>();
            count += primitives.len();
        }
        if used_meshes + count > MAX_MESHES || used_triangles + cost > MAX_TRIANGLES {
            continue;
        }
        let mut entities = vec![];
        for (style, lod, band, t) in panels {
            for primitive in &kit.sources[style][lod].bands.as_ref().unwrap()[band] {
                entities.push(
                    commands
                        .spawn((
                            Name::new(format!("authored {} / polygon face {i}", FAMILIES[style])),
                            AuthoredMesh { face: i },
                            Mesh3d(primitive.mesh.clone()),
                            MeshMaterial3d(primitive.material.clone()),
                            t,
                            Visibility::Hidden,
                        ))
                        .id(),
                );
            }
        }
        kit.active.insert(
            i,
            Group {
                entities,
                weight: 0.,
                wanted: true,
                triangles: cost,
            },
        );
        used_meshes += count;
        used_triangles += cost;
        added += 1;
    }
    kit.mesh_instances = used_meshes;
    kit.triangles = used_triangles;
    kit.peak_meshes = kit.peak_meshes.max(used_meshes);
    kit.peak_triangles = kit.peak_triangles.max(used_triangles);
    assert!(used_meshes <= MAX_MESHES && used_triangles <= MAX_TRIANGLES);
}
/// Only visibility data changes here, after the actual render camera has propagated.
pub fn synchronize(
    kit: Res<Kit>,
    camera: Query<&GlobalTransform, With<Camera3d>>,
    mut draws: Query<(
        Entity,
        &AuthoredMesh,
        &GlobalTransform,
        &mut Visibility,
        Option<&mut bevy::camera::visibility::VisibilityRange>,
    )>,
    mut commands: Commands,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let p = camera.translation();
    for (entity, mesh, transform, mut visibility, range) in &mut draws {
        let w = kit.active.get(&mesh.face).map_or(0., |g| g.weight);
        visibility.set_if_neq(if w == 0. {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        });
        if w == 0. {
            continue;
        }
        // Settled detail is fully visible independent of camera distance. Keep a
        // stable range so thousands of static draws do not become changed render
        // inputs every frame; only an active fade needs camera-relative margins.
        let d = if w >= 1. {
            f32::MAX
        } else {
            transform.translation().distance(p)
        };
        let r = bevy::camera::visibility::VisibilityRange {
            start_margin: f32::MIN..f32::MIN,
            end_margin: (d - (1. - w) * 8.)..(d + w * 8.),
            use_aabb: false,
        };
        if let Some(mut old) = range {
            old.set_if_neq(r);
        } else {
            commands.entity(entity).insert(r);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Resource, Default)]
    struct Changes(Vec<(usize, usize)>);
    fn record_changes(
        visibility: Query<(), (With<AuthoredMesh>, Changed<Visibility>)>,
        ranges: Query<
            (),
            (
                With<AuthoredMesh>,
                Changed<bevy::camera::visibility::VisibilityRange>,
            ),
        >,
        mut changes: ResMut<Changes>,
    ) {
        changes
            .0
            .push((visibility.iter().count(), ranges.iter().count()));
    }
    #[test]
    fn settled_facades_do_not_dirty_static_render_data_when_camera_moves() {
        let mut app = App::new();
        app.init_resource::<Changes>();
        app.insert_resource(Kit {
            designs: vec![],
            sources: vec![],
            faces: vec![],
            cells: default(),
            warm: vec![],
            ready: true,
            loaded_at: std::time::Instant::now(),
            ready_seconds: 0.,
            elapsed: 0.,
            triangles: 1,
            mesh_instances: 1,
            peak_meshes: 1,
            peak_triangles: 1,
            active: BTreeMap::from([(
                0,
                Group {
                    entities: vec![],
                    weight: 1.,
                    wanted: true,
                    triangles: 1,
                },
            )]),
        });
        let camera = app
            .world_mut()
            .spawn((Camera3d::default(), GlobalTransform::IDENTITY))
            .id();
        let face = app
            .world_mut()
            .spawn((
                AuthoredMesh { face: 0 },
                GlobalTransform::IDENTITY,
                Visibility::Hidden,
            ))
            .id();
        app.add_systems(Update, (synchronize, record_changes).chain());
        app.update();
        app.world_mut()
            .entity_mut(camera)
            .insert(GlobalTransform::from_translation(Vec3::X * 20.));
        app.update();
        assert_eq!(app.world().resource::<Changes>().0[1], (0, 0));
        let range = app
            .world()
            .get::<bevy::camera::visibility::VisibilityRange>(face)
            .unwrap();
        assert_eq!(range.end_margin, f32::MAX..f32::MAX);
        app.world_mut()
            .resource_mut::<Kit>()
            .active
            .get_mut(&0)
            .unwrap()
            .weight = 0.5;
        app.update();
        let range = app
            .world()
            .get::<bevy::camera::visibility::VisibilityRange>(face)
            .unwrap();
        assert_eq!(range.end_margin, 16.0..24.0);
        assert_eq!(app.world().resource::<Changes>().0[2], (0, 1));
        app.world_mut()
            .resource_mut::<Kit>()
            .active
            .get_mut(&0)
            .unwrap()
            .weight = 0.;
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(face).unwrap(),
            Visibility::Hidden
        );
        app.world_mut()
            .entity_mut(camera)
            .insert(GlobalTransform::from_translation(Vec3::X * 40.));
        app.update();
        assert_eq!(app.world().resource::<Changes>().0[4], (0, 0));
    }
    #[test]
    fn source_storeys_cover_height_and_fit_straight_polygon_edges() {
        let c = Catalog::from_json(crate::world_gen::architecture::CATALOG).unwrap();
        let designs: Vec<_> = FAMILIES
            .iter()
            .map(|id| c.records.iter().find(|d| d.id == *id).unwrap().clone())
            .collect();
        for style in 0..FAMILIES.len() {
            for height in [6., 32., 110.] {
                let f = Face {
                    style,
                    a: Vec3::ZERO,
                    b: Vec3::new(12., 0., 0.),
                    base: 5.,
                    height,
                };
                let panels = f.panels(&designs);
                assert!(!panels.is_empty());
                if style == 1 {
                    assert_eq!(
                        panels
                            .iter()
                            .filter(|(s, _, band, _)| *s == 1 && *band == 1)
                            .count(),
                        1,
                        "unique RADIO sign must not repeat by floor or bay"
                    );
                }
                let mut top = 0f32;
                for (style, lod, band, t) in panels {
                    let d = &designs[style];
                    assert!(
                        t.translation.is_finite()
                            && t.scale.is_finite()
                            && t.scale.cmpgt(Vec3::ZERO).all()
                    );
                    assert!(t.scale.z * d.lods[lod + 1].bounds[band].max[2] <= 0.221);
                    top = top.max(t.translation.y + t.scale.y * d.heights[band]);
                    let across =
                        t.rotation.inverse() * (t.translation - (f.a + f.b) * 0.5).with_y(0.);
                    assert!(across.x.abs() + d.width * t.scale.x * 0.5 <= 6.001);
                }
                assert!((top - 5. - height).abs() < 0.001);
            }
        }
    }
    #[test]
    fn full_face_selection_survives_tangent_camera_jitter() {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(std::time::Duration::from_secs_f32(0.11));
        app.insert_resource(time);
        app.insert_resource(Kit {
            designs: vec![],
            sources: vec![],
            warm: vec![],
            faces: (0..MAX_FACES)
                .map(|_| Face {
                    style: 0,
                    a: Vec3::ZERO,
                    b: Vec3::X * 12.,
                    base: 0.,
                    height: 20.,
                })
                .collect(),
            cells: HashMap::from([((0, 0), (0..MAX_FACES).collect())]),
            ready: true,
            loaded_at: std::time::Instant::now(),
            ready_seconds: 0.,
            elapsed: 0.,
            triangles: MAX_FACES,
            mesh_instances: 0,
            peak_meshes: 0,
            peak_triangles: MAX_FACES,
            active: (0..MAX_FACES)
                .map(|i| {
                    (
                        i,
                        Group {
                            entities: vec![],
                            weight: 1.,
                            wanted: true,
                            triangles: 1,
                        },
                    )
                })
                .collect(),
        });
        let camera = app
            .world_mut()
            .spawn((Camera3d::default(), Transform::default()))
            .id();
        app.add_systems(Update, update);
        for z in [-0.1, 0.1, -0.05, 0.05, -0.1] {
            app.world_mut()
                .entity_mut(camera)
                .insert(Transform::from_xyz(6., 5., z));
            app.update();
            let kit = app.world().resource::<Kit>();
            assert_eq!(kit.active.len(), MAX_FACES);
            assert!(kit.active.values().all(|g| g.wanted && g.weight == 1.));
        }
    }
    #[test]
    fn face_selection_hysteresis_retains_visible_but_not_retiring_detail() {
        let face = Face {
            style: 0,
            a: Vec3::ZERO,
            b: Vec3::X * 12.,
            base: 0.,
            height: 20.,
        };
        for z in [-0.1, 0.1, -0.05, 0.05] {
            assert!(face.priority(Vec3::new(6., 5., z), true).is_some());
        }
        assert!(face.priority(Vec3::new(6., 5., 2.1), true).is_none());
        assert!(face.priority(Vec3::new(6., 5., 0.1), false).is_none());
        let p = Vec3::new(6., 5., -185.);
        assert!(face.priority(p, true).is_some());
        assert!(face.priority(p, false).is_none());
        let p = Vec3::new(6., 5., -100.);
        assert_eq!(face.priority(p, true), Some(80.));
        assert_eq!(face.priority(p, false), Some(100.));
    }
    #[test]
    fn outward_normals_and_segment_distance_respect_diagonal_boundaries() {
        let f = Face {
            style: 0,
            a: Vec3::ZERO,
            b: Vec3::new(6., 0., 8.),
            base: 10.,
            height: 30.,
        };
        assert!((f.width() - 10.).abs() < 0.001);
        assert!((f.normal().dot(f.b - f.a)).abs() < 0.001);
        assert!(f.normal().abs_diff_eq(Vec3::new(0.8, 0., -0.6), 0.001));
        assert!((f.distance(Vec3::new(3., 12., 4.) + f.normal() * 5.) - 5.).abs() < 0.001);
    }
}
