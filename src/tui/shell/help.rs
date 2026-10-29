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
    let rows = [
        ("list [filter]", "numbered fleet picker; text narrows it"),
        ("show <harness>", "one harness in detail"),
        ("plan [harness] <cap>", "preview, don't run"),
        (
            "use <harness> | <number> | <name>",
            "switch the active harness",
        ),
        ("run <harness> [args]", "open the harness ui"),
        ("install <harness>", "install behind the security gate"),
        ("uninstall <harness>", "remove a harness and its binary"),
        ("update <harness>", "update through the same gate"),
        (
            "gate [status|enable|disable|run]",
            "control the optional Trivy gate",
        ),
        (
            "auth | config | cache | security",
            "inspect local setup and posture",
        ),
        ("/debug [on|off]", "raw, verbose view"),
        (
            "converse <turns> <a> <b> <topic...>",
            "two agents talk; dangerous plans need --allow-dangerous",
        ),
        ("home | clear", "reset the frame"),
        ("exit | quit", "leave without state changes"),
    ];
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
