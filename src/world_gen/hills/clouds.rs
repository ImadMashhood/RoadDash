//! Bounded, original 3D density clouds. Presentation only; no world collision.
//! The baked display sky is the inexpensive fallback, never a claimed speedup.
use crate::track::Track;
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::NoFrustumCulling,
    image::{ImageSampler, ImageSamplerDescriptor},
    light::NotShadowCaster,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d, ShaderType, TextureDimension, TextureFormat},
    shader::ShaderRef,
};

pub const BANKS: usize = 8;
pub const STEPS: u32 = 12;
pub const RESOLUTION: usize = 48;
#[derive(Component)]
pub struct HillsCloud;
#[derive(Resource, serde::Serialize)]
pub struct CloudBudget {
    pub volumetric: bool,
    pub banks: usize,
    pub steps: u32,
    pub density_bytes: usize,
}
fn quality(value: Option<&str>, legacy: Option<&str>) -> u32 {
    match value.or(legacy) {
        Some("0" | "off") => 0,
        Some("low") => 6,
        _ => STEPS,
    }
}
pub fn steps() -> u32 {
    quality(
        std::env::var("ROADDASH_CLOUDS").ok().as_deref(),
        std::env::var("ROADDASH_HILLS_CLOUDS").ok().as_deref(),
    )
}
pub fn enabled() -> bool {
    steps() > 0
}
fn smooth(a: f32, b: f32, value: f32) -> f32 {
    let t = ((value - a) / (b - a)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}
fn noise(p: Vec3) -> f32 {
    let base = p.floor().as_ivec3();
    let t = p - p.floor();
    let t = t * t * (Vec3::splat(3.) - t * 2.);
    let mut value = 0.;
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                let v = base + IVec3::new(x, y, z);
                let mut h = (v.x as u32).wrapping_mul(374761393)
                    ^ (v.y as u32).wrapping_mul(668265263)
                    ^ (v.z as u32).wrapping_mul(2246822519);
                h = (h ^ (h >> 13)).wrapping_mul(1274126177);
                h ^= h >> 16;
                value += (h & 65535) as f32 / 65535.
                    * if x == 0 { 1. - t.x } else { t.x }
                    * if y == 0 { 1. - t.y } else { t.y }
                    * if z == 0 { 1. - t.z } else { t.z };
            }
        }
    }
    value
}
fn density(uv: Vec3) -> f32 {
    // A union of genuinely three-dimensional rising lobes, not a noise slab.
    // The shared field has a compact, uneven silhouette with a broad base and
    // distinct taller turrets; rotating/scaling it yields eight spatial clouds.
    let mut envelope = 0.0_f32;
    for (center, radius) in [
        (Vec3::new(0.50, 0.28, 0.50), Vec3::new(0.40, 0.19, 0.32)),
        (Vec3::new(0.29, 0.40, 0.47), Vec3::new(0.22, 0.25, 0.26)),
        (Vec3::new(0.70, 0.39, 0.53), Vec3::new(0.23, 0.24, 0.25)),
        (Vec3::new(0.48, 0.60, 0.49), Vec3::new(0.24, 0.31, 0.27)),
        (Vec3::new(0.57, 0.45, 0.69), Vec3::new(0.24, 0.25, 0.19)),
        (Vec3::new(0.44, 0.40, 0.29), Vec3::new(0.24, 0.24, 0.18)),
        (Vec3::new(0.77, 0.48, 0.34), Vec3::new(0.14, 0.18, 0.16)),
    ] {
        envelope = envelope.max(1. - smooth(0.62, 1.0, ((uv - center) / radius).length()));
    }
    let p = uv * Vec3::new(8.5, 9.2, 8.5) + Vec3::new(11., 7., 23.);
    let n = noise(p) * 0.72 + noise(p * 2.03) * 0.28;
    let border = uv.min(Vec3::ONE - uv);
    let edge = smooth(0.015, 0.06, border.min_element());
    ((envelope - (1. - n) * 0.25) * edge).clamp(0., 1.)
}

fn cloud_transform(track: &Track, index: usize) -> Transform {
    let station = index as f32 * track.length / (BANKS - 1) as f32;
    let (p, q) = track.sample_route(0, station, 0.);
    let side = if index.is_multiple_of(2) { -260. } else { 320. };
    let p = p + q * Vec3::X * side;
    let size = match index % 4 {
        0 => Vec3::new(850., 380., 650.),
        1 => Vec3::new(1050., 460., 700.),
        2 => Vec3::new(760., 400., 620.),
        _ => Vec3::new(950., 440., 780.),
    };
    // Keep the whole cloud above the highest road, so hills never cut into its
    // low face. Metropolis and shorter legacy courses need no 12.5 km constant.
    Transform::from_xyz(p.x, track.bounds.1.y + 470. + (index % 3) as f32 * 55., p.z)
        .with_scale(size)
        .with_rotation(Quat::from_rotation_y(index as f32 * 0.73))
}

