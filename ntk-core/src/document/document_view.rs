use crate::{
    document::document::Document,
    event::RenderCommandWriter,
    render::{
        RenderCommand, RenderId, RenderIdAllocator,
        types::{Color, Rect},
    },
};

const MARGIN: f32 = 10.0;
const DOCUMENT_MARGIN: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct DocumentView {
    // for now -> change to an id/hashmap system
    document: Document,
    rect: Rect,

    doc_rect: Rect,
    scroll_x: f32,
    scroll_y: f32,
    zoom: f32,

    quad_id: RenderId,
    grid_id: RenderId,
    text_id: RenderId,
    dirty: bool,
}

impl DocumentView {
    pub fn new(rect: Rect, ids: &mut RenderIdAllocator) -> Self {
        let mut view = Self {
            document: Document::default(),
            rect,

            doc_rect: rect,
            scroll_x: 0.0,
            scroll_y: 0.0,
            zoom: 1.0,

            quad_id: ids.next(),
            grid_id: ids.next(),
            text_id: ids.next(),
            dirty: true,
        };

        view.update_view();
        view
    }

    pub fn render(&mut self, cmd_writer: &mut RenderCommandWriter) {
        if !self.dirty {
            return;
        }

        cmd_writer.push(RenderCommand::Quad {
            id: self.quad_id,
            rect: Rect {
                x: self.doc_rect.x - DOCUMENT_MARGIN,
                y: self.doc_rect.y - DOCUMENT_MARGIN,
                width: self.doc_rect.width + DOCUMENT_MARGIN * 2.0,
                height: self.doc_rect.height + DOCUMENT_MARGIN * 2.0,
            },
            color: Color::BLACK,
        });

        cmd_writer.push(RenderCommand::DocumentGrid {
            id: self.grid_id,
            rect: self.doc_rect,
            color: Color::GREY,

            x_offset: self.scroll_x,
            y_offset: self.scroll_y,
            cell_size: CELL_SIZE * self.zoom,
        });

        cmd_writer.push(RenderCommand::DocumentText {
            id: self.text_id,
            rect: self.doc_rect,
            text: self.document.text(),
            color: Color::WHITE,
        });

        self.dirty = false;
    }

    pub fn set_rect(&mut self, rect: Rect) {
        if self.rect == rect {
            return;
        }

        self.rect = rect;
        self.update_view();
        self.dirty = true;
    }

    pub fn set_zoom(&mut self, delta: f32) {
        let zoom = (self.zoom + delta).max(0.1);
        if self.zoom == zoom {
            return;
        }

        self.zoom = zoom;
        self.update_view();
        self.dirty = true;
    }

    fn update_view(&mut self) {
        // allocated space with margins applied to them -> smaller rect
        let inner = Rect {
            x: self.rect.x + MARGIN,
            y: self.rect.y + MARGIN,
            width: (self.rect.width - MARGIN * 2.0).max(0.0),
            height: (self.rect.height - MARGIN * 2.0).max(0.0),
        };

        // applying document margins yields the maximum width of the document
        let max_width = (inner.width - DOCUMENT_MARGIN * 2.0).max(0.0);

        // if the document with the applied zoom or due to viewport is bigger than it can be it is
        // clamped
        let width = (self.document.logical_width() * self.zoom).min(max_width);

        self.doc_rect = Rect {
            x: inner.x + DOCUMENT_MARGIN + (max_width - width) * 0.5,
            y: inner.y + DOCUMENT_MARGIN,
            width,
            height: (inner.height - DOCUMENT_MARGIN * 2.0).max(0.0),
        }
    }
}
