use crate::{
    Frame, Viewport,
    document::{UserMode, Workspace},
};

#[derive(Debug, thiserror::Error)]
pub enum EditorError {
    #[error("Failed to render current frame")]
    Rendering,
}

#[derive(Debug)]
pub struct Editor {
    mode: UserMode,
    viewport: Viewport,

    workspace: Workspace,
}

impl Editor {
    pub fn new(viewport: Viewport) -> Self {
        Self {
            mode: UserMode::default(),
            viewport,

            workspace: Workspace::default(),
        }
    }

    pub fn resize(&mut self, viewport: Viewport) {
        if viewport.width == 0 || viewport.height == 0 {
            return;
        }

        self.viewport = viewport;
    }

    pub fn render(&mut self) -> Frame {
        let mut new_frame = Frame::new(self.viewport);

        self.workspace.render(&mut new_frame, self.mode);

        new_frame
    }
}
