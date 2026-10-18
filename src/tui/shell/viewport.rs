//! Viewport prompt: paints the composed frame and reads one line. In a
//! usable viewport the read is a raw-mode key session with scroll and an
//! in-frame editor; anything else falls back to the classic line reader.

use crate::contracts::Harness;
use crate::tui::input::Indicator;
use std::io::Write;
use std::path::Path;

pub fn welcome(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> Vec<String> {
    let o = crate::tui::home::collect(harnesses, catalog_root, state_home);
    let inner = crate::tui::screen::size().inner_cols();
    crate::tui::screen::welcome(&o.name, o.ready, o.total, inner)
}

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
) {
    let _ = std::io::stdout().flush();
    let state =
        super::viewport_raw::ViewportState::collect(indicator, harnesses, catalog_root, state_home);
    let size = crate::tui::screen::size();
    let mut draft = state.base_draft(hint, body);
    // A pending decision owns the prompt row: the cursor and the question are
    // the same place, and the modeline hint stands down for it.
    if let Some(question) = crate::tui::input::pending_question() {
        draft.prompt = question;
        draft.hint = String::new();
        // Bottom-anchor so the plan the question is about sits directly above
        // it, in one glance, instead of in the middle of the frame.
        draft.bottom = true;
    }
    draft.offset = offset;
    let cells = crate::tui::screen::visible_width(&draft.prompt) + 1;
    let painted = crate::tui::screen::parked(crate::tui::screen::frame(size, &draft), size, cells);
    print!("{painted}");
    std::io::stdout().flush().ok();
}

#[allow(clippy::too_many_arguments)]
pub fn prompt(
    indicator: &Indicator,
    hint: &str,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    body: &[String],
    history: &[String],
    offset: &mut usize,
) -> Option<String> {
    paint(
        indicator,
        hint,
        harnesses,
        catalog_root,
        state_home,
        body,
        *offset,
    );
    let state =
        super::viewport_raw::ViewportState::collect(indicator, harnesses, catalog_root, state_home);
    let mut session = super::viewport_raw::Session {
        state: &state,
        hint,
        body,
        history,
        offset,
    };
    match crate::tui::term::enable_raw() {
        Some(_guard) => super::viewport_raw::run(&mut session),
        None => crate::tui::input::raw_line(),
    }
}

/// Chat-mode boot banner: the welcome frame above the first prompt.
pub fn chat_banner(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) {
    let mut out = std::io::stdout();
    crate::tui::home::render(&mut out, harnesses, catalog_root, state_home);
}

/// The scroll offset that follows the bottom of `body`.
pub fn pinned(body: &[String]) -> usize {
    crate::tui::screen::max_offset(body.len(), crate::tui::screen::size().body_rows())
}
