use crate::{
    state::{RendererState, RendererStateError},
    text::{TextRenderer, TextRendererError},
    types::RawQuad,
};
use std::sync::Arc;
use wgpu::RenderPipeline;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
enum RendererError {
    #[error("Failed to initialize the renderer backend: {0}")]
    RendererStateCreation(#[from] RendererStateError),

    #[error("Failed to create a text renderer: {0}")]
    TextRendererCreation(#[from] TextRendererError),
}

pub struct Renderer {
    state: RendererState,
    text: TextRenderer,

    pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new<T: wgpu::DisplayAndWindowHandle + 'static>(
        window: Arc<T>,
        width: u32,
        height: u32,
    ) -> Result<Self, RendererError> {
        let state = RendererState::new(window, width, height)?;
        let text = TextRenderer::new(&state.device)?;

        let shader = state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });
        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<RawQuad>() as u64,
                // make instance buffer
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x2,
                    1 => Float32x2,
                    2 => Float32x4,
                    3 => Float32x2,
                    4 => Float32x2,
                ],
            })],
        };
        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: state.config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };

        let pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[Some(&text.bind_layout)],
                    immediate_size: 0,
                });
        let pipeline = state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: vertex_state,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(fragment_state),
                multiview_mask: None,
                cache: None,
            });

        Ok(Self {
            state,
            text,
            pipeline,
        })
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.state.config.width = width;
        self.state.config.height = height;

        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }

    //pub fn render()
}
