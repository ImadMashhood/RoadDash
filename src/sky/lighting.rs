//! Once-per-start sky IBL filtering. Original implementation of standard
//! cosine-weighted diffuse and GGX split-sum prefiltering; provenance in
//! docs/testing/next-lighting/RESEARCH.md. Never filters tonemapped display pixels.
use super::*;

const SPECULAR_SIZE: usize = 128;
const DIFFUSE_SIZE: usize = 16;
const SAMPLES: u32 = 64;

pub(super) fn legacy(value: Option<&str>) -> bool {
    value.is_some_and(|v| v.eq_ignore_ascii_case("legacy"))
}

fn linear(rgb: Vec3) -> Vec3 {
    let c = Color::srgb(rgb.x, rgb.y, rgb.z).to_linear();
    Vec3::new(c.red, c.green, c.blue)
}
fn encoded(rgb: Vec3) -> Vec3 {
    let c = Color::linear_rgb(rgb.x, rgb.y, rgb.z).to_srgba();
    Vec3::new(c.red, c.green, c.blue)
}
fn basis(n: Vec3) -> (Vec3, Vec3) {
    let axis = if n.y.abs() < 0.99 { Vec3::Y } else { Vec3::X };
    let tangent = axis.cross(n).normalize();
    (tangent, n.cross(tangent))
}
fn sample(i: u32) -> Vec2 {
    Vec2::new(
        (i as f32 + 0.5) / SAMPLES as f32,
        i.reverse_bits() as f32 / 4294967296.0,
    )
}
fn diffuse(n: Vec3, radiance: &impl Fn(Vec3) -> Vec3) -> Vec3 {
    let (t, b) = basis(n);
    let mut sum = Vec3::ZERO;
    for i in 0..SAMPLES {
        let xi = sample(i);
        let phi = std::f32::consts::TAU * xi.y;
        let r = xi.x.sqrt();
        let d = t * (r * phi.cos()) + b * (r * phi.sin()) + n * (1.0 - xi.x).sqrt();
        sum += radiance(d);
    }
    // Cosine-weighted integral divided by pi, as expected by Bevy's IBL shader.
    sum / SAMPLES as f32
}
fn specular(n: Vec3, perceptual_roughness: f32, radiance: &impl Fn(Vec3) -> Vec3) -> Vec3 {
    if perceptual_roughness == 0.0 {
        return radiance(n);
    }
    let (t, b) = basis(n);
    let alpha_squared = perceptual_roughness.powi(4);
    let mut sum = Vec3::ZERO;
    let mut weight = 0.0;
    for i in 0..SAMPLES {
        let xi = sample(i);
        let phi = std::f32::consts::TAU * xi.y;
        let cos_theta = ((1.0 - xi.x) / (1.0 + (alpha_squared - 1.0) * xi.x)).sqrt();
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
        let h = t * (sin_theta * phi.cos()) + b * (sin_theta * phi.sin()) + n * cos_theta;
        let l = 2.0 * n.dot(h) * h - n;
        let ndotl = n.dot(l).max(0.0);
        sum += radiance(l) * ndotl;
        weight += ndotl;
    }
    sum / weight.max(1e-6)
}

pub(super) fn maps(c: &rr_content::world_art::Config) -> (Image, Image) {
    // The direct sun remains owned by the shadowed DirectionalLight. The sky
    // source has a bounded disc/halo, not an unshadowed high-energy second sun.
    let radiance = |d: Vec3| {
        if d.y < 0.0 {
            let ground = Vec3::new(0.19, 0.23, 0.16);
            linear(Vec3::from_array(c.horizon_color_srgb).lerp(ground, (-d.y * 3.0).min(1.0)))
        } else {
            linear(sky_radiance(d, c))
        }
    };
    // Both skybox and environment-map shaders negate Z when sampling cubes.
    let world_direction = |d: Vec3| d * Vec3::new(1.0, 1.0, -1.0);
    let diffuse_map = cube_from(DIFFUSE_SIZE, |d| {
        encoded(diffuse(world_direction(d), &radiance))
    });
    let mut specular_map = cube_from(SPECULAR_SIZE, |_| Vec3::ZERO);
    let levels = SPECULAR_SIZE.ilog2() + 1;
    let mut data = Vec::new();
    // wgpu's default texture upload layout is layer-major, then mip-major.
    for face in 0..6 {
        for mip in 0..levels {
            let side = SPECULAR_SIZE >> mip;
            let roughness = mip as f32 / (levels - 1) as f32;
            for y in 0..side {
                for x in 0..side {
                    let d = world_direction(direction(
                        face,
                        (x as f32 + 0.5) / side as f32 * 2.0 - 1.0,
                        (y as f32 + 0.5) / side as f32 * 2.0 - 1.0,
                    ));
                    let rgb =
                        encoded(specular(d, roughness, &radiance)).clamp(Vec3::ZERO, Vec3::ONE);
                    data.extend([
                        (rgb.x * 255.0).round() as u8,
                        (rgb.y * 255.0).round() as u8,
                        (rgb.z * 255.0).round() as u8,
                        255,
                    ]);
                }
            }
        }
    }
    specular_map.data = Some(data);
    (diffuse_map, specular_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_preserve_constant_radiance_and_reduce_a_directional_peak() {
        let constant = Vec3::new(0.08, 0.35, 0.8);
        let source = |_| constant;
        for n in [
            Vec3::X,
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::Z,
            Vec3::ONE.normalize(),
        ] {
            assert!(diffuse(n, &source).distance(constant) < 1e-5);
            for roughness in [0.0, 0.1, 0.4, 0.7, 1.0] {
                assert!(specular(n, roughness, &source).distance(constant) < 1e-5);
            }
        }
        let peak = |d: Vec3| Vec3::splat(d.y.max(0.0).powi(32));
        assert!(specular(Vec3::Y, 0.8, &peak).x < specular(Vec3::Y, 0.2, &peak).x * 0.5);
        assert!(diffuse(Vec3::NEG_Y, &peak).length() < 1e-6);
    }
    #[test]
    fn legacy_selection_is_explicit_and_linear_filtering_is_not_srgb_averaging() {
        assert!(legacy(Some("legacy")));
        assert!(legacy(Some("LEGACY")));
        assert!(!legacy(None));
        assert!(!legacy(Some("balanced")));
        assert!(encoded((linear(Vec3::ZERO) + linear(Vec3::ONE)) * 0.5).x > 0.73);
    }
}
