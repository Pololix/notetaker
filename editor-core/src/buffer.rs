use ropey::Rope;
// use std::{
//     fs::File,
//     path::{Path, PathBuf},
// };

pub type BufferId = usize;

#[derive(Debug)]
pub struct Buffer {
    // path: Option<PathBuf>,
    contents: Rope,
    // flags: BufferFlags,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            contents: Rope::new(),
        }
    }

    pub fn get_text(&self) -> String {
        "Default text............................................................................................................................".to_string()
    }

    // quit

    // open file
    // write

    // insert
    // delete
    // backspace

    // move cursor
}
