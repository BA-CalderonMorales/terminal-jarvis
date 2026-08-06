//! Handle: executes a resolved line. `list` renders the numbered picker
//! (not the headless table), `status` takes the canonical diagnostics
//! route, `home`/`clear` reset to the pristine frame, everything else
//! dispatches through the same guards as the headless cli.

use super::Resolved;
use crate::cli::args;
use crate::cli::{self, dispatch, style};
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
            match cli::status(catalog_root, state_home, harnesses) {
                Ok(body) => print!("{body}"),
                Err(message) => eprintln!("{}", style::error(&message)),
            }
            false
        }
        _ => dispatch_action(action, options, harnesses, catalog_root, state_home),
    }
}

fn dispatch_action(
    action: args::Action,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match dispatch(action, options, harnesses, catalog_root, state_home) {
        Ok((_, body)) => {
            if !body.is_empty() {
                print!("{body}");
                if !body.ends_with('\n') {
                    println!();
                }
            }
            false
        }
        Err(message) => {
            eprintln!("{message}");
            false
        }
    }
}

#[cfg(test)]
#[path = "../tests/handle.rs"]
mod tests;
