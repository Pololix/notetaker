use crate::workspace::{MoveDirection, SplitMode};
use editor_common::geometry::Viewport;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceCommand {
    OpenBuffer { viewport: Viewport },
    SplitBuffer { mode: SplitMode },
    QuitBuffer,

    MoveActive { direction: MoveDirection },
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceEvent {
    BufferOpened,
    BufferQuit,

    ActiveChanged,
}
