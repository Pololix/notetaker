use ntk_core::render::types::Rect;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawQuad {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
}

impl RawQuad {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        0 => Float32x2, // position
        1 => Float32x2, // size
        2 => Float32x4, // color
        3 => Float32x2, // uv min
        4 => Float32x2, // uv max
    ];

    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &Self::ATTRIBUTES,
    };

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.position[0],
            y: self.position[1],
            width: self.size[0],
            height: self.size[1],
        }
    }
}
