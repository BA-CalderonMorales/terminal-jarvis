//! Outcome: applies one resolved command to the loop state -- body
//! absorption for the viewport, chat printing, hint/indicator refreshes.

use super::{status, Next};
use crate::contracts::Harness;
use std::path::Path;

/// Applies one command's outcome; false means the loop ends.
pub fn step(
    next: Next,
    sink: Vec<u8>,
    state: &mut super::state::LoopState,
    state_home: &Path,
    harnesses: &[Harness],
    catalog_root: &Path,
) -> bool {
    match next {
        Next::Exit => false,
        Next::Again {
            picker_shown,
            reset,
        } => {
            state.hint = status::modeline(state_home, picker_shown, state.debug);
            status::refresh_indicator(&mut state.indicator, state_home, state.debug);
            absorb(
                &mut state.body,
                &mut state.offset,
                sink,
                reset,
                harnesses,
                catalog_root,
                state_home,
            );
            true
        }
        Next::Stream { action, options } => super::stream::apply(
            &action,
            &options,
            state,
            harnesses,
            catalog_root,
            state_home,
        ),
        Next::Converse(seed) => {
            let width = crate::tui::screen::size().inner_cols();
            match crate::converse::wire::open(seed, state_home, width) {
                Ok((live, lines)) if crate::tui::screen::active() => {
                    state.body = lines;
                    state.converse = Some(live);
                    state.offset = super::viewport::pinned(&state.body);
                    state.hint = crate::converse::wire::hint(&state.converse).unwrap_or_default();
                }
                Ok(_) => state.body = vec!["converse runs in the viewport tui only".into()],
                Err(lines) => {
                    state.body = lines;
                    state.offset = super::viewport::pinned(&state.body);
                    state.hint = status::modeline(state_home, false, state.debug);
                }
            }
            true
        }
        Next::Debug(toggle) => {
            state.debug = toggle.unwrap_or(!state.debug);
            state.options.narrate = state.debug;
            state.hint = status::modeline(state_home, false, state.debug);
            status::refresh_indicator(&mut state.indicator, state_home, state.debug);
            let line = format!("debug view {}", if state.debug { "on" } else { "off" });
            if crate::tui::screen::active() {
                state.body.push(line);
            } else {
                println!("{line}");
            }
            true
        }
    }
}

/// Viewport absorbs captured output as the next body; chat prints it above
/// the prompt. A reset restores the primer.
pub fn absorb(
    body: &mut Vec<String>,
    offset: &mut usize,
    sink: Vec<u8>,
    reset: bool,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) {
    let text = String::from_utf8_lossy(&sink).to_string();
    if reset {
        *body = super::viewport::welcome(harnesses, catalog_root, state_home);
    } else if !text.is_empty() {
        *body = text.lines().map(String::from).collect();
    }
    *offset = super::viewport::pinned(body);
    if !crate::tui::screen::active() {
        print!("{text}");
        println!();
    }
}
