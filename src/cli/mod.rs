pub mod args;
mod compat;
mod dispatch;
mod help;
mod invoke;
mod output;
mod resolve;

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
    let harnesses = catalog::load(catalog_root).map_err(|error| error.to_string())?;
    let errors = catalog::validate(&harnesses);
    if !errors.is_empty() {
        return Err(errors.join("; "));
    }
    dispatch::dispatch(action, &harnesses, catalog_root, home)
}
