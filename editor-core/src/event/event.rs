use crate::{input_event::InputEvent, util::Viewport};

#[derive(Debug, Clone)]
pub enum EditorEvent {
    // global
    Resized(Viewport),
    RedrawRequested,
    Input(InputEvent),
}
