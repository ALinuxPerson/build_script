pub enum Kind {
    Dependency,
    Crate,
    Native,
    Framework,
    All,
}

impl Kind {
    pub const DEPENDENCY: &'static str = "dependency";
    pub const CRATE: &'static str = "crate";
    pub const NATIVE: &'static str = "native";
    pub const FRAMEWORK: &'static str = "framework";
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
