//! Art: the first-boot primer. The header owns identity; the body greets
//! with the live fleet state, then a sectioned, role-styled command primer --
//! centered by the paint pass -- that yields to real content after the first
//! command. Every line names a role, so the primer follows the theme, and a
//! narrow viewport drops the purpose column instead of clipping it.

use crate::tui::screen::{paint, section, visible_width, Role};

/// The default body: a centered greeting over a sectioned command block, in
/// the order a new user reaches for them. Pure so tests can pin the shape;
/// `inner` is the body's width in cells, so the primer can degrade.
pub fn welcome(active: &str, ready: usize, total: usize, inner: usize) -> Vec<String> {
    let commands: [(&str, &str); 10] = [
        ("home", "the command center"),
        ("status", "readiness dashboard"),
        ("list", "numbered fleet picker"),
        ("<number|harness>", "instant switch"),
        ("plan <h> <cap>", "preview before running"),
        ("install <h>", "add a harness to the fleet"),
        ("uninstall <h>", "remove a harness and its binary"),
        ("show <h>", "harness details"),
        ("debug", "raw view · help full table"),
        ("exit", "leave the command center"),
    ];
    let cmd_w = commands
        .iter()
        .map(|(command, _)| command.chars().count())
        .max()
        .unwrap_or(0);
    let purpose_w = commands
        .iter()
        .map(|(_, purpose)| purpose.chars().count())
        .max()
        .unwrap_or(0);
    let wide = inner >= 2 + cmd_w + 2 + purpose_w;
    let rows: Vec<String> = commands
        .iter()
        .map(|(command, purpose)| {
            if wide {
                format!(
                    "  {}  {}",
                    paint(Role::Label, &format!("{command:<cmd_w$}")),
                    paint(Role::Dim, purpose)
                )
            } else {
                format!("  {}", paint(Role::Label, command))
            }
        })
        .collect();
    let greeting = format!("Welcome back -- {active} is at the helm.");
    let long = format!("{ready} of {total} harnesses are ready; the rest are one install away.");
    let fleet = if inner >= long.chars().count() {
        long
    } else {
        format!("{ready} of {total} ready")
    };
    let block = rows.iter().map(|row| visible_width(row)).max().unwrap_or(0);
    let axis = block
        .max(greeting.chars().count())
        .max(fleet.chars().count());
    let mut lines = vec![
        center(
            &paint(Role::Title, &greeting),
            axis,
            greeting.chars().count(),
        ),
        String::new(),
        center(&paint(Role::Accent, &fleet), axis, fleet.chars().count()),
        section("commands", Role::Accent, block),
    ];
    lines.extend(rows);
    lines
}

/// Centers already-styled text over the block axis by its visible width.
fn center(styled: &str, axis: usize, visible: usize) -> String {
    let pad = axis.saturating_sub(visible) / 2;
    format!("{}{styled}", " ".repeat(pad))
}

/// The header tagline: purpose plus the live fleet state, right-aligned on
/// the header row.
pub fn tagline(_active: &str, _ready: usize, _total: usize) -> String {
    "context command center".to_string()
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
