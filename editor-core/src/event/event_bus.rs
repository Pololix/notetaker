use crate::event::{EditorCommand, EditorEvent};
use std::collections::VecDeque;

#[derive(Debug)]
struct CommandWriter<'a> {
    queue: &'a mut VecDeque<EditorCommand>,
}

impl CommandWriter<'_> {
    pub fn push(&mut self, cmd: EditorCommand) {
        self.queue.push_back(cmd);
    }
}

#[derive(Debug)]
struct EventWriter<'a> {
    queue: &'a mut VecDeque<EditorEvent>,
}

impl EventWriter<'_> {
    pub fn push(&mut self, event: EditorEvent) {
        self.queue.push_back(event);
    }
}

#[derive(Default)]
pub struct EventBus {
    cmd_queue: VecDeque<EditorCommand>,
    cmd_handlers: Vec<Box<dyn FnMut(&EditorCommand, &mut EventWriter)>>,

    event_queue: VecDeque<EditorEvent>,
    event_handlers: Vec<Box<dyn FnMut(&EditorEvent, &mut CommandWriter)>>,
}

impl EventBus {
    // subscriptions
    pub fn on_command<F: FnMut(&EditorCommand, &mut EventWriter) + 'static>(&mut self, handler: F) {
        self.cmd_handlers.push(Box::new(handler));
    }

    pub fn on_event<F: FnMut(&EditorEvent, &mut CommandWriter) + 'static>(&mut self, handler: F) {
        self.event_handlers.push(Box::new(handler))
    }

    // submissions
    pub fn push_command(&mut self, cmd: EditorCommand) {
        self.cmd_queue.push_back(cmd);
    }

    pub fn push_event(&mut self, event: EditorEvent) {
        self.event_queue.push_back(event);
    }

    // processing
    pub fn update(&mut self) {
        // events emmited by commands are immediatly acted upon
        while let Some(cmd) = self.cmd_queue.pop_front() {
            let mut event_writer = EventWriter {
                queue: &mut self.event_queue,
            };

            self.cmd_handlers
                .iter_mut()
                .for_each(|handler| handler(&cmd, &mut event_writer));
        }
        // commands emmited by events are stored for the next iteration
        while let Some(event) = self.event_queue.pop_front() {
            let mut cmd_writer = CommandWriter {
                queue: &mut self.cmd_queue,
            };

            self.event_handlers
                .iter_mut()
                .for_each(|handler| handler(&event, &mut cmd_writer));
        }
    }
}
