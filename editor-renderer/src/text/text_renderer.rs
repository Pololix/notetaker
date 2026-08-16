use crate::{Color, Quad, text::atlas::GlyphAtlas};

pub struct TextRenderer {
    font_system: cosmic_text::FontSystem,
    buffer: cosmic_text::Buffer,
    attributes: cosmic_text::Attrs<'static>,
    cache: cosmic_text::SwashCache,
    atlas: GlyphAtlas,
}

impl TextRenderer {
    pub fn new() -> Self {
        let mut database = cosmic_text::fontdb::Database::new();
        database
            .load_font_file(
                "/home/Pablo/repos/notetaker/defaults/fonts/JetBrainsMonoNerdFontMono-Regular.ttf",
            )
            .expect("Failed to load font");
        let mut font_system =
            cosmic_text::FontSystem::new_with_locale_and_db("en-US".to_string(), database);

        let metrics = cosmic_text::Metrics::relative(100.0, 1.2);
        let buffer = cosmic_text::Buffer::new(&mut font_system, metrics);

        let attributes = cosmic_text::Attrs::new()
            .family(cosmic_text::Family::Name("JetBrainsMono Nerd Font Mono"));
        let cache = cosmic_text::SwashCache::new();
        let atlas = GlyphAtlas::new(1024, 1024);

        Self {
            font_system,
            buffer,
            attributes,
            cache,
            atlas,
        }
    }
    pub fn layout_text(&mut self, text: &str, x: u32, y: u32, color: Color) -> Vec<Quad> {
        let mut buffer = self.buffer.borrow_with(&mut self.font_system);
        buffer.set_text(
            text,
            &self.attributes,
            cosmic_text::Shaping::Basic,
            Some(cosmic_text::Align::Left),
        );

        let glyphs: Vec<cosmic_text::LayoutGlyph> = buffer
            .layout_runs()
            .flat_map(|run| run.glyphs.iter().cloned())
            .collect();
        let mut quads: Vec<Quad> = vec![];
        let mut current_x = x;
        for glyph in glyphs {
            let (key, _offset_x, _offset_y) = cosmic_text::CacheKey::new(
                glyph.font_id,
                glyph.glyph_id,
                glyph.font_size,
                (glyph.x, glyph.y),
                glyph.font_weight,
                glyph.cache_key_flags,
            );
            if let Some(image) = self.cache.get_image(&mut self.font_system, key) {
                let glyph_position = self.atlas.add(key, image);
                quads.push(Quad {
                    x: current_x,
                    y: y,
                    width: glyph_position.width,
                    height: glyph_position.height,
                    color: color,
                    min_u: glyph_position.x as f32 / self.atlas.max_x as f32,
                    min_v: glyph_position.y as f32 / self.atlas.max_y as f32,
                    max_u: (glyph_position.x + glyph_position.width) as f32
                        / self.atlas.max_x as f32,
                    max_v: (glyph_position.y + glyph_position.height) as f32
                        / self.atlas.max_y as f32,
                });
                current_x += glyph.w as u32;
            }
        }
        quads
    }

    pub fn create_atlas_texture(&self, device: &wgpu::Device) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: self.atlas.max_x,
                height: self.atlas.max_y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    }

    pub fn write_atlas_texture(&self, queue: &wgpu::Queue, texture: &wgpu::Texture) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.atlas.contents,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.atlas.max_x),
                rows_per_image: Some(self.atlas.max_y),
            },
            wgpu::Extent3d {
                width: self.atlas.max_x,
                height: self.atlas.max_y,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn create_atlas_view(&self, texture: &wgpu::Texture) -> wgpu::TextureView {
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn create_atlas_sampler(&self, device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        })
    }
}
