use crate::document::attachment::Attachment;
use ropey::Rope;

#[derive(Debug, Clone)]
pub struct Document {
    text: Rope,

    pub cell_size: f32,
    pub grid_width: u32,
    pub grid_height: u32,
    grid_attachments: Vec<Attachment>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            text: Rope::from_str(
                "NTK — Native Text Editor\n\
                 \n\
                 This is the first document.\n\
                 \n\
                 The renderer is currently being developed.\n\
                 This text is only here for debugging.\n\
                 \n\
                 Line 7\n\
                 Line 8\n\
                 Line 9\n\
                 \n\
                 Scrolling and input will come later.\n\
                 For now, we are just testing rendering.",
            ),

            cell_size: 20.0,
            grid_width: 32,
            grid_height: 96,
            grid_attachments: Vec::new(),
        }
    }
}

impl Document {
    pub fn occupied_cells(&self) -> Vec<usize> {
        for attachment in &self.grid_attachments {}
        vec![]
    }

    pub fn text(&self) -> String {
        self.text.to_string()
    }
}
