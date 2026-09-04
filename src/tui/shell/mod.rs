//! Shell: the read-prompt loop. Viewport mode paints a full-screen frame
//! (header, zoned body, footer prompt) and repaints it after every command;
//! chat mode keeps the classic print-above flow. One resolver, one guard.

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

pub fn run(
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    mut options: args::Options,
) {
    let mut debug = false;
    let mut indicator = super::input::Indicator {
        active: "none".into(),
        debug: false,
    };
    let viewport = super::screen::boot();
    let in_viewport = viewport.is_some();
    if !in_viewport {
        viewport::chat_banner(harnesses, catalog_root, state_home);
    }
    super::sigint::guarded(move || {
        let mut hint = status::modeline(state_home, false, debug);
        status::refresh_indicator(&mut indicator, state_home, debug);
        let mut body = viewport::welcome(harnesses, catalog_root, state_home);
        loop {
            crate::tui::screen::ensure_usable();
            let input = if in_viewport && crate::tui::screen::active() {
                viewport::prompt(
                    &indicator,
                    &hint,
                    harnesses,
                    catalog_root,
                    state_home,
                    &body,
                )
            } else {
                super::input::read_line(&indicator, &hint)
            };
            let Some(input) = input else { break };
            let mut sink = Vec::new();
            let next = handle(
                &mut sink,
                harnesses,
                catalog_root,
                state_home,
                &options,
                &input,
            );
            if !outcome::step(
                next,
                &mut body,
                sink,
                &mut hint,
                &mut options,
                &mut debug,
                &mut indicator,
                state_home,
                harnesses,
                catalog_root,
            ) {
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
