use std::fmt;

#[derive(Clone)]
pub enum Prefix {
    Cargo,
    Custom(String),
}

impl Default for Prefix {
    fn default() -> Self {
        Self::Cargo
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cargo => write!(f, "cargo"),
            Self::Custom(prefix) => write!(f, "{}", prefix),
        }
    }
}
