#[derive(Debug, Clone, Copy)]
pub struct MousePress {
    button: MouseButton,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    LMB,
    RMB,
    MMB,
}
