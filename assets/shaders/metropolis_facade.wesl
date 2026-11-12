import bevy_pbr::render::forward_io::{VertexOutput, FragmentOutput};
import bevy_pbr::render::pbr_fragment::pbr_input_from_standard_material;
import bevy_pbr::render::pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing};
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var facade_atlas: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var facade_sampler: sampler;

@group(constants::MATERIAL_BIND_GROUP) @binding(102) var facade_packed: texture_2d<f32>;

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> FragmentOutput {
    let tile = u32(round(input.color.r));
    let cell = vec2<f32>(f32(tile % 8u), f32(tile / 8u));
    let atlas_size = vec2<f32>(1024.0, 1152.0);
    let uv = (cell * 128.0 + 16.0 + fract(input.uv) * 96.0) / atlas_size;
    // Differentiate unwrapped facade coordinates: repeating bays must not force
    // an unrelated mip level at each window seam. Wrapped gutters protect tiles.
    let dx = dpdx(input.uv) * 96.0 / atlas_size;
    let dy = dpdy(input.uv) * 96.0 / atlas_size;
    var vertex = input;
    vertex.color = vec4<f32>(1.0);
    var pbr = pbr_input_from_standard_material(vertex, front);
    pbr.material.base_color *= textureSampleGrad(facade_atlas, facade_sampler, uv, dx, dy);
    // Linear packed source factors share the exact band UV/derivatives/gutters.
    let packed = textureSampleGrad(facade_packed, facade_sampler, uv, dx, dy);
    pbr.material.perceptual_roughness *= packed.g;
    pbr.material.metallic *= packed.b;
    var output: FragmentOutput;
    output.color = main_pass_post_lighting_processing(pbr, apply_pbr_lighting(pbr));
    return output;
}
