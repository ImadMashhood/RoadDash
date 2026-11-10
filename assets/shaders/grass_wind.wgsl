// Original RoadDash grass wind. Uses Bevy's installed public PBR vertex interfaces.
#import bevy_pbr::{mesh_functions, forward_io::{Vertex, VertexOutput}, view_transformations::position_world_to_clip, mesh_view_bindings::globals}
@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> settings: vec4<f32>;
@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let transform = mesh_functions::get_world_from_local(v.instance_index);
    var world = mesh_functions::mesh_position_local_to_world(transform, vec4<f32>(v.position,1.0));
    let phase = world.x * 0.31 + world.z * 0.19 + globals.time * settings.y;
    let bend = v.uv.y * v.uv.y * settings.x;
    world.x += sin(phase) * bend;
    world.z += cos(phase * 0.73) * bend * 0.45;
    out.world_position = world;
    out.position = position_world_to_clip(world.xyz);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(v.normal, v.instance_index);
    out.uv = v.uv;
#ifdef VERTEX_COLORS
    out.color = v.color;
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = v.instance_index;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(v.instance_index, transform[3]);
#endif
    return out;
}
