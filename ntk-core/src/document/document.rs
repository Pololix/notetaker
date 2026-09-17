use crate::{
    document::{UserMode, grid::Grid},
    render::{Frame, Rgba},
};
use ropey::Rope;

const MIN_MARGIN: f32 = 10.0;
const PAGE_WIDTH: f32 = 20.0;

#[derive(Debug, Default)]
pub struct Document {
    rope: Rope,
    grid: Grid,
}

impl Document {
    pub fn render(&self, frame: &mut Frame, mode: UserMode) {
        let x = ((frame.viewport.width as f32 - PAGE_WIDTH) * 0.5).max(MIN_MARGIN);
        let y = MIN_MARGIN;
        let width = PAGE_WIDTH.min(frame.viewport.width as f32 - MIN_MARGIN * 2.0);
        let height = frame.viewport.height as f32 - MIN_MARGIN * 2.0;
        let color = Some(Rgba {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        });

        frame.quads.push(crate::render::Quad {
            x,
            y,
            width,
            height,
            color,
        });

        frame.text.push_str(&self.rope.to_string());
    }
}
