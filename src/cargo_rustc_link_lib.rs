pub enum Kind {
    DynamicLibrary,
    Static,
    Framework,
}

impl Kind {
    pub const DYNAMIC_LIBRARY: &'static str = "dylib";
    pub const STATIC: &'static str = "static";
    pub const FRAMEWORK: &'static str = "framework";
}

#[allow(clippy::from_over_into)]
impl Into<String> for Kind {
    fn into(self) -> String {
        match self {
            Self::DynamicLibrary => Self::DYNAMIC_LIBRARY.into(),
            Self::Static => Self::STATIC.into(),
            Self::Framework => Self::FRAMEWORK.into(),
        }
    }
}
