use crate::{
    gpu_state::{GpuState, GpuStateError},
    text::{TextRenderer, TextRendererError},
};
use egui_wgpu::{Renderer as EguiRenderer, RendererOptions as EguiRendererOptions};
use ntk_core::{
    CommandHandler,
    render::{Frame, Viewport},
};
use std::sync::Arc;
use wgpu::{CurrentSurfaceTexture, DisplayAndWindowHandle};

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

    text_renderer: TextRenderer,
    egui_renderer: EguiRenderer,
}

impl Renderer {
    pub fn new<W: DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        let state = GpuState::new(target, viewport)?;

        let text_renderer = TextRenderer::new(&state.device)?;
        let egui_renderer = EguiRenderer::new(
            &state.device,
            state.config.format,
            EguiRendererOptions::default(),
        );

        Ok(Self {
            state,

            text_renderer,
            egui_renderer,
        })
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
        let status = self.state.surface.get_current_texture();
        let texture = match status {
            CurrentSurfaceTexture::Success(texture)
            | CurrentSurfaceTexture::Suboptimal(texture) => texture,
            _ => panic!(),
        };

        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok(())
    }
}
