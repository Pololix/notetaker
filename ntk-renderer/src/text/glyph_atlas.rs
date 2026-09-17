use crate::uv::UvCoords;
use cosmic_text::{CacheKey, SwashImage};
use std::collections::HashMap;

const ATLAS_SIZE: u32 = 1024;

#[derive(Debug, thiserror::Error)]
pub enum GlyphAtlasError {
    #[error("")]
    Overflow,
}

#[derive(Debug)]
pub struct GlyphAtlas {
    width: u32,
    height: u32,
    texture: wgpu::Texture,
    texture_contents: Vec<u8>,
    dirty: bool,
    cache: HashMap<CacheKey, UvCoords>,

    next_x: u32,
    next_y: u32,
    row_height: u32,
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device) -> Self {
        // empty atlas texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph atlas texture"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Self {
            width: ATLAS_SIZE,
            height: ATLAS_SIZE,
            texture,
            texture_contents: vec![0; (ATLAS_SIZE * ATLAS_SIZE * 4) as usize],
            dirty: false,
            cache: HashMap::new(),

            next_x: 0,
            next_y: 0,
            row_height: 0,
        }
    }

    pub fn get_glyph_uv(
        &mut self,
        key: CacheKey,
        image: SwashImage,
    ) -> Result<UvCoords, GlyphAtlasError> {
        if let Some(cached_uv) = self.cache.get(&key) {
            return Ok(*cached_uv);
        }

        if self.next_x + image.placement.width > self.width {
            self.next_x = 0;
            self.next_y += self.row_height;
            self.row_height = 0;
        }

        todo!("copy image data to contents");

        if self.next_y + image.placement.height > self.height {
            return Err(GlyphAtlasError::Overflow);
        }

        self.next_x += image.placement.width;
        self.row_height = self.row_height.max(image.placement.height);

        todo!("return normalized uvs");
    }

    pub fn write_texture(&self, queue: &wgpu::Queue) {
        if !self.dirty {
            return;
        }

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            &self.texture_contents,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.width * 4),
                rows_per_image: Some(self.height),
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
    }
}
