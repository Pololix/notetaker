use crate::text::glyph_atlas::GlyphAtlas;
use cosmic_text::{Buffer, FontSystem, Metrics, SwashCache};

const FONT_SIZE: f32 = 24.0;
const FONT_FACT: f32 = 1.2;

#[derive(Debug, thiserror::Error)]
pub enum TextRendererError {}

#[derive(Debug)]
pub struct TextRenderer {
    font_system: FontSystem,
    cache: SwashCache,
    atlas: GlyphAtlas,
    buffer: Buffer,
}

impl TextRenderer {
    pub fn new() -> Result<Self, TextRendererError> {
        let mut font_system = FontSystem::new();

        let cache = SwashCache::new();
        let atlas = GlyphAtlas::new();
        let metrics = Metrics::relative(FONT_SIZE, FONT_FACT);

        let buffer = Buffer::new(&mut font_system, metrics);

        Ok(Self {
            font_system,
            cache,
            atlas,
            buffer,
        })
    }
}
