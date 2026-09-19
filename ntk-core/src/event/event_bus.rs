use ntk_common::RenderCommand;

use crate::event::{Command, Event};

#[derive(Debug, Default)]
pub struct EventBus {
    event_queue: Vec<Event>,
    cmd_queue: Vec<Command>,
    render_cmd_queue: Vec<RenderCommand>,
}

impl EventBus {
    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn push_command(&mut self, cmd: Command) {
        self.cmd_queue.push(cmd);
    }

    pub fn push_render_command(&mut self, cmd: RenderCommand) {
        self.render_cmd_queue.push(cmd);
    }

    pub fn get_event_writer(&mut self) -> EventWriter<'_> {
        EventWriter(&mut self.event_queue)
    }

    pub fn get_command_writer(&mut self) -> CommandWriter<'_> {
        CommandWriter(&mut self.cmd_queue)
    }

    pub fn get_render_command_writer(&mut self) -> RenderCommandWriter<'_> {
        RenderCommandWriter(&mut self.render_cmd_queue)
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        self.event_queue.drain(..).collect()
    }

    pub fn get_commands(&mut self) -> Vec<Command> {
        self.cmd_queue.drain(..).collect()
    }

    pub fn get_render_commands(&mut self) -> Vec<RenderCommand> {
        self.render_cmd_queue.drain(..).collect()
    }
}

pub trait EventHandler {
    fn on_event(&mut self, event: &Event, cmd_writer: &mut CommandWriter);
}

pub trait CommandHandler {
    fn on_command(&mut self, cmd: &Command, event_writer: &mut EventWriter);
}

#[derive(Debug)]
pub struct EventWriter<'a>(&'a mut Vec<Event>);

impl EventWriter<'_> {
    pub fn push(&mut self, event: Event) {
        self.0.push(event);
    }
}

#[derive(Debug)]
pub struct CommandWriter<'a>(&'a mut Vec<Command>);

impl CommandWriter<'_> {
    pub fn push(&mut self, cmd: Command) {
        self.0.push(cmd);
    }
}

#[derive(Debug)]
pub struct RenderCommandWriter<'a>(&'a mut Vec<RenderCommand>);

impl RenderCommandWriter<'_> {
    pub fn push(&mut self, cmd: RenderCommand) {
        self.0.push(cmd);
    }
}
