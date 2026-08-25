const QUAD_VERTICES = array(
    vec2(0.0, 0.0),  // top-left
    vec2(0.0, -1.0), // bottom-left
    vec2(1.0, 0.0),  // top-right
    vec2(1.0, -1.0), // bottom-right
);

const UV_VERTICES = array(
    vec2(0.0, 0.0), // top-left
    vec2(0.0, 1.0), // bottom-left
    vec2(1.0, 0.0), // top-right
    vec2(1.0, 1.0), // bottom-right
);

// group 0: text renderer
@group(0) @binding(0)
var atlas_texture: texture_2d<f32>;
@group(0) @binding(1)
var atlas_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32,
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) min_uv: vec2<f32>,
    @location(4) max_uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(QUAD_VERTICES[index] * size + position, 0.0, 1.0);
    out.color = color;
    out.uv = mix(min_uv, max_uv, UV_VERTICES[index]);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var sampled = textureSample(atlas_texture, atlas_sampler, in.uv);
    return vec4<f32>(in.color.rgb, in.color.a * sampled.r);
}
