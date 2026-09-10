#[derive(Debug, Clone)]
pub struct KeyPress {
    key: Key,
    mods: Mods,
}

#[derive(Debug, Clone)]
pub enum Key {
    Char(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Mods(u8);

impl Mods {
    const SHIFT: u8 = 1;
    const ALT: u8 = 1 << 1;
    const CTRL: u8 = 1 << 2;
    const SUPER: u8 = 1 << 3;

    pub fn contains(self, key: u8) -> bool {
        self.0 & key != 0
    }

    pub fn with_key(self, key: u8) -> Self {
        if self.contains(key) {
            return self;
        }

        Self(self.0 + key)
    }
}
