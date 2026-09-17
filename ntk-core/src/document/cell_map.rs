use crate::document::attachment::Attachment;

#[derive(Debug)]
pub struct CellMap {
    width: u32,
    height: u32,
    attachments: Vec<Attachment>,
}

impl Default for CellMap {
    fn default() -> Self {
        Self {
            width: 32,
            height: 96,
            attachments: Vec::new(),
        }
    }
}

impl CellMap {
    pub fn cell_size(&self, usable_width: f32) -> f32 {
        usable_width / self.width as f32
    }
}
