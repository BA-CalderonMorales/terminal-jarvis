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
    println!(
        "{}",
        style::banner(
            "Terminal Jarvis",
            "Command center for orchestrating context switching between coding-agent harnesses"
        )
    );
    println!(
        "{}  {}",
        style::label("ACTIVE"),
        active.as_deref().unwrap_or("none")
    );
    println!(
        "{}  {}",
        style::label("CWD"),
        std::env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    );
    let ready = readiness(harnesses, catalog_root, state_home, active.as_deref());
    println!(
        "{}  {} / {} ready",
        style::label("READY"),
        ready,
        harnesses.len()
    );
    println!();
}

fn readiness(
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    active: Option<&str>,
) -> usize {
    let mut runtime = diagnostics::RuntimeInput::local(false, false, false, 100, "tui");
    runtime.probes = false;
    let input = diagnostics::DiagnosticInput::local(
        catalog_root,
        state_home,
        active.map(String::from),
        harnesses,
        runtime,
    );
    diagnostics::collect(&input).ready_harnesses
}
