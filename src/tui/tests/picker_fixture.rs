//! Fixtures shared by the picker's unit tests: a three-harness fleet with one
//! ready, plus the constructors the policy and row suites both need.

use super::{Picker, Step};
use crate::contracts::Harness;
use crate::tui::input::Key;
use std::collections::BTreeSet;

pub(crate) fn harness(name: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

pub(crate) fn fleet() -> [Harness; 3] {
    [harness("alpha"), harness("beta"), harness("gamma")]
}

/// A fleet taller than a small frame, to pin the picker's scroll window.
pub(crate) fn long_fleet() -> Vec<Harness> {
    (1..=25).map(|at| harness(&format!("h{at:02}"))).collect()
}

pub(crate) fn ready(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| name.to_string()).collect()
}

pub(crate) fn picker<'a>(harnesses: &'a [Harness]) -> Picker<'a> {
    Picker::new(harnesses, None, ready(&["beta"]), None)
}

pub(crate) fn select_index(picker: &mut Picker<'_>, key: Key) -> Option<usize> {
    match picker.step(key) {
        Step::Select(index) => Some(index),
        _ => None,
    }
}
