use crate::{
    editor_events::{EditorEvent, EditorInputEvent},
    workspace::{Workspace, WorkspaceId},
};

#[derive(Debug)]
enum UserMode {
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
        let default_workspace = Workspace::new(0);

        // create null-dimension viewport and resize on window creation/resize
        Self {
            viewport: (0, 0),
            workspaces: vec![default_workspace],
            active_id: 0,
            mode: UserMode::Normal,
        }
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport = (width, height);
        let index = self.get_index(self.active_id);
        self.workspaces[index].adapt_to_viewport(width, height);
    }

    // pub fn render

    pub fn handle_input_event(&mut self, input_event: EditorInputEvent) {
        let event = match self.mode {
            UserMode::Normal => Self::normal_input_event(input_event),
            UserMode::Insert => Self::insert_input_event(input_event),
        };

        match event {
            Some(editor_event) => {
                let index = self.get_index(self.active_id);
                self.workspaces[index].handle_event(editor_event);
            }
            None => return,
        }
    }

    fn normal_input_event(input_event: EditorInputEvent) -> Option<EditorEvent> {
        match input_event {
            _ => None,
        }
    }

    fn insert_input_event(input_event: EditorInputEvent) -> Option<EditorEvent> {
        match input_event {
            _ => None,
        }
    }

    fn get_index(&self, id: WorkspaceId) -> usize {
        self.workspaces
            .iter()
            .position(|workspace| workspace.id == id)
            .expect("Failed to retrieve workspace from id")
    }
}
