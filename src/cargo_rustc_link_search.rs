//! A wrapper for [`cargo_rustc_link_search`](crate::BuildScript::cargo_rustc_link_search).
/// A kind for [`cargo_rustc_link_search`](crate::BuildScript::cargo_rustc_link_search).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Kind {
    /// Known to the compiler as [`dependency`](Self::DEPENDENCY).
    Dependency,

    /// Known to the compiler as [`crate`](Self::CRATE).
    Crate,

    /// Known to the compiler as [`native`](Self::NATIVE).
    Native,

    /// Known to the compiler as [`framework`](Self::FRAMEWORK).
    Framework,

    /// Known to the compiler as [`all`](Self::ALL).
    All,
}

impl Kind {
    /// Known to this library as [`Dependency`](Self::Dependency).
    pub const DEPENDENCY: &'static str = "dependency";

    /// Known to this library as [`Crate`](Self::Crate).
    pub const CRATE: &'static str = "crate";

    /// Known to this library as [`Native`](Self::Native).
    pub const NATIVE: &'static str = "native";

    /// Known to this library as [`Framework`](Self::Framework).
    pub const FRAMEWORK: &'static str = "framework";

    /// Known to this library as [`All`](Self::All).
    pub const ALL: &'static str = "all";
}

#[allow(clippy::from_over_into)]
impl Into<String> for Kind {
    fn into(self) -> String {
        match self {
            Self::Dependency => Self::DEPENDENCY.into(),
            Self::Crate => Self::CRATE.into(),
            Self::Native => Self::NATIVE.into(),
            Self::Framework => Self::FRAMEWORK.into(),
            Self::All => Self::ALL.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kind;

    #[test]
    fn test_into_string() {
        let kind: String = Kind::Dependency.into();
        assert_eq!(kind, Kind::DEPENDENCY);
        let kind: String = Kind::Crate.into();
        assert_eq!(kind, Kind::CRATE);
        let kind: String = Kind::Native.into();
        assert_eq!(kind, Kind::NATIVE);
        let kind: String = Kind::Framework.into();
        assert_eq!(kind, Kind::FRAMEWORK);
        let kind: String = Kind::All.into();
        assert_eq!(kind, Kind::ALL);
    }
}
