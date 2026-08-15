use crate::text::atlas;

pub struct TextRenderer {
    font_system: cosmic_text::FontSystem,
    cache: cosmic_text::SwashCache,
    atlas: GlyphAtlas,
}

impl TextRenderer {
    pub fn new() -> Self {
        // se load fonts dir
        let mut database = cosmic_text::fontdb::Database::new();
        database
            .load_font_file(
                "/home/Pablo/repos/notetaker/defaults/fonts/JetBrainsMonoNerdFontMono-Regular.ttf",
            )
            .expect("Failed to load font");

        let mut font_system =
            cosmic_text::FontSystem::new_with_locale_and_db("en-US".to_string(), database);
        let metrics = cosmic_text::Metrics::relative(12.0, 1.2);
        let attributes = cosmic_text::Attrs::new()
            .family(cosmic_text::Family::Name("JetBrainsMono Nerd Font Mono"));

        let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut font_system);
        buffer.set_text(
            "Hello world",
            &attributes,
            cosmic_text::Shaping::Basic,
            Some(cosmic_text::Align::Left),
        );
        let layout_runs = buffer.layout_runs();
        let glyphs: Vec<cosmic_text::LayoutGlyph> = layout_runs
            .flat_map(|run| run.glyphs.iter().cloned())
            .collect();

        let mut swash_cache = cosmic_text::SwashCache::new();
        for glyph in glyphs {
            let (key, offset_x, offset_y) = cosmic_text::CacheKey::new(
                glyph.font_id,
                glyph.glyph_id,
                glyph.font_size,
                (glyph.x, glyph.y),
                glyph.font_weight,
                glyph.cache_key_flags,
            );
            swash_cache.get_image(&mut font_system, key);
            if let Some(image) = swash_cache.get_image(&mut font_system, key) {
                println!(
                    "glyph image: {}x{}",
                    image.placement.width, image.placement.height
                );
            }
        }

        Self { font_system }
    }

    // load user fonts
    // load default fonts
}
