//! Status: the tui's readiness dashboard. Built from the same diagnostics
//! collection as headless `check` (no version probes), it answers the only
//! two questions a switcher needs: which agent is active, and how ready is
//! the fleet. The full evidence table stays in headless `check`; the tui
//! never renders forty rows of red.

use crate::contracts::Harness;
use crate::tui::screen::{pair, section, visible_width, Role};
use std::path::Path;

pub fn active_name(state_home: &Path) -> String {
    super::fleet::active(state_home).unwrap_or_else(|| "none".to_string())
}

pub fn modeline(state_home: &Path, picker_shown: bool, debug: bool) -> String {
    if picker_shown {
        return "pick a number to switch agents · list <text> filters · home resets".to_string();
    }
    let active = active_name(state_home);
    let debug = if debug { " (debug)" } else { "" };
    format!("active: {active}{debug} | number or name switches · list, status, help, home, exit")
}

/// Announces a frame -> chat downgrade instead of letting the surface change
/// under the reader: a stranger who shrank the terminal learns why the chrome
/// left and what brings it back.
pub fn announce(framed_before: bool) {
    if framed_before && !crate::tui::screen::active() {
        println!("frame paused: this terminal is too small; enlarge it and the frame returns");
    }
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
    let active = active_name(state_home);
    let ready = super::fleet::ready(harnesses, catalog_root, state_home);
    let ready: Vec<&str> = ready.iter().map(String::as_str).collect();
    let not_ready: Vec<&str> = harnesses
        .iter()
        .map(|harness| harness.name.as_str())
        .filter(|name| !ready.contains(name))
        .collect();
    let active_row = pair("ACTIVE", &active, 6);
    let ready_row = pair(
        "READY",
        &format!("{} of {} ready", ready.len(), harnesses.len()),
        6,
    );
    let width = visible_width(&active_row)
        .max(visible_width(&ready_row))
        .max("one install away".len());
    let mut lines = vec![
        section("readiness", Role::Accent, width),
        active_row,
        ready_row,
    ];
    if !ready.is_empty() {
        lines.push(section("ready now", Role::Ok, width));
        lines.extend(rows::wrap(&ready));
    }
    if !not_ready.is_empty() {
        lines.push(section("one install away", Role::Warn, width));
        lines.extend(rows::wrap(&not_ready));
    }
    lines.join("\n")
}

#[path = "status_rows.rs"]
mod rows;

#[cfg(test)]
#[path = "../tests/status.rs"]
mod tests;
