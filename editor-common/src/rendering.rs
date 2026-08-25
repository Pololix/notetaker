use crate::{color::Color, geometry::Rect};

#[derive(Debug)]
pub struct RenderFrame {
    pub cmds: Vec<RenderCommand>,
}

#[derive(Debug)]
pub enum RenderCommand {
    Quad {
        surface: Rect,
        color: Color,
    },
    Text {
        surface: Rect,
        text: String,
        color: Color,
    },
}
