#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SupportState {
    Verified,
    Expected,
    Manual,
    Stub,
    Unsupported,
    Disabled,
    Unknown,
}

impl SupportState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Expected => "expected",
            Self::Manual => "manual",
            Self::Stub => "stub",
            Self::Unsupported => "unsupported",
            Self::Disabled => "disabled",
            Self::Unknown => "unknown",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "verified" => Ok(Self::Verified),
            "expected" => Ok(Self::Expected),
            "manual" => Ok(Self::Manual),
            "stub" => Ok(Self::Stub),
            "unsupported" => Ok(Self::Unsupported),
            "disabled" => Ok(Self::Disabled),
            "unknown" => Ok(Self::Unknown),
            other => Err(format!("unknown support state '{other}'")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceMode {
    Deterministic,
    DisposableReal,
    Manual,
    Unsupported,
}

impl EvidenceMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deterministic => "deterministic",
            Self::DisposableReal => "disposable-real",
            Self::Manual => "manual",
            Self::Unsupported => "unsupported",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "deterministic" => Ok(Self::Deterministic),
            "disposable-real" => Ok(Self::DisposableReal),
            "manual" => Ok(Self::Manual),
            "unsupported" => Ok(Self::Unsupported),
            other => Err(format!("unknown evidence mode '{other}'")),
        }
    }
}
