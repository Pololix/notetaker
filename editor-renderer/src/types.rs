#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    // pixels
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawQuad {
    // clip position
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
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
        }
    }
}
