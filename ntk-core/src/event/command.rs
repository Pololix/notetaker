use crate::event::event_bus::EventWriter;

pub trait CommandHandler {
    fn on_command(&mut self, cmd: &Command, event_writer: &mut EventWriter);
}

#[derive(Debug, Clone)]
pub enum Command {}
