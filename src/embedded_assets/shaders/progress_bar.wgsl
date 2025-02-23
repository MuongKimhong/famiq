#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> u_time: f32;
@group(1) @binding(1)
var<uniform> u_color: vec3<f32>;
@group(1) @binding(2)
var<uniform> u_blend: f32; // 0.0 = No Blend, 1.0 = Blend

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let speed = 1.6;
    let shift = fract(uv.x + u_time * speed); // Moves transparency smoothly

    let alpha_start = 1.0; // Fully opaque
    let alpha_end   = 0.15; // Partially transparent

    let alpha_blend1 = smoothstep(0.0, 0.5, shift);
    let alpha_blend2 = smoothstep(0.5, 1.0, shift);

    let alpha = mix(alpha_start, alpha_end, alpha_blend1);
    let final_alpha = mix(alpha, alpha_start, alpha_blend2);

    let blended_alpha = mix(1.0, final_alpha, u_blend); // If `u_blend == 0.0`, no blend
    return vec4<f32>(u_color, blended_alpha);
}
