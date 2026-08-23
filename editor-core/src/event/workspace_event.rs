use crate::workspace::SplitMode;
use editor_common::Viewport;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceCommand {
    OpenBuffer { viewport: Viewport },
    QuitBuffer,
    SplitBuffer { mode: SplitMode },
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceEvent {
    BufferOpened,
    BufferQuit,
    BufferSplitted,
}
