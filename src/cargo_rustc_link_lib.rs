//! A wrapper for [`cargo_rustc_link_lib`](crate::BuildScript::cargo_rustc_link_lib).
/// A kind for [`cargo_rustc_link_lib`](crate::BuildScript::cargo_rustc_link_lib).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Kind {
    /// Known to the compiler as [`dylib`](Self::DYNAMIC_LIBRARY).
    DynamicLibrary,

    /// Known to the compiler as [`static`](Self::STATIC).
    Static,

    /// Known to the compiler as [`framework`](Self::FRAMEWORK).
    Framework,
}

impl Kind {
    /// Known to this library as [`DynamicLibrary`](Self::DynamicLibrary).
    pub const DYNAMIC_LIBRARY: &'static str = "dylib";

    /// Known to this library as [`Static`](Self::Static).
    pub const STATIC: &'static str = "static";

    /// Known to this library as [`Framework`](Self::Framework).
    pub const FRAMEWORK: &'static str = "framework";
}

impl From<Kind> for &'static str {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::DynamicLibrary => Kind::DYNAMIC_LIBRARY,
            Kind::Static => Kind::STATIC,
            Kind::Framework => Kind::FRAMEWORK,
        }
    }
}

impl From<Kind> for String {
    fn from(kind: Kind) -> Self {
        let kind: &str = kind.into();
        kind.into()
    }
}

#[cfg(test)]
mod tests {
    use super::Kind;

    #[test]
    fn test_into_string() {
        let kind: String = Kind::DynamicLibrary.into();
        assert_eq!(kind, Kind::DYNAMIC_LIBRARY);
        let kind: String = Kind::Static.into();
        assert_eq!(kind, Kind::STATIC);
        let kind: String = Kind::Framework.into();
        assert_eq!(kind, Kind::FRAMEWORK)
    }
}
