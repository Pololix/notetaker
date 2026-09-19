use crate::render::Viewport;

pub trait RendererProtocol {
    fn render(&mut self, cmds: &[RenderCommand]);
}

#[derive(Debug, Clone, Copy)]
pub enum RenderCommand {
    Resize(Viewport),
    RedrawFrame,
}
