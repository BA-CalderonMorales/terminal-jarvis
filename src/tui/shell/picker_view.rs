//! PickerView: the overlay's rows -- the fleet as seen from the keyboard,
//! cursor as a pointer and stable picker positions as the number a user can
//! still type. Rendering only; the key policy lives in `picker`.

use super::Picker;
use crate::contracts::Harness;

impl<'a> Picker<'a> {
    /// The visible fleet and each row's original picker position.
    pub fn shown(&self) -> Vec<(usize, &'a Harness)> {
        let needle = self.filter.to_lowercase();
        self.harnesses
            .iter()
            .enumerate()
            .filter(|(_, harness)| {
                needle.is_empty()
                    || harness.name.to_lowercase().contains(&needle)
                    || harness.description.to_lowercase().contains(&needle)
            })
            .collect()
    }

    pub fn selected(&self) -> Option<&'a Harness> {
        self.shown().get(self.cursor).map(|(_, harness)| *harness)
    }

    pub(crate) fn clamp(&mut self) {
        let len = self.shown().len();
        self.cursor = self.cursor.min(len.saturating_sub(1));
    }

    pub(crate) fn position(&self, name: &str) -> usize {
        self.harnesses
            .iter()
            .position(|harness| harness.name == name)
            .unwrap_or(0)
    }

    /// The window offset that keeps the cursor row on screen: the list starts
    /// at the top and scrolls only once the cursor reaches the last row.
    pub fn offset(&self, rows: usize) -> usize {
        let len = self.shown().len();
        let span = len.saturating_sub(rows);
        let start = self.cursor.saturating_add(1).saturating_sub(rows).min(span);
        span - start
    }

    /// The rows the overlay paints, cursor first: stable number, name,
    /// readiness, description, and the active marker.
    pub fn rows(&self, inner: usize) -> Vec<String> {
        let shown = self.shown();
        let name_width = shown
            .iter()
            .map(|(_, harness)| harness.name.chars().count())
            .max()
            .unwrap_or(0);
        let marker = if self.active.is_some() { 11 } else { 0 };
        let fixed = 2 + 2 + name_width + 2 + 7 + 2 + marker;
        let room = inner.checked_sub(fixed).filter(|room| *room >= 12);
        shown
            .iter()
            .enumerate()
            .map(|(row, (position, harness))| {
                let chip = if self.ready.contains(&harness.name) {
                    "[ready]"
                } else {
                    "[install]"
                };
                let line = self.row(*position + 1, name_width, harness, chip, room);
                format!("{} {line}", if row == self.cursor { "▸" } else { " " })
            })
            .collect()
    }
}

#[path = "picker_row.rs"]
mod row;

#[cfg(test)]
#[path = "../tests/picker_rows.rs"]
mod tests;
