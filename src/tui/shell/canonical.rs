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
    let started = std::time::Instant::now();
    let lifecycle = match &action {
        crate::cli::args::Action::Install(name) => Some((name.clone(), "installed")),
        crate::cli::args::Action::Update(Some(name)) => Some((name.clone(), "updated")),
        _ => None,
    };
    crate::tui::sigint::child_running(true);
    let outcome = dispatch(action, options, harnesses, catalog_root, state_home);
    crate::tui::sigint::child_running(false);
    match &outcome {
        Ok((_, body)) if !body.is_empty() => {
            print!("{body}");
            if !body.ends_with('\n') {
                println!();
            }
        }
        Err(message) => {
            eprintln!("{message}");
        }
        _ => {}
    }
    if !options.narrate {
        if let Some((name, verb)) = &lifecycle {
            let binary_on_path = harnesses
                .iter()
                .find(|harness| harness.name == *name)
                .is_some_and(|harness| crate::security::command_on_path(&harness.binary));
            super::verdict::settle(
                name,
                verb,
                binary_on_path,
                &outcome,
                started.elapsed(),
                state_home,
            );
        }
    }
    false
}
