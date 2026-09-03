#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UserMode {
    Normal,
    Visual,
    Insert,
    Command,
    Terminal,
}
