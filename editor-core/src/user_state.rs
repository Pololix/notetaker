pub enum UserMode {
    Normal,
    //PendingOp,
    Insert,
    Cmdline,
    //Terminal,
    //Visual,
}

pub struct UserState {
    mode: UserMode,
}
