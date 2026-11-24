//! The picker's empty-filter state, shown behind the `tui-pages` Labs flag.
//! Off, an empty filter paints no rows, as main does; on, one calm line says
//! why the list is empty and which keys change the filter.

use super::Picker;
use crate::tui::screen::{clip_line, paint, Role};

/// The rows the overlay paints: the picker's rows, or the empty-state
/// guidance when the filter matches nothing and the flag is on.
pub fn painted(picker: &Picker<'_>, inner: usize) -> Vec<String> {
    if !picker.shown().is_empty() || !crate::flags::enabled("tui-pages") {
        return picker.rows(inner);
    }
    [
        paint(Role::Title, "No agents match this filter"),
        paint(Role::Dim, "Backspace edits · Ctrl+U clears · Esc closes"),
    ]
    .iter()
    .map(|row| clip_line(row, inner))
    .collect()
}

#[cfg(test)]
#[path = "../tests/picker_empty.rs"]
mod tests;
