use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct GlyphPosition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct GlyphAtlas {
    pub max_x: u32,
    pub max_y: u32,
    next_x: u32,
    next_y: u32,
    current_shelf_height: u32,

    cache: HashMap<cosmic_text::CacheKey, GlyphPosition>,
    pub contents: Vec<u8>,
}

impl GlyphAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            max_x: width,
            max_y: height,
            next_x: 0,
            next_y: 0,
            current_shelf_height: 0,

            cache: HashMap::new(),
            contents: vec![0; (width * height) as usize],
        }
    }

    pub fn add(
        &mut self,
        key: cosmic_text::CacheKey,
        image: &cosmic_text::SwashImage,
    ) -> GlyphPosition {
        // early return if already mapped
        if self.cache.contains_key(&key) {
            return self.cache[&key];
        }

        // check for horizontal space
        if self.next_x + image.placement.width > self.max_x {
            self.next_y += self.current_shelf_height;
            self.next_x = 0;
            self.current_shelf_height = 0;
        }
        // check for vvertical space
        if self.next_y + image.placement.height > self.max_y {
            todo!("Handle vertical overflow on glyph atlas");
        }
        // overwrite current_shelf_height if necessary
        if image.placement.height > self.current_shelf_height {
            self.current_shelf_height = image.placement.height;
        }

        // insert to the cache
        let position = GlyphPosition {
            x: self.next_x,
            y: self.next_y,
            width: image.placement.width,
            height: image.placement.height,
        };
        self.cache.insert(key, position);

        // copy contents pixel by pixel
        for y in 0..image.placement.height {
            for x in 0..image.placement.width {
                let src_index = (y * image.placement.width + x) as usize;
                let dst_x = self.next_x + x;
                let dst_y = self.next_y + y;
                let dst_index = (dst_y * self.max_x + dst_x) as usize;
                self.contents[dst_index] = image.data[src_index];
            }
        }

        // move pointer
        self.next_x += image.placement.width;

        return position;
    }
}
