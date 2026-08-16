use std::{fs::File, path::Path};

pub enum CursorDirection {
    Left,
    Right,
    //Up,
    //Down,
}

pub struct Buffer {
    pub rope: ropey::Rope,
    cursor: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: ropey::Rope::new(),
            cursor: 0,
        }
    }

    pub fn new_from_file(path: &Path) -> Self {
        Self {
            rope: ropey::Rope::from_reader(File::open(path).expect("Failed to open the file"))
                .expect("Failed to read from file"),
            cursor: 0,
        }
    }

    // later include vertical movement and vim motions
    pub fn move_cursor(&mut self, dir: CursorDirection) {
        match dir {
            CursorDirection::Left => {
                if self.cursor == 0 {
                    return;
                }
                self.cursor -= 1;
            }
            CursorDirection::Right => {
                if self.cursor >= self.rope.len_chars() {
                    return;
                }
                self.cursor += 1;
            }
        }
    }

    // for now a single char -> sub or add a new method for chunks
    // see also for moving whole chunks of text (visual mode)
    pub fn insert(&mut self, char: char) {
        self.rope.insert_char(self.cursor, char);
        self.move_cursor(CursorDirection::Right);
    }
    // for now single char -> sub or add method for chunks (visual mode or motions might trigger a
    // chunk instead of a single byte)
    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let target = self.cursor - 1;
        self.rope.remove(target..=target);
        self.move_cursor(CursorDirection::Left);
    }

    pub fn delete(&mut self) {
        // see panicking on cursor = 0, len = 0 but no chars: empty rope -> panic
        if self.cursor >= self.rope.len_chars() || self.rope.len_chars() == 0 {
            return;
        }

        self.rope.remove(self.cursor..=self.cursor);
    }
}
