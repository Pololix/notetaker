// TODO
// - hold several workspaces

use crate::{
    editor_events::{EditorEvent, EditorInputEvent},
    workspace::{Workspace, WorkspaceId},
};
use editor_renderer::Quad;

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

    pub fn handle_input_event(&mut self, input_event: EditorInputEvent) {
        let event = match self.mode {
            UserMode::Normal => Self::normal_input_event(input_event),
            UserMode::Insert => Self::insert_input_event(input_event),
        };

        if let Some(editor_event) = event {
            let index = self.get_index(self.active_id);
            self.workspaces[index].handle_event(editor_event);
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
