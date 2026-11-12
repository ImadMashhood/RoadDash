//! Original bounded vertex wind; no physics or per-frame mesh uploads.
use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct GrassWind {
    #[uniform(100)]
    pub settings: Vec4,
}
impl MaterialExtension for GrassWind {
    fn vertex_shader() -> ShaderRef {
        "shaders/grass_wind.wesl".into()
    }
}
pub type GrassMaterial = ExtendedMaterial<StandardMaterial, GrassWind>;
