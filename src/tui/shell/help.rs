//! The tui's own command table. Headless-only flags (`--plain`, `--json`,
//! `--no-input`, ...) and bare-invocation rows that cannot be typed here are
//! omitted; the headless `terminal-jarvis help` keeps the full grammar. The
//! security line anchors what the gate actually guards in this surface, and
//! a narrow viewport drops the purpose column instead of clipping it.

use crate::tui::screen::{paint, section, Role};

#[cfg(test)]
#[path = "../tests/help.rs"]
mod tests;

pub fn text(inner: usize) -> String {
    if crate::flags::enabled("tui-pages") {
        return super::help_pages::text(inner);
    }
    let rows = super::help_table::ROWS;
    let cmd_w = rows
        .iter()
        .map(|(command, _)| command.chars().count())
        .max()
        .unwrap_or(0);
    let purpose_w = rows
        .iter()
        .map(|(_, purpose)| purpose.chars().count())
        .max()
        .unwrap_or(0);
    let wide = inner >= 2 + cmd_w + 2 + purpose_w;
    let width = if wide {
        2 + cmd_w + 2 + purpose_w
    } else {
        2 + cmd_w
    };
    let mut body = format!("{}\n", section("Commands", Role::Accent, width));
    for (command, purpose) in rows {
        if wide {
            body.push_str(&format!(
                "  {}  {}\n",
                paint(Role::Label, &format!("{command:<cmd_w$}")),
                paint(Role::Value, purpose)
            ));
        } else {
            body.push_str(&format!("  {}\n", paint(Role::Label, command)));
        }
    }
    let keys = "keys: ↑↓ or jk scroll · i types · esc normal · tab completes · ? help · q quits";
    body.push_str(&format!("  {}\n", paint(Role::Dim, keys)));
    let footer = "installs and runs pass the Trivy gate";
    body.push_str(&format!("  {}\n", paint(Role::Dim, footer)));
    body
}
