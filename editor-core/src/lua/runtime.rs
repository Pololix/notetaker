use crate::{event::EventBus, lua::keybinds::KeybindRegistry};
use mlua::{Lua, StdLib};
use std::{cell::RefCell, path::Path, rc::Rc};

// todo: works on single pc
const CONFIG_DIR: &str = "/home/Pablo/.config";

#[derive(Debug, thiserror::Error)]
pub enum LuaRuntimeError {
    #[error("Failed to execute a Lua operation: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Failed to retrieve a config file: {0}")]
    InvalidConfigPath(#[from] std::io::Error),

    // todo: add more info (file name, line...)
    #[error("Invalid argument at position {0}")]
    InvalidArg(u8),
}

impl From<LuaRuntimeError> for mlua::Error {
    fn from(err: LuaRuntimeError) -> Self {
        mlua::Error::external(err)
    }
}
#[derive(Debug)]
pub struct LuaRuntime {
    lua: Lua,

    pub keybinds: Rc<RefCell<KeybindRegistry>>,
}

impl LuaRuntime {
    pub fn new(event_bus: &mut EventBus) -> Result<Self, LuaRuntimeError> {
        let lua = Lua::new();
        // implement package for require function
        lua.load_std_libs(StdLib::PACKAGE)?;

        // create subsystems and load lua api
        let editor_table = lua.create_table()?;

        let keybinds = Rc::new(RefCell::new(KeybindRegistry::default()));
        KeybindRegistry::load_lua_api(&keybinds, &lua, &editor_table)?;

        // register systems to the bus if necessary
        KeybindRegistry::subscribe_to_bus(&keybinds, &mut event_bus);

        // assemble and fetch config
        let mut runtime = LuaRuntime { lua, keybinds };
        runtime.load_config(Path::new(CONFIG_DIR))?;

        Ok(runtime)
    }

    fn load_config(&mut self, config_dir: &Path) -> Result<(), LuaRuntimeError> {
        let source_path = config_dir.join("editor").join("editor.lua");
        let source = std::fs::read_to_string(source_path)?;

        // todo: act upon invalid or null config file (load fallback or panic if unable)
        self.lua
            .load(&source)
            .exec()
            .expect("Failed to load lua config file");

        Ok(())
    }
}
