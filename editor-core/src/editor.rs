use crate::{
    event::{EventBus, input_event::InputEvent},
    user_mode::UserMode,
};

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum EditorError {}

pub struct Editor {
    event_bus: EventBus,

    user_mode: UserMode,
}

impl Editor {
    pub fn new() -> Result<Self, EditorError> {
        Ok(Self {
            event_bus: EventBus::default(),

            user_mode: UserMode::Normal,
        })
    }

    pub fn handle_input(&mut self, input_event: InputEvent) {}
}
