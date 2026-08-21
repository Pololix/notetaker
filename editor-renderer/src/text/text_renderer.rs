// TODO:
// - remove hardcoded fallbacks and metrics (i.e. locale, fallback path, metrics...)
// - gather dynamically the family/ies

use crate::{
    text::glyph_atlas::GlyphAtlas,
    types::{Color, Quad},
};
use editor_common::Rect;

const LOCALE: &str = "en-US";
const DEFAULT_PATH: &str =
    "/home/Pablo/repos/notetaker/defaults/fonts/JetBrainsMonoNerdFontMono-Regular.ttf";

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TextRendererError {
    #[error("Failed to load the font: {0}")]
    FontLoading(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct TextRenderer {
    atlas: GlyphAtlas,

    atlas_texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub bind_layout: wgpu::BindGroupLayout,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device) -> Result<Self, TextRendererError> {
        // load user fonts
        // if unable fallback to defaults
        // only if unable return (no way to render text without a font)
        // let mut fonts = cosmic_text::fontdb::Database::new();
        // fonts.load_font_file(DEFAULT_PATH)?;

        // let mut font_system =
        //     cosmic_text::FontSystem::new_with_locale_and_db(LOCALE.to_string(), fonts);
        // let attrs = cosmic_text::Attrs::new()
        //     .family(cosmic_text::Family::Name("JetBrainsMono Nerd Font Mono"));
        // let cache = cosmic_text::SwashCache::new();

        let atlas = GlyphAtlas::new(1024, 1024);

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
            atlas_texture,
            bind_group,
            bind_layout,
        })
    }

    // pub fn render_text(&mut self, text: &str, rect: Rect, color: Color) -> Vec<Quad> {}

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
