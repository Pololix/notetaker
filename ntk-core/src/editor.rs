use ntk_common::{AppCommand, AppEvent};

use crate::{
    document::{UserMode, Workspace},
    event::{
        CommandHandler, CommandWriter, EditorCommand, EditorEvent, EventBus, EventHandler,
        EventWriter,
    },
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

    event_bus: EventBus<EditorEvent, EditorCommand>,

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

impl EventHandler<AppEvent, AppCommand> for Editor {
    fn on_event(&mut self, event: &AppEvent, cmd_writer: &mut CommandWriter<AppCommand>) {}
}

impl CommandHandler<AppEvent, AppCommand> for Editor {
    fn on_command(&mut self, cmd: &AppCommand, event_writer: &mut EventWriter<AppEvent>) {}
}
