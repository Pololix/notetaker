use crate::{
    event::{
        EditorCommand,
        input_event::{Key, KeyPress, Mods},
    },
    lua::LuaRuntimeError,
    user_mode::UserMode,
};
use mlua::{Lua, Table};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Debug, Clone)]
pub enum KeybindResolveResult {
    Match(EditorCommand),
    MatchOrPending(EditorCommand),
    Pending,
    NoMatch,
}

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<KeyPress, TrieNode>,
    cmd: Option<EditorCommand>,
}

#[derive(Debug, Clone)]
struct PendingState {
    pub cmd: Option<EditorCommand>,
    pub sequence: Vec<KeyPress>,
    pub deadline: Instant,
}

#[derive(Debug, Default)]
pub struct KeybindRegistry {
    binds: HashMap<UserMode, TrieNode>,

    pending: Option<PendingState>,
}

impl KeybindRegistry {
    pub fn load_lua_api(
        registry: &Rc<RefCell<Self>>,
        lua: &Lua,
        editor_table: &Table,
    ) -> Result<(), LuaRuntimeError> {
        let registry = Rc::clone(registry);
        let keybinds_table = lua.create_table()?;

        let set =
            lua.create_function(move |_, (mode, sequence, cmd): (String, String, String)| {
                let mode = Self::parse_mode(&mode)?;
                let sequence = Self::parse_sequence(&sequence)?;
                let cmd = Self::parse_command(&cmd)?;

                registry.borrow_mut().register(mode, &sequence, cmd);

                Ok(())
            })?;
        keybinds_table.set("set", set)?;

        editor_table.set("keybinds", keybinds_table)?;

        Ok(())
    }

    pub fn check_pending_deadline(&mut self, now: Instant) -> Option<EditorCommand> {
        if let Some(state) = self.pending.clone()
            && state.deadline <= now
        {
            self.pending = None;
            return state.cmd;
        }

        None
    }

    pub fn resolve(&mut self, mode: UserMode, key: KeyPress, now: Instant) -> KeybindResolveResult {
        // push to pending or create new sequence
        let sequence = match &self.pending {
            Some(state) => {
                let mut sequence = state.sequence.clone();
                sequence.push(key);
                sequence
            }
            None => vec![key],
        };

        let Some(mut node) = self.binds.get_mut(&mode) else {
            return KeybindResolveResult::NoMatch;
        };
        // get the current node and exit early if no match at any point of the sequence
        for key in &sequence {
            node = match node.children.get_mut(&key) {
                Some(child) => child,
                None => return KeybindResolveResult::NoMatch,
            }
        }

        match (node.cmd, node.children.is_empty()) {
            (Some(cmd), true) => {
                self.pending = None;
                KeybindResolveResult::Match(cmd)
            }
            (Some(cmd), false) => {
                self.pending = Some(PendingState {
                    cmd: Some(cmd),
                    sequence,
                    deadline: now + TIMEOUT,
                });
                KeybindResolveResult::MatchOrPending(cmd)
            }
            (None, true) => {
                self.pending = Some(PendingState {
                    cmd: None,
                    sequence,
                    deadline: now + TIMEOUT,
                });
                KeybindResolveResult::Pending
            }
            (None, false) => {
                self.pending = None;
                KeybindResolveResult::NoMatch
            }
        }
    }

    fn register(&mut self, mode: UserMode, sequence: &[KeyPress], cmd: EditorCommand) {
        let mut node = self.binds.entry(mode).or_default();
        for key in sequence {
            node = node.children.entry(key.clone()).or_default();
        }

        node.cmd = Some(cmd);
    }

    fn parse_mode(input: &str) -> Result<UserMode, LuaRuntimeError> {
        if input.is_empty() {
            return Err(LuaRuntimeError::InvalidArg(0));
        }

        match input {
            // use short or long form
            // todo: add suport for several modes (map the same in each mode)
            "n" | "normal" => Ok(UserMode::Normal),
            "v" | "visual" => Ok(UserMode::Visual),
            "i" | "insert" => Ok(UserMode::Insert),
            "c" | "command" => Ok(UserMode::Command),
            "t" | "terminal" => Ok(UserMode::Terminal),

            _ => Err(LuaRuntimeError::InvalidArg(0)),
        }
    }

