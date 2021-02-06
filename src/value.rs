//! This contains the [`Value`](Value) struct.
use std::fmt;

/// The value of an [`Instruction`](crate::Instruction).
#[derive(Clone)]
pub enum Value {
    /// A singular value.
    Singular(String),

    /// A mapping (2 values).
    Mapping(String, String),

    /// A mapping with an optional key.
    OptionalKey(Option<String>, String),

    /// A mapping with an optional key. Once printed, the value won't have quotes.
    UnquotedOptionalKey(Option<String>, String),

    /// A mapping with an optional value.
    OptionalValue(String, Option<String>),

    /// A mapping with an optional value. Once printed, if the value exists, the value won't have
    /// quotes.
    UnquotedOptionalValue(String, Option<String>),

    /// A mapping with the value not having quotes.
    UnquotedMapping(String, String),
}

impl Value {
    /// Returns `true` if the value is a [`Singular`](Self::Singular) value.
    pub fn is_singular(&self) -> bool {
        matches!(*self, Self::Singular(_))
    }

    /// Returns `true` if the value is a [`Mapping`](Self::Mapping) value.
    pub fn is_mapping(&self) -> bool {
        matches!(*self, Self::Mapping(_, _))
    }

    /// Returns `true` if the value is an [`OptionalKey`](Self::OptionalKey) value.
    pub fn is_optional_key(&self) -> bool {
        matches!(*self, Self::OptionalKey(_, _))
    }

    /// Returns `true` if the value is an [`UnquotedOptionalKey`](Self::UnquotedOptionalKey) value.
    pub fn is_unquoted_optional_key(&self) -> bool {
        matches!(*self, Self::UnquotedOptionalKey(_, _))
    }

    /// Returns `true` if the value is an [`OptionalValue`](Self::OptionalValue) value.
    pub fn is_optional_value(&self) -> bool {
        matches!(*self, Self::OptionalValue(_, _))
    }

    /// Returns `true` if the value is an [`UnquotedOptionalValue`](Self::UnquotedOptionalValue) value.
    pub fn is_unquoted_optional_value(&self) -> bool {
        matches!(*self, Self::Mapping(_, _))
    }

    /// Returns `true` if the value is an [`UnquotedMapping`](Self::UnquotedMapping) value.
    pub fn is_unquoted_mapping(&self) -> bool {
        matches!(*self, Self::UnquotedMapping(_, _))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Singular(value) => write!(f, "{}", value),
            Self::Mapping(key, value) => write!(f, "{}=\"{}\"", key, value),
            Self::OptionalKey(key, value) => {
                if let Some(key) = key {
                    write!(f, "{}=\"{}\"", key, value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Self::UnquotedOptionalKey(key, value) => {
                if let Some(key) = key {
                    write!(f, "{}={}", key, value)
                } else {
                    write!(f, "{}", value)
                }
            }
            Self::OptionalValue(key, value) => {
                if let Some(value) = value {
                    write!(f, "{}=\"{}\"", key, value)
                } else {
                    write!(f, "{}", key)
                }
            }
            Self::UnquotedOptionalValue(key, value) => {
                if let Some(value) = value {
                    write!(f, "{}={}", key, value)
                } else {
                    write!(f, "{}", key)
                }
            }
            Self::UnquotedMapping(key, value) => write!(f, "{}={}", key, value),
        }
    }
}
