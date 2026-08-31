use mlua::{Lua, StdLib};
use std::path::Path;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum LuaRuntimeError {
    #[error("Failed to retrieve Home directory")]
    NullHomeDir,

    #[error("Failed to retrieve editor.lua from config dir: {0}")]
    NullConfig(#[from] std::io::Error),

    #[error("Failed to perform a Lua operation: {0}")]
    Lua(#[from] mlua::Error),
}

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new() -> Result<Self, LuaRuntimeError> {
        let lua = Lua::new();
        lua.load_std_libs(StdLib::PACKAGE)?; // require capabilities

        let mut new_runtime = Self { lua };

        let config_dir = match std::env::home_dir() {
            Some(dir) => dir.join(".config").join("editor").join("editor.lua"),
            None => return Err(LuaRuntimeError::NullHomeDir),
        };
        new_runtime.load_config(&config_dir)?;

        Ok(new_runtime)
    }

    pub fn load_config(&mut self, path: &Path) -> Result<(), LuaRuntimeError> {
        let source = std::fs::read_to_string(&path)?;
        self.lua.load(&source).exec()?;

        Ok(())
    }
}
