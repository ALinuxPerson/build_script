use std::fmt;

#[derive(Clone)]
pub enum Value {
    Singular(String),
    Mapping(String, String),
    OptionalKey(Option<String>, String),
    UnquotedOptionalKey(Option<String>, String),
    OptionalValue(String, Option<String>),
    UnquotedOptionalValue(String, Option<String>),
    UnquotedMapping(String, String),
}

impl Value {
    pub fn is_singular(&self) -> bool {
        matches!(*self, Self::Singular(_))
    }

    pub fn is_mapping(&self) -> bool {
        matches!(*self, Self::Mapping(_, _))
    }

    pub fn is_optional_key(&self) -> bool {
        matches!(*self, Self::OptionalKey(_, _))
    }

    pub fn is_unquoted_optional_key(&self) -> bool {
        matches!(*self, Self::UnquotedOptionalKey(_, _))
    }

    pub fn is_optional_value(&self) -> bool {
        matches!(*self, Self::OptionalValue(_, _))
    }

    pub fn is_unquoted_optional_value(&self) -> bool {
        matches!(*self, Self::Mapping(_, _))
    }

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
