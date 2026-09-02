#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(KeyPress),
    // mouse, touchscreen...
}

#[derive(Debug, Clone)]
pub struct KeyPress {
    key: Key,
    mods: Mods,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Mods(u8);

impl Mods {
    const SHIFT: u8 = 1 << 0;
    const CTRL: u8 = 1 << 1;
    const ALT: u8 = 1 << 2;
    const SUPER: u8 = 1 << 3;

    pub fn contains(self, mask: u8) -> bool {
        self.0 & mask != 0
    }
}
