// Original RoadDash grass wind. Uses Bevy's installed public PBR vertex interfaces.
import bevy_pbr::render::mesh_functions;
import bevy_pbr::render::forward_io::{Vertex, VertexOutput, decompress_vertex};
import bevy_pbr::render::view_transformations::position_world_to_clip;
import bevy_pbr::render::mesh_view_bindings::globals;
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> settings: vec4<f32>;
@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let local = decompress_vertex(v, v.instance_index);
    let transform = mesh_functions::get_world_from_local(v.instance_index);
    var world = mesh_functions::mesh_position_local_to_world(transform, vec4<f32>(local.position,1.0));
    let phase = world.x * 0.31 + world.z * 0.19 + globals.time * settings.y;
    let bend = local.uv.y * local.uv.y * settings.x;
    world.x += sin(phase) * bend;
    world.z += cos(phase * 0.73) * bend * 0.45;
    out.world_position = world;
    out.position = position_world_to_clip(world.xyz);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(local.normal, v.instance_index);
    out.uv = local.uv;
    @if(VERTEX_COLORS) {
    out.color = local.color;
    }
    @if(VERTEX_OUTPUT_INSTANCE_INDEX) {
    out.instance_index = v.instance_index;
    }
    @if(VISIBILITY_RANGE_DITHER) {
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(v.instance_index, transform[3]);
    }
    return out;
}
