use crate::primitive::{Quad, invalidation::RenderInvalidation};
use ntk_core::render::{RenderId, Viewport, types::Rect};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Frame {
    invalidation: RenderInvalidation,
    quads: HashMap<RenderId, Vec<Quad>>,
}

impl Frame {
    pub fn clear(&mut self) {
        self.quads.clear();
    }

    pub fn empty_invalidation(&mut self) {
        self.invalidation = RenderInvalidation::Full
    }

    pub fn full_invalidation(&mut self) {
        self.invalidation = RenderInvalidation::Full
    }

    pub fn upload(&mut self, id: RenderId, quads: &[Quad]) {
        // remove previous render items and insert new ones
        self.remove(id);

        // invalidate newly occupied space
        for quad in quads {
            self.invalidation.partial(quad.rect());
        }
        self.quads.insert(id, quads.to_vec());
    }

    pub fn remove(&mut self, id: RenderId) {
        // invalidate previously occupied space
        if let Some(quads) = self.quads.remove(&id) {
            for quad in quads {
                self.invalidation.partial(quad.rect());
            }
        };
    }

    pub fn get_quads(&mut self, viewport: Viewport) -> Option<(Rect, Vec<Quad>)> {
        let (rect, quads) = match self.invalidation {
            RenderInvalidation::Empty => return None,
            RenderInvalidation::Partial(rect) => (
                rect,
                self.quads
                    .values()
                    .flatten()
                    .filter(|quad| quad.rect().intersects(rect))
                    .copied()
                    .collect(),
            ),
            RenderInvalidation::Full => (
                viewport.to_rect(),
                self.quads.values().flatten().copied().collect(),
            ),
        };

        self.invalidation = RenderInvalidation::Empty;

        Some((rect, quads))
    }
}
