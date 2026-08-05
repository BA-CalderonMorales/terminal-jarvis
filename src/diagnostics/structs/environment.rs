use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueState {
    Missing,
    Empty,
    Malformed,
    Present,
}

impl ValueState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Empty => "empty",
            Self::Malformed => "malformed",
            Self::Present => "present",
        }
    }

    pub(crate) fn code(self) -> super::Code {
        match self {
            Self::Missing => super::Code::Missing,
            Self::Empty => super::Code::Empty,
            Self::Malformed => super::Code::Malformed,
            Self::Present => super::Code::Ready,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Environment {
    values: BTreeMap<String, OsString>,
}

impl Environment {
    pub fn process() -> Self {
        Self {
            values: std::env::vars_os()
                .map(|(k, v)| (k.to_string_lossy().into(), v))
                .collect(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<OsString>) {
        self.values.insert(name.into(), value.into());
    }

    pub fn remove(&mut self, name: &str) {
        self.values.remove(name);
    }

    pub fn state(&self, name: &str) -> ValueState {
        match self.values.get(name) {
            None => ValueState::Missing,
            Some(value) => match value.to_str() {
                None => ValueState::Malformed,
                Some(value) if value.trim().is_empty() => ValueState::Empty,
                Some(_) => ValueState::Present,
            },
        }
    }

    pub(crate) fn value(&self, name: &str) -> Option<&OsStr> {
        self.values.get(name).map(OsString::as_os_str)
    }

    pub(crate) fn text(&self, name: &str) -> Option<&str> {
        self.value(name)?.to_str()
    }

    pub(crate) fn paths(&self) -> Vec<PathBuf> {
        self.value("PATH")
            .map(std::env::split_paths)
            .into_iter()
            .flatten()
            .collect()
    }
}
