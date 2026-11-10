// Original static wheat deformation; only the small damage field changes on contact.
#import bevy_pbr::{mesh_functions, forward_io::{Vertex, VertexOutput}, view_transformations::position_world_to_clip}
@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> origin: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var<storage,read> cells: array<vec4<f32>>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var<storage,read> clock: array<vec4<f32>>;
@vertex fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let transform=mesh_functions::get_world_from_local(v.instance_index);
    var world=mesh_functions::mesh_position_local_to_world(transform,vec4<f32>(v.position,1.0));
    let damage=cells[u32(v.uv.x+0.01)];
    let amount=damage.z*smoothstep(0.0,0.32,max(0.0,clock[0].x-damage.w));
    let angle=amount*1.49;
    let height=v.uv.y;
    world.x+=damage.x*sin(angle)*height;
    world.z+=damage.y*sin(angle)*height;
    world.y+=(cos(angle)-1.0)*height;
    world.x+=sin(world.x*0.31+world.z*0.19+clock[0].x*1.3)*height*height*0.035*(1.0-amount);
    out.world_position=world;
    out.position=position_world_to_clip(world.xyz);
    let normal=mesh_functions::mesh_normal_local_to_world(v.normal,v.instance_index);
    let axis=vec3<f32>(damage.y,0.0,-damage.x);
    out.world_normal=normalize(normal*cos(angle)+cross(axis,normal)*sin(angle)+axis*dot(axis,normal)*(1.0-cos(angle)));
    out.uv=vec2<f32>(0.0,height);
#ifdef VERTEX_COLORS
    out.color=v.color;
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index=v.instance_index;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither=mesh_functions::get_visibility_range_dither_level(v.instance_index,transform[3]);
#endif
    return out;
}
