#[derive(Debug, thiserror::Error)]
pub enum EditorError {}

#[derive(Debug, Default)]
pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Self {}
    }
}
