//! This contains the [`Prefix`](Prefix) enum.
use std::fmt;

/// The prefix. Usually [`Cargo`](Self::Cargo).
#[derive(Clone)]
pub enum Prefix {
    /// The cargo prefix. 99% of the time this is used.
    Cargo,

    /// Other, custom prefixes.
    Custom(String),
}

impl Default for Prefix {
    /// The default prefix is [`Cargo`](Self::Cargo).
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
