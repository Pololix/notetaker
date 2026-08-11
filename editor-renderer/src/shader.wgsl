@group(0) @binding(0)
var<uniform> time: f32;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

const TRI_VERTICES = array(
  vec4(-0.3, -0.3, 0., 1.),
  vec4(0.0, 0.5, 0., 1.),
  vec4(0.3, -0.3, 0., 1.),
);

@vertex
fn vs_main(@builtin(vertex_index) in: u32) -> VertexOutput {
    var out: VertexOutput;
    
    out.clip_position = TRI_VERTICES[in];

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var r: f32 = sin(time);
    var g: f32 = sin(time + 2.093);
    var b: f32 = sin(time + 4.186);
    var a: f32 = 1.0;

    return vec4<f32>(r, g, b, a);
}
