//! PickerRow: one painted row -- stable number, name, readiness chip,
//! description, and the active marker. Column math only; the window and its
//! cursor live in `picker_view`.

use super::super::Picker;
use crate::contracts::Harness;

pub(super) use crate::tui::switcher::CHIP;
pub(super) const CURRENT: &str = "  (current)";

impl<'a> Picker<'a> {
    pub(crate) fn row(
        &self,
        index: usize,
        name_width: usize,
        harness: &Harness,
        chip: Option<&str>,
        room: Option<usize>,
    ) -> String {
        let marker = if self.active.as_deref() == Some(harness.name.as_str()) {
            CURRENT
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
        let name = crate::tui::switcher::clip(&harness.name, name_width);
        let padding =
            " ".repeat(name_width.saturating_sub(crate::tui::screen::visible_width(&name)));
        let chip = chip
            .map(|label| format!("  {label:<CHIP$}"))
            .unwrap_or_default();
        format!("{index:>2}  {name}{padding}{chip}{description}{marker}")
    }
}
