//! Public face of the cli domain: the command-line entry point.
//!
//! Consumers call `cli::run(args, catalog_root, home)` (see `src/main.rs`)
//! or tap `cli::args` for the parse surface. The domain's internals live in
//! `logic/`; structs/ holds the data shapes produced by parsing.

#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

use crate::cli::logic::{entry, execute, json};
use crate::cli::structs::response::Response;
use std::path::Path;

pub use crate::cli::logic::args;
pub use crate::cli::logic::canonical;
pub use crate::cli::logic::output_truth;
pub use crate::cli::logic::style;

pub fn dispatch(
    action: args::Action,
    options: &args::Options,
    harnesses: &[crate::contracts::Harness],
    catalog_root: &Path,
    home: &Path,
) -> Result<(i32, String), String> {
    crate::cli::logic::dispatch::dispatch(action, options, harnesses, catalog_root, home)
        .map_err(|failure| format!("{}: {}", failure.message, failure.next_action))
}

pub fn run<I>(args: I, catalog_root: &Path, home: &Path) -> i32
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let all = args.into_iter().map(Into::into).collect::<Vec<String>>();
    let parsed = match args::parse_cli(all.clone()) {
        Ok(parsed) => parsed,
        Err(error) => return entry::parse_failure(&all, &error),
    };
    let command = entry::action_name(&parsed.action);
    let mode = parsed.options.output;
    let plain = mode != args::OutputMode::Rich;
    let previous = style::set(plain, parsed.options.no_color || plain);
    let result = execute::run(parsed, catalog_root, home);
    let code = match result {
        Ok(response) => {
            let Response {
                exit_code: code,
                body,
                json: document,
            } = response;
            if mode == args::OutputMode::Json {
                print!(
                    "{}",
                    document.unwrap_or_else(|| json::outcome(&command, code, &body))
                );
            } else if !body.is_empty() {
                print!("{body}");
            }
            code
        }
        Err(error) => {
            if mode == args::OutputMode::Json {
                print!(
                    "{}",
                    json::failure(
                        &command,
                        error.exit_code,
                        error.code,
                        &error.message,
                        &error.next_action,
                    )
                );
            } else {
                eprint!("{}", style::error(&error.message));
                eprintln!("next action: {}", error.next_action);
            }
            error.exit_code
        }
    };
    style::restore(previous);
    code
}
