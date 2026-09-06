use std::time::Instant;

use crate::{
    EditorEvent,
    event::EventBus,
    lua::{LuaRuntime, LuaRuntimeError},
    workspace::WorkspaceRegistry,
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

    workspaces: WorkspaceRegistry,
}

impl Editor {
    pub fn new() -> Result<Self, EditorError> {
        Ok(Self {
            event_bus: EventBus::default(),
            lua_runtime: LuaRuntime::new()?,

            workspaces: WorkspaceRegistry::new(),
        })
    }

    pub fn push_event(&mut self, event: EditorEvent) {
        self.event_bus.push_event(event);
    }

    pub fn update(&mut self, now: Instant) {
        self.event_bus.push_event(EditorEvent::Update(now));
        self.event_bus.update();
    }
}
