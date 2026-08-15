use crate::TextRenderer;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct RendererState {
    _window: Arc<dyn wgpu::DisplayAndWindowHandle>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    _text: TextRenderer,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,

    time_start: std::time::Instant,
    time_buffer: wgpu::Buffer,
}

impl RendererState {
    pub fn new<T: wgpu::DisplayAndWindowHandle + 'static>(
        window: Arc<T>,
        width: u32,
        height: u32,
    ) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create a window");

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("Failed to create an adapter");

        let config = surface
            .get_default_config(&adapter, width, height)
            .expect("Failed to fecth default surface configuration");
        //let capabilities = surface.get_capabilities(&adapter);
        //then here change any necessary fields

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
                .expect("Failed to create a device");

        surface.configure(&device, &config);

        let time_start = std::time::Instant::now();
        let time_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &time_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let vertex_state = wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<RawQuad>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x2,
                    1 => Float32x2,
                    2 => Float32x4,
                ],
            })],
        };
        let fragment_state = wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bind_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        Self {
            _window: window,
            surface,
            config,
            _adapter: adapter,
            device,
            queue,

            _text: TextRenderer::new(),
            bind_group,
            pipeline,

            time_start,
            time_buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&self, quads: &[Quad]) {
        let status = self.surface.get_current_texture();

        match status {
            wgpu::CurrentSurfaceTexture::Success(surface_texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                let raw_quads: Vec<RawQuad> = quads
                    .iter()
                    .map(|q| RawQuad::from_quad(*q, self.config.width, self.config.height))
                    .collect();

                //vertices
                let instance_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::cast_slice(&raw_quads),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &surface_texture
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
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
                    render_pass.set_bind_group(0, &self.bind_group, &[]);
                    render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
                    render_pass.draw(0..4, 0..raw_quads.len() as u32);
                }

                let command = encoder.finish();

                self.queue.write_buffer(
                    &self.time_buffer,
                    0,
                    bytemuck::bytes_of(&self.time_start.elapsed().as_secs_f32()),
                );
                self.queue.submit([command]);
                self.queue.present(surface_texture);
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Outdated
            | wgpu::CurrentSurfaceTexture::Lost
            | wgpu::CurrentSurfaceTexture::Validation => panic!(),
        }
    }
}
