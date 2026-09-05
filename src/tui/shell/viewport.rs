//! Viewport prompt: paints the composed frame and reads one line. In a
//! usable viewport the read is a raw-mode key session with scroll and an
//! in-frame editor; anything else falls back to the classic line reader.

use crate::contracts::Harness;
use crate::tui::input::Indicator;
use std::io::Write;
use std::path::Path;

pub fn welcome(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> Vec<String> {
    let o = crate::tui::home::collect(harnesses, catalog_root, state_home);
    crate::tui::screen::welcome(&o.name, o.ready, o.total)
}

/// Paints the composed frame without reading -- the converse loop repaints
/// between turns while the child owns the wait.
#[allow(clippy::too_many_arguments)]
pub fn paint(
    indicator: &Indicator,
    hint: &str,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    body: &[String],
) {
    let _ = std::io::stdout().flush();
    let state =
        super::viewport_raw::ViewportState::collect(indicator, harnesses, catalog_root, state_home);
    let size = crate::tui::screen::size();
    let draft = state.base_draft(hint, body);
    let cells = state.prefix_cells;
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
) -> Option<String> {
    paint(indicator, hint, harnesses, catalog_root, state_home, body);
    let state =
        super::viewport_raw::ViewportState::collect(indicator, harnesses, catalog_root, state_home);
    let session = super::viewport_raw::Session {
        state: &state,
        hint,
        body,
    };
    match crate::tui::term::enable_raw() {
        Some(_guard) => super::viewport_raw::run(&session),
        None => crate::tui::input::raw_line(),
    }
}

/// Chat-mode boot banner: the welcome frame above the first prompt.
pub fn chat_banner(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) {
    let mut out = std::io::stdout();
    crate::tui::home::render(&mut out, harnesses, catalog_root, state_home);
}
