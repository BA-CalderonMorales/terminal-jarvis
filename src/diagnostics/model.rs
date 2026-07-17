#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Code {
    Ready,
    Missing,
    Empty,
    Malformed,
    Unsupported,
    Conflicting,
    Stale,
    PermissionDenied,
    Unknown,
}

impl Code {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Missing => "missing",
            Self::Empty => "empty",
            Self::Malformed => "malformed",
            Self::Unsupported => "unsupported",
            Self::Conflicting => "conflicting",
            Self::Stale => "stale",
            Self::PermissionDenied => "permission-denied",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub key: String,
    pub code: Code,
    pub severity: Severity,
    pub value: String,
    pub action: Option<String>,
}

impl Record {
    pub(crate) fn new(
        key: impl Into<String>,
        code: Code,
        severity: Severity,
        value: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            code,
            severity,
            value: value.into(),
            action: None,
        }
    }

    pub(crate) fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub records: Vec<Record>,
    pub ready_harnesses: usize,
    pub ok: bool,
}
