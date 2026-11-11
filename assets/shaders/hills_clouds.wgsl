// Original bounded cloud raymarch: one shared 3D density texture, twelve steps.
#import bevy_pbr::{forward_io::VertexOutput, mesh_view_bindings::view}
#import bevy_core_pipeline::tonemapping::tone_mapping
struct Settings {
    local_from_world: mat4x4<f32>,
    sun_direction: vec4<f32>,
    radiance: vec4<f32>,
};
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> cloud: Settings;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var density_texture: texture_3d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var density_sampler: sampler;
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let direction = normalize(input.world_position.xyz - view.world_position);
    let origin = (cloud.local_from_world * vec4(view.world_position, 1.0)).xyz;
    let ray = (cloud.local_from_world * vec4(direction, 0.0)).xyz;
    let safe_ray = select(vec3(-1.0), vec3(1.0), ray >= vec3(0.0)) * max(abs(ray), vec3(0.0000001));
    let a = (-vec3(0.5) - origin) / safe_ray;
    let b = (vec3(0.5) - origin) / safe_ray;
    let near = min(a, b);
    let far = max(a, b);
    let start = max(0.0, max(near.x, max(near.y, near.z)));
    let end = min(far.x, min(far.y, far.z));
    if end <= start { discard; }
    let step_length = (end - start) / cloud.radiance.y;
    let sunlight = (cloud.local_from_world * vec4(cloud.sun_direction.xyz * 70.0, 0.0)).xyz;
    var transmittance = 1.0;
    var colour = vec3(0.0);
    for (var i = 0u; i < 12u; i += 1u) {
        if f32(i) >= cloud.radiance.y { break; }
        let p = origin + ray * (start + (f32(i) + 0.5) * step_length) + vec3(0.5);
        let density = textureSampleLevel(density_texture, density_sampler, p, 0.0).r;
        let sun_density = textureSampleLevel(density_texture, density_sampler, p + sunlight, 0.0).r;
        // One directional density probe per view step approximates self-shadowing.
        // The warm lit turrets and cooler shaded belly expose volume thickness.
        let ambient = mix(0.30, 0.58, smoothstep(0.16, 0.76, p.y));
        let direct = 0.42 * exp(-sun_density * 2.2);
        let shade = clamp(ambient + direct + (density - sun_density) * 0.35, 0.28, 1.04);
        let lit_colour = vec3(1.0, 0.98, 0.93) * shade
            + vec3(0.35, 0.47, 0.62) * (1.0 - min(shade, 1.0)) * 0.18;
        let opacity = 1.0 - exp(-density * cloud.radiance.z * step_length);
        colour += transmittance * opacity * lit_colour;
        transmittance *= 1.0 - opacity;
        if transmittance < 0.015 { break; }
    }
    // Fade very distant silhouettes gradually into the atmosphere, retaining
    // parallax and a finite world position instead of painting the display sky.
    let range_fade = 1.0 - smoothstep(4500.0, 8000.0, start);
    let accumulated_alpha = 1.0 - transmittance;
    let alpha = accumulated_alpha * range_fade;
    if alpha < 0.002 { discard; }
    // Match the scene's existing exposure/tonemap; never change its sun or shadows.
    var result = vec4(colour / max(accumulated_alpha, 0.0001) * cloud.radiance.x * view.exposure, alpha);
#ifdef TONEMAP_IN_SHADER
    result = tone_mapping(result, view.color_grading);
#endif
    return result;
}
