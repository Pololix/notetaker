use std::time::Instant;

use crate::{
    event::EventBus,
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
    pub event_bus: EventBus,
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

    pub fn update(&mut self, now: Instant) {
        // check for pending events due to timed-out keybinds
        if let Some(cmd) = self
            .lua_runtime
            .keybinds
            .borrow_mut()
            .check_pending_deadline(now)
        {
            self.event_bus.push_command(cmd);
        }

        self.event_bus.update();
    }
}
