@group(0) @binding(0) var scene: texture_2d<f32>;
@group(0) @binding(1) var scene_sampler: sampler;

struct RoadEffects {
    weights: vec4<f32>,
    heat_center: vec4<f32>,
}

@group(0) @binding(2) var<uniform> effect: RoadEffects;

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let seconds = effect.weights.w;
    var source = uv;
    if effect.weights.z > 0.0 {
        let offset = (uv - effect.heat_center.xy) / vec2<f32>(0.085, 0.065);
        let heat = (1.0 - smoothstep(0.12, 1.0, length(offset))) * effect.weights.z;
        if heat > 0.0 {
            let ripple = vec2<f32>(sin(uv.y * 200.0 + seconds * 21.0), cos(uv.x * 152.0 - seconds * 17.0));
            source = clamp(uv + ripple * heat * 0.0025, vec2<f32>(0.0), vec2<f32>(1.0));
        }
    }
    let sampled = textureSample(scene, scene_sampler, source);
    var color = sampled.rgb;

    if effect.weights.x > 0.0 {
        let luminance = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
        let warm = smoothstep(0.2, 1.6, luminance);
        let tint = mix(vec3<f32>(0.978, 1.0, 1.025), vec3<f32>(1.025, 1.0, 0.976), warm);
        color = mix(color, color * tint, effect.weights.x);
    }

    let center = uv - vec2<f32>(0.5, 0.44);
    if effect.weights.y > 0.0 && abs(center.x) > 0.28 {
        let border = smoothstep(0.28, 0.49, abs(center.x));
        let ray = sin(atan2(center.y, center.x) * 152.0 + 1.7 * sin(seconds * 2.4));
        let dash = sin(length(center) * 54.0 - seconds * 18.0);
        let streak = smoothstep(0.955, 0.999, ray) * smoothstep(0.21, 0.82, dash);
        let wind = border * streak * effect.weights.y * 0.082;
        color = mix(color, max(color, vec3<f32>(0.44, 0.51, 0.58)), wind);
    }
    return vec4<f32>(color, sampled.a);
}
