use crate::{input_event::InputEvent, util::Viewport};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum EditorEvent {
    // global
    Resized(Viewport),
    RedrawRequested,
    Input(InputEvent),
    Update(Instant),
}
