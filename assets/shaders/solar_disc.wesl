// Original analytic sun. Two triangles, zero textures, no extra lights or bloom.
import bevy_pbr::render::forward_io::{Vertex, decompress_vertex};
import bevy_pbr::render::mesh_view_bindings::view;
@if(TONEMAP_IN_SHADER) import bevy_core_pipeline::tonemapping::tone_mapping;
struct SolarSettings {
    direction_radius: vec4<f32>,
    range_radiance: vec4<f32>,
};
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> solar: SolarSettings;
struct SolarVertex {
    @builtin(position) position: vec4<f32>,
    @location(0) radius: vec2<f32>,
};
@vertex
fn vertex(input: Vertex) -> SolarVertex {
    let local = decompress_vertex(input, input.instance_index);
    let direction = solar.direction_radius.xyz;
    let right = normalize(cross(direction, vec3(0.0, 1.0, 0.0)));
    let up = normalize(cross(right, direction));
    let radius = local.position.xy * 2.0 * solar.range_radiance.w;
    // The final extracted render view is authoritative for this presentation.
    // No propagated transform latency and no game/body transform is written.
    let world = view.world_position + solar.range_radiance.x * (
        direction + (right * radius.x + up * radius.y) * solar.direction_radius.w
    );
    var out: SolarVertex;
    out.position = view.clip_from_world * vec4(world, 1.0);
    out.radius = radius;
    return out;
}
@fragment
fn fragment(input: SolarVertex) -> @location(0) vec4<f32> {
    let r = length(input.radius);
    let edge = max(fwidth(r), 0.002);
    let core = 1.0 - smoothstep(1.0 - edge, 1.0 + edge, r);
    let halo = exp(-r * r * 0.7) * (1.0 - smoothstep(3.8, 5.0, r)) * 0.22;
    let alpha = max(core, halo);
    if alpha < 0.001 { discard; }
    let core_colour = mix(vec3(1.0, 0.99, 0.94), vec3(1.0, 0.93, 0.73), min(r*r, 1.0) * 0.35);
    let radiance = mix(vec3(1.0, 0.73, 0.39) * solar.range_radiance.z,
        core_colour * solar.range_radiance.y, core);
    var result = vec4(radiance * view.exposure, alpha);
    @if(TONEMAP_IN_SHADER) {
    result = tone_mapping(result, view.color_grading);
    }
    return result;
}
