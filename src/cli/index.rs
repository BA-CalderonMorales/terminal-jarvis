//! Public face of the cli domain: the command-line entry point.
//!
//! Consumers call `cli::run(args, catalog_root, home)` (see `src/main.rs`)
//! or tap `cli::args` for the parse surface. The domain's internals live in
//! `logic/`; structs/ holds the data shapes produced by parsing.

use crate::cli::logic::entry;
use crate::cli::logic::execute;
use crate::cli::logic::json;
use crate::cli::logic::style;
use crate::cli::structs::response::Response;
use std::path::Path;

pub use crate::cli::logic::args;

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
