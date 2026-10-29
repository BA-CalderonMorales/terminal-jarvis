//! Fleet: the one reading of fleet truth the tui repeats -- which harness is
//! active and which are ready -- so the picker, the dashboard, and the shell
//! hint cannot disagree about the fleet. Readiness reuses the dashboard's
//! record shape (no version probes), so a tool is listed once even after
//! repeated installs appended duplicate records to the cache.

use crate::contracts::Harness;
use crate::{context, diagnostics};
use std::collections::BTreeSet;
use std::path::Path;

pub fn active(state_home: &Path) -> Option<String> {
    context::load(state_home)
        .ok()
        .flatten()
        .map(|session| session.active_harness)
}

pub fn ready(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> BTreeSet<String> {
    let mut runtime = diagnostics::RuntimeInput::local(false, false, false, 100, "tui");
    runtime.probes = false;
    let input =
        diagnostics::DiagnosticInput::local(catalog_root, state_home, None, harnesses, runtime);
    diagnostics::collect(&input)
        .records
        .iter()
        .filter(|record| record.key.ends_with(".readiness") && record.value == "ready")
        .filter_map(|record| record.key.strip_prefix("harness."))
        .map(|key| key.strip_suffix(".readiness").unwrap_or(key).to_string())
        .collect()
}

#[cfg(test)]
#[path = "../tests/fleet.rs"]
mod tests;
