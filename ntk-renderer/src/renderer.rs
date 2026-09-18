use crate::gpu_state::{GpuState, GpuStateError};
use ntk_common::{RenderCommand, RendererProtocol, Viewport};
use std::sync::Arc;
use wgpu::DisplayAndWindowHandle;

#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("{0}")]
    GpuState(#[from] GpuStateError),
}

pub struct Renderer {
    state: GpuState,
}

impl RendererProtocol for Renderer {
    fn handle_commands(&mut self, cmds: &[RenderCommand]) {
        for cmd in cmds {
            match cmd {
                RenderCommand::Resize(viewport) => self.resize(*viewport),
                RenderCommand::Redraw(frame) => {}
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

        Ok(Self { state })
    }

    fn resize(&mut self, viewport: Viewport) {
        if viewport.width == 0 || viewport.height == 0 {
            return;
        }

        self.state.config.width = viewport.width;
        self.state.config.height = viewport.height;
        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }
}
