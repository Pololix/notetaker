use crate::{
    state::{RendererState, RendererStateError},
    text::{TextRenderer, TextRendererError},
    types::Quad,
};
use editor_common::{
    geometry::Viewport,
    rendering::{RenderCommand, RenderFrame},
};
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum RendererError {
    #[error("Failed to initialize the renderer backend: {0}")]
    RendererStateCreation(#[from] RendererStateError),

    #[error("Failed to create a text renderer: {0}")]
    TextRendererCreation(#[from] TextRendererError),
}

pub struct Renderer<'a> {
    state: RendererState,
    text: TextRenderer<'a>,

    pipeline: wgpu::RenderPipeline,
}

impl Renderer<'_> {
    pub fn new<T: wgpu::DisplayAndWindowHandle + 'static>(
        window: Arc<T>,
        viewport: Viewport,
    ) -> Result<Self, RendererError> {
        let state = RendererState::new(window, viewport)?;
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
                array_stride: std::mem::size_of::<Quad>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x2, // pos
                    1 => Float32x2, // size
                    2 => Float32x4, // color
                    3 => Float32x2, // min uvs
                    4 => Float32x2, // max uvs
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

    pub fn set_viewport(&mut self, viewport: Viewport) {
        if viewport.width == 0 || viewport.height == 0 {
            return;
        }

        self.state.config.width = viewport.width;
        self.state.config.height = viewport.height;

        self.state
            .surface
            .configure(&self.state.device, &self.state.config);
    }

    pub fn render(&mut self, frame: RenderFrame) {
        let quads: Vec<Quad> = frame
            .cmds
            .iter()
            .filter_map(|cmd| match cmd {
                RenderCommand::Text {
                    surface,
                    text,
                    color,
                } => self
                    .text
                    .render_text(
                        *surface,
                        text,
                        *color,
                        Viewport {
                            width: self.state.config.width,
                            height: self.state.config.height,
                        },
                    )
                    .ok(),

                RenderCommand::Quad { .. } => None,
            })
            .flatten()
            .collect();

        self.draw(&quads);
    }

    fn draw(&self, quads: &[Quad]) {
        let status = self.state.surface.get_current_texture();

        match status {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                let view = texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let instance_buffer =
                    self.state
                        .device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Instance buffer of quads"),
                            contents: bytemuck::cast_slice(&quads),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                let mut encoder = self
                    .state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });

                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, &self.text.bind_group, &[]);
                render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
                render_pass.draw(0..4, 0..quads.len() as u32);

                drop(render_pass);
                let command = encoder.finish();

                self.text.write_texture(&self.state.queue);
                self.state.queue.submit([command]);
                self.state.queue.present(texture);
            }
            _ => todo!("Add behaviour to status other than succesful/suboptimal"),
        }
    }
}
