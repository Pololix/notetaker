use crate::{
    editor::UserMode,
    event::workspace_event::{WorkspaceCommand, WorkspaceEvent},
};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum EditorCommand {
    ChangeUserMode { to: UserMode },
    CreateWorkspace,
    DeleteWorkspace,

    Workspace(WorkspaceCommand),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum EditorEvent {
    UserModeChanged,
    WorkspaceCreated,
    WorkspaceDeleted,

    Workspace(WorkspaceEvent),
}
