use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type Handler = Box<dyn FnMut(&dyn Event)>;

pub trait Event: Any {
    fn as_any(&self) -> &dyn Any;
}

pub struct EventBus {
    handlers: HashMap<TypeId, Vec<Handler>>,
    queue: Vec<Box<dyn Event>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            queue: Vec::new(),
        }
    }

    pub fn on<E, F>(&mut self, mut handler: F)
    where
        E: Event,
        F: FnMut(&E) + 'static,
    {
        // wrap handler inside a function that downcasts the event type
        let handler = Box::new(move |event: &dyn Event| {
            let event = event
                .as_any()
                .downcast_ref::<E>()
                .expect("Failed to match event type to the closures one");

            handler(event);
        });

        // store callback at the event id entry or create a new one if none
        self.handlers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(handler);
    }

    pub fn publish<E: Event>(&mut self, event: E) {
        self.queue.push(Box::new(event));
    }

    pub fn flush_queue(&mut self) {
        let events: Vec<_> = self.queue.drain(..).collect();
        for event in events {
            if let Some(handlers) = self.handlers.get_mut(&event.type_id()) {
                for handler in handlers {
                    handler(event.as_ref());
                }
            }
        }
    }
}
