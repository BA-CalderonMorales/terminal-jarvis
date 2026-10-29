//! Tagline: the status rule's words -- active harness, fleet readiness, and
//! gate posture -- so the frame's truth is readable without scrollback.
//! Worded, not only coloured, so monochrome says the same thing a theme does.

use crate::tui::screen::{paint, Role};

/// The status rule: the one line that answers "what am I looking at" --
/// active harness, fleet readiness, and gate posture -- without reading
/// scrollback. Worded, not only coloured, so a monochrome terminal says the
/// same thing a themed one does.
pub fn tagline(active: &str, ready: usize, total: usize, gate: Option<&str>) -> String {
    let all = ready == total;
    let fleet = if all {
        format!("all {total} ready")
    } else {
        format!("{ready}/{total} ready")
    };
    let gate = match gate {
        Some(name) => paint(Role::Ok, name),
        None => paint(Role::Dim, "off"),
    };
    format!(
        "{} {} · {} {} · {} {}",
        paint(Role::Dim, "active"),
        paint(Role::Accent, active),
        paint(Role::Dim, "fleet"),
        paint(if all { Role::Ok } else { Role::Warn }, &fleet),
        paint(Role::Dim, "gate"),
        gate
    )
}

#[cfg(test)]
#[path = "../../tests/tagline.rs"]
mod tests;
