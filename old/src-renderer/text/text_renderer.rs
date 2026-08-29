use crate::{
    text::glyph_atlas::{GlyphAtlas, GlyphAtlasError},
    types::{Quad, UvCoords},
};
use editor_common::{
    color::Color,
    geometry::{Point, Rect, Viewport},
};

// placeholders (fetch from config or automatically from user)
const LOCALE: &str = "en-US";
const DEFAULT_PATH: &str =
    "/home/Pablo/repos/notetaker/defaults/fonts/JetBrainsMonoNerdFontMono-Regular.ttf";
const FONT_SIZE: f32 = 15.0;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TextRendererError {
    #[error("Failed to load the font: {0}")]
    FontLoading(#[from] std::io::Error),

    #[error("Error ocurred when dealing with the atlas: {0}")]
    Atlas(#[from] GlyphAtlasError),
}

#[derive(Debug)]
pub struct TextRenderer<'a> {
    atlas: GlyphAtlas,
    font_system: cosmic_text::FontSystem,
    cache: cosmic_text::SwashCache,
    attrs: cosmic_text::Attrs<'a>,
    buffer: cosmic_text::Buffer,

    atlas_texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub bind_layout: wgpu::BindGroupLayout,

    pub plain_uvs: UvCoords, // update if the atlas ever resizes
}

impl TextRenderer<'_> {
    pub fn new(device: &wgpu::Device) -> Result<Self, TextRendererError> {
        // load user declared fonts in the config, if unable fallback to a defualt one
        let mut fonts = cosmic_text::fontdb::Database::new();
        fonts.load_font_file(DEFAULT_PATH)?;

        let mut font_system =
            cosmic_text::FontSystem::new_with_locale_and_db(LOCALE.to_string(), fonts);
        // let cache = cosmic_text::SwashCache::new();

        let atlas = GlyphAtlas::new(1024, 1024);
        let plain_uvs = atlas.get_plain_uvs();
        let cache = cosmic_text::SwashCache::new();
        let attrs = cosmic_text::Attrs::new();
        let metrics = cosmic_text::Metrics::relative(FONT_SIZE, 1.2);
        let buffer = cosmic_text::Buffer::new(&mut font_system, metrics);

        // create gpu resources
        // note: we use the r channel (u8, norm 0.0 to 1.0) to express alpha
        let atlas_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: atlas.width,
                height: atlas.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        // expect a texture (binding 0) and a sampler (binding 1)
        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("TextRenderer binding group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("TextRenderer binding group"),
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok(Self {
            atlas,
            font_system,
            cache,
            attrs,
            buffer,

            atlas_texture,
            bind_group,
            bind_layout,

            plain_uvs,
        })
    }

    pub fn render_text(
        &mut self,
        surface: Rect,
        text: &str,
        color: Color,
        viewport: Viewport,
    ) -> Result<Vec<Quad>, TextRendererError> {
        let mut quads = Vec::new();

        // config buffer
        let mut buffer = self.buffer.borrow_with(&mut self.font_system);
        buffer.set_wrap(cosmic_text::Wrap::None);
        buffer.set_size(Some(surface.width), Some(surface.height));
        buffer.set_text(
            text,
            &self.attrs,
            cosmic_text::Shaping::Advanced,
            Some(cosmic_text::Align::Left),
        );

        // collect glyphs from each line as PhysicalGlyph
        let glyphs: Vec<cosmic_text::PhysicalGlyph> = buffer
            .layout_runs()
            .flat_map(|line| {
                line.glyphs
                    .iter()
                    .map(|glyph| {
                        glyph.physical((surface.coords.x, surface.coords.y + line.line_y), 1.0)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for glyph in glyphs {
            // add bitmap to the atlas if not in already
            let image = match self.cache.get_image(&mut self.font_system, glyph.cache_key) {
                Some(image) => image,
                None => continue, // skip if no image for glyph
            };
            let uv_coords = self.atlas.add(&image, glyph.cache_key)?;

            quads.push(Quad::new(
                Rect {
                    coords: Point {
                        x: glyph.x as f32 + image.placement.left as f32,
                        y: glyph.y as f32 - image.placement.top as f32,
                    },
                    width: image.placement.width as f32,
                    height: image.placement.height as f32,
                },
                viewport,
                color,
                uv_coords,
            ));
        }

        Ok(quads)
    }

    pub fn write_texture(&self, queue: &wgpu::Queue) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.atlas.contents,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.atlas.width),
                rows_per_image: Some(self.atlas.height),
            },
            wgpu::Extent3d {
                width: self.atlas.width,
                height: self.atlas.height,
                depth_or_array_layers: 1,
            },
        );
    }
}
