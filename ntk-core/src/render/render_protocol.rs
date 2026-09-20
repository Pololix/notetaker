use crate::render::RenderCommand;

pub type RenderId = u64;

#[derive(Debug, Default)]
pub struct RenderIdAllocator {
    next_id: RenderId,
}

impl RenderIdAllocator {
    pub fn next(&mut self) -> RenderId {
        let next = self.next_id;
        self.next_id += 1;

        next
    }
}

pub trait RenderProtocol {
    fn render(&mut self, cmds: &[RenderCommand]);
}
