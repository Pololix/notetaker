use crate::{
    event::{EventBus, input_event::InputEvent},
    lua::{LuaRuntime, LuaRuntimeError},
    user_mode::UserMode,
};

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("{0}")]
    LuaRuntime(#[from] LuaRuntimeError),
}

pub struct Editor {
    event_bus: EventBus,
    lua_runtime: LuaRuntime,

    user_mode: UserMode,
}

impl Editor {
    pub fn new() -> Result<Self, EditorError> {
        Ok(Self {
            event_bus: EventBus::default(),
            lua_runtime: LuaRuntime::new()?,

            user_mode: UserMode::Normal,
        })
    }

    pub fn handle_input(&mut self, input_event: InputEvent) {
        // ask lua for Result
        // publish result on the bus
    }
}
