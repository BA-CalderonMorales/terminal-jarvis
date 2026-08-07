//! The dispatch gate for actions the headless cli pre-routes before catalog
//! dispatch (mirrored in `cli::canonical` and `execute.rs`): `tui` (we are
//! already inside it), `version`, command help, and `self-update`. Without
//! this gate those actions fall into dispatch's `unreachable!` arms and
//! panic the shell. Everything else dispatches through the same guarded
//! surface as headless automation.

use crate::cli;
use crate::cli::{args, dispatch, style};
use crate::contracts::Harness;
use std::path::Path;

pub fn run(
    action: args::Action,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match action {
        args::Action::Tui => {
            eprintln!("{}", style::error("you are already in the tui"));
            false
        }
        action @ (args::Action::Version { .. }
        | args::Action::CommandHelp(_)
        | args::Action::SelfUpdate { .. }) => canonical(action, catalog_root, state_home),
        action => dispatch_action(action, options, harnesses, catalog_root, state_home),
    }
}

fn canonical(action: args::Action, catalog_root: &Path, state_home: &Path) -> bool {
    match cli::canonical::text(action, catalog_root, state_home) {
        Ok(body) => {
            print!("{body}");
            if !body.ends_with('\n') {
                println!();
            }
            false
        }
        Err(message) => {
            eprintln!("{}", style::error(&message));
            false
        }
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
