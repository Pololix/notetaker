struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

const TRI_VERTICES = array(
  vec4(-0.3, -0.3, 0., 1.),
  vec4(0.0, 0.5, 0., 1.),
  vec4(0.3, -0.3, 0., 1.),
);

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    var out: VertexOutput;
    
    out.clip_position = TRI_VERTICES[i];

    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.3, 0.3, 1.0);
}
