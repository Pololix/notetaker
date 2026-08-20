use crate::{
    editor_events::EditorInputEvent,
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

    pub fn handle_input_event(&self, input_event: EditorInputEvent) {
        let event = match self.mode {
            UserMode::Normal => {}
            UserMode::Insert => {}
        };

        todo!("populate input event matches");
        // self.workspaces[self.get_index(self.active_id)].handle_event(event);
    }

    fn get_index(&self, id: WorkspaceId) -> usize {
        self.workspaces
            .iter()
            .position(|workspace| workspace.id == id)
            .expect("Failed to retrieve workspace from id")
    }
}
