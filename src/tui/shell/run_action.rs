//! RunAction: the picker and dashboard actions -- render to the sink and
//! report whether the picker was shown; everything else is a session run.

use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;

pub fn run(
    out: &mut dyn Write,
    action: crate::cli::args::Action,
    options: &crate::cli::args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match action {
        crate::cli::args::Action::List => {
            let active = crate::context::load(state_home)
                .ok()
                .flatten()
                .map(|session| session.active_harness);
            let body = crate::tui::switcher::pick(harnesses, active.as_deref());
            let _ = writeln!(out, "{}", crate::cli::style::heading("Available Harnesses"));
            let _ = write!(out, "{body}");
            true
        }
        crate::cli::args::Action::Check => {
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
