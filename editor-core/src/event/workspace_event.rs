use crate::workspace::SplitMode;
use editor_common::geometry::Viewport;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceCommand {
    OpenBuffer { viewport: Viewport },
    SplitBuffer { mode: SplitMode },
    QuitBuffer,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceEvent {
    BufferOpened,
    BufferQuit,
}
