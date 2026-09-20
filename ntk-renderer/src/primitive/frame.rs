use crate::{invalidation::RenderInvalidation, primitive::raw_quad::RawQuad};
use ntk_core::render::{RenderId, types::Rect};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Frame {
    quads: HashMap<RenderId, Vec<RawQuad>>,
}

impl Frame {
    pub fn clear(&mut self) {
        self.quads.clear();
    }

    pub fn upload(
        &mut self,
        id: RenderId,
        quads: &[RawQuad],
        invalidation: &mut RenderInvalidation,
    ) {
        // remove previous render items and insert new ones
        self.remove(id, invalidation);
        self.quads.insert(id, quads.to_vec());
    }

    pub fn remove(&mut self, id: RenderId, invalidation: &mut RenderInvalidation) {
        // rerender previously occupied space
        if let Some(quads) = self.quads.remove(&id) {
            for quad in quads {
                invalidation.partial(quad.rect());
            }
        };
    }

    pub fn get_intersecting(&self, rect: Rect) -> Vec<RawQuad> {
        self.quads
            .values()
            .flatten()
            .filter(|quad| quad.rect().intersects(rect))
            .copied()
            .collect()
    }

    pub fn get_quads(&self) -> Vec<RawQuad> {
        self.quads.values().flatten().copied().collect()
    }
}
