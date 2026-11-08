//! One facade atlas bind group for all authored styles; no per-building material swaps.
use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct FacadeAtlas {
    #[texture(100)]
    #[sampler(101)]
    pub image: Handle<Image>,
}
impl MaterialExtension for FacadeAtlas {
    fn fragment_shader() -> ShaderRef {
        "shaders/metropolis_facade.wgsl".into()
    }
}
pub type FacadeMaterial = ExtendedMaterial<StandardMaterial, FacadeAtlas>;
