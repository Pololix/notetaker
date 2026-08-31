use crate::user_mode::UserMode;
use mlua::{Lua, Result};

#[derive(Debug)]
struct Keybind {
    mode: UserMode,
    sequence: String,
    action: String, // not a string
}

#[derive(Debug)]
pub struct KeybindRegistry {
    binds: Vec<Keybind>, // consider using hashmap
}

impl KeybindRegistry {
    pub fn new(lua: &Lua) -> Result<Self> {
        let map =
            lua.create_function(|_, (mode, sequence, action): (String, String, String)| {
                // turn mode into UserMode
                // store the sequence as String
                // get the action
                Ok(())
            })?;

        lua.globals().set("editor.keybinds.map", map)?;

        Ok(Self { binds: Vec::new() })
    }
}
