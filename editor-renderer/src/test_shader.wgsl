const QUAD_VERTICES = array(
    vec2(0.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 0.0),
    vec2(1.0, 1.0),
);

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32,
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    // Ignore position/size, just prove that the vertex shader still runs.
    out.clip_position = vec4(
        QUAD_VERTICES[index] * 2.0 - 1.0,
        0.0,
        1.0
    );

    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);
}
