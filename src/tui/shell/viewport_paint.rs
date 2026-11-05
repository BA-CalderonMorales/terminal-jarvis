//! ViewportPaint: the composed frame painted without reading -- the converse
//! loop and the stream repaint between turns while a child owns the wait, so
//! the cursor's place stays honest.

use super::viewport_chrome::ViewportState;
use crate::contracts::Harness;
use crate::tui::input::Indicator;
use std::io::Write;
use std::path::Path;

/// Paints the composed frame without reading -- the converse loop repaints
/// between turns while the child owns the wait. `offset` is the scroll
/// position the owning loop keeps (the converse live surface scrolls it).
#[allow(clippy::too_many_arguments)]
pub fn paint(
    indicator: &Indicator,
    hint: &str,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    body: &[String],
    offset: usize,
    bottom: bool,
) {
    let _ = std::io::stdout().flush();
    let state = ViewportState::collect(indicator, harnesses, catalog_root, state_home);
    let size = crate::tui::screen::size();
    let mut draft = state.base_draft(hint, body, bottom);
    // A pending decision owns the prompt row: the cursor and the question are
    // the same place, and the modeline hint stands down for it.
    if let Some(question) = crate::tui::input::prompt_for(size.inner_cols()) {
        draft.prompt = question;
        draft.hint = String::new();
        draft.bottom = true;
    }
    draft.offset = offset;
    let cells = crate::tui::screen::visible_width(&draft.prompt) + 1;
    let painted = crate::tui::screen::parked(crate::tui::screen::frame(size, &draft), size, cells);
    print!("{painted}");
    std::io::stdout().flush().ok();
}
