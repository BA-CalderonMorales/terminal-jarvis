mod action;
pub mod args;
mod cache;
mod compat;
mod dispatch;
mod help;
mod invoke;
mod output;
mod resolve;
mod version;

use crate::catalog;
use args::Action;
use std::path::Path;

pub fn run<I>(args: I, catalog_root: &Path, home: &Path) -> i32
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    match execute(args, catalog_root, home) {
        Ok((code, body)) => {
            if !body.is_empty() {
                print!("{body}");
            }
            code
        }
        Err(error) => {
            eprintln!("error: {error}");
            2
        }
    }
}

fn execute<I>(args: I, catalog_root: &Path, home: &Path) -> Result<(i32, String), String>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let action = args::parse(args)?;
    if action == Action::Help {
        return Ok((0, output::help().to_string()));
    }
    if let Action::Version { verbose } = action {
        return Ok((0, version::text(verbose, catalog_root, home)));
    }
    let harnesses =
        catalog::load(catalog_root).map_err(|error| catalog_error(catalog_root, error))?;
    let errors = catalog::validate(&harnesses);
    if !errors.is_empty() {
        return Err(errors.join("; "));
    }
    dispatch::dispatch(action, &harnesses, catalog_root, home)
}

fn catalog_error(path: &Path, error: std::io::Error) -> String {
    if error.kind() == std::io::ErrorKind::NotFound {
        return format!(
            "harness catalog is missing at {}; reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG",
            path.display()
        );
    }
    format!(
        "failed to load harness catalog at {}: {error}",
        path.display()
    )
}
