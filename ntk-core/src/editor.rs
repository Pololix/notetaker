use crate::{
    document::{UserMode, Workspace},
    event::EventBus,
    platform::PlatformEvent,
    render::{RenderCommand, Viewport},
};

#[derive(Debug, thiserror::Error)]
pub enum EditorError {}

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

            workspace: Workspace::new(viewport),
        }
    }

    pub fn handle_platform_event(&mut self, event: PlatformEvent) {
        match event {
            PlatformEvent::WindowResized(viewport) => self
                .event_bus
                .push_render_command(RenderCommand::Resize(viewport)),

            PlatformEvent::RedrawRequested => self
                .event_bus
                .push_render_command(RenderCommand::RedrawFrame),

            PlatformEvent::ExitRequested => {}
        }
    }

    pub fn update(&mut self, dt: f32) {
        // commands produced by events are immediately processed
        let events = self.event_bus.get_events();
        let mut cmd_writer = self.event_bus.get_command_writer();
        for event in events {

            // handle events
        }

        // events produced by commands are stored for the next iteration
        let cmds = self.event_bus.get_commands();
        let mut event_writer = self.event_bus.get_event_writer();
        for cmd in cmds {

            // process commands
        }
    }

    pub fn render(&mut self) -> Vec<RenderCommand> {
        let mut cmd_writer = self.event_bus.get_render_command_writer();

        self.workspace.render(&mut cmd_writer);

        self.event_bus.get_render_commands()
    }
}
