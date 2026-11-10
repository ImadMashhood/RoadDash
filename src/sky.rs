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
mod lighting;
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
fn sun(c: &rr_content::world_art::Config) -> Vec3 {
    Vec3::from_array(c.sun_direction).normalize()
}
fn tonemapping(c: &rr_content::world_art::Config) -> bevy::core_pipeline::tonemapping::Tonemapping {
    use bevy::core_pipeline::tonemapping::Tonemapping;
    match c.tonemapping {
        rr_content::world_art::ToneMapping::Reinhard => Tonemapping::Reinhard,
        rr_content::world_art::ToneMapping::SomewhatBoringDisplayTransform => {
            Tonemapping::SomewhatBoringDisplayTransform
        }
    }
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
fn colour(d: Vec3, coverage: f32, diffuse: bool, sun_direction: Vec3, horizon: Vec3) -> Vec3 {
    let high = Vec3::new(0.21, 0.43, 0.71);
    let mut base = horizon.lerp(high, d.y.max(0.0).sqrt());
    if d.y < 0.0 {
        return horizon.lerp(Vec3::new(0.25, 0.29, 0.27), (-d.y * 2.5).clamp(0.0, 1.0));
    }
    let toward_sun = d.dot(sun_direction);
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
// A display cube is separate from linear environment radiance: the supported
// non-HDR sky pass does not apply the PBR shader's tone operator.
fn display_colour(rgb: Vec3, c: &rr_content::world_art::Config) -> Vec3 {
    let linear = LinearRgba::from(Color::srgb(rgb.x, rgb.y, rgb.z));
    let v = Vec3::new(linear.red, linear.green, linear.blue)
        * c.sky_nits
        * Exposure {
            ev100: c.exposure_ev100,
        }
        .exposure();
    let mapped = match c.tonemapping {
        rr_content::world_art::ToneMapping::Reinhard => v / (Vec3::ONE + v),
        rr_content::world_art::ToneMapping::SomewhatBoringDisplayTransform => {
            // Tomasz Stachowiak's analytic operator, adapted from Bevy 0.17.3
            // (MIT/Apache-2.0); see pass10/PROVENANCE.md and third_party notices.
            let luma = v.dot(Vec3::new(0.2126, 0.7152, 0.0722));
            let chroma = Vec2::new(
                v.dot(Vec3::new(-0.1146, -0.3854, 0.5)),
                v.dot(Vec3::new(0.5, -0.4542, -0.0458)),
            );
            let blend = 1.0 - (-chroma.length() * 2.4).exp();
            let desat = ((blend - 0.7) * 0.8).max(0.0).powi(2);
            let neutral = v.lerp(Vec3::splat(luma), desat);
            let curved =
                Vec3::ONE - Vec3::new((-neutral.x).exp(), (-neutral.y).exp(), (-neutral.z).exp());
            (v * ((1.0 - (-luma).exp()) / luma.max(1e-5)).max(0.0)).lerp(curved, blend * blend)
                * 0.97
        }
    };
    let srgb = Color::linear_rgb(mapped.x, mapped.y, mapped.z).to_srgba();
    Vec3::new(srgb.red, srgb.green, srgb.blue)
}
fn sky_colour(d: Vec3, c: &rr_content::world_art::Config) -> Vec3 {
    display_colour(sky_radiance(d, c), c)
}
fn sky_radiance(d: Vec3, c: &rr_content::world_art::Config) -> Vec3 {
    let horizon = Vec3::from_array(c.horizon_color_srgb);
    let altitude = d.y.max(0.0);
    let high = Vec3::new(0.24, 0.44, 0.70);
    let mut base = horizon.lerp(high, altitude.powf(0.65));
    let solar = d.dot(sun(c));
    // Fade all directional tint/clouds to the exact shared fog colour at y=0.
    let clear_horizon = smooth(0.0, 0.12, altitude);
    base = base.lerp(
        Vec3::new(0.96, 0.83, 0.67),
        ((solar - 1.0) * 12.0).exp() * 0.20 * clear_horizon,
    );
    // Continuous direction-domain noise has no longitude wrap or planar horizon
    // stretching. Broad banks plus smaller lobes provide a bounded angular scale.
    let p = d * Vec3::new(5.0, 11.0, 5.0) + Vec3::new(17.0, 31.0, 8.0);
    let bank = noise(p) * 0.65 + noise(p * 2.03) * 0.25 + noise(p * 4.11) * 0.10;
    let cloud = smooth(
        0.61 - c.cloud_coverage * 0.19,
        0.78 - c.cloud_coverage * 0.19,
        bank,
    ) * smooth(0.025, 0.20, altitude);
    let sunward = noise(p + sun(c) * 0.22) - noise(p - sun(c) * 0.22);
    let light = (0.62 + bank * 0.27 - sunward * 0.8).clamp(0.55, 0.96);
    let cloud_colour = Vec3::new(light * 1.02, light, light * 0.97);
    base = base.lerp(cloud_colour, cloud);
    // Fine high veil, weaker and smaller than the main cloud banks.
    let veil = smooth(0.62, 0.80, noise(d * 19.0 + Vec3::new(4.0, 9.0, 21.0)))
        * smooth(0.18, 0.6, altitude)
        * 0.10
        * (1.0 - cloud);
    base = base.lerp(Vec3::new(0.83, 0.86, 0.89), veil);
    base = base.lerp(Vec3::new(1.0, 0.98, 0.90), smooth(0.9996, 0.9999, solar));
    base.clamp(Vec3::ZERO, Vec3::ONE)
}
fn cubemap(size: usize, coverage: f32, diffuse: bool, sun_direction: Vec3, horizon: Vec3) -> Image {
    cube_from(size, |d| {
        colour(d, coverage, diffuse, sun_direction, horizon)
    })
}
fn cube_from(size: usize, sample: impl Fn(Vec3) -> Vec3) -> Image {
    let mut base = Vec::with_capacity(size * size * 24);
    for face in 0..6 {
        for y in 0..size {
            for x in 0..size {
                let d = direction(
                    face,
                    (x as f32 + 0.5) / size as f32 * 2.0 - 1.0,
                    (y as f32 + 0.5) / size as f32 * 2.0 - 1.0,
                );
                let c = sample(d);
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
// Bevy applies distance fog after lighting exposure, before tonemapping. Match
// the skybox's exposed linear horizon radiance instead of mixing unscaled sRGB.
fn atmosphere(c: &rr_content::world_art::Config) -> DistanceFog {
    let horizon = LinearRgba::from(Color::srgb_from_array(c.horizon_color_srgb));
    let scale = c.sky_nits
        * Exposure {
            ev100: c.exposure_ev100,
        }
        .exposure();
    DistanceFog {
        color: Color::linear_rgb(
            horizon.red * scale,
            horizon.green * scale,
            horizon.blue * scale,
        ),
        falloff: FogFalloff::ExponentialSquared {
            density: c.fog_density_per_m,
        },
        ..default()
    }
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
    let started = std::time::Instant::now();
    let display_cube = cube_from(c.sky_resolution as usize, |d| {
        // Bevy skybox sampling reverses Z; author in world directions.
        sky_colour(d * Vec3::new(1.0, 1.0, -1.0), &c)
    });
    info!(
        "Sky display cube: {} bytes, generated in {:.3}ms; stationary world sun/cloud field",
        display_cube.data.as_ref().map_or(0, Vec::len),
        started.elapsed().as_secs_f64() * 1000.0
    );
    let display_cube = images.add(display_cube);
    let legacy = lighting::legacy(std::env::var("ROADDASH_LIGHTING").ok().as_deref());
    let (diffuse_image, specular_image) = if legacy {
        (
            cubemap(
                16,
                c.cloud_coverage,
                true,
                sun(&c),
                Vec3::from_array(c.horizon_color_srgb),
            ),
            cubemap(
                c.sky_resolution as usize,
                c.cloud_coverage,
                false,
                sun(&c),
                Vec3::from_array(c.horizon_color_srgb),
            ),
        )
    } else {
        lighting::maps(&c)
    };
    info!(
        "Lighting profile: {}; sky and environment ready in {:.3}ms; environment pixel payload {} bytes",
        if legacy {
            "legacy"
        } else {
            "balanced prefiltered IBL"
        },
        started.elapsed().as_secs_f64() * 1000.0,
        diffuse_image.data.as_ref().map_or(0, Vec::len)
            + specular_image.data.as_ref().map_or(0, Vec::len)
    );
    let diffuse = images.add(diffuse_image);
    let specular = images.add(specular_image);
    commands.insert_resource(AmbientLight {
        color: Color::srgb_from_array(c.ambient_color_srgb),
        brightness: if legacy {
            c.ambient_nits
        } else {
            c.ambient_nits * 0.35
        },
        ..default()
    });
    for (mut light, mut transform) in &mut lights {
        light.color = Color::srgb_from_array(c.sun_color_srgb);
        light.illuminance = c.sun_lux;
        *transform = Transform::from_translation(sun(&c) * 1000.0).looking_at(Vec3::ZERO, Vec3::Y);
    }
    for entity in &cameras {
        commands.entity(entity).insert((
            Skybox {
                image: display_cube.clone(),
                // Cancel only the built-in exposure multiplier: pixels already
                // encode the selected exposed/tonemapped display-linear result.
                brightness: 1.0
                    / Exposure {
                        ev100: c.exposure_ev100,
                    }
                    .exposure(),
                ..default()
            },
            EnvironmentMapLight {
                diffuse_map: diffuse.clone(),
                specular_map: specular.clone(),
                intensity: if legacy {
                    c.environment_nits
                } else {
                    c.environment_nits * 3.0
                },
                ..default()
            },
            tonemapping(&c),
            Exposure {
                ev100: c.exposure_ev100,
            },
            atmosphere(&c),
        ));
    }
    // Distant clouds are held still. Rotating the radiance environment or solar
    // disc as wind is incorrect; real cloud-only motion needs its own renderer.
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn displayed_horizon_matches_reinhard_fog_and_all_cube_edges() {
        let c = rr_content::world_art::Config::from_json(JSON).unwrap();
        let linear = atmosphere(&c).color.to_linear();
        let fog = Vec3::new(linear.red, linear.green, linear.blue);
        let expected = fog / (Vec3::ONE + fog);
        for i in 0..64 {
            let angle = i as f32 * std::f32::consts::TAU / 64.0;
            let rgb = sky_colour(Vec3::new(angle.cos(), 0.0, angle.sin()), &c);
            let actual = Color::srgb(rgb.x, rgb.y, rgb.z).to_linear();
            assert!(Vec3::new(actual.red, actual.green, actual.blue).distance(expected) < 1e-6);
        }
        for face in 0..6 {
            for edge in 0..4 {
                for t in [-0.9, -0.4, 0.0, 0.4, 0.9] {
                    let (u, v) = match edge {
                        0 => (-1.0, t),
                        1 => (1.0, t),
                        2 => (t, -1.0),
                        _ => (t, 1.0),
                    };
                    let d = direction(face, u, v);
                    let mut paired = false;
                    for other in 0..6 {
                        if other == face {
                            continue;
                        }
                        for (a, b) in [
                            (-1.0, t),
                            (1.0, t),
                            (t, -1.0),
                            (t, 1.0),
                            (-1.0, -t),
                            (1.0, -t),
                            (-t, -1.0),
                            (-t, 1.0),
                        ] {
                            let e = direction(other, a, b);
                            if d.distance(e) < 1e-6 {
                                assert!(sky_colour(d, &c).distance(sky_colour(e, &c)) < 1e-6);
                                paired = true;
                            }
                        }
                    }
                    assert!(paired);
                }
            }
        }
    }
    #[test]
    fn solar_direction_is_world_fixed_and_clouds_have_bounded_detail() {
        let c = rr_content::world_art::Config::from_json(JSON).unwrap();
        let world_sun = sun(&c);
        let cube_sun = world_sun * Vec3::new(1.0, 1.0, -1.0);
        assert!((cube_sun * Vec3::new(1.0, 1.0, -1.0)).distance(world_sun) < 1e-6);
        assert!(sky_colour(world_sun, &c).element_sum() > sky_colour(-world_sun, &c).element_sum());
        let mut clear = c.clone();
        clear.cloud_coverage = 0.0;
        let mut differences = 0;
        for face in 0..6 {
            for y in 0..32 {
                for x in 0..32 {
                    let d = direction(face, x as f32 / 16.0 - 1.0, y as f32 / 16.0 - 1.0);
                    let rgb = sky_colour(d, &c);
                    assert!(
                        rgb.is_finite() && rgb.min_element() >= 0.0 && rgb.max_element() <= 1.0
                    );
                    if rgb.distance(sky_colour(d, &clear)) > 0.02 {
                        differences += 1;
                    }
                    // No hard cloud edge or cube-wrap step at subpixel angular offsets.
                    let adjacent = (d + Vec3::splat(0.0001)).normalize();
                    assert!(rgb.distance(sky_colour(adjacent, &c)) < 0.03);
                }
            }
        }
        assert!(differences > 100);
    }
    #[test]
    fn haze_preserves_near_contrast_and_matches_exposed_horizon() {
        let c = rr_content::world_art::Config::from_json(JSON).unwrap();
        let fog = atmosphere(&c);
        let FogFalloff::ExponentialSquared { density } = fog.falloff else {
            panic!("nonlinear haze required")
        };
        let transmission = |d: f32| (-(d * density).powi(2)).exp();
        assert!(transmission(100.0) > 0.99); // traffic/opponent reading distance
        assert!(transmission(180.0) > 0.98);
        assert!(transmission(1050.0) > 0.5); // old profile was fully opaque
        assert!(transmission(1800.0) < 0.2);
        let horizon = LinearRgba::from(Color::srgb_from_array(c.horizon_color_srgb));
        let actual = LinearRgba::from(fog.color);
        let scale = c.sky_nits
            * Exposure {
                ev100: c.exposure_ev100,
            }
            .exposure();
        for (a, h) in [
            (actual.red, horizon.red),
            (actual.green, horizon.green),
            (actual.blue, horizon.blue),
        ] {
            assert!((a - h * scale).abs() < 1e-6 && a.is_finite());
        }
    }
    #[test]
    fn installation_applies_one_lighting_profile_without_changing_shadow_policy() {
        let c = rr_content::world_art::Config::from_json(JSON).unwrap();
        let mut app = App::new();
        app.init_resource::<Assets<Image>>()
            .add_systems(Startup, install);
        let camera = app.world_mut().spawn(crate::game::ChaseCamera).id();
        let light = app
            .world_mut()
            .spawn((
                DirectionalLight {
                    shadows_enabled: true,
                    shadow_depth_bias: 0.07,
                    ..default()
                },
                Transform::default(),
            ))
            .id();
        app.update();
        let world = app.world();
        let legacy = lighting::legacy(std::env::var("ROADDASH_LIGHTING").ok().as_deref());
        assert_eq!(
            world.get::<Exposure>(camera).unwrap().ev100,
            c.exposure_ev100
        );
        assert_eq!(
            *world
                .get::<bevy::core_pipeline::tonemapping::Tonemapping>(camera)
                .unwrap(),
            tonemapping(&c)
        );
        assert_eq!(
            world.resource::<AmbientLight>().color,
            Color::srgb_from_array(c.ambient_color_srgb)
        );
        assert_eq!(
            world.resource::<AmbientLight>().brightness,
            c.ambient_nits * if legacy { 1.0 } else { 0.35 }
        );
        assert_eq!(
            world.get::<DistanceFog>(camera).unwrap().color,
            atmosphere(&c).color
        );
        let actual = world.get::<DirectionalLight>(light).unwrap();
        assert_eq!(actual.color, Color::srgb_from_array(c.sun_color_srgb));
        assert_eq!(actual.illuminance, c.sun_lux);
        assert!(actual.shadows_enabled);
        assert_eq!(actual.shadow_depth_bias, 0.07);
        assert!(
            world
                .get::<Transform>(light)
                .unwrap()
                .forward()
                .as_vec3()
                .distance(-sun(&c))
                < 1e-5
        );
        assert_eq!(
            world.get::<EnvironmentMapLight>(camera).unwrap().intensity,
            c.environment_nits * if legacy { 1.0 } else { 3.0 }
        );
        assert!(
            (world.get::<Skybox>(camera).unwrap().brightness
                * Exposure {
                    ev100: c.exposure_ev100
                }
                .exposure()
                - 1.0)
                .abs()
                < 1e-6
        );
        let maps = world.resource::<Assets<Image>>();
        let environment = world.get::<EnvironmentMapLight>(camera).unwrap();
        let specular = maps.get(&environment.specular_map).unwrap();
        let diffuse = maps.get(&environment.diffuse_map).unwrap();
        assert_eq!(diffuse.texture_descriptor.size.width, 16);
        if !legacy {
            assert_eq!(specular.texture_descriptor.size.width, 128);
            assert_eq!(specular.texture_descriptor.mip_level_count, 8);
            assert_eq!(specular.data.as_ref().unwrap().len(), 524280);
            // Sky irradiance is stronger above than ground bounce below; the
            // directional fill must not collapse back into uniform ambient.
            let data = diffuse.data.as_ref().unwrap();
            let stride = 4 * (256 + 64 + 16 + 4 + 1);
            let top = &data[2 * stride + (8 * 16 + 8) * 4..][..3];
            let bottom = &data[3 * stride + (8 * 16 + 8) * 4..][..3];
            assert!(
                top.iter().map(|v| u32::from(*v)).sum::<u32>()
                    > bottom.iter().map(|v| u32::from(*v)).sum::<u32>()
            );
        }
        assert_ne!(
            world.get::<Skybox>(camera).unwrap().image,
            world
                .get::<EnvironmentMapLight>(camera)
                .unwrap()
                .specular_map
        );
    }
    #[test]
    fn sky_faces_share_edge_directions_and_mips_are_complete() {
        for t in [-1.0, 0.0, 1.0] {
            assert!(direction(0, -1.0, t).distance(direction(4, 1.0, t)) < 1e-6);
        }
        let image = cubemap(
            16,
            0.48,
            false,
            sun(&rr_content::world_art::Config::from_json(JSON).unwrap()),
            Vec3::new(0.68, 0.76, 0.82),
        );
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
