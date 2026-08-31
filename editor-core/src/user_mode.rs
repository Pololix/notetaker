#[derive(Debug, Clone, Copy)]
pub enum UserMode {
    Normal,
    PendOp,
    Insert,
    Cmdline,
    Terminal,
}
