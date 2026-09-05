//! RunAction: the picker and dashboard actions -- render to the sink and
//! report whether the picker was shown; everything else is a session run.

use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;

/// The harness a run/direct action targets, when its binary is missing.
fn missing_target(action: &crate::cli::args::Action, harnesses: &[Harness]) -> Option<String> {
    let name = match action {
        crate::cli::args::Action::Direct { harness, .. } => Some(harness.clone()),
        crate::cli::args::Action::Run(words) => words.first().cloned(),
        _ => None,
    }?;
    let harness = harnesses.iter().find(|h| h.name == name)?;
    let missing = !crate::security::command_on_path(&harness.binary);
    missing.then(|| harness.name.clone())
}

pub fn run(
    out: &mut dyn Write,
    action: crate::cli::args::Action,
    options: &crate::cli::args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    if let Some(name) = missing_target(&action, harnesses) {
        let harness = harnesses.iter().find(|h| h.name == name).unwrap();
        let _ = writeln!(
            out,
            "{}",
            crate::cli::style::dim(&format!(
                "{} isn't installed yet -- no worries, setting it up now (gated as always).",
                harness.display
            ))
        );
        let installed = super::session::run(
            out,
            crate::cli::args::Action::Install(Some(name.clone())),
            options,
            harnesses,
            catalog_root,
            state_home,
        );
        if !crate::security::command_on_path(&harness.binary) {
            let _ = writeln!(
                out,
                "{}",
                crate::cli::style::error(&format!(
                    "{} still isn't on PATH after the install attempt; fix PATH and try again.",
                    harness.binary
                ))
            );
            return installed;
        }
        let _ = writeln!(
            out,
            "{}",
            crate::cli::style::dim("installed -- resuming your command.")
        );
    }
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
