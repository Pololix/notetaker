use crate::{
    gpu_state::{GpuState, GpuStateError},
    primitive::{Frame, RawQuad, ShapeRenderer},
};
use ntk_core::render::{RenderCommand, RenderId, RenderProtocol, Viewport};
use std::{collections::HashMap, sync::Arc};
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

    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,

    viewport_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
}

impl RenderProtocol for Renderer {
    fn render(&mut self, cmds: &[RenderCommand]) {
        // stored first because different cmds may access the same id
        let mut quads_by_id: HashMap<RenderId, Vec<RawQuad>> = HashMap::new();

        // FIFO processing
        // everything converges into RawQuad for simple instanced rendering
        for cmd in cmds {
            match cmd {
                // global invalidations/modifications
                RenderCommand::Resize(viewport) => {
                    self.state.set_viewport(*viewport);
                    self.frame.invalidate();
                }
                RenderCommand::RedrawFrame => {
                    self.frame.invalidate();
                }
                RenderCommand::ClearFrame => {
                    self.frame.clear();
                    self.frame.invalidate();
                }

                // semantic elements
                RenderCommand::Quad { id, rect, color } => {
                    let quad = self.shapes.plain_quad(*rect, *color);
                    quads_by_id.entry(*id).or_default().push(quad);
                }

                RenderCommand::DocumentGrid { .. } => {}

                RenderCommand::DocumentText { .. } => {}

                // removal
                RenderCommand::Remove(id) => self.frame.remove(*id),
            }
        }

        // upload quads to the frame
        quads_by_id.iter().for_each(|(id, quads)| {
            self.frame.upload(*id, quads);
        });

        self.draw();
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

    fn draw(&mut self) {
        let Some(quads) = self.frame.get_quads() else {
            return;
        };
    }
}
