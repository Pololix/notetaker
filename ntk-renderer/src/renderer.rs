use crate::{
    gpu_state::{GpuState, GpuStateError},
    text::TextRendererError,
};
use ntk_common::{Frame, Viewport};
use std::sync::Arc;
use wgpu::DisplayAndWindowHandle;

#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("{0}")]
    GpuState(#[from] GpuStateError),

    #[error("{0}")]
    TextRendering(#[from] TextRendererError),

    #[error("Failed to render frame due to an invalid viewport input")]
    InvalidViewport,
}

pub struct Renderer {
    state: GpuState,
}

impl Renderer {
    pub fn new<W: DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        let state = GpuState::new(target, viewport)?;

        Ok(Self { state })
    }

    pub fn resize(&mut self, viewport: Viewport) {
        if viewport.width == 0 || viewport.height == 0 {
            return;
        }

        self.state.config.width = viewport.width;
        self.state.config.height = viewport.height;
        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }

    pub fn render(&mut self, frame: Frame) -> Result<(), RendererError> {
        todo!("Render");
    }
}
