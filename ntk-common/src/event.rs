use crate::{Viewport, input::InputEvent};

#[derive(Debug, Clone)]
pub enum PlatformEvent {
    Input(InputEvent),
    Resize(Viewport),
    FullRedraw,
    Close,
}
