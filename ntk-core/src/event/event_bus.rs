pub struct EventBus<E, C> {
    event_queue: Vec<E>,
    event_handlers: Vec<Box<dyn EventHandler<E, C>>>,
    cmd_queue: Vec<C>,
    cmd_handlers: Vec<Box<dyn CommandHandler<E, C>>>,
}

impl<E, C> Default for EventBus<E, C> {
    fn default() -> Self {
        Self {
            event_queue: Vec::new(),
            event_handlers: Vec::new(),
            cmd_queue: Vec::new(),
            cmd_handlers: Vec::new(),
        }
    }
}

impl<E, C> EventBus<E, C> {
    pub fn push_event(&mut self, event: E) {
        self.event_queue.push(event);
    }

    pub fn push_command(&mut self, command: C) {
        self.cmd_queue.push(command);
    }

    pub fn get_event_writer(&mut self) -> EventWriter<'_, E> {
        EventWriter(&mut self.event_queue)
    }

    pub fn get_command_writer(&mut self) -> CommandWriter<'_, C> {
        CommandWriter(&mut self.cmd_queue)
    }

    pub fn get_events(&mut self) -> Vec<E> {
        self.event_queue.drain(..).collect()
    }

    pub fn get_commands(&mut self) -> Vec<C> {
        self.cmd_queue.drain(..).collect()
    }
}

pub trait EventHandler<E, C> {
    fn on_event(&mut self, event: &E, cmd_writer: &mut CommandWriter<C>);
}

pub trait CommandHandler<E, C> {
    fn on_command(&mut self, cmd: &C, event_writer: &mut EventWriter<E>);
}

#[derive(Debug)]
pub struct EventWriter<'a, E>(&'a mut Vec<E>);

impl<E> EventWriter<'_, E> {
    pub fn push(&mut self, event: E) {
        self.0.push(event);
    }
}

#[derive(Debug)]
pub struct CommandWriter<'a, C>(&'a mut Vec<C>);

impl<C> CommandWriter<'_, C> {
    pub fn push(&mut self, cmd: C) {
        self.0.push(cmd);
    }
}
