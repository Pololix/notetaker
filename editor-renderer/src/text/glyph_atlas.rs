use crate::types::UvCoords;
use std::collections::HashMap;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum GlyphAtlasError {
    #[error(
        "Failed to load new glyph because of a lack of space:
        atlas is already packed or glyph is too large to fit"
    )]
    Overflow,
}

#[derive(Debug)]
pub struct GlyphAtlas {
    pub width: u32,
    pub height: u32,

    next_x: u32,
    next_y: u32,
    current_shelf_height: u32,

    pub contents: Vec<u8>,
    cache: HashMap<cosmic_text::CacheKey, UvCoords>,
}

impl GlyphAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            next_x: 0,
            next_y: 0,
            current_shelf_height: 0,

            contents: vec![0; (width * height) as usize],
            cache: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        image: &cosmic_text::SwashImage,
        key: cosmic_text::CacheKey,
    ) -> Result<UvCoords, GlyphAtlasError> {
        // early return if already mapped
        if self.cache.contains_key(&key) {
            return Ok(self.cache[&key]);
        }

        // check for space
        if self.next_x + image.placement.width > self.width {
            self.next_y += self.current_shelf_height;
            self.next_x = 0;
            self.current_shelf_height = 0;
        }
        if self.next_y + image.placement.height > self.height {
            return Err(GlyphAtlasError::Overflow);
        }
        if image.placement.height > self.current_shelf_height {
            self.current_shelf_height = image.placement.height;
        }

        // copy data to the atlas contents row by row
        for i in 0..image.placement.height {
            let dst_start = ((self.next_y + i) * self.width + self.next_x) as usize;
            let dst_end = dst_start + image.placement.width as usize;

            let src_start = (i * image.placement.width) as usize;
            let src_end = src_start + image.placement.width as usize;

            self.contents[dst_start..dst_end].copy_from_slice(&image.data[src_start..src_end]);
        }

        // map new entry and move pointer for the next write
        let min_u = self.next_x as f32 / self.width as f32;
        let min_v = self.next_y as f32 / self.height as f32;
        let max_u = min_u + (image.placement.width as f32 / self.width as f32);
        let max_v = min_v + (image.placement.height as f32 / self.height as f32);

        let uvs = UvCoords {
            min_u,
            min_v,
            max_u,
            max_v,
        };
        self.cache.insert(key, uvs);
        self.next_x += image.placement.width;

        Ok(uvs)
    }
}