fn texture() -> Image {
    let mut data = Vec::with_capacity(RESOLUTION.pow(3));
    for z in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            for x in 0..RESOLUTION {
                let uv = Vec3::new(x as f32, y as f32, z as f32) / (RESOLUTION - 1) as f32;
                data.push((density(uv) * 255.) as u8);
            }
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: RESOLUTION as u32,
            height: RESOLUTION as u32,
            depth_or_array_layers: RESOLUTION as u32,
        },
        TextureDimension::D3,
        data,
        TextureFormat::R8Unorm,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    image
}
#[derive(Clone, ShaderType)]
pub struct CloudSettings {
    pub local_from_world: Mat4,
    pub sun_direction: Vec4,
    pub radiance: Vec4,
}
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CloudMaterial {
    #[uniform(0)]
    pub settings: CloudSettings,
    #[texture(1, dimension = "3d")]
    #[sampler(2)]
    pub density: Handle<Image>,
}
impl Material for CloudMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hills_clouds.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
pub fn spawn(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<CloudMaterial>,
    track: &Track,
    sun: Vec3,
    sky_nits: f32,
) {
    commands.insert_resource(CloudBudget {
        volumetric: true,
        banks: BANKS,
        steps: steps(),
        density_bytes: RESOLUTION.pow(3),
    });
    let density = images.add(texture());
    let cube = meshes.add(Cuboid::default());
    for i in 0..BANKS {
        let transform = cloud_transform(track, i);
        let material = materials.add(CloudMaterial {
            settings: CloudSettings {
                local_from_world: transform.to_matrix().inverse(),
                sun_direction: sun.extend(0.),
                radiance: Vec4::new(sky_nits, steps() as f32, 0.012, 0.),
            },
            density: density.clone(),
        });
        commands.spawn((
            Name::new(format!("Spatial cumulus cloud {i}")),
            HillsCloud,
            // Sky volumes remain separate world-space meshes outside the ordinary
            // scenery culling radius. Bevy's infinite reverse-Z projection keeps
            // distant clouds visible; no global scenery distance is increased.
            NoFrustumCulling,
            Mesh3d(cube.clone()),
            MeshMaterial3d(material),
            transform,
            NotShadowCaster,
        ));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quality_controls_are_bounded_and_preserve_legacy_fallback() {
        assert_eq!(quality(None, None), 12);
        assert_eq!(quality(None, Some("0")), 0);
        assert_eq!(quality(Some("full"), Some("0")), 12);
        assert_eq!(quality(Some("low"), None), 6);
        assert_eq!(quality(Some("off"), None), 0);
        assert_eq!(quality(Some("invalid"), None), 12);
    }
    #[test]
    fn density_is_bounded_soft_at_all_box_faces_and_has_real_depth() {
        let mut filled = 0;
        let mut empty = 0;
        for z in 0..17 {
            for y in 0..17 {
                for x in 0..17 {
                    let p = Vec3::new(x as f32, y as f32, z as f32) / 16.;
                    let d = density(p);
                    assert!((0. ..=1.).contains(&d));
                    if [x, y, z].iter().any(|v| *v == 0 || *v == 16) {
                        assert_eq!(d, 0.);
                    }
                    filled += usize::from(d > 0.15);
                    empty += usize::from(d < 0.01);
                }
            }
        }
        assert!(filled > 100 && empty > 1000);
        let image = texture();
        assert_eq!(image.data.as_ref().unwrap().len(), RESOLUTION.pow(3));
        assert_eq!(image.texture_descriptor.dimension, TextureDimension::D3);
        assert_eq!(STEPS, 12);
    }
    #[test]
    fn cumulus_silhouette_rises_into_narrower_turrets() {
        let slice = |y| {
            (0..41)
                .flat_map(|z| (0..41).map(move |x| Vec3::new(x as f32 / 40., y, z as f32 / 40.)))
                .filter(|p| density(*p) > 0.12)
                .count()
        };
        let base = slice(0.30);
        let turret = slice(0.75);
        assert!(base > 200 && turret > 30 && turret < base / 2);
        assert_eq!(density(Vec3::splat(0.02)), 0.);
        assert!(density(Vec3::new(0.48, 0.65, 0.49)) > 0.5);
    }

    #[test]
    fn spatial_clouds_follow_each_course_length_with_bounded_depth_and_clear_air() {
        for track in [Track::hills().unwrap(), Track::metropolis().unwrap()] {
            for i in 0..BANKS {
                let t = cloud_transform(&track, i);
                assert!(t.translation.is_finite() && t.rotation.is_finite());
                assert!(t.translation.y - t.scale.y * 0.5 > track.bounds.1.y + 200.);
                assert!(t.scale.y / t.scale.x > 0.4);
                assert!(t.scale.x <= 1050. && t.scale.y <= 460. && t.scale.z <= 780.);
                let station = i as f32 * track.length / (BANKS - 1) as f32;
                let expected = track.sample_route(0, station, 0.).0;
                assert!((t.translation - expected).xz().length() <= 321.);
            }
        }
        assert_eq!(BANKS, 8);
        assert_eq!(RESOLUTION.pow(3), 110592);
    }
}
