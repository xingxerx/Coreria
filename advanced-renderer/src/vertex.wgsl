struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

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

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let world_position = vec4<f32>(input.position, 1.0);
    out.world_position = world_position.xyz;
    out.normal = input.normal;
    out.clip_position = uniforms.view_proj * world_position;
    
    return out;
}
