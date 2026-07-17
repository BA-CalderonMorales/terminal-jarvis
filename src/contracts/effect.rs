#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Effect {
    ReadOnly,
    StateChanging,
    Dangerous,
}

impl Effect {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::StateChanging => "state-changing",
            Self::Dangerous => "dangerous",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "read-only" => Ok(Self::ReadOnly),
            "state-changing" => Ok(Self::StateChanging),
            "dangerous" => Ok(Self::Dangerous),
            other => Err(format!("unknown effect '{other}'")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Interaction {
    Noninteractive,
    Interactive,
}

impl Interaction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Noninteractive => "noninteractive",
            Self::Interactive => "interactive",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "noninteractive" => Ok(Self::Noninteractive),
            "interactive" => Ok(Self::Interactive),
            other => Err(format!("unknown interaction '{other}'")),
        }
    }
}
