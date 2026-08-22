//! The dispatch gate for actions the headless cli pre-routes before catalog
//! dispatch (mirrored in `cli::canonical` and `execute.rs`): `tui` (we are
//! already inside it), `version`, command help, and `self-update`. Without
//! this gate those actions fall into dispatch's `unreachable!` arms and
//! panic the shell. Everything else dispatches through the same guarded
//! surface as headless automation.

use crate::cli;
use crate::cli::{args, style};
use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;

pub fn run(
    out: &mut dyn Write,
    action: args::Action,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match action {
        args::Action::Tui => {
            let _ = writeln!(out, "{}", style::error("you are already in the tui"));
            false
        }
        action @ (args::Action::Version { .. }
        | args::Action::CommandHelp(_)
        | args::Action::SelfUpdate { .. }) => canonical(out, action, catalog_root, state_home),
        action => super::dispatch::run(out, action, options, harnesses, catalog_root, state_home),
    }
}

fn canonical(
    out: &mut dyn Write,
    action: args::Action,
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match cli::canonical::text(action, catalog_root, state_home) {
        Ok(body) => {
            emit(out, &body);
            false
        }
        Err(message) => {
            let _ = writeln!(out, "{}", style::error(&message));
            false
        }
    }
}

fn emit(out: &mut dyn Write, body: &str) {
    let _ = out.write_all(body.as_bytes());
    if !body.ends_with('\n') {
        let _ = writeln!(out);
    }
}
