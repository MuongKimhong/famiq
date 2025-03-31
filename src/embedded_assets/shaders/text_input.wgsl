#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(1) var<uniform> color: vec3<f32>;
@group(1) @binding(2) var texture: texture_2d<f32>;
@group(1) @binding(3) var texture_sampler: sampler;

@fragment
fn fragment(in: bevy_ui::ui_vertex_output::UiVertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(texture, texture_sampler, in.uv);

    // Example: make all text appear pure black, preserve transparency
    // let target_color = vec3<f32>(1.0, 1.0, 1.0); // black

    // if texture's alpha is greater than 0.3, boosted the alpha and use the boosted alpha.
    // the value 0.3 and 1.18 are manually tweaked. this is not a good idea. 
    // guess we'll figure it out.
    if tex.a > 0.3 {
        let boosted_alpha = clamp(tex.a * 1.18, 0.0, 1.0);
        return vec4<f32>(color, boosted_alpha);
    }

    // otherwise, return as transparent
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);

}