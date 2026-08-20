#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// cpu facing (in pixels)
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub min_u: f32,
    pub min_v: f32,
    pub max_u: f32,
    pub max_v: f32,
}

// gpu facing (in clip space)
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawQuad {
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    min_uv: [f32; 2],
    max_uv: [f32; 2],
}

impl RawQuad {
    pub fn from_quad(quad: Quad, screen_width: u32, screen_height: u32) -> Self {
        Self {
            position: [
                (quad.x as f32 / screen_width as f32) * 2.0 - 1.0,
                1.0 - (quad.y as f32 / screen_height as f32) * 2.0,
            ],
            size: [
                (quad.width as f32 / screen_width as f32) * 2.0,
                (quad.height as f32 / screen_height as f32) * 2.0,
            ],
            color: [quad.color.r, quad.color.g, quad.color.b, quad.color.a],
            min_uv: [quad.min_u, quad.min_v],
            max_uv: [quad.max_u, quad.max_v],
        }
    }
}
