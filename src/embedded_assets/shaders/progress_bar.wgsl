#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> u_time: f32;
@group(1) @binding(1)
var<uniform> u_color: vec3<f32>;
@group(1) @binding(2)
var<uniform> u_blend: f32; // 0.0 = No Blend, 1.0 = Blend
@group(1) @binding(3)
var<uniform> u_size: vec2<f32>; // w & h

// Signed Distance Function for Rounded Rectangle
// Source: https://github.com/bevyengine/bevy/blob/main/crates/bevy_ui/src/render/ui.wgsl
fn sd_rounded_box(point: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let half_size = size * 0.5;
    let corner_dist = abs(point) - half_size + radius;
    let dist_outside = length(max(corner_dist, vec2(0.0)));
    let dist_inside = min(max(corner_dist.x, corner_dist.y), 0.0);
    return dist_outside + dist_inside - radius;
}

const u_radius: f32 = 5.0;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let speed = 1.6;
    let shift = fract(uv.x + u_time * speed);

    let alpha_start = 1.0;
    let alpha_end   = 0.15;
    let alpha_blend1 = smoothstep(0.0, 0.5, shift);
    let alpha_blend2 = smoothstep(0.5, 1.0, shift);

    let alpha = mix(alpha_start, alpha_end, alpha_blend1);
    let final_alpha = mix(alpha, alpha_start, alpha_blend2);
    let local_pos = (uv - 0.5) * u_size;

    // Compute Signed Distance (Rounded Corners)
    let dist = sd_rounded_box(local_pos, u_size, u_radius);
    let inside_shape = smoothstep(0.0, -1.5, dist);
    let blended_alpha = mix(1.0, final_alpha, u_blend);

    return vec4<f32>(u_color, blended_alpha * inside_shape);
}
