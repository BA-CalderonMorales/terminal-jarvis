//! Status: the tui's readiness dashboard. Built from the same diagnostics
//! collection as headless `check` (no version probes), it answers the only
//! two questions a switcher needs: which agent is active, and how ready is
//! the fleet. The full evidence table stays in headless `check`; the tui
//! never renders forty rows of red.

use crate::cli::style;

use crate::contracts::Harness;
use crate::{context, diagnostics};
use std::path::Path;

pub fn active_name(state_home: &Path) -> String {
    context::load(state_home)
        .ok()
        .flatten()
        .map(|session| session.active_harness)
        .unwrap_or_else(|| "none".to_string())
}

pub fn modeline(state_home: &Path, picker_shown: bool, debug: bool) -> String {
    if picker_shown {
        return "pick a number to switch agents, or type 'home' to go back".to_string();
    }
    let active = active_name(state_home);
    format!(
        "active: {active}{} | a number or name switches, list, status, help, home, exit",
        if debug { " (debug)" } else { "" }
    )
}

/// (coverage) builds and caches the indicator; probes the harness version
/// only when the active harness changes, and syncs the SIGINT redraw frame.
pub fn refresh_indicator(
    cached: &mut crate::tui::input::Indicator,
    state_home: &Path,
    debug: bool,
) {
    let active = active_name(state_home);
    let next = crate::tui::input::Indicator { active, debug };
    if *cached == next {
        return;
    }
    *cached = next.clone();
    let ansi = crate::tui::term::ansi_enabled();
    crate::tui::sigint::remember_prefix(Box::leak(next.render(ansi).into_boxed_str()));
}

pub fn render(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> String {
    let mut runtime = diagnostics::RuntimeInput::local(false, false, false, 100, "tui");
    runtime.probes = false;
    let input =
        diagnostics::DiagnosticInput::local(catalog_root, state_home, None, harnesses, runtime);
    let report = diagnostics::collect(&input);
    let active = active_name(state_home);
    let ready: Vec<&str> = report
        .records
        .iter()
        .filter(|record| record.key.ends_with(".readiness") && record.value == "ready")
        .filter_map(|record| record.key.strip_prefix("harness."))
        .map(|key| key.strip_suffix(".readiness").unwrap_or(key))
        .collect();
    format!(
        "{}  {}\n{}  {} of {} ready{}",
        style::label("ACTIVE"),
        active,
        style::label("READY"),
        ready.len(),
        harnesses.len(),
        if ready.is_empty() {
            String::new()
        } else {
            format!(": {}", ready.join(", "))
        }
    )
}

#[cfg(test)]
#[path = "../tests/status.rs"]
mod tests;
