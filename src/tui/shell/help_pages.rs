//! Grouped help, shown behind the `tui-pages` Labs flag. `help::text` is the
//! one place that chooses it. The layout: the keys come first, then the
//! commands under their groups. A narrow frame drops the purpose column
//! (and clips a command that is still too wide) rather than wrapping.

use crate::tui::screen::{clip_line, paint, section, Role};

#[path = "help_groups.rs"]
mod groups;

use groups::{GROUPS, KEYS};

pub fn text(inner: usize) -> String {
    let all = || GROUPS.iter().flat_map(|(_, rows)| rows.iter());
    let cmd_w = all()
        .map(|(command, _)| command.chars().count())
        .max()
        .unwrap_or(0);
    let purpose_w = all()
        .map(|(_, purpose)| purpose.chars().count())
        .max()
        .unwrap_or(0);
    let wide = inner >= 2 + cmd_w + 2 + purpose_w;
    let width = inner.min(88);
    let key_w = KEYS
        .iter()
        .map(|(key, _)| key.chars().count())
        .max()
        .unwrap_or(0);
    let mut body = format!("{}\n", section("Keys", Role::Title, width));
    for (key, purpose) in KEYS {
        body.push_str(&grouped_row(key, purpose, key_w, wide, inner));
    }
    for (title, rows) in GROUPS {
        body.push_str(&format!("\n{}\n", section(title, Role::Accent, width)));
        for (command, purpose) in rows.iter() {
            body.push_str(&grouped_row(command, purpose, cmd_w, wide, inner));
        }
    }
    let footer = "installs and runs pass the Trivy gate";
    body.push_str(&format!("\n  {}\n", paint(Role::Dim, footer)));
    body
}

/// One row: the command padded into its column beside the purpose when the
/// frame is wide; the command alone, clipped to the frame, otherwise.
fn grouped_row(command: &str, purpose: &str, cmd_w: usize, wide: bool, inner: usize) -> String {
    if wide {
        format!(
            "  {}  {}\n",
            paint(Role::Label, &format!("{command:<cmd_w$}")),
            paint(Role::Value, purpose)
        )
    } else {
        let fitted = clip_line(command, inner.saturating_sub(2));
        format!("  {}\n", paint(Role::Label, &fitted))
    }
}

#[cfg(test)]
#[path = "../tests/help_pages.rs"]
mod tests;
