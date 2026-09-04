#[derive(Debug, Clone, Copy)]
pub enum EditorCommand {
    // global
    Quit,
    Undo,
    Redo,

    // workspaces
    OpenWorkspace,
    QuitWorkspace,

    // modes
    EnterNormalMode,
    EnterVisualMode,
    EnterInsertMode,
    EnterCmdlineMode,
    EnterTerminalMode,

    // buffers
    MoveFocus,
    OpenBuffer,
    SplitBuffer,
    WriteBuffer,
    QuitBuffer,

    // buffer ops
    MoveCursor,
    Insert,
    Delete,
    DeleteSelection,
    // todo: plugin commands
}
