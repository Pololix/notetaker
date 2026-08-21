use std::collections::HashMap;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
enum GlyphAtlasError {
    #[error(
        "Failed to load new glyph because of a lack of space:
        atlas is already packed or glyph is too large to fit"
    )]
    Overflow,
}

// atlas region with (0,0) at left-lower corner
#[derive(Debug, Clone, Copy)]
struct AtlasRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct GlyphAtlas {
    pub width: u32,
    pub height: u32,

    next_x: u32,
    next_y: u32,
    current_shelf_height: u32,

    pub contents: Vec<u8>,
    cache: HashMap<cosmic_text::CacheKey, AtlasRegion>,
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
        key: cosmic_text::CacheKey,
        image: &cosmic_text::SwashImage,
    ) -> Result<AtlasRegion, GlyphAtlasError> {
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
        let position = AtlasRegion {
            x: self.next_x,
            y: self.next_y,
            width: image.placement.width,
            height: image.placement.height,
        };
        self.cache.insert(key, position);
        self.next_x += image.placement.width;

        Ok(position)
    }
}
