use crate::{
    gpu_state::{GpuState, GpuStateError},
    invalidation::RenderInvalidation,
    primitive::{Frame, ShapeRenderer},
};
use ntk_core::render::{RenderCommand, RenderProtocol, Viewport, types::Rect};
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

    shapes: ShapeRenderer,
}

impl RenderProtocol for Renderer {
    fn render(&mut self, cmds: &[RenderCommand]) {
        let mut invalidation = RenderInvalidation::Empty;

        // FIFO processing
        for cmd in cmds {
            match cmd {
                RenderCommand::Resize(viewport) => self.state.resize(*viewport),
                RenderCommand::RedrawFrame => invalidation.full(),
                RenderCommand::ClearFrame => {
                    self.frame.clear();
                    invalidation.full();
                }

                RenderCommand::Quad { id, rect, color } => {
                    let quad = self.shapes.plain_quad(*rect, *color);
                    self.frame.upload(*id, &[quad], &mut invalidation);

                    invalidation.partial(*rect);
                }

                RenderCommand::DocumentGrid { .. } => {}

                RenderCommand::DocumentText { .. } => {}
            }
        }

        match invalidation {
            RenderInvalidation::Empty => {}
            RenderInvalidation::Partial(rect) => self.draw_partial(rect),
            RenderInvalidation::Full => self.draw_full(),
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

            shapes: ShapeRenderer::new(),
        })
    }

    fn draw_partial(&self, rect: Rect) {}

    fn draw_full(&self) {}
}
