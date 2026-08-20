pub fn resize(&mut self, width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }

    self.config.width = width;
    self.config.height = height;

    self.surface.configure(&self.device, &self.config);
}

pub fn render(&mut self, quads: &[Quad]) {
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
            render_pass.set_bind_group(0, &self.time_bind_group, &[]);
            render_pass.set_bind_group(1, &self.text_bind_group, &[]);
            render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
            render_pass.draw(0..4, 0..raw_quads.len() as u32);

            drop(render_pass);
            let command = encoder.finish();

            self.text
                .write_atlas_texture(&self.queue, &self.atlas_texture);
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
