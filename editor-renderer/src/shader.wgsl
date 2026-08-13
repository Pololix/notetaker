@group(0) @binding(0)
var<uniform> time: f32;

struct VertexOutput {
    @builtin(position)
    clip_position: vec4<f32>,
    @location(0)
    color: vec4<f32>,
};
const QUAD_VERTICES = array(
    vec2(-0.5, -0.5), // bottom-left
    vec2(-0.5, 0.5),  // top-left
    vec2(0.5, -0.5),  // bottom-right
    vec2(0.5, 0.5),   // top-right
);

@vertex
fn vs_main(@builtin(vertex_index)index: u32, @location(0) position: vec2<f32>, @location(1) size: vec2<f32>, @location(2) color: vec4<f32>) -> VertexOutput {
    var out: VertexOutput;

    var pos = QUAD_VERTICES[index] * size + position;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.color = color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
