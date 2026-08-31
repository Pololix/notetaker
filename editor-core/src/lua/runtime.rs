use mlua::Lua;
use std::path::Path;

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn source_config(&mut self, path: &Path) {}
}
