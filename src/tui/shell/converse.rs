//! ConverseWire: the shell-side application of the conversation state
//! machine. One turn per frame; the loop repaints between every exchange.

use super::{outcome, status, viewport};
use crate::contracts::Harness;
use std::path::Path;

/// Runs at most one conversation turn: paint the current transcript, invoke
/// the next speaker, hand the body delta back. `None` when no conversation
/// is live or the turn budget is spent.
pub fn tick(
    state: &mut outcome::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> Option<Vec<String>> {
    if !crate::tui::screen::active() {
        return None;
    }
    viewport::paint(
        &state.indicator,
        &state.hint,
        harnesses,
        catalog_root,
        state_home,
        &state.body,
    );
    let mut speak =
        |name: &str, prompt: &str| crate::cli::headless_one_shot(harnesses, name, prompt);
    let lines = crate::converse::wire::pending(&mut state.converse, &mut speak)?;
    state.hint = crate::converse::wire::hint(&state.converse)
        .unwrap_or_else(|| status::modeline(state_home, false, state.debug));
    Some(lines)
}
