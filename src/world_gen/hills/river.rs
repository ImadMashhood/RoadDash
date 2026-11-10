//! Original shallow canyon river reach, seated on the resident collision terrain.
//! Water is presentation only: the canyon floor and accepted fall physics remain.
use super::{Track, bridge, geometry};
use crate::game::{Phase, Race};
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    light::NotShadowCaster,
    math::Affine2,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

const HALF_LENGTH: f32 = 160.;
const SEGMENTS: usize = 160;
const LANES: usize = 12;
const WATER_LIFT: f32 = 0.18;
#[derive(Component)]
pub struct CanyonWater;
#[derive(Component)]
pub struct RiverBank;
#[derive(Resource, serde::Serialize)]
pub struct RiverCensus {
    pub length_m: f32,
    pub water_triangles: usize,
    pub bank_triangles: usize,
    pub maximum_width_m: f32,
    pub maximum_longitudinal_grade: f32,
    pub surface_lift_m: f32,
    pub minimum_deck_clearance_m: f32,
}
#[derive(Resource)]
pub struct RiverFlow {
    material: Handle<StandardMaterial>,
    phase: f32,
}
struct Strip {
    vertices: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    uv: Vec<[f32; 2]>,
    colours: Vec<[f32; 4]>,
}
impl Strip {
    fn new() -> Self {
        Self {
            vertices: vec![],
            triangles: vec![],
            uv: vec![],
            colours: vec![],
        }
    }
    fn mesh(&self, origin: Vec3) -> Mesh {
        let vertices: Vec<_> = self.vertices.iter().map(|v| *v - origin).collect();
        let mut mesh = crate::environment::mesh(&vertices, &self.triangles);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uv.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colours.clone());
        mesh
    }
}
fn edge_fade(along: f32) -> f32 {
    let t = ((HALF_LENGTH - along.abs()) / 22.).clamp(0., 1.);
    t * t * (3. - 2. * t)
}
fn channel(track: &Track, along: f32) -> (Vec3, Vec3, f32) {
    let (p, q) = bridge::deck_sample(track, bridge::CENTER, 0.);
    let across = (q * Vec3::X).with_y(0.).normalize();
    let forward = (q * Vec3::Z).with_y(0.).normalize();
    let offset = 15. * (along / 62.).sin() + 5. * (along / 29.).sin();
    let derivative = 15. / 62. * (along / 62.).cos() + 5. / 29. * (along / 29.).cos();
    let tangent = (across + forward * derivative).normalize();
    let normal = tangent.cross(Vec3::Y).normalize();
    let mut c = p + across * along + forward * offset;
    c.y = geometry::ground_height(track, c.x, c.z);
    let half_width = (8.5 + 1.4 * (along / 43.).cos()) * (0.2 + 0.8 * edge_fade(along));
    (c, normal, half_width)
}
fn plan(track: &Track) -> (Strip, Strip, RiverCensus) {
    let mut water = Strip::new();
    let mut banks = Strip::new();
    let mut length = 0.;
    let mut grade = 0f32;
    let mut width = 0f32;
    let mut previous: Option<Vec3> = None;
    let deck = bridge::deck_sample(track, bridge::CENTER, 0.).0.y;
    let mut clearance = f32::INFINITY;
    for i in 0..=SEGMENTS {
        let along = -HALF_LENGTH + i as f32 * (2. * HALF_LENGTH / SEGMENTS as f32);
        let (p, normal, half) = channel(track, along);
        if let Some(old) = previous {
            let distance = p.xz().distance(old.xz());
            grade = grade.max((p.y - old.y).abs() / distance);
            length += p.distance(old);
        }
        previous = Some(p);
        width = width.max(half * 2.);
        for lane in 0..=LANES {
            let u = lane as f32 / LANES as f32;
            let mut v = p + normal * half * (u * 2. - 1.);
            v.y = geometry::ground_height(track, v.x, v.z) + WATER_LIFT;
            clearance = clearance.min(deck - v.y);
            water.vertices.push(v);
            water.uv.push([u * 2., along / 11.]);
            let edge = (u * 2. - 1.).abs();
            water.colours.push([
                0.68 + edge * 0.32,
                0.80 + edge * 0.20,
                0.88 + edge * 0.12,
                (1. - edge * 0.15) * edge_fade(along),
            ]);
        }
        for side in [-1., 1.] {
            for outside in [false, true] {
                let mut v = p + normal * side * (half + if outside { 3.5 } else { -0.4 });
                v.y = geometry::ground_height(track, v.x, v.z) + 0.035;
                banks.vertices.push(v);
                banks.uv.push([if outside { 1. } else { 0. }, along / 6.]);
                banks.colours.push(if outside {
                    [0.40, 0.34, 0.25, 0.]
                } else {
                    [0.27, 0.29, 0.23, edge_fade(along)]
                });
            }
        }
        if i > 0 {
            let k = (i * (LANES + 1)) as u32;
            let old = k - (LANES + 1) as u32;
            for lane in 0..LANES as u32 {
                water.triangles.extend([
                    [old + lane, k + lane, old + lane + 1],
                    [old + lane + 1, k + lane, k + lane + 1],
                ]);
            }
            let k = (i * 4) as u32;
            for side in 0..2 {
                let a = k - 4 + side * 2;
                let b = k + side * 2;
                // Choose winding by the actual resident world-space vertices.
                for mut tri in [[a, b, a + 1], [a + 1, b, b + 1]] {
                    let [x, y, z] = tri.map(|j| banks.vertices[j as usize]);
                    if (y - x).cross(z - x).y < 0. {
                        tri.swap(1, 2);
                    }
                    banks.triangles.push(tri);
                }
            }
        }
    }
    // The chosen normal/tangent convention is explicit, but keep the authored
    // top face valid independently of route handedness and future path changes.
    for tri in &mut water.triangles {
        let [a, b, c] = tri.map(|j| water.vertices[j as usize]);
        if (b - a).cross(c - a).y < 0. {
            tri.swap(1, 2);
        }
    }
    let census = RiverCensus {
        length_m: length,
        water_triangles: water.triangles.len(),
        bank_triangles: banks.triangles.len(),
        maximum_width_m: width,
        maximum_longitudinal_grade: grade,
        surface_lift_m: WATER_LIFT,
        minimum_deck_clearance_m: clearance,
    };
    (water, banks, census)
}
fn ripple_texture() -> Image {
    const SIZE: usize = 64;
    let mut pixels = Vec::with_capacity(SIZE * SIZE * 4);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let u = x as f32 / SIZE as f32 * std::f32::consts::TAU;
            let v = y as f32 / SIZE as f32 * std::f32::consts::TAU;
            // Periodic, warped ripples with patchy highlights rather than full
            // bright contour lines. Integer frequencies keep the repeat seam
            // continuous; low contrast keeps the small tile quiet at distance.
            let warp = (u + v * 2.).sin() * 1.3 + (u * 3. - v).cos() * 0.7;
            let crest = (v * 3. + warp).sin().max(0.).powi(6);
            let patches = ((u * 2. - v).sin() * 0.5 + (u * 3. + v * 2. + 1.7).cos() * 0.3 - 0.15)
                .max(0.)
                .powi(2);
            let broad = (u - v + (v * 2.).sin() * 0.4).sin();
            let value = (220. + 3. * broad + 14. * crest * patches) as u8;
            pixels.extend([value, value, value, 255]);
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: SIZE as u32,
            height: SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    image
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: Option<&mut Assets<Image>>,
    track: &Track,
) {
    let (water, banks, census) = plan(track);
    let origin = bridge::deck_sample(track, bridge::CENTER, 0.).0;
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.11, 0.43, 0.49, 0.94),
        base_color_texture: images.map(|images| images.add(ripple_texture())),
        perceptual_roughness: 0.19,
        reflectance: 0.7,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((
        Name::new("Hills shallow canyon river"),
        CanyonWater,
        Mesh3d(meshes.add(water.mesh(origin))),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(origin),
        NotShadowCaster,
    ));
    let bank_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.97,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((
        Name::new("Hills wet gravel river banks"),
        RiverBank,
        Mesh3d(meshes.add(banks.mesh(origin))),
        MeshMaterial3d(bank_material),
        Transform::from_translation(origin),
        NotShadowCaster,
    ));
    commands.insert_resource(RiverFlow {
        material,
        phase: 0.,
    });
    commands.insert_resource(census);
}
/// One material uniform changes; no per-frame mesh/image uploads or physics edits.
pub fn animate(
    time: Res<Time>,
    race: Option<Res<Race>>,
    flow: Option<ResMut<RiverFlow>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    if race.is_some_and(|race| race.phase == Phase::Paused) {
        return;
    }
    let (Some(mut flow), Some(mut materials)) = (flow, materials) else {
        return;
    };
    flow.phase = (flow.phase + time.delta_secs().min(0.1) * 0.09).rem_euclid(1.);
    if let Some(material) = materials.get_mut(&flow.material) {
        material.uv_transform = Affine2::from_translation(Vec2::new(0., flow.phase));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn river_reach_stays_on_gentle_resident_bed_beneath_bridge() {
        let track = Track::hills().unwrap();
        let (water, banks, census) = plan(&track);
        assert!((320. ..350.).contains(&census.length_m));
        assert!((18. ..21.).contains(&census.maximum_width_m));
        assert!(
            census.maximum_longitudinal_grade < 0.18,
            "river grade {}",
            census.maximum_longitudinal_grade
        );
        assert!(census.minimum_deck_clearance_m > 65.);
        assert_eq!(census.water_triangles, SEGMENTS * LANES * 2);
        assert_eq!(census.bank_triangles, SEGMENTS * 4);
        for (strip, lift) in [(&water, WATER_LIFT), (&banks, 0.035)] {
            for vertex in &strip.vertices {
                let ground = geometry::ground_height(&track, vertex.x, vertex.z);
                assert!((vertex.y - ground - lift).abs() < 0.001);
            }
            for triangle in &strip.triangles {
                let [a, b, c] = triangle.map(|i| strip.vertices[i as usize]);
                assert!((b - a).cross(c - a).y > 0.);
                let center = (a + b + c) / 3.;
                let ground = geometry::ground_height(&track, center.x, center.z);
                assert!(
                    (center.y - ground - lift).abs() < 0.10,
                    "triangle terrain error {}",
                    center.y - ground - lift
                );
            }
        }
    }
    #[test]
    fn river_entities_do_not_add_collision_or_a_hidden_support_plane() {
        use bevy::ecs::system::RunSystemOnce;
        let mut app = App::new();
        app.insert_resource(Track::hills().unwrap())
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>();
        app.world_mut()
            .run_system_once(
                |mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>,
                 track: Res<Track>| {
                    spawn(&mut commands, &mut meshes, &mut materials, None, &track);
                },
            )
            .unwrap();
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<CanyonWater>>()
                .iter(app.world())
                .count(),
            1
        );
        assert_eq!(
            app.world_mut()
                .query::<&bevy_rapier3d::prelude::Collider>()
                .iter(app.world())
                .count(),
            0
        );
        assert_eq!(
            app.world_mut()
                .query::<&bevy_rapier3d::prelude::RigidBody>()
                .iter(app.world())
                .count(),
            0
        );
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<crate::world::Ground>>()
                .iter(app.world())
                .count(),
            0
        );
    }
}
