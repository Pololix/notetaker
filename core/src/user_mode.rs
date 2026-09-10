#[derive(Debug, Clone, Copy)]
pub enum UserMode {
    Normal,
    Insert,
}

impl Default for UserMode {
    fn default() -> Self {
        Self::Normal
    }
}
