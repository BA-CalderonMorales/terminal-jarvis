//! Canonical: text bodies for the actions the headless cli pre-routes before
//! catalog dispatch (see `execute.rs`): version, command help, self-update.
//! The tui reaches them through `cli::canonical` so a slash command can never
//! fall through to dispatch's `unreachable!` arms. Self-update is always
//! advise-only here: the tui never swaps its own binary.

use super::{help_command, self_update, version};
use crate::cli::args;
use std::path::Path;

pub fn text(action: args::Action, catalog_root: &Path, home: &Path) -> Result<String, String> {
    match action {
        args::Action::CommandHelp(command) => Ok(help_command::text(&command)),
        args::Action::Version { verbose } => Ok(version::text(verbose, catalog_root, home)),
        args::Action::SelfUpdate { .. } => Ok(format!(
            "{}\napply it from a normal shell: terminal-jarvis --update --dry-run, then --no-input --confirm=self-update:terminal-jarvis --update",
            self_update::preview()
        )),
        _ => Err("this action has no canonical text form".into()),
    }
}

#[cfg(test)]
#[path = "../tests/canonical_test.rs"]
mod tests;
