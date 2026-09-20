use crate::{
    document::document_view::DocumentView,
    event::RenderCommandWriter,
    render::{RenderIdAllocator, Viewport, types::Rect},
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceError {}

#[derive(Debug)]
pub struct Workspace {
    render_ids: RenderIdAllocator,
    view: DocumentView,
}

impl Workspace {
    pub fn new(viewport: Viewport) -> Self {
        let mut render_ids = RenderIdAllocator::default();
        let view = DocumentView::new(
            Rect {
                x: 0.0,
                y: 0.0,
                width: viewport.width as f32,
                height: viewport.height as f32,
            },
            render_ids.next(),
        );

        Self { render_ids, view }
    }

    pub fn render(&mut self, cmd_writer: &mut RenderCommandWriter) {
        self.view.render(cmd_writer);
    }
}
