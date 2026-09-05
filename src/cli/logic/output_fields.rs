//! OutputFields: the human-line painter behind the rich screens -- padded
//! label/value rows whose wrapped continuations align under the value, so
//! `show` and the gate screens read as one family.

use super::output::wrap;
use super::table;

/// One field block: the `  label     value` line plus aligned continuations.
pub fn field(label: &str, value: &str, width: usize) -> Vec<String> {
    section(format!("  {label:<10} "), value, width)
}

/// A padded block: first line carries `pad`, continuations align under it.
pub fn section(pad: String, text: &str, width: usize) -> Vec<String> {
    wrap(text, width.saturating_sub(pad.chars().count()))
        .split('\n')
        .enumerate()
        .map(|(step, line)| {
            if step == 0 {
                format!("{pad}{line}")
            } else {
                format!("{}{line}", " ".repeat(pad.chars().count()))
            }
        })
        .collect()
}

/// The terminal width the rich screens compose against.
pub fn width() -> usize {
    table::terminal_width()
}
