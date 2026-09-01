use crate::event::{EditorCommand, EditorEvent};

type EventHandler = Box<dyn FnMut(&EditorEvent, &mut CommandWriter)>;
type CommandHandler = Box<dyn FnMut(&EditorCommand, &mut EventWriter)>;

struct EventWriter<'a> {
    pub queue: &'a mut Vec<EditorEvent>,
}

struct CommandWriter<'a> {
    pub queue: &'a mut Vec<EditorCommand>,
}

#[derive(Default)]
pub struct EventBus {
    event_queue: Vec<EditorEvent>,
    event_handlers: Vec<EventHandler>,

    cmd_queue: Vec<EditorCommand>,
    cmd_handlers: Vec<CommandHandler>,
}

impl EventBus {
    pub fn on_event(&mut self, event_handler: EventHandler) {
        self.event_handlers.push(event_handler);
    }

    pub fn on_command(&mut self, cmd_handler: CommandHandler) {
        self.cmd_handlers.push(cmd_handler);
    }

    pub fn push_event(&mut self, event: EditorEvent) {
        self.event_queue.push(event);
    }

    pub fn push_command(&mut self, cmd: EditorCommand) {
        self.cmd_queue.push(cmd);
    }

    pub fn update(&mut self) {
        let cmds: Vec<_> = self.cmd_queue.drain(..).collect();
        for cmd in cmds {
            let mut event_writer = EventWriter {
                queue: &mut self.event_queue,
            };

            for handler in &mut self.cmd_handlers {
                handler(&cmd, &mut event_writer);
            }
        }
        let events: Vec<_> = self.event_queue.drain(..).collect();
        for event in events {
            let mut cmd_writer = CommandWriter {
                queue: &mut self.cmd_queue,
            };

            for handler in &mut self.event_handlers {
                handler(&event, &mut cmd_writer);
            }
        }
    }
}
