use crate::{
    frame::Frame,
    gpu_state::{GpuState, GpuStateError},
    invalidation::RenderInvalidation,
};
use ntk_core::render::{RenderCommand, RenderProtocol, Viewport};
use std::sync::Arc;
use wgpu::DisplayAndWindowHandle;

#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("{0}")]
    GpuState(#[from] GpuStateError),
}

pub struct Renderer {
    state: GpuState,
    frame: Frame,
}

impl RenderProtocol for Renderer {
    fn render(&mut self, cmds: &[RenderCommand]) {
        let mut invalidation = RenderInvalidation::Empty;

        // FIFO processing
        for cmd in cmds {
            match cmd {
                RenderCommand::Resize(viewport) => self.state.resize(*viewport),
                RenderCommand::RedrawFrame => invalidation.full(),

                RenderCommand::Quad { id, rect, color } => {}

                RenderCommand::DocumentText {
                    id,
                    rect,
                    text,
                    color,
                } => {}
            }
        }
    }
}

impl Renderer {
    pub fn new<W: DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        let state = GpuState::new(target, viewport)?;

        Ok(Self {
            state,
            frame: Frame::default(),
        })
    }
}
