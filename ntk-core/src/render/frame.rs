use crate::{Viewport, render::Quad};

#[derive(Debug)]
pub struct Frame {
    pub viewport: Viewport,

    pub quads: Vec<Quad>,
    pub text: String,
}

impl Frame {
    pub fn new(viewport: Viewport) -> Self {
        Self {
            viewport,

            quads: Vec::new(),
            text: String::new(),
        }
    }
}
