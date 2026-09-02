use std::path::Path;

use mlua::{Lua, StdLib};

// todo: works on single pc
const CONFIG_DIR: &str = "/home/Pablo/.config";

#[derive(Debug, thiserror::Error)]
pub enum LuaRuntimeError {
    #[error("Failed to execute a Lua operation: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Failed to retrieve a config file: {0}")]
    InvalidConfigPath(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new() -> Result<Self, LuaRuntimeError> {
        let lua = Lua::new();
        // implement package for require function
        lua.load_std_libs(StdLib::PACKAGE)?;

        let mut new_runtime = LuaRuntime { lua };

        // load subsystems api

        new_runtime.load_config(Path::new(CONFIG_DIR))?;

        Ok(new_runtime)
    }

    fn load_config(&mut self, config_dir: &Path) -> Result<(), LuaRuntimeError> {
        let source_path = config_dir.join("editor").join("editor.lua");
        let source = std::fs::read_to_string(source_path)?;

        match self.lua.load(&source).exec() {
            Ok(_) => Ok(()),
            Err(_) => todo!("Implement fallback after null config file"),
        }
    }
}
