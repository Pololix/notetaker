#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const GREY: Self = Self {
        r: 0.15,
        g: 0.15,
        b: 0.15,
        a: 1.0,
    };

    pub const DARK_GREY: Self = Self {
        r: 0.10,
        g: 0.10,
        b: 0.10,
        a: 1.0,
    };

    pub const BLACK: Self = Self {
        r: 0.05,
        g: 0.05,
        b: 0.05,
        a: 1.0,
    };
}
