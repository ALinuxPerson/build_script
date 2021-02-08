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

#[cfg(test)]
mod tests {
    use crate::Prefix;

    #[test]
    fn test_default() {
        let prefix = Prefix::default();
        assert!(matches!(prefix, Prefix::Cargo))
    }

    #[test]
    fn test_display_cargo() {
        let prefix = Prefix::Cargo;
        let string = format!("{}", prefix);
        assert_eq!(string, "cargo")
    }

    #[test]
    fn test_display_custom() {
        let prefix = Prefix::Custom("custom".into());
        let string = format!("{}", prefix);
        assert_eq!(string, "custom")
    }
}
