#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum InputEvent {
    Key {
        key: Key,
        state: KeyState,
        mods: Modifiers,
    },
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}
