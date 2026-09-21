use crate::render::RenderId;
use crate::render::types::Color;
use crate::render::{Viewport, types::Rect};

#[derive(Debug, Clone)]
pub enum RenderCommand {
    Resize(Viewport),
    RedrawFrame,
    ClearFrame,

    Quad {
        id: RenderId,
        rect: Rect,
        color: Color,
    },

    DocumentGrid {
        id: RenderId,
        rect: Rect,
        color: Color,

        x_offset: f32,
        y_offset: f32,
        cell_size: f32,
    },

    DocumentText {
        id: RenderId,
        rect: Rect,
        color: Color,

        text: String,
        grid_occupancy: Vec<usize>,
        grid_width: u32,
        cell_size: f32,
    },

    Remove(RenderId),
}
