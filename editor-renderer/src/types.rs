#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// cpu-facing region screen coordinates
// note: (0,0) is left uppper corner
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

// gpu-backend-facing object with rendering info
pub struct Quad {}

// gpu-facing object ready to render
pub struct RawQuad {}
