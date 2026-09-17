use crate::{
    document::document::Document,
    event::{Command, CommandHandler, CommandWriter, Event, EventHandler, EventWriter},
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceError {}

#[derive(Debug, Default)]
pub struct Workspace {
    document: Document,
}

impl Workspace {}

impl EventHandler for Workspace {
    fn on_event(&mut self, event: &Event, cmd_writer: &mut CommandWriter) {}
}

impl CommandHandler for Workspace {
    fn on_command(&mut self, cmd: &Command, event_writer: &mut EventWriter) {}
}
