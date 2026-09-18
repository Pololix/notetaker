use crate::{Frame, Viewport};

pub trait RendererProtocol {
    fn handle_commands(&mut self, cmds: &[RenderCommand]);
}

pub enum RenderCommand {
    Resize(Viewport),
    Redraw(Frame),
}
