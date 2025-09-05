struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
    time_of_day: f32,
    weather_intensity: f32,
    transition_factor: f32,
    is_day: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let normal = normalize(input.normal);
    
    // Basic cel-shading
    let dot_product = max(dot(normal, light_dir), 0.0);
    let step_count = 3.0;
    let stepped_light = floor(dot_product * step_count) / step_count;
    
    // Epoch of Elria color palette
    let base_color = vec3<f32>(0.18, 0.25, 0.33); // Deep blue
    let accent_color = vec3<f32>(1.0, 0.70, 0.07); // Neon orange
    
    // Day/night color mixing
    let day_factor = uniforms.is_day * uniforms.transition_factor;
    let night_factor = (1.0 - uniforms.is_day) * (1.0 - uniforms.transition_factor);
    
    let final_color = mix(
        base_color,
        accent_color,
        stepped_light * (day_factor + night_factor * 0.3)
    );
    
    // Add atmospheric effects
    let atmosphere_tint = vec3<f32>(0.5, 0.2, 0.8) * uniforms.weather_intensity;
    let atmospheric_color = mix(final_color, atmosphere_tint, 0.1);
    
    return vec4<f32>(atmospheric_color, 1.0);
}
