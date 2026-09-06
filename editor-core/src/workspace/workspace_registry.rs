use crate::{
    user_mode::UserMode,
    util::Viewport,
    workspace::{
        BufferRegistry,
        workspace::{Workspace, WorkspaceId},
    },
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WorkspaceRegistry {
    user_mode: UserMode,
    viewport: Viewport,

    workspaces: HashMap<WorkspaceId, Workspace>,
    buffers: BufferRegistry,
}

impl WorkspaceRegistry {
    pub fn new() -> Self {
        Self {
            user_mode: UserMode::Normal,
            viewport: Viewport {
                width: 0,
                height: 0,
            },

            workspaces: HashMap::new(),
            buffers: BufferRegistry::default(),
        }
    }
}
