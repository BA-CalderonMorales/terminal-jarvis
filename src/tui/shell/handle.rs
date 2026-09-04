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

fn again_plain() -> Next {
    Next::Again {
        picker_shown: false,
        reset: false,
    }
}

pub fn handle(
    out: &mut dyn Write,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    options: &args::Options,
    input: &str,
) -> Next {
    match super::resolve(input, harnesses) {
        Resolved::Empty => again_plain(),
        Resolved::Exit => Next::Exit,
        Resolved::Help => {
            let _ = write!(out, "{}", super::help::text());
            again_plain()
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
                super::run_action::run(out, action, options, harnesses, catalog_root, state_home);
            Next::Again {
                picker_shown,
                reset: false,
            }
        }
        Resolved::Theme(choice) => {
            let _ = write!(out, "{}", theme_reply(choice.as_deref()));
            again_plain()
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

/// `/theme` reply: with no argument, cycle to the next theme; with one,
/// pin it. Unknown names keep the list discoverable.
fn theme_reply(choice: Option<&str>) -> String {
    match choice {
        None => format!("theme '{}' applied", crate::tui::screen::cycle_theme()),
        Some(name) if crate::tui::screen::apply_theme(name) => format!("theme '{name}' applied"),
        Some(name) => format!(
            "unknown theme '{name}'; themes: {}",
            crate::tui::screen::theme_names().join(", ")
        ),
    }
}

#[cfg(test)]
#[path = "../tests/handle.rs"]
mod tests;
