use crate::prefix::Prefix;
use crate::value::Value;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Instruction {
    pub prefix: Prefix,
    pub name: Option<String>,
    pub value: Value,
}

impl Instruction {
    pub fn new(name: &str, value: Value) -> Self {
        Self {
            value,
            name: Some(name.into()),
            prefix: Default::default(),
        }
    }

    pub fn new_mapping(value: Value) -> Self {
        if value.is_mapping() || value.is_unquoted_mapping() {
            Self {
                value,
                name: None,
                prefix: Default::default(),
            }
        } else {
            panic!("value type must be [Unquoted]Mapping")
        }
    }

    pub fn prefix(&mut self, prefix: Prefix) -> &mut Self {
        self.prefix = prefix;

        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.into());

        self
    }
}

#[allow(clippy::collapsible_if)]
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}:{}={}", self.prefix, name, self.value)
        } else {
            if let Value::Mapping(key, value) = &self.value {
                write!(f, "{}:{}={}", self.prefix, key, value)
            } else if let Value::UnquotedMapping(key, value) = &self.value {
                write!(f, "{}:{}={}", self.prefix, key, value)
            } else {
                panic!("value type must be [Unquoted]Mapping")
            }
        }
    }
}
