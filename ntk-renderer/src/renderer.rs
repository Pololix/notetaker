use crate::gpu_state::{GpuState, GpuStateError};
use ntk_common::{Frame, RenderCommand, RendererProtocol, Viewport};
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

impl RendererProtocol for Renderer {
    fn render(&mut self, cmds: &[RenderCommand]) {
        let mut redraw = false;

        for cmd in cmds {
            match cmd {
                RenderCommand::Resize(viewport) => self.state.resize(*viewport),
                RenderCommand::RedrawFrame => redraw = true,
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
