use crate::primitive::raw_quad::RawQuad;
use ntk_core::render::types::{Color, Rect};

#[derive(Debug)]
pub struct ShapeRenderer {}

impl ShapeRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn line() {}

    pub fn plain_quad(&self, rect: Rect, color: Color) -> RawQuad {
        RawQuad {
            position: [rect.x, rect.y],
            size: [rect.width, rect.height],
            color: [color.r, color.g, color.b, color.a],
            uv_min: [0.0, 0.0],
            uv_max: [1.0, 1.0],
        }
    }

    pub fn grid() {}
}
