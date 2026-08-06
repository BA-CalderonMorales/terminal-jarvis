//! Public face of the tui domain: the interactive switcher for the harness
//! catalog. `run` is invoked by the cli domain for the `tui` action and for
//! bare invocations on a terminal. The domain is a thin, line-oriented shell
//! over the headless command surface -- every slash command and selection
//! maps onto the exact `Action` grammar the cli parses, so TUI and headless
//! never drift. `guard` is pure and unit-tested; `run` only executes it.

pub mod home;
pub mod input;
pub mod palette;
pub mod shell;
pub mod sigint;
pub mod switcher;
pub mod term;

use crate::cli;
use crate::contracts::Harness;
use std::io::IsTerminal;
use std::path::Path;

pub fn run(
    catalog_root: &Path,
    state_home: &Path,
    plain: bool,
    harnesses: &[Harness],
) -> Result<(i32, String), String> {
    guard(
        plain,
        std::io::stdin().is_terminal(),
        std::io::stdout().is_terminal(),
    )?;
    let options = cli::args::Options::default();
    home::render(harnesses, catalog_root, state_home);
    shell::run(harnesses, catalog_root, state_home, &options);
    Ok((0, String::new()))
}

pub fn guard(plain: bool, stdin_terminal: bool, stdout_terminal: bool) -> Result<(), String> {
    if !stdin_terminal || !stdout_terminal {
        return Err(
            "the tui requires an interactive terminal; use headless commands instead".into(),
        );
    }
    if plain {
        return Err("the tui cannot run with --plain; drop the flag and run again".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/guard.rs"]
mod tests;
