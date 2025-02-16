#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> u_color: vec3<f32>;
@group(1) @binding(1)
var<uniform> u_time: f32;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(uv - center);

    let outer_radius = 0.5;
    let inner_radius = outer_radius * 0.7;
    let edge_smoothness = 0.03;

    let fade_outer = smoothstep(outer_radius - edge_smoothness, outer_radius, dist);
    let fade_inner = smoothstep(inner_radius, inner_radius + edge_smoothness, dist);
    let alpha = fade_inner * (1.0 - fade_outer);

    if alpha < 0.01 {
        discard;
    }

    let rotation_speed = -6.0; // anti-clockwise direction
    let angle = atan2((uv - center).y, (uv - center).x) + u_time * rotation_speed;

    let pi = 3.14159265359;
    let normalized_angle = fract((angle / (2.0 * pi))); // Keeps range [0,1]

    let color_start = vec4<f32>(u_color, 1.0);
    let color_end   = vec4<f32>(u_color, 0.05);

    let blended_color = mix(color_start.rgb, color_end.rgb, normalized_angle);
    let blended_alpha = mix(color_start.a, color_end.a, normalized_angle);

    return vec4<f32>(blended_color, blended_alpha * alpha);
}
