//! Overview: the one-line fleet truth (active harness, working directory,
//! readiness count) shared by the chat banner and the viewport status row.

use crate::contracts::Harness;
use crate::{context, diagnostics};
use std::path::Path;

pub struct Overview {
    pub name: String,
    pub cwd: String,
    pub ready: usize,
    /// The maintained fleet: a harness upstream has retired, or announced it
    /// will retire, is counted here instead of in `total`.
    pub total: usize,
    /// Upstream has ended support.
    pub retired: usize,
    /// Upstream has announced an end date that has not arrived. The tool still
    /// works, so the header says "retiring" rather than "retired".
    pub retiring: usize,
}

pub fn collect(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> Overview {
    let active = context::load(state_home)
        .ok()
        .flatten()
        .map(|session| session.active_harness);
    let ready = readiness(harnesses, catalog_root, state_home, active.as_deref());
    let today = crate::catalog::today_utc();
    let retired = harnesses
        .iter()
        .filter(|harness| harness.retirement_adjective(&today) == Some("retired"))
        .count();
    let retiring = harnesses
        .iter()
        .filter(|harness| harness.retirement_adjective(&today) == Some("retiring"))
        .count();
    Overview {
        name: active.unwrap_or_else(|| "none".to_string()),
        cwd: super::cwd_label(),
        ready,
        total: harnesses.len().saturating_sub(retired + retiring),
        retired,
        retiring,
    }
}

/// `ACTIVE x · CWD y · READY n/m` in plain form (tests, width math).
pub fn plain(o: &Overview) -> String {
    format!(
        "ACTIVE {} · CWD {} · READY {}/{} ready{}",
        o.name,
        o.cwd,
        o.ready,
        o.total,
        super::overview_banner::retirement(o)
    )
}

fn readiness(h: &[Harness], root: &Path, home: &Path, active: Option<&str>) -> usize {
    let mut runtime = diagnostics::RuntimeInput::local(false, false, false, 100, "tui");
    runtime.probes = false;
    let input =
        diagnostics::DiagnosticInput::local(root, home, active.map(String::from), h, runtime);
    diagnostics::collect(&input).ready_harnesses
}
