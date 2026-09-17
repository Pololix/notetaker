use crate::{
    document::UserMode,
    event::{Event, EventBus},
    render::Viewport,
};

#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("Failed to render current frame")]
    Rendering,
}

pub struct Editor {
    mode: UserMode,
    viewport: Viewport,

    event_bus: EventBus,
}

impl Editor {
    pub fn new(viewport: Viewport) -> Self {
        Self {
            mode: UserMode::default(),
            viewport,

            event_bus: EventBus::default(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        // stream app-incoming events directly to the bus
        self.event_bus.push_event(event);
    }

    pub fn update(&mut self) {
        // commands produced by events are immediately processed
        let events: Vec<_> = self.event_bus.get_events().collect();
        for event in events {
            let mut command_writer = self.event_bus.get_command_writer();

            // handle events
        }

        // events produced by commands are stored for the next iteration
        let commands: Vec<_> = self.event_bus.get_commands().collect();
        for command in commands {
            let mut event_writer = self.event_bus.get_event_writer();

            // process commands
        }
    }
}
