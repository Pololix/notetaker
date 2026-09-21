use ntk_core::render::types::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderInvalidation {
    Empty,
    Partial(Rect),
    Full,
}

impl Default for RenderInvalidation {
    fn default() -> Self {
        Self::Empty
    }
}

impl RenderInvalidation {
    pub fn partial(&mut self, new: Rect) {
        match self {
            Self::Empty => *self = Self::Partial(new),
            Self::Partial(rect) => {
                rect.union(new);
            }
            Self::Full => {}
        }
    }
}
