//! Original analytic solar disc: one camera-relative sky quad, no CPU follow pass.
use bevy::{
    camera::visibility::NoFrustumCulling,
    light::NotShadowCaster,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

pub const ANGULAR_RADIUS_RADIANS: f32 = 0.6_f32.to_radians();
const DISTANCE: f32 = 100_000.;
const HALO_RADII: f32 = 5.;
#[derive(Clone, ShaderType)]
pub struct SolarSettings {
    pub direction_radius: Vec4,
    pub range_radiance: Vec4,
}
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct SolarMaterial {
    #[uniform(0)]
    pub settings: SolarSettings,
}
impl Material for SolarMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/solar_disc.wesl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/solar_disc.wesl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn depth_bias(&self) -> f32 {
        // Bevy's transparent sort uses ascending view-space Z. The shader puts
        // this sky quad behind every cloud, independently of its ECS origin.
        // This is sorting only: real reverse-Z depth still hides it behind land.
        -100_000_000.
    }
}
#[derive(Component)]
pub struct SolarDisc;
#[derive(Resource, serde::Serialize)]
pub struct SolarBudget {
    pub direction: [f32; 3],
    pub angular_diameter_degrees: f32,
    pub triangles: usize,
    pub halo_radius_multiple: f32,
}
fn settings(direction: Vec3) -> SolarSettings {
    SolarSettings {
        direction_radius: direction.normalize().extend(ANGULAR_RADIUS_RADIANS.tan()),
        range_radiance: Vec4::new(DISTANCE, 8_000_000., 120_000., HALO_RADII),
    }
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<SolarMaterial>,
    direction: Vec3,
) {
    commands.insert_resource(SolarBudget {
        direction: direction.normalize().to_array(),
        angular_diameter_degrees: ANGULAR_RADIUS_RADIANS.to_degrees() * 2.,
        triangles: 2,
        halo_radius_multiple: HALO_RADII,
    });
    commands.spawn((
        Name::new("Analytic world-aligned solar disc"),
        SolarDisc,
        NoFrustumCulling,
        NotShadowCaster,
        Mesh3d(meshes.add(Rectangle::default())),
        MeshMaterial3d(materials.add(SolarMaterial {
            settings: settings(direction),
        })),
        Transform::IDENTITY,
    ));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analytic_solar_frame_is_world_fixed_front_facing_and_translation_invariant() {
        let sun = Vec3::new(-0.78, 0.57, -0.26).normalize();
        let s = settings(sun);
        let right = sun.cross(Vec3::Y).normalize();
        let up = right.cross(sun).normalize();
        assert!(right.cross(up).distance(-sun) < 1e-6);
        for camera in [Vec3::ZERO, Vec3::new(200., 320., 12_000.)] {
            let center = camera + sun * s.range_radiance.x;
            assert!((center - camera).normalize().distance(sun) < 1e-6);
            let edge = center + right * s.range_radiance.x * s.direction_radius.w;
            let angle = (edge - camera).normalize().angle_between(sun);
            assert!((angle - ANGULAR_RADIUS_RADIANS).abs() < 0.00002);
        }
        assert!(s.range_radiance.x > 8_000.); // behind all visible cloud banks
        assert_eq!(SolarMaterial { settings: s }.depth_bias(), -100_000_000.);
    }
    #[test]
    fn solar_disc_remains_bright_at_supported_exposure_without_bloom() {
        let exposure = bevy::camera::Exposure { ev100: 12.7 }.exposure();
        let core = 8_000_000. * exposure;
        let clear_sky = 17_000. * exposure;
        let reinhard = |v: f32| v / (1. + v);
        assert!(reinhard(core) > 0.99);
        assert!(reinhard(core) - reinhard(clear_sky) > 0.1);
        assert!((ANGULAR_RADIUS_RADIANS.to_degrees() * 2. - 1.2).abs() < 1e-5);
    }
}
