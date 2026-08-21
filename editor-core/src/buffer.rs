use editor_common::Rect;
use ropey::Rope;
// use std::{
//     fs::File,
//     path::{Path, PathBuf},
// };

pub type BufferId = usize;

#[derive(Debug)]
pub struct Buffer {
    pub id: BufferId,
    // path: Option<PathBuf>,
    contents: Rope,
    cursor: usize,
    // flags: BufferFlags,
}

#[derive(Debug)]
pub struct BufferView {
    pub surface: Rect,
    pub text: String,
    pub cursor: usize,
}

impl Buffer {
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            contents: Rope::new(),
            cursor: 0,
        }
    }

    pub fn get_view(&self, surface: Rect) -> BufferView {
        BufferView {
            surface,
            text: self.contents.to_string(),
            cursor: self.cursor,
        }
    }

    // quit

    // open file
    // write

    // insert
    // delete
    // backspace

    // move cursor
}
