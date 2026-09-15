use crate::{
    gpu_state::{GpuState, GpuStateError},
    text::{TextRenderer, TextRendererError},
};
use std::sync::Arc;
use wgpu::DisplayAndWindowHandle;

pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("{0}")]
    GpuState(#[from] GpuStateError),

    #[error("{0}")]
    TextRendering(#[from] TextRendererError),
}

pub struct Renderer {
    state: GpuState,
    text_renderer: TextRenderer,
}

impl Renderer {
    pub fn new<W: DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        Ok(Self {
            state: GpuState::new(target, viewport)?,
            text_renderer: TextRenderer::new()?,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.state.config.width = width;
        self.state.config.height = height;
        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }

    fn build_ui(ctx: &egui::Context) {
        // side panel not found
    }

    pub fn render(&mut self) -> Result<(), RendererError> {}
}
