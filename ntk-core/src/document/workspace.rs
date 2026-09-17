use crate::{
    document::{UserMode, document::Document},
    render::Frame,
};

#[derive(Debug, Default)]
pub struct Workspace {
    document: Document,
}

impl Workspace {
    pub fn render(&self, frame: &mut Frame, mode: UserMode) {
        // for now it is single document so render the single document
        // todo: add support for several document views
        self.document.render(frame, mode);
    }
}
