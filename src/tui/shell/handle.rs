//! Handle: executes a resolved line. `list` renders the numbered picker
//! (not the headless table), `status` takes the canonical diagnostics
//! route, `home`/`clear` reset to the pristine frame, everything else
//! dispatches through the same guards as the headless cli. All output goes
//! to the caller's sink: stdout in chat mode, a capture buffer in viewport.

use super::{Next, Resolved};
use crate::cli::{args, style};
use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;

pub fn handle(
    out: &mut dyn Write,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    options: &args::Options,
    input: &str,
) -> Next {
    match super::resolve(input, harnesses) {
        Resolved::Empty => Next::Again {
            picker_shown: false,
            reset: false,
        },
        Resolved::Exit => Next::Exit,
        Resolved::Help => {
            let _ = write!(out, "{}", super::help::text());
            Next::Again {
                picker_shown: false,
                reset: false,
            }
        }
        Resolved::Home => {
            let _ = write!(out, "{}", crate::tui::term::clear_screen());
            crate::tui::home::render(out, harnesses, catalog_root, state_home);
            Next::Again {
                picker_shown: false,
                reset: true,
            }
        }
        Resolved::Run(action) => {
            let picker_shown =
                run_action(out, action, options, harnesses, catalog_root, state_home);
            Next::Again {
                picker_shown,
                reset: false,
            }
        }
        Resolved::Debug(toggle) => Next::Debug(toggle),
        Resolved::Error(message) => {
            let _ = writeln!(out, "{}", style::error(&message));
            Next::Again {
                picker_shown: false,
                reset: false,
            }
        }
    }
}

fn run_action(
    out: &mut dyn Write,
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
            let _ = writeln!(out, "{}", style::heading("Available Harnesses"));
            let _ = write!(out, "{body}");
            true
        }
        args::Action::Check => {
            let _ = write!(
                out,
                "{}",
                super::status::render(harnesses, catalog_root, state_home)
            );
            false
        }
        action => super::session::run(out, action, options, harnesses, catalog_root, state_home),
    }
}

#[cfg(test)]
#[path = "../tests/handle.rs"]
mod tests;
