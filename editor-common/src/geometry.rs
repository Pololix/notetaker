#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// note: (0,0) is left uppper corner of the screen and (x, y) of the rect
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub coords: Point,
    pub width: f32,
    pub height: f32,
}
