use editor_common::{Rect, Viewport};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UvCoords {
    pub min_u: f32,
    pub min_v: f32,
    pub max_u: f32,
    pub max_v: f32,
}

// gpu-backend-facing object with rendering info
// note: (x, y) is left upper corner
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub uv_coords: UvCoords,
}

// translate screen space ((0,0) at upper-left) to
// clip space ((0,0) at center and (-1,1) range on both axis)
impl Quad {
    pub fn from_rect(rect: Rect, viewport: Viewport, color: Color, uv_coords: UvCoords) -> Self {
        let (width, height) = (viewport.width as f32, viewport.height as f32);

        Self {
            x: rect.x * 2.0 / width - 1.0,
            y: 1.0 - rect.y * 2.0 / height,
            width: rect.width * 2.0 / width,
            height: rect.height * 2.0 / height,
            color,
            uv_coords,
        }
    }
}
