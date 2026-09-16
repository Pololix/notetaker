use crate::document::grid::filler::FillerAttachment;

#[derive(Debug)]
pub struct GridAttachment {
    rect: GridRect,
    ty: AttachmentType,
}

impl GridAttachment {
    pub fn occupies_row(&self, row: u32) -> bool {
        row >= self.rect.row_start && row < self.rect.row_end
    }

    pub fn occupies_col(&self, col: u32) -> bool {
        col >= self.rect.col_start && col < self.rect.col_end
    }

    pub fn occupies_cell(&self, row: u32, col: u32) -> bool {
        self.occupies_row(row) && self.occupies_col(col)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GridRect {
    pub col_start: u32,
    pub row_start: u32,
    pub col_end: u32,
    pub row_end: u32,
}

#[derive(Debug)]
pub enum AttachmentType {
    Filler(FillerAttachment),
}
