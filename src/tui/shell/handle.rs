//! Handle: executes a resolved line. `list` renders the numbered picker
//! (not the headless table), `status` takes the canonical diagnostics
//! route, `home`/`clear` reset to the pristine frame, everything else
//! dispatches through the same guards as the headless cli.

use super::Resolved;
use crate::cli::{args, style};
use crate::contracts::Harness;
use std::path::Path;

pub use super::Next;

pub fn handle(
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    options: &args::Options,
    input: &str,
) -> Next {
    match super::resolve(input, harnesses) {
        Resolved::Empty => Next::Again {
            picker_shown: false,
        },
        Resolved::Exit => Next::Exit,
        Resolved::Home => {
            print!("{}", crate::tui::term::clear_screen());
            crate::tui::home::render(harnesses, catalog_root, state_home);
            Next::Again {
                picker_shown: false,
            }
        }
        Resolved::Run(action) => {
            let picker_shown = run_action(action, options, harnesses, catalog_root, state_home);
            Next::Again { picker_shown }
        }
        Resolved::Error(message) => {
            eprintln!("{}", style::error(&message));
            Next::Again {
                picker_shown: false,
            }
        }
    }
}

fn run_action(
    action: args::Action,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match action {
        args::Action::List => {
            let active = crate::context::load(state_home)
                .ok()
                .flatten()
                .map(|session| session.active_harness);
            let body = crate::tui::switcher::pick(harnesses, active.as_deref());
            print!("{}\n{body}", style::heading("Available Harnesses"));
            true
        }
        args::Action::Check => {
            print!("{}", super::status::render(harnesses, catalog_root, state_home));
            false
        }
        action => super::canonical::run(action, options, harnesses, catalog_root, state_home),
    }
}

#[cfg(test)]
#[path = "../tests/handle.rs"]
mod tests;
