//! Overview: the one-line fleet truth (active harness, working directory,
//! readiness count) shared by the chat banner and the viewport status row.

use crate::contracts::Harness;
use crate::{context, diagnostics};
use std::path::Path;

pub struct Overview {
    pub name: String,
    pub cwd: String,
    pub ready: usize,
    pub total: usize,
}

pub fn collect(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> Overview {
    let active = context::load(state_home)
        .ok()
        .flatten()
        .map(|session| session.active_harness);
    let ready = readiness(harnesses, catalog_root, state_home, active.as_deref());
    Overview {
        name: active.unwrap_or_else(|| "none".to_string()),
        cwd: super::cwd_label(),
        ready,
        total: harnesses.len(),
    }
}

/// `ACTIVE x · CWD y · READY n/m` in plain form (tests, width math).
pub fn plain(o: &Overview) -> String {
    format!(
        "ACTIVE {} · CWD {} · READY {}/{} ready",
        o.name, o.cwd, o.ready, o.total
    )
}

/// The viewport header: one merged line, priority-ordered so a narrow
/// terminal loses the working directory before the readiness verdict.
pub fn header(o: &Overview) -> String {
    use crate::tui::screen::{accent, dim};
    let ready_view = verdict(o);
    format!(
        "{} · {} {} · {} {}",
        accent("Terminal Jarvis"),
        dim("ACTIVE"),
        accent(&o.name),
        dim("READY"),
        ready_view,
    )
}

fn verdict(o: &Overview) -> String {
    use crate::cli::style;
    if o.ready == o.total {
        format!("{}/{} ready", o.ready, o.total)
    } else {
        style::warning(&format!("{}/{} ready", o.ready, o.total))
    }
}

/// The same line, styled for a terminal that honors ANSI.
pub fn styled(o: &Overview) -> String {
    use crate::cli::style;
    format!(
        "{} {} · {} {} · {} {}",
        style::label("ACTIVE"),
        style::heading(&o.name),
        style::label("CWD"),
        o.cwd,
        style::label("READY"),
        verdict(o)
    )
}

fn readiness(h: &[Harness], root: &Path, home: &Path, active: Option<&str>) -> usize {
    let mut runtime = diagnostics::RuntimeInput::local(false, false, false, 100, "tui");
    runtime.probes = false;
    let input =
        diagnostics::DiagnosticInput::local(root, home, active.map(String::from), h, runtime);
    diagnostics::collect(&input).ready_harnesses
}
