//! The data shapes of the flag registry.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    Stable,
    Preview,
    Labs,
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        match self {
            Stage::Stable => "stable",
            Stage::Preview => "preview",
            Stage::Labs => "labs",
        }
    }
}

/// One declared flag, exactly as `flags/<name>/index.toml` states it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flag {
    pub name: String,
    pub default: bool,
    pub stage: Stage,
    pub description: String,
    pub since: String,
}

/// Where a flag's effective value came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    Default,
    Settings,
    Env,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Source::Default => "default",
            Source::Settings => "settings",
            Source::Env => "env",
        }
    }
}

/// A flag with its effective value and the layer that decided it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resolved {
    pub flag: Flag,
    pub enabled: bool,
    pub source: Source,
}
