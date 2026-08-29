use crate::event::{EditorCommand, EditorEvent};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type EventHandler = Box<dyn FnMut(&dyn Any, &mut CommandWriter<'_>)>;
type CommandHandler = Box<dyn FnMut(&dyn Any, &mut EventWriter<'_>)>;

// helper struct to allow commands to write events
pub struct EventWriter<'a> {
    queue: &'a mut Vec<Box<dyn EditorEvent>>,
}
impl EventWriter<'_> {
    pub fn push<E: EditorEvent>(&mut self, event: E) {
        self.queue.push(Box::new(event));
    }
}

// helper struct to allow events to write commands
pub struct CommandWriter<'a> {
    queue: &'a mut Vec<Box<dyn EditorCommand>>,
}
impl CommandWriter<'_> {
    pub fn push<C: EditorCommand>(&mut self, command: C) {
        self.queue.push(Box::new(command));
    }
}

#[derive(Default)]
pub struct EventBus {
    event_queue: Vec<Box<dyn EditorEvent>>,
    event_handlers: HashMap<TypeId, Vec<EventHandler>>,

    cmd_queue: Vec<Box<dyn EditorCommand>>,
    cmd_handlers: HashMap<TypeId, Vec<CommandHandler>>,
}

impl EventBus {
    // subscriptions
    // handlers are stored wrapped alongside type downscasting and are passed a writer on bus update
    pub fn on_event<E, F>(&mut self, mut handler: F)
    where
        E: EditorEvent,
        F: FnMut(&E, &mut CommandWriter<'_>) + 'static,
    {
        // create wraapped handler
        let handler = Box::new(move |event: &dyn Any, writer: &mut CommandWriter<'_>| {
            let event = event.downcast_ref::<E>().expect("event type mismatch");

            handler(event, writer);
        });

        // store at entry or create a new one
        self.event_handlers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(handler);
    }

    pub fn on_cmd<C, F>(&mut self, mut handler: F)
    where
        C: EditorCommand,
        F: FnMut(&C, &mut EventWriter<'_>) + 'static,
    {
        // create wraapped handler
        let handler = Box::new(move |event: &dyn Any, writer: &mut EventWriter<'_>| {
            let event = event.downcast_ref::<C>().expect("cmd type mismatch");

            handler(event, writer);
        });

        // store at entry or create a new one
        self.cmd_handlers
            .entry(TypeId::of::<C>())
            .or_default()
            .push(handler);
    }

    // submissions
    pub fn submit_event<E: EditorEvent>(&mut self, event: E) {
        self.event_queue.push(Box::new(event));
    }

    pub fn submit_cmd<C: EditorCommand>(&mut self, cmd: C) {
        self.cmd_queue.push(Box::new(cmd));
    }

    // cmd/event processing
    pub fn update(&mut self) {
        // flush command queue (generates events)
        for cmd in &self.cmd_queue {
            if let Some(handlers) = self.cmd_handlers.get_mut(&cmd.as_any().type_id()) {
                let mut event_writer = EventWriter {
                    queue: &mut self.event_queue,
                };
                for handler in handlers {
                    handler(cmd.as_any(), &mut event_writer);
                }
            }
        }
        self.cmd_queue.clear();

        // flush event queue (generates cmds for the next iteration)
        for event in &self.event_queue {
            if let Some(handlers) = self.event_handlers.get_mut(&event.as_any().type_id()) {
                let mut cmd_writer = CommandWriter {
                    queue: &mut self.cmd_queue,
                };
                for handler in handlers {
                    handler(event.as_any(), &mut cmd_writer);
                }
            }
        }
        self.event_queue.clear();
    }
}
