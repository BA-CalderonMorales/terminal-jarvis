//! Command: one typed line through the grammar and the applier -- the shell
//! loop's whole per-line decision, so the loop itself stays a paint cycle.

use super::{outcome, Next};
use crate::contracts::Harness;
use std::path::Path;

/// Runs one typed line against the session; false ends the shell.
pub fn run(
    input: &str,
    state: &mut super::state::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    let mut sink = Vec::new();
    let next = match super::state::notice(state, input) {
        Some(seed) => Next::Converse(Some(seed)),
        None => super::handle(
            &mut sink,
            harnesses,
            catalog_root,
            state_home,
            &state.options,
            input,
        ),
    };
    outcome::step(next, sink, state, state_home, harnesses, catalog_root)
}
