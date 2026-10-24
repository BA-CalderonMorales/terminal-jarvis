//! ValueBlock: the one renderer for a card of label/value rows.
//!
//! A card is a list of groups. A group is a heading and a set of rows. The
//! rules are the same wherever a card is read:
//!
//! - a value that fits the row keeps the label/value shape;
//! - a value that does not takes its own lines, wrapped, hanging under its
//!   label, so a long sentence reads as a paragraph instead of a fragment
//!   clipped at the frame edge;
//! - a group is a heading above its rows, so a card reads as sections rather
//!   than as one undifferentiated wall of peers.
//!
//! The tui and the piped output render through this, so a card looks the
//! same in the frame and in the terminal.

use super::text_wrap::wrap;
use crate::cli::structs::value_block::{Group, Row};

/// The indent of a group heading.
pub const HEADING_INDENT: usize = 2;
/// The indent of a row under its heading.
pub const ROW_INDENT: usize = 4;
/// The narrowest a label column gets before the value starts on its own line.
const LABEL_FLOOR: usize = 8;
/// The gap between a label and the value beside it.
const LABEL_GAP: usize = 1;

/// A card: groups of rows, rendered top to bottom.
pub fn card(groups: &[Group], width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for (index, entry) in groups.iter().enumerate() {
        if index > 0 {
            lines.push(String::new());
        }
        lines.extend(group(entry, width));
    }
    lines
}

/// One group: a heading, then its rows.
pub fn group(entry: &Group, width: usize) -> Vec<String> {
    if entry.rows.is_empty() {
        return Vec::new();
    }
    let label_width = entry
        .rows
        .iter()
        .map(|row| row.label.chars().count())
        .max()
        .unwrap_or(0)
        .max(LABEL_FLOOR);
    let mut lines = vec![format!("{}{}", " ".repeat(HEADING_INDENT), entry.heading)];
    for row in &entry.rows {
        lines.extend(render_row(row, label_width, width));
    }
    lines
}

/// One row: the label with its value beside it, or the value beneath it.
fn render_row(row: &Row, label_width: usize, width: usize) -> Vec<String> {
    let label = format!("{}{}", " ".repeat(ROW_INDENT), row.label);
    let value_width = width.saturating_sub(ROW_INDENT + label_width + LABEL_GAP);
    if cells(&row.value) <= value_width {
        return vec![format!(
            "{label:<width$} {value}",
            width = ROW_INDENT + label_width,
            value = row.value
        )];
    }
    // The value does not fit beside its label: it takes its own lines,
    // hanging under the label rather than being clipped mid-word.
    let hang = " ".repeat(ROW_INDENT + 2);
    let mut lines = vec![label];
    for line in wrap(&row.value, width.saturating_sub(ROW_INDENT + 2)) {
        lines.push(format!("{hang}{line}"));
    }
    lines
}

/// The visible cells of a value, wide glyphs counting two.
fn cells(text: &str) -> usize {
    text.chars().map(crate::cli::char_cells).sum()
}

#[cfg(test)]
#[path = "../tests/value_block_test.rs"]
mod tests;
