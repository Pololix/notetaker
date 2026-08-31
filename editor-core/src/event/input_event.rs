#[derive(Debug, Clone)]
pub enum InputEvent {
    Key { key: Key, mods: Modifiers },
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
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}
