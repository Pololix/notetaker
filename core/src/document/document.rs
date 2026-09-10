#[derive(Clone, Debug)]
pub struct Document {
    pub title: String,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
        }
    }
}
