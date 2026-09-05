//! ConverseWire: the shell-side application of the conversation state
//! machine. One turn per frame; the loop repaints between every exchange,
//! with a splunk-style marker while the current speaker is mid-response.

use super::{outcome, status, viewport};
use crate::contracts::Harness;
use std::path::Path;

/// Runs at most one conversation turn: paint the current transcript, mark
/// the responding agent, invoke it, then hand the body delta back.
pub fn tick(
    state: &mut outcome::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> Option<Vec<String>> {
    if !crate::tui::screen::active() {
        return None;
    }
    paint_frame(state, harnesses, catalog_root, state_home);
    let thinking = state
        .converse
        .as_ref()
        .map(crate::converse::wire::thinking_line);
    if let Some(marker) = &thinking {
        state.body.push(marker.clone());
        paint_frame(state, harnesses, catalog_root, state_home);
    }
    let mut speak =
        |name: &str, prompt: &str| crate::cli::headless_one_shot(harnesses, name, prompt);
    let width = crate::tui::screen::size().inner_cols();
    let lines = crate::converse::wire::pending(&mut state.converse, &mut speak, width);
    if thinking.is_some() {
        state.body.pop();
    }
    let lines = lines?;
    state.hint = crate::converse::wire::hint(&state.converse)
        .unwrap_or_else(|| status::modeline(state_home, false, state.debug));
    Some(lines)
}

fn paint_frame(
    state: &mut outcome::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) {
    viewport::paint(
        &state.indicator,
        &state.hint,
        harnesses,
        catalog_root,
        state_home,
        &state.body,
    );
}
