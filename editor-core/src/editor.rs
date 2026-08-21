// TODO
// - hold several workspaces
// - make default keybinds thorugh the Lua API

use crate::{
    event::input_event::{Key, KeyState},
    event::{
        editor_event::EditorCommand, input_event::InputEvent, workspace_event::WorkspaceCommand,
    },
    workspace::{SplitMode, Workspace, WorkspaceId},
};
use editor_renderer::Quad;

#[derive(Debug, Clone, Copy)]
pub enum UserMode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct Editor {
    viewport: (u32, u32),
    workspaces: Vec<Workspace>,
    active_id: WorkspaceId,
    mode: UserMode,
}

impl Editor {
    pub fn new() -> Self {
        let viewport = (0, 0);
        let default_workspace = Workspace::new(0, viewport);

        // create null-dimension viewport and resize on window creation/resize
        Self {
            viewport,
            workspaces: vec![default_workspace],
            active_id: 0,
            mode: UserMode::Normal,
        }
    }

    pub fn set_viewport(&mut self, viewport: (u32, u32)) {
        self.viewport = viewport;
        let index = self.get_index(self.active_id);
        self.workspaces[index].adapt_to_viewport(viewport);
    }

    pub fn render_active(&self) -> Vec<Quad> {
        let index = self.get_index(self.active_id);
        self.workspaces[index].draw(self.viewport)
    }

    pub fn handle_input_event(&mut self, input_event: InputEvent) {
        // route events to be handled based on the current mode
        let cmd = match self.mode {
            UserMode::Normal => self.normal_input_event(input_event),
            UserMode::Insert => self.insert_input_event(input_event),
        };

        match cmd {
            Some(cmd) => self.handle_command(cmd),
            None => return, // no command generated for the given input
        }
    }

    fn normal_input_event(&self, event: InputEvent) -> Option<EditorCommand> {
        match event {
            InputEvent::Key {
                key,
                state,
                mods: _,
            } => {
                if state != KeyState::Pressed {
                    return None;
                }

                match key {
                    Key::Character(key) => match key.as_str() {
                        "n" => Some(EditorCommand::Workspace(WorkspaceCommand::OpenBuffer {
                            viewport: self.viewport,
                        })),
                        "d" => Some(EditorCommand::Workspace(WorkspaceCommand::CloseBuffer)),
                        "v" => Some(EditorCommand::Workspace(WorkspaceCommand::SplitBuffer {
                            mode: SplitMode::Vertical,
                        })),
                        "h" => Some(EditorCommand::Workspace(WorkspaceCommand::SplitBuffer {
                            mode: SplitMode::Horizontal,
                        })),
                        _ => None,
                    },

                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn insert_input_event(&self, event: InputEvent) -> Option<EditorCommand> {
        match event {
            _ => None,
        }
    }

    fn handle_command(&mut self, cmd: EditorCommand) {
        match cmd {
            EditorCommand::Workspace(cmd) => {
                let index = self.get_index(self.active_id);
                self.workspaces[index].handle_command(cmd);
            }
            _ => return,
        }
    }

    fn get_index(&self, id: WorkspaceId) -> usize {
        self.workspaces
            .iter()
            .position(|workspace| workspace.id == id)
            .expect("Failed to retrieve workspace from id")
    }
}
