//! PickerRow: one painted row -- stable number, name, readiness chip,
//! description, and the active marker. Column math only; the window and its
//! cursor live in `picker_view`.

use super::super::Picker;
use crate::contracts::Harness;

impl<'a> Picker<'a> {
    pub(crate) fn row(
        &self,
        index: usize,
        name_width: usize,
        harness: &Harness,
        chip: &str,
        room: Option<usize>,
    ) -> String {
        let marker = if self.active.as_deref() == Some(harness.name.as_str()) {
            "  (current)"
        } else {
            ""
        };
        let description = match room {
            Some(room) => format!(
                "  {}",
                crate::tui::switcher::clip(&harness.description, room)
            ),
            None => String::new(),
        };
        format!(
            "{index:>2}  {name:<name_width$}  {chip:<9}{description}{marker}",
            name = harness.name
        )
    }
}
