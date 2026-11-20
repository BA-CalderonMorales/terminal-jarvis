//! Shell: the read-prompt loop -- frame repaints per command, chat fallback.
use crate::{cli::args, contracts::Harness};
use std::path::Path;
mod canonical;
mod command;
mod complete;
mod converse;
mod converse_live;
mod converse_turn;
mod dispatch;
mod fleet;
mod handle;
pub(crate) mod help;
mod live_nav;
mod outcome;
mod picker;
mod reply;
mod run_action;
mod session;
mod state;
mod status;
mod stream;
mod stream_finish;
mod stream_plan;
mod stream_rows;
mod verdict;
mod viewport;
mod viewport_chrome;
mod viewport_nav;
mod viewport_page;
mod viewport_paint;
mod viewport_raw;
pub use handle::handle;
pub fn run(harnesses: &[Harness], catalog_root: &Path, state_home: &Path, options: args::Options) {
    let debug = false;
    let indicator = super::input::Indicator {
        active: "none".into(),
        debug,
    };
    let viewport = super::screen::boot();
    let in_viewport = viewport.is_some();
    if !in_viewport {
        viewport::chat_banner(harnesses, catalog_root, state_home);
    }
    super::sigint::guarded(move || {
        let mut state = state::LoopState {
            converse: None,
            pending_converse: None,
            offset: 0,
            history: Vec::new(),
            body: viewport::welcome(harnesses, catalog_root, state_home),
            hint: status::modeline(state_home, false, debug),
            bottom: false,
            options,
            debug,
            indicator,
        };
        status::refresh_indicator(&mut state.indicator, state_home, state.debug);
        loop {
            let framed = crate::tui::screen::active();
            crate::tui::screen::ensure_usable();
            status::announce(framed);
            if let Some(lines) = converse::tick(&mut state, harnesses, catalog_root, state_home) {
                state.body.extend(lines);
                state.offset = viewport::pinned(&state.body);
                continue;
            }
            let input = if in_viewport && crate::tui::screen::active() {
                viewport::prompt(
                    &state.indicator,
                    &state.hint,
                    harnesses,
                    catalog_root,
                    state_home,
                    &state.body,
                    &state.history,
                    &mut state.offset,
                    state.bottom,
                )
            } else {
                super::input::read_line(&state.indicator, &state.hint)
            };
            let Some(input) = input else { break };
            if state.history.last() != Some(&input) {
                state.history.push(input.clone());
            }
            if !command::run(&input, &mut state, harnesses, catalog_root, state_home) {
                break;
            }
        }
    });
    drop(viewport);
    if !in_viewport {
        println!();
    }
}
mod parse;
pub use parse::{resolve, Next, Resolved};
