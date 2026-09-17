use crate::event::{
    command::{Command, CommandHandler},
    event::{Event, EventHandler},
};

#[derive(Default)]
pub struct EventBus {
    event_queue: Vec<Event>,
    event_handlers: Vec<Box<dyn EventHandler>>,
    cmd_queue: Vec<Command>,
    cmd_handlers: Vec<Box<dyn CommandHandler>>,
}

impl EventBus {
    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn push_command(&mut self, command: Command) {
        self.cmd_queue.push(command);
    }

    pub fn get_event_writer(&mut self) -> EventWriter<'_> {
        EventWriter(&mut self.event_queue)
    }

    pub fn get_command_writer(&mut self) -> CommandWriter<'_> {
        CommandWriter(&mut self.cmd_queue)
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        self.event_queue.drain(..).collect()
    }

    pub fn get_commands(&mut self) -> Vec<Command> {
        self.cmd_queue.drain(..).collect()
    }
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
