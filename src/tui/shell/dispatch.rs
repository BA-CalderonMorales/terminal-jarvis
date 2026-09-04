//! Dispatch: runs one resolved action and reports the outcome through the
//! caller's sink, with the security gate verdict card for lifecycle verbs.

use crate::cli::{args, dispatch};
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
    let started = std::time::Instant::now();
    let lifecycle = match &action {
        crate::cli::args::Action::Install(name) => Some((name.clone(), "installed")),
        crate::cli::args::Action::Update(Some(name)) => Some((name.clone(), "updated")),
        _ => None,
    };
    crate::tui::sigint::child_running(true);
    let suspended = crate::tui::screen::suspend();
    let outcome = dispatch(action, options, harnesses, catalog_root, state_home);
    crate::tui::screen::resume(suspended);
    crate::tui::sigint::child_running(false);
    match &outcome {
        Ok((_, body)) if !body.is_empty() => {
            let _ = out.write_all(body.as_bytes());
            if !body.ends_with('\n') {
                let _ = writeln!(out);
            }
        }
        Err(message) => {
            let _ = writeln!(out, "{message}");
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
                out,
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
