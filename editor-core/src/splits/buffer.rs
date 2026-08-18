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

    // pub fn new_from_path(x: f32, y: f32, width: f32, height: f32, path: &Path) -> Self {

    // pub fn set_path(&mut self, path: &Path) {

    // insert
    // delete
    // backspace
    // write
    // quit

    // move cursor
}
