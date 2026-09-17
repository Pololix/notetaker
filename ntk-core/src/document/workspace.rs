use crate::document::document::Document;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceError {}

#[derive(Debug, Default)]
pub struct Workspace {
    document: Document,
}

impl Workspace {}
