use crate::workspace::SplitMode;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceCommand {
    OpenBuffer { viewport: (u32, u32) },
    CloseBuffer,
    SplitBuffer { mode: SplitMode },
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceEvent {
    BufferOpened,
    BufferClosed,
    BufferSplitted,
}
