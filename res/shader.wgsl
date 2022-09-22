struct CameraUniform {
    view_proj: mat4x4<f32>
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) base_color: vec4<f32>
};

struct InstanceInput {
    @location(2) matrix_0: vec4<f32>,
    @location(3) matrix_1: vec4<f32>,
    @location(4) matrix_2: vec4<f32>,
    @location(5) matrix_3: vec4<f32>,
    @location(6) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
};

@vertex
fn vs_main(
    input: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.matrix_0,
        instance.matrix_1,
        instance.matrix_2,
        instance.matrix_3
    );
    var out: VertexOutput;
    out.color = input.base_color * vec4<f32>(instance.color, 1.0);
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(input.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
