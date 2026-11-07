//! Original seamless procedural sky, cloud field and bounded daylight environment.
use bevy::camera::Exposure;
use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::Skybox,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
    },
};
#[derive(Resource)]
pub struct Settings(pub rr_content::world_art::Config);
const JSON: &str = include_str!("../content/world/art.json");
fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
fn noise(p: Vec3) -> f32 {
    let f = p.floor();
    let u = p - f;
    let u = u * u * (Vec3::splat(3.0) - u * 2.0);
    let base = f.as_ivec3();
    let mut value = 0.0;
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                let at = base + IVec3::new(x, y, z);
                let mut hash = (at.x as u32).wrapping_mul(374761393)
                    ^ (at.y as u32).wrapping_mul(668265263)
                    ^ (at.z as u32).wrapping_mul(2246822519);
                hash = (hash ^ (hash >> 13)).wrapping_mul(1274126177);
                hash ^= hash >> 16;
                let weight = if x == 0 { 1.0 - u.x } else { u.x }
                    * if y == 0 { 1.0 - u.y } else { u.y }
                    * if z == 0 { 1.0 - u.z } else { u.z };
                value += (hash & 65535) as f32 / 65535.0 * weight;
            }
        }
    }
    value
}
fn sun() -> Vec3 {
    Vec3::new(-0.6, 0.85, -0.2).normalize()
}
fn direction(face: usize, u: f32, v: f32) -> Vec3 {
    match face {
        0 => Vec3::new(1.0, -v, -u),
        1 => Vec3::new(-1.0, -v, u),
        2 => Vec3::new(u, 1.0, v),
        3 => Vec3::new(u, -1.0, -v),
        4 => Vec3::new(u, -v, 1.0),
        _ => Vec3::new(-u, -v, -1.0),
    }
    .normalize()
}
fn colour(d: Vec3, coverage: f32, diffuse: bool) -> Vec3 {
    let high = Vec3::new(0.21, 0.43, 0.71);
    let horizon = Vec3::new(0.68, 0.76, 0.82);
    let mut base = horizon.lerp(high, d.y.max(0.0).sqrt());
    if d.y < 0.0 {
        return horizon.lerp(Vec3::new(0.25, 0.29, 0.27), (-d.y * 2.5).clamp(0.0, 1.0));
    }
    let toward_sun = d.dot(sun());
    let halo = ((toward_sun - 1.0) * 28.0).exp() * 0.22;
    base = base.lerp(Vec3::new(1.0, 0.89, 0.72), halo);
    let p = d * 7.0 + Vec3::new(17.0, 31.0, 8.0);
    let cloud = if diffuse {
        coverage * 0.3
    } else {
        let n = noise(p) * 0.60 + noise(p * 2.1) * 0.27 + noise(p * 4.3) * 0.13;
        smooth(0.70 - coverage * 0.32, 0.86 - coverage * 0.32, n) * smooth(0.015, 0.13, d.y)
    };
    let grey = if diffuse {
        0.9
    } else {
        0.78 + noise(p * 3.0) * 0.20
    };
    base = base.lerp(Vec3::new(grey, grey * 0.995, grey * 0.98), cloud);
    if !diffuse {
        base = base.lerp(
            Vec3::new(1.0, 0.98, 0.90),
            smooth(0.9996, 0.9999, toward_sun),
        );
    }
    base.clamp(Vec3::ZERO, Vec3::ONE)
}
fn cubemap(size: usize, coverage: f32, diffuse: bool) -> Image {
    let mut base = Vec::with_capacity(size * size * 24);
    for face in 0..6 {
        for y in 0..size {
            for x in 0..size {
                let d = direction(
                    face,
                    (x as f32 + 0.5) / size as f32 * 2.0 - 1.0,
                    (y as f32 + 0.5) / size as f32 * 2.0 - 1.0,
                );
                let c = colour(d, coverage, diffuse);
                base.extend([
                    (c.x * 255.0) as u8,
                    (c.y * 255.0) as u8,
                    (c.z * 255.0) as u8,
                    255,
                ]);
            }
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        base.clone(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let mut data = Vec::new();
    for face in base.chunks_exact(size * size * 4) {
        let mut previous = face.to_vec();
        let mut side = size;
        data.extend(&previous);
        while side > 1 {
            let next = side / 2;
            let mut mip = vec![0; next * next * 4];
            for y in 0..next {
                for x in 0..next {
                    for c in 0..4 {
                        mip[(y * next + x) * 4 + c] = ([(0, 0), (1, 0), (0, 1), (1, 1)]
                            .into_iter()
                            .map(|(dx, dy)| {
                                u32::from(previous[((y * 2 + dy) * side + x * 2 + dx) * 4 + c])
                            })
                            .sum::<u32>()
                            / 4) as u8;
                    }
                }
            }
            data.extend(&mip);
            previous = mip;
            side = next;
        }
    }
    image.data = Some(data);
    image.texture_descriptor.mip_level_count = size.ilog2() + 1;
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    image
}
#[allow(clippy::type_complexity)]
pub fn install(
    mut commands: Commands,
    images: Option<ResMut<Assets<Image>>>,
    cameras: Query<Entity, With<crate::game::ChaseCamera>>,
    mut lights: Query<(&mut DirectionalLight, &mut Transform)>,
) {
    let Some(mut images) = images else { return };
    let c = rr_content::world_art::Config::from_json(JSON).expect("validated world art");
    let specular = images.add(cubemap(c.sky_resolution as usize, c.cloud_coverage, false));
    let diffuse = images.add(cubemap(16, c.cloud_coverage, true));
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.78, 0.86, 1.0),
        brightness: c.ambient_nits,
        ..default()
    });
    for (mut light, mut transform) in &mut lights {
        light.color = Color::srgb(1.0, 0.94, 0.83);
        light.illuminance = c.sun_lux;
        *transform = Transform::from_translation(sun() * 1000.0).looking_at(Vec3::ZERO, Vec3::Y);
    }
    for entity in &cameras {
        commands.entity(entity).insert((
            Skybox {
                image: specular.clone(),
                brightness: c.sky_nits,
                ..default()
            },
            EnvironmentMapLight {
                diffuse_map: diffuse.clone(),
                specular_map: specular.clone(),
                intensity: c.environment_nits,
                ..default()
            },
            Exposure {
                ev100: c.exposure_ev100,
            },
            DistanceFog {
                color: Color::srgb(0.66, 0.73, 0.78),
                falloff: FogFalloff::Linear {
                    start: c.fog_start_m,
                    end: c.fog_end_m,
                },
                ..default()
            },
        ));
    }
    commands.insert_resource(Settings(c));
}
pub fn drift(
    time: Res<Time>,
    settings: Res<Settings>,
    mut sky: Query<(&mut Skybox, &mut EnvironmentMapLight)>,
) {
    let rotation = Quat::from_rotation_y(time.elapsed_secs() * settings.0.wind_radians_per_second);
    for (mut sky, mut env) in &mut sky {
        sky.rotation = rotation;
        env.rotation = rotation;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sky_faces_share_edge_directions_and_mips_are_complete() {
        for t in [-1.0, 0.0, 1.0] {
            assert!(direction(0, -1.0, t).distance(direction(4, 1.0, t)) < 1e-6);
        }
        let image = cubemap(16, 0.48, false);
        assert_eq!(image.texture_descriptor.size.depth_or_array_layers, 6);
        assert_eq!(image.texture_descriptor.mip_level_count, 5);
        assert_eq!(
            image.data.unwrap().len(),
            6 * (0..5).map(|i| (16usize >> i).pow(2) * 4).sum::<usize>()
        );
        assert!(rr_content::world_art::Config::from_json(JSON).is_ok());
        let bad = JSON.replace("256", "16384");
        assert!(rr_content::world_art::Config::from_json(&bad).is_err());
    }
}
