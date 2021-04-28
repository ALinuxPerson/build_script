//! This contains the [`Value`](Value) struct.
use std::fmt;

/// The value of an [`Instruction`](crate::Instruction).
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
        matches!(*self, Self::UnquotedOptionalValue(_, _))
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

#[cfg(test)]
mod tests {
    use crate::Value;

    macro_rules! new_display_test {
        ($name:ident, $value:expr, $expected:literal) => {
            #[test]
            fn $name() {
                let value = $value;
                let output = format!("{}", value);
                let expected = $expected;

                assert_eq!(output, expected)
            }
        };
    }

    #[test]
    fn test_is_singular() {
        assert!(Value::Singular("".into()).is_singular())
    }

    new_display_test!(test_singular_display, Value::Singular("singular".into()), "singular");

    #[test]
    fn test_is_mapping() {
        assert!(Value::Mapping("".into(), "".into()).is_mapping())
    }

    new_display_test!(test_mapping_display, Value::Mapping("key".into(), "value".into()), "key=\"value\"");

    #[test]
    fn test_is_optional_key() {
        assert!(Value::OptionalKey(None, "".into()).is_optional_key())
    }

    new_display_test!(test_optional_key_display, Value::OptionalKey(Some("key".into()), "value".into()), "key=\"value\"");
    new_display_test!(test_optional_key_display_none, Value::OptionalKey(None, "value".into()), "value");

    #[test]
    fn test_is_unquoted_optional_key() {
        assert!(Value::UnquotedOptionalKey(None, "".into()).is_unquoted_optional_key())
    }

    new_display_test!(test_unquoted_optional_key_display, Value::UnquotedOptionalKey(Some("key".into()), "value".into()), "key=value");
    new_display_test!(test_unquoted_optional_key_display_none, Value::UnquotedOptionalKey(None, "value".into()), "value");

    #[test]
    fn test_is_optional_value() {
        assert!(Value::OptionalValue("".into(), None).is_optional_value())
    }

    new_display_test!(test_optional_value_display, Value::OptionalValue("key".into(), Some("value".into())), "key=\"value\"");
    new_display_test!(test_optional_value_display_none, Value::OptionalValue("key".into(), None), "key");

    #[test]
    fn test_is_unquoted_optional_value() {
        assert!(Value::UnquotedOptionalValue("".into(), None).is_unquoted_optional_value())
    }

    new_display_test!(test_unquoted_optional_value_display, Value::UnquotedOptionalValue("key".into(), Some("value".into())), "key=value");
    new_display_test!(test_unquoted_optional_value_display_none, Value::UnquotedOptionalValue("key".into(), None), "key");

    #[test]
    fn test_is_unquoted_mapping() {
        assert!(Value::UnquotedMapping("".into(), "".into()).is_unquoted_mapping())
    }

    new_display_test!(test_unquoted_mapping_display, Value::UnquotedMapping("key".into(), "value".into()), "key=value");
}
