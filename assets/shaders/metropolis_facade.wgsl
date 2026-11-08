#import bevy_pbr::{forward_io::{VertexOutput, FragmentOutput}, pbr_fragment::pbr_input_from_standard_material, pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing}}
@group(#{MATERIAL_BIND_GROUP}) @binding(100) var facade_atlas: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var facade_sampler: sampler;

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> FragmentOutput {
    let tile = u32(round(input.color.r));
    let cell = vec2<f32>(f32(tile % 6u), f32(tile / 6u));
    let atlas_size = vec2<f32>(768.0, 512.0);
    let uv = (cell * 128.0 + 16.0 + fract(input.uv) * 96.0) / atlas_size;
    // Differentiate unwrapped facade coordinates: repeating bays must not force
    // an unrelated mip level at each window seam. Wrapped gutters protect tiles.
    let dx = dpdx(input.uv) * 96.0 / atlas_size;
    let dy = dpdy(input.uv) * 96.0 / atlas_size;
    var vertex = input;
    vertex.color = vec4<f32>(1.0);
    var pbr = pbr_input_from_standard_material(vertex, front);
    pbr.material.base_color *= textureSampleGrad(facade_atlas, facade_sampler, uv, dx, dy);
    var output: FragmentOutput;
    output.color = main_pass_post_lighting_processing(pbr, apply_pbr_lighting(pbr));
    return output;
}
