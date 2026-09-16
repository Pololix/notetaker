use crate::{
    gpu_state::{GpuState, GpuStateError},
    primitives::PrimitiveRenderer,
    text::{TextRenderer, TextRendererError},
};
use egui_wgpu::{Renderer as EguiRenderer, RendererOptions as EguiRendererOptions};
use ntk_core::{Frame, Viewport};
use std::sync::Arc;
use wgpu::DisplayAndWindowHandle;

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
    egui_renderer: EguiRenderer,
    primitive_renderer: PrimitiveRenderer,
}

impl Renderer {
    pub fn new<W: DisplayAndWindowHandle + 'static>(
        target: Arc<W>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        let state = GpuState::new(target, viewport)?;

        let text_renderer = TextRenderer::new()?;
        let egui_renderer = EguiRenderer::new(
            &state.device,
            state.config.format,
            EguiRendererOptions::default(),
        );
        let primitive_renderer = PrimitiveRenderer::new();

        Ok(Self {
            state,

            text_renderer,
            egui_renderer,
            primitive_renderer,
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

    fn build_ui(ctx: &egui::Context) {
        // side panel not found
    }

    pub fn render(&mut self, frame: Frame) -> Result<(), RendererError> {}
}
