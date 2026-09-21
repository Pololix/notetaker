use crate::{
    gpu_state::{GpuState, GpuStateError},
    primitive::{Frame, Quad},
    shapes::ShapeRenderer,
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

    viewport_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
}

impl RenderProtocol for Renderer {
    fn render(&mut self, cmds: &[RenderCommand]) {
        // stored first because different cmds may access the same id
        let mut quads_by_id: HashMap<RenderId, Vec<Quad>> = HashMap::new();

        // FIFO processing
        // everything converges into RawQuad for simple instanced rendering
        for cmd in cmds {
            match cmd {
                // global invalidations/modifications
                RenderCommand::Resize(viewport) => {
                    self.update_viewport(*viewport);
                    self.frame.full_invalidation();
                }
                RenderCommand::RedrawFrame => {
                    self.frame.full_invalidation();
                }
                RenderCommand::ClearFrame => {
                    self.frame.clear();
                    self.frame.full_invalidation();
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

        let instance_buffer_capacity = 256;
        let instance_buffer = Quad::instance_buffer(&state.device, instance_buffer_capacity);

        let viewport_buffer = state.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Viewport buffer"),
            size: std::mem::size_of::<[f32; 2]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout =
            state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Main bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: viewport_buffer.as_entire_binding(),
            }],
        });

        let shader = state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Main shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Main render pipeline layout"),
                    bind_group_layouts: &[Some(&bind_group_layout)],
                    immediate_size: 0,
                });

        let pipeline = state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Main render pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[Some(Quad::LAYOUT)],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: state.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });

        Ok(Self {
            state,
            frame: Frame::default(),

            shapes: ShapeRenderer::new(),

            pipeline,
            bind_group,

            viewport_buffer,
            instance_buffer,
            instance_buffer_capacity,
        })
    }

    fn update_viewport(&mut self, viewport: Viewport) {
        self.state.set_viewport(viewport);

        self.state.queue.write_buffer(
            &self.viewport_buffer,
            0,
            bytemuck::cast_slice(&[viewport.width as f32, viewport.height as f32]),
        );
    }

    fn draw(&mut self) {
        let Some((invalid_rect, quads)) = self.frame.get_quads(self.state.get_viewport()) else {
            return; // no redraw needed if empty invalidation
        };

        if !quads.is_empty() {
            // if necessary, update buffer capacity by creating a new buffer with a bigger capacity
            if quads.len() > self.instance_buffer_capacity {
                self.instance_buffer_capacity = quads.len().next_power_of_two();
                self.instance_buffer =
                    Quad::instance_buffer(&self.state.device, self.instance_buffer_capacity);
            }

            self.state
                .queue
                .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&quads));
        }

        let texture = match self.state.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,

            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.state
                    .surface
                    .configure(&self.state.device, &self.state.config);
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Main encoder"),
                });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        if !quads.is_empty() {
            render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
            render_pass.draw(0..4, 0..quads.len() as u32);
        }

        drop(render_pass);
        self.state.queue.submit(Some(encoder.finish()));
        self.state.queue.present(texture);
    }
}
