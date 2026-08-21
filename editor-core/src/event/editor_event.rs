use crate::{editor::UserMode, event::workspace_event::WorkspaceCommand};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum EditorCommand {
    ChangeUserMode { to: UserMode },

    Workspace(WorkspaceCommand),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum EditorEvent {
    UserModeChanged,
}
