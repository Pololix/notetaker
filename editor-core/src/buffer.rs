// use ropey::Rope;
// use std::{
//     fs::File,
//     path::{Path, PathBuf},
// };

pub type BufferId = usize;

#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    pub id: BufferId,
    // path: Option<PathBuf>,
    // contents: Rope,
    // cursor: usize,
    // flags: BufferFlags,
}

impl Buffer {
    pub fn new(id: BufferId) -> Self {
        Self { id }
    }

    // quit

    // open file
    // write

    // insert
    // delete
    // backspace

    // move cursor
}
