//! Viewport prompt: paints the composed frame and reads one line. In a
//! usable viewport the read is a raw-mode key session with scroll and an
//! in-frame editor; anything else falls back to the classic line reader.

use crate::contracts::Harness;
use crate::tui::input::Indicator;
use std::path::Path;

pub fn welcome(harnesses: &[Harness], catalog_root: &Path, state_home: &Path) -> Vec<String> {
    let o = crate::tui::home::collect(harnesses, catalog_root, state_home);
    let inner = crate::tui::screen::size().inner_cols();
    crate::tui::screen::welcome(&o.name, o.ready, o.total, inner)
}

pub use super::viewport_paint::paint;

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
    bottom: bool,
) -> Option<String> {
    paint(
        indicator,
        hint,
        harnesses,
        catalog_root,
        state_home,
        body,
        *offset,
        bottom,
    );
    let state = super::viewport_chrome::ViewportState::collect(
        indicator,
        harnesses,
        catalog_root,
        state_home,
    );
    let names: Vec<String> = harnesses
        .iter()
        .map(|harness| harness.name.clone())
        .collect();
    let mut session = super::viewport_raw::Session {
        state: &state,
        hint,
        body,
        history,
        offset,
        names: &names,
    };
    match crate::tui::term::enable_raw() {
        Some(_guard) => super::viewport_raw::run(&mut session, bottom),
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
