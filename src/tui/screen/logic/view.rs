//! View: the frame's composition primitives. Every layout builds its rows
//! from these, so alignment, rhythm, and emphasis are decided once -- a
//! renderer names a role and a column, never a raw SGR code or a hand-rolled
//! pad. Widths are the block's own, so the paint pass can center the whole
//! assembly without a renderer knowing the terminal size.

use super::canvas;
use super::roles::Role;
use super::theme;

/// A section heading: the title in `role`, then a rule to `width` cells.
/// This is the one cue that names the screen the user is looking at, so
/// every layout and every transition carries the same signpost.
pub fn section(title: &str, role: Role, width: usize) -> String {
    let used = canvas::visible_width(title);
    let fill = width.saturating_sub(used + 2);
    let painted = theme::paint(role, title);
    if fill == 0 {
        return painted;
    }
    format!("{painted} {}", theme::paint(Role::Rule, &"─".repeat(fill)))
}

/// An aligned label/value row: the label padded to `label_width` cells, the
/// value flush after it, each painted in its own role.
pub fn pair(label: &str, value: &str, label_width: usize) -> String {
    format!(
        "{}  {}",
        theme::paint(Role::Label, &format!("{label:<label_width$}")),
        theme::paint(Role::Value, value)
    )
}

/// A bracketed state chip, e.g. `[ready]`, painted in `role`.
pub fn chip(text: &str, role: Role) -> String {
    theme::paint(role, &format!("[{text}]"))
}

/// The width of the widest line in a block, for sizing a section rule.
pub fn block_width(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| canvas::visible_width(line))
        .max()
        .unwrap_or(0)
}
