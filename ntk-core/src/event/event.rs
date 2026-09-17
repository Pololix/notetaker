use crate::event::event_bus::CommandWriter;

pub trait EventHandler {
    fn on_event(&mut self, event: &Event, cmd_writer: &mut CommandWriter);
}

#[derive(Debug, Clone)]
pub enum Event {}
