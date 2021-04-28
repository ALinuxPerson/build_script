//! This contains the [`Instruction`](Instruction) struct.
use crate::prefix::Prefix;
use crate::value::Value;
use std::fmt;
use std::fmt::Formatter;

/// An instruction. Used as a rusty way to parse arguments in build scripts.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Instruction {
    /// The prefix. Usually [`Cargo`](Prefix::Cargo).
    pub prefix: Prefix,

    /// The name of the instruction. Most of the time it's filled in, only when a new mapping
    /// [`new_mapping()`](Self::new_mapping) is created it's not.
    pub name: Option<String>,

    /// The [`Value`](Value) of the Instruction.
    pub value: Value,
}

impl Instruction {
    /// Create a new Instruction. 99% of the time this should suffice instead of
    /// [`new_mapping()`](Self::new_mapping).
    pub fn new(name: &str, value: Value) -> Self {
        Self {
            value,
            name: Some(name.into()),
            prefix: Default::default(),
        }
    }

    /// Create a new Instruction Mapping. Only 1% of the time is this proven useful, instead use
    /// [new()](Self::new).
    /// # Panics
    /// This panics if `value` is not a [`Mapping`](Value::Mapping) or
    /// [`UnquotedMapping`](Value::UnquotedMapping).
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

    /// Set the prefix.
    pub fn prefix(&mut self, prefix: Prefix) -> &mut Self {
        self.prefix = prefix;

        self
    }

    /// Set the name.
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

#[cfg(test)]
mod tests {
    use crate::{Instruction, Value, Prefix};

    macro_rules! new_wrong_value_test {
        ($name:ident, $value:expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                Instruction::new_mapping($value);
            }
        };
    }

    macro_rules! new_right_value_test {
        ($name:ident, $value:expr) => {
            #[test]
            fn $name() {
                Instruction::new_mapping($value);
            }
        };
    }

    #[test]
    fn test_new() {
        let value = Value::Singular("singular".into());
        let instruction = Instruction::new("name", value);
        assert!(matches!(instruction.prefix, Prefix::Cargo));
        assert_eq!(instruction.name.expect("name is none"), "name");
        assert!(instruction.value.is_singular());
    }

    #[test]
    fn test_new_mapping() {
        let value = Value::Mapping("key".into(), "value".into());
        let instruction = Instruction::new_mapping(value);
        assert!(instruction.name.is_none());
    }

    new_wrong_value_test!(test_new_mapping_fails_if_value_singular, Value::Singular("".into()));
    new_wrong_value_test!(test_new_mapping_fails_if_value_optional_key, Value::OptionalKey(None, "".into()));
    new_wrong_value_test!(test_new_mapping_fails_if_value_unquoted_optional_key, Value::UnquotedOptionalKey(None, "".into()));
    new_wrong_value_test!(test_new_mapping_fails_if_value_optional_value, Value::OptionalValue("".into(), None));
    new_wrong_value_test!(test_new_mapping_fails_if_value_unquoted_optional_value, Value::UnquotedOptionalValue("".into(), None));
    new_right_value_test!(test_new_mapping_succeeds_if_value_mapping, Value::Mapping("".into(), "".into()));
    new_right_value_test!(test_new_mapping_succeeds_if_value_unquoted_mapping, Value::UnquotedMapping("".into(), "".into()));
}
