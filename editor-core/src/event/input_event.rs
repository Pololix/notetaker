#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(KeyPress),
    // mouse, touchscreen...
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyPress {
    pub key: Key,
    pub mods: Mods,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Key {
    Character(String),

    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Delete,

    F(u8), // supported up to f12

    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Mods(u8);

impl Mods {
    pub const SHIFT: u8 = 1 << 0;
    pub const CTRL: u8 = 1 << 1;
    pub const ALT: u8 = 1 << 2;
    pub const SUPER: u8 = 1 << 3;

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn with(&mut self, key: u8) {
        // mods are only assigned if not in already
        if !self.contains(key) {
            self.0 += key;
        }
    }

    pub fn contains(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }
}
