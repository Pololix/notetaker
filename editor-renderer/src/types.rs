#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color(r: {}, g: {}, b: {}, a: {})",
            self.r, self.g, self.b, self.a
        )
    }
}

// cpu-facing region screen coordinates
// note: (0,0) is left uppper corner of the screen and (x, y) of the rect
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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
}

impl std::fmt::Display for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quad {{ x: {}, y: {}, width: {}, height: {}, color: {} }}",
            self.x, self.y, self.width, self.height, self.color,
        )
    }
}
// translate screen space ((0,0) at upper-left) to
// clip space ((0,0) at center and (-1,1) range on both axis)
impl Quad {
    pub fn from_rect(rect: Rect, viewport: (u32, u32), color: Color) -> Self {
        let (width, height) = (viewport.0 as f32, viewport.1 as f32);

        Self {
            x: rect.x * 2.0 / width - 1.0,
            y: 1.0 - rect.y * 2.0 / height,
            width: rect.width * 2.0 / width,
            height: rect.height * 2.0 / height,
            color,
        }
    }
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct TexturedQuad {
//     pub x: f32,
//     pub y: f32,
//     pub width: f32,
//     pub height: f32,
//     pub min_u: f32,
//     pub min_v: f32,
//     pub max_u: f32,
//     pub max_v: f32,
//     pub color: Color,
// }
//
// impl TexturedQuad {
//  pub fn from_rect()
// }
