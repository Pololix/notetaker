use ropey::Rope;
pub type BufferId = usize;

#[derive(Debug, Default, Clone)]
pub struct Buffer {
    text: Rope,
}

impl Buffer {}
