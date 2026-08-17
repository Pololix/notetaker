use ropey::Rope;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    path: Option<PathBuf>,
    contents: Rope,
    cursor: usize,
    //flags: BUfferFlags,
}

impl Buffer {
    // constructors
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,

            path: None,
            contents: Rope::new(),
            cursor: 0,
        }
    }

    pub fn new_from_path(x: f32, y: f32, width: f32, height: f32, path: &Path) -> Self {
        Self {
            x,
            y,
            width,
            height,

            path: Some(path.to_path_buf()),
            contents: Rope::from_reader(File::open(path).expect("Failed to open file"))
                .expect("Failed to read from file"),
            cursor: 0,
        }
    }

    pub fn quit(&mut self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }

    // path
    pub fn set_path(&mut self, path: &Path) {
        self.path = Some(path.to_path_buf());
    }

    // contents
    // insert
    // delete
    // backspace

    // cursor
    // move cursor
}
