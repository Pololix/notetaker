use crate::primitive::{Quad, invalidation::RenderInvalidation};
use ntk_core::render::RenderId;
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

    pub fn invalidate(&mut self) {
        self.invalidation.full();
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

    pub fn get_quads(&mut self) -> Option<Vec<Quad>> {
        let quads = match self.invalidation {
            RenderInvalidation::Empty => return None,
            RenderInvalidation::Full => self.quads.values().flatten().copied().collect(),
            RenderInvalidation::Partial(rect) => self
                .quads
                .values()
                .flatten()
                .filter(|quad| quad.rect().intersects(rect))
                .copied()
                .collect(),
        };

        // reset and deliver
        self.invalidation = RenderInvalidation::Empty;
        Some(quads)
    }
}
