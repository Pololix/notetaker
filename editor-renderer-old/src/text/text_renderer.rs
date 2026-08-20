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
                max_u: (glyph_position.x + glyph_position.width) as f32 / self.atlas.max_x as f32,
                max_v: (glyph_position.y + glyph_position.height) as f32 / self.atlas.max_y as f32,
            });
            current_x += glyph.w as u32;
        }
    }
    quads
}
