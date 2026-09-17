use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UserMode {
    Normal,
    Insert,
}

impl Default for UserMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromStr for UserMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "insert" => Ok(Self::Insert),

            _ => Err(()),
        }
    }
}