    fn parse_sequence(input: &str) -> Result<Vec<KeyPress>, LuaRuntimeError> {
        if input.is_empty() {
            return Err(LuaRuntimeError::InvalidArg(1));
        }

        let mut sequence = Vec::new();
        while let Some(c) = input.chars().next() {
            match c != '<' {
                // read single chars as they are
                true => {
                    sequence.push(KeyPress {
                        key: Key::Character(c.to_string()),
                        mods: Mods::empty(),
                    });

                    continue;
                }

                // read special and modded keys inside angle brackets
                // mods are uppercase and separated between each and the key with a dash
                false => {
                    let mut token = String::new();
                    loop {
                        // return err if the string ends before closing the special key
                        let Some(c) = input.chars().next() else {
                            return Err(LuaRuntimeError::InvalidArg(1));
                        };

                        // end on closing bracket
                        if c == '>' {
                            sequence.push(Self::parse_token(&token)?);
                        }

                        // if some and non-'>' push to the token
                        token.push(c);
                    }
                }
            }
        }

        if sequence.is_empty() {
            return Err(LuaRuntimeError::InvalidArg(1));
        }

        Ok(sequence)
    }

    fn parse_command(input: &str) -> Result<EditorCommand, LuaRuntimeError> {
        if input.is_empty() {
            return Err(LuaRuntimeError::InvalidArg(2));
        }

        match input {
            // global
            "undo" => Ok(EditorCommand::Undo),
            "redo" => Ok(EditorCommand::Redo),

            // workspaces
            "workspace.open" => Ok(EditorCommand::OpenWorkspace),
            "workspace.quit" => Ok(EditorCommand::QuitWorkspace),

            // modes
            "mode.normal" => Ok(EditorCommand::EnterNormalMode),
            "mode.insert" => Ok(EditorCommand::EnterInsertMode),
            "mode.cmdline" => Ok(EditorCommand::EnterCmdlineMode),
            "mode.terminal" => Ok(EditorCommand::EnterTerminalMode),

            // buffers
            "buffer.move_focus" => Ok(EditorCommand::MoveFocus),
            "buffer.open" => Ok(EditorCommand::OpenBuffer),
            "buffer.split" => Ok(EditorCommand::SplitBuffer),
            "buffer.write" => Ok(EditorCommand::WriteBuffer),
            "buffer.quit" => Ok(EditorCommand::QuitBuffer),

            // buffer ops
            "buffer.op.move_cursor" => Ok(EditorCommand::MoveCursor),
            "buffer.op.insert" => Ok(EditorCommand::Insert),
            "buffer.op.delete" => Ok(EditorCommand::Delete),
            "buffer.op.delete_selection" => Ok(EditorCommand::DeleteSelection),

            _ => Err(LuaRuntimeError::InvalidArg(2)),
        }
    }

    fn parse_token(input: &str) -> Result<KeyPress, LuaRuntimeError> {
        let mut parts: Vec<_> = input.split('-').collect();

        // fetch key
        let key = match parts.pop() {
            Some(key) => Self::parse_key(key)?,
            None => return Err(LuaRuntimeError::InvalidArg(1)),
        };

        // fecth mods from spare parts
        // if no mods, key pressed is assigned an empty Mods
        let mut mods = Mods::empty();
        for part in parts {
            match part {
                "SHIFT" => mods.with(Mods::SHIFT),
                "CTRL" => mods.with(Mods::CTRL),
                "ALT" => mods.with(Mods::ALT),
                "SUPER" => mods.with(Mods::SUPER),

                _ => return Err(LuaRuntimeError::InvalidArg(1)),
            }
        }

        Ok(KeyPress { key, mods })
    }

    fn parse_key(input: &str) -> Result<Key, LuaRuntimeError> {
        match input {
            // if a single char treat as char
            input if input.chars().count() == 1 => {
                Ok(Key::Character(input.chars().next().unwrap().to_string()))
            }

            "Space" => Ok(Key::Space),
            "Enter" => Ok(Key::Enter),
            "Escape" => Ok(Key::Escape),
            "Backspace" => Ok(Key::Backspace),
            "Tab" => Ok(Key::Tab),
            "Delete" => Ok(Key::Delete),

            "F1" => Ok(Key::F(1)),
            "F2" => Ok(Key::F(2)),
            "F3" => Ok(Key::F(3)),
            "F4" => Ok(Key::F(4)),
            "F5" => Ok(Key::F(5)),
            "F6" => Ok(Key::F(6)),
            "F7" => Ok(Key::F(7)),
            "F8" => Ok(Key::F(8)),
            "F9" => Ok(Key::F(9)),
            "F10" => Ok(Key::F(10)),
            "F11" => Ok(Key::F(11)),
            "F12" => Ok(Key::F(12)),

            "Up" => Ok(Key::Up),
            "Down" => Ok(Key::Down),
            "Left" => Ok(Key::Left),
            "Right" => Ok(Key::Right),

            _ => Err(LuaRuntimeError::InvalidArg(1)),
        }
    }
}
