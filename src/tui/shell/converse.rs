use super::converse_live;
use super::status;
use crate::contracts::Harness;
use std::path::Path;

pub fn tick(
    state: &mut super::state::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> Option<Vec<String>> {
    if !crate::tui::screen::active() {
        return None;
    }
    let thinking = state
        .converse
        .as_ref()
        .map(crate::converse::wire::thinking_line);
    if let Some(marker) = &thinking {
        state.body.push(marker.clone());
    }
    let super::state::LoopState {
        body,
        offset,
        converse,
        indicator,
        hint,
        ..
    } = state;
    let mut turn = converse_live::Turn {
        body,
        offset,
        indicator,
        hint,
        harnesses,
        catalog_root,
        state_home,
    };
    turn.push_flush();
    let allow_dangerous = converse.as_ref().is_some_and(|live| live.allow_dangerous);
    let mut speak = |name: &str, prompt: &str| {
        super::converse_turn::speak_turn(&mut turn, harnesses, name, prompt, allow_dangerous)
    };
    let width = crate::tui::screen::size().inner_cols();
    let lines = crate::converse::wire::pending(converse, &mut speak, width);
    if let Some(marker) = thinking {
        if let Some(index) = state.body.iter().position(|row| row == &marker) {
            state.body.remove(index);
        }
    }
    let lines = lines?;
    state.hint = crate::converse::wire::hint(&state.converse)
        .unwrap_or_else(|| status::modeline(state_home, false, state.debug));
    Some(lines)
}
