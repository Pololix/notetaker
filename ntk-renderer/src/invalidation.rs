use ntk_core::render::types::Rect;

#[derive(Debug, Clone, Copy)]
pub enum RenderInvalidation {
    Empty,
    Partial(Rect),
    Full,
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

    pub fn full(&mut self) {
        *self = Self::Full
    }
}
