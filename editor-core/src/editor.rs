use crate::{event::EventBus, lua::LuaRuntime, user_mode::UserMode};

pub struct Editor {
    // exposed for the app crate to see
    pub event_bus: EventBus,
    lua: LuaRuntime,
    user_mode: UserMode,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            event_bus: EventBus::default(),
            lua: LuaRuntime::new(),
            user_mode: UserMode::Normal,
        }
    }
}
