//! Shell: the read-prompt loop -- frame repaints per command, chat fallback.

use crate::{cli::args, contracts::Harness};
use std::path::Path;

#[path = "./canonical.rs"]
mod canonical;
#[path = "./dispatch.rs"]
mod dispatch;
#[path = "./handle.rs"]
mod handle;
#[path = "./help.rs"]
mod help;
#[path = "./outcome.rs"]
mod outcome;
#[path = "./run_action.rs"]
mod run_action;
#[path = "./session.rs"]
mod session;
#[path = "./status.rs"]
mod status;
#[path = "./verdict.rs"]
mod verdict;
#[path = "./viewport.rs"]
mod viewport;
#[path = "./viewport_raw.rs"]
mod viewport_raw;

pub use handle::handle;

#[path = "./converse.rs"]
mod converse;
#[path = "./stream.rs"]
mod stream;
#[path = "./stream_plan.rs"]
mod stream_plan;

pub fn run(harnesses: &[Harness], catalog_root: &Path, state_home: &Path, options: args::Options) {
    let debug = false;
    let indicator = super::input::Indicator {
        active: "none".into(),
        debug: false,
    };
    let viewport = super::screen::boot();
    let in_viewport = viewport.is_some();
    if !in_viewport {
        viewport::chat_banner(harnesses, catalog_root, state_home);
    }
    super::sigint::guarded(move || {
        let mut state = outcome::LoopState {
            converse: None,
            body: viewport::welcome(harnesses, catalog_root, state_home),
            hint: status::modeline(state_home, false, debug),
            options,
            debug,
            indicator,
        };
        status::refresh_indicator(&mut state.indicator, state_home, state.debug);
        loop {
            crate::tui::screen::ensure_usable();
            if let Some(lines) = converse::tick(&mut state, harnesses, catalog_root, state_home) {
                state.body.extend(lines);
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
                )
            } else {
                super::input::read_line(&state.indicator, &state.hint)
            };
            let Some(input) = input else { break };
            let mut sink = Vec::new();
            let next = handle(
                &mut sink,
                harnesses,
                catalog_root,
                state_home,
                &state.options,
                &input,
            );
            if !outcome::step(next, sink, &mut state, state_home, harnesses, catalog_root) {
                break;
            }
        }
    });
    drop(viewport);
    if !in_viewport {
        println!();
    }
}

#[path = "./parse.rs"]
mod parse;
pub use parse::{resolve, Next, Resolved};
