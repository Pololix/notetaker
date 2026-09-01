#[derive(Debug, Clone)]
pub enum EditorCommand {
    Quit,

    // workspaces
    OpenWorkspace,
    QuitWorkspace,

    // modes
    EnterNormalMode,
    EnterPendOpMode,
    EnterInsertMode,
    EnterCmdlineMode,
    EnterTerminalMode,

    // buffers
    MoveFocus,
    OpenBuffer,
    QuitBuffer,
    SplitBuffer,
    AddBuffer,

    // buffer ops
    MoveCursor,
    Insert,
    Delete,
    DeleteSelection,
    Write,

    Undo,
    Redo,
    // todo: plugin commands
}
