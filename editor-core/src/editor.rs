use crate::{
    event::{
        editor_event::EditorCommand,
        input_event::{
            InputEvent, Key,
            KeyState::{self, Released},
        },
        workspace_event::WorkspaceCommand,
    },
    workspace::{SplitMode, Workspace, WorkspaceError, WorkspaceId},
};
use editor_common::{geometry::Viewport, rendering::RenderFrame};
use std::collections::HashMap;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("Failed to retrieve the active workspace")]
    NullActive,

    #[error("Failed to retrieve a workspace from the given id")]
    InvalidWorkspaceId,

    #[error("Cant add another workspace because the limit has been reached")]
    Overflow,

    #[error("Error ocurred at workspace level: {0}")]
    Workspace(#[from] WorkspaceError),
}

#[derive(Debug, Clone, Copy)]
pub enum UserMode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct Editor {
    viewport: Viewport,
    mode: UserMode,

    workspaces: HashMap<WorkspaceId, Workspace>,
    workspace_count: u32,
    active_id: WorkspaceId, // at least one (even if empty) while the app is running
    next_id: WorkspaceId,
}

impl Editor {
    pub fn new() -> Result<Self, EditorError> {
        // create arbitrary-dimensioned viewport and resize on window creation/resize
        let mut new_self = Self {
            viewport: Viewport {
                width: 1000,
                height: 1000,
            },
            mode: UserMode::Normal,

            workspaces: HashMap::new(),
            workspace_count: 0,
            active_id: 0,
            next_id: 0,
        };

        let _event = new_self.add_workspace(new_self.viewport)?;

        Ok(new_self)
    }

    pub fn set_viewport(&mut self, viewport: Viewport) -> Result<(), EditorError> {
        self.viewport = viewport;
        self.get_mut_workspace(self.active_id)?
            .adapt_to_viewport(viewport)?;

        Ok(())
    }

    pub fn render(&self) -> Result<RenderFrame, EditorError> {
        let active = self.get_workspace(self.active_id)?;
        let cmds = active.render();

        Ok(RenderFrame { cmds })
    }

    pub fn handle_input_event(&mut self, event: InputEvent) -> Result<(), EditorError> {
        // route events to be handled based on the current mode
        let cmd = match self.mode {
            UserMode::Normal => self.normal_event(event),
            UserMode::Insert => None,
        };

        if let Some(cmd) = cmd {
            self.handle_command(cmd)?
        }

        Ok(())
    }

    fn normal_event(&self, event: InputEvent) -> Option<EditorCommand> {
        match event {
            InputEvent::Key { key, state, .. } => {
                if state == KeyState::Released {
                    return None; // discard released events
                }

                match key {
                    Key::Character(c) => {
                        match c.as_str() {
                            "n" => Some(EditorCommand::Workspace(WorkspaceCommand::OpenBuffer {
                                viewport: self.viewport,
                            })),
                            "v" => Some(EditorCommand::Workspace(WorkspaceCommand::SplitBuffer {
                                mode: SplitMode::Vertical,
                            })),
                            "h" => Some(EditorCommand::Workspace(WorkspaceCommand::SplitBuffer {
                                mode: SplitMode::Horizontal,
                            })),
                            "q" => Some(EditorCommand::Workspace(WorkspaceCommand::QuitBuffer)),
                            _ => None, // for unused chars
                        }
                    }
                    _ => None, // for unused key events
                }
            }
            _ => None, // for unused input events
        }
    }

    fn handle_command(&mut self, cmd: EditorCommand) -> Result<(), EditorError> {
        let _event = match cmd {
            EditorCommand::CreateWorkspace => self.add_workspace(self.viewport)?,
            EditorCommand::DeleteWorkspace => self.delete_active(),
            EditorCommand::Workspace(cmd) => {
                self.get_mut_workspace(self.active_id)?
                    .handle_command(cmd)?;
            }
            _ => {}
        };

        // self.handle_event(event)

        Ok(())
    }

    // fn handle_event(&mut self, event: EditorEvent) {
    //     match event {
    //         _ => {}
    //     }
    // }

    fn add_workspace(&mut self, viewport: Viewport) -> Result</*EditorEvent*/ (), EditorError> {
        if self.workspace_count > 9 {
            return Err(EditorError::Overflow);
        }

        let new_workspace = Workspace::new(viewport)?;
        self.workspaces.insert(self.next_id, new_workspace);
        self.workspace_count += 1;
        self.next_id += 1;

        // Ok(EditorEvent::WorkspaceCreated)
        Ok(())
    }

    fn delete_active(&mut self) // -> EditorEvent
    {
        self.workspaces.remove(&self.active_id);
        self.workspace_count -= 1;

        if self.workspace_count == 0 {
            panic!(); // for now
        }

        // EditorEvent::WorkspaceDeleted
    }

    fn get_workspace(&self, id: WorkspaceId) -> Result<&Workspace, EditorError> {
        match self.workspaces.get(&id) {
            Some(workspace) => Ok(workspace),
            None => return Err(EditorError::NullActive),
        }
    }

    fn get_mut_workspace(&mut self, id: WorkspaceId) -> Result<&mut Workspace, EditorError> {
        match self.workspaces.get_mut(&id) {
            Some(workspace) => Ok(workspace),
            None => return Err(EditorError::NullActive),
        }
    }
}
