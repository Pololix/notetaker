use crate::{
    document::{UserMode, Workspace},
    event::EventBus,
};
use ntk_common::{PlatformEvent, Viewport};

#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("Failed to render current frame")]
    Rendering,
}

pub struct Editor {
    mode: UserMode,
    viewport: Viewport,

    event_bus: EventBus,

    workspace: Workspace,
}

impl Editor {
    pub fn new(viewport: Viewport) -> Self {
        Self {
            mode: UserMode::default(),
            viewport,

            event_bus: EventBus::default(),

            workspace: Workspace::default(),
        }
    }

    pub fn handle_platform_event(&mut self, event: PlatformEvent) {
        match event {
            _ => {}
        }
    }

    pub fn update(&mut self, dt: f32) {
        // commands produced by events are immediately processed
        let events = self.event_bus.get_events();
        for event in events {
            let mut command_writer = self.event_bus.get_command_writer();

            // handle events
        }

        // events produced by commands are stored for the next iteration
        let cmds = self.event_bus.get_commands();
        for cmd in cmds {
            let mut event_writer = self.event_bus.get_event_writer();

            // process commands
        }
    }
}
