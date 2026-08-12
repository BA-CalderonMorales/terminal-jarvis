//! Home dashboard: the welcome shown when the tui starts. Deliberately small
//! -- banner, active harness, readiness, working directory, one hint line.
//! The tool list is not part of the welcome; `/list` renders it and a bare
//! number selects, so first-run users see something they can admire.

use crate::cli::style;
use crate::contracts::Harness;
use crate::{context, diagnostics};
use std::path::Path;

pub fn render(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) {
    let active = context::load(state_home)
        .ok()
        .flatten()
        .map(|session| session.active_harness);
    let ready = readiness(harnesses, catalog_root, state_home, active.as_deref());
    let name = active.as_deref().unwrap_or("none");
    let cwd = cwd_label();
    let ready_plain = format!("{ready} / {} ready", harnesses.len());
    let plain = format!("ACTIVE {name} · CWD {cwd} · READY {ready_plain}");
    let ready_view = if ready == harnesses.len() {
        ready_plain.clone()
    } else {
        style::warning(&ready_plain)
    };
    let styled = format!(
        "{} {} · {} {} · {} {}",
        style::label("ACTIVE"),
        style::heading(name),
        style::label("CWD"),
        cwd,
        style::label("READY"),
        ready_view
    );
    let title = "Terminal Jarvis";
    let subtitle =
        "Command center for orchestrating context switching between coding-agent harnesses";
    let width = crate::tui::term::columns();
    if title.chars().count() + 3 + plain.chars().count() > width {
        println!(
            "{}\n{}\n{}\n",
            style::heading(title),
            style::dim(subtitle),
            styled
        );
    } else {
        println!(
            "{}{}{}\n{}\n",
            style::heading(title),
            " ".repeat(width - title.chars().count() - plain.chars().count()),
            styled,
            style::dim(subtitle)
        );
    }
}

#[path = "cwd.rs"]
mod cwd;
pub use cwd::cwd_label;

#[cfg(test)]
#[path = "../tests/home.rs"]
mod tests;

fn readiness(h: &[Harness], root: &Path, home: &Path, active: Option<&str>) -> usize {
    let mut runtime = diagnostics::RuntimeInput::local(false, false, false, 100, "tui");
    runtime.probes = false;
    let input =
        diagnostics::DiagnosticInput::local(root, home, active.map(String::from), h, runtime);
    diagnostics::collect(&input).ready_harnesses
}
