//! Public face of the cli domain: the command-line entry point.
//!
//! Consumers call `cli::run(args, catalog_root, home)` (see `src/main.rs`)
//! or tap `cli::args` for the parse surface. The domain's internals live in
//! `logic/`; structs/ holds the data shapes produced by parsing.

use crate::cli::logic::{entry, execute, json, output, self_update, table};
use crate::cli::structs::response::Response;
use crate::diagnostics;
use std::path::Path;

pub use crate::cli::logic::args;
pub use crate::cli::logic::output_truth;
pub use crate::cli::logic::style;

/// The tui's canonical diagnostics route: same collection and rendering as
/// the headless `check`, as a String, without reaching into `cli::logic`.
pub fn status(
    catalog_root: &Path,
    home: &Path,
    harnesses: &[crate::contracts::Harness],
) -> Result<String, String> {
    let (stdout_tty, stderr_tty, color) = style::diagnostic_decisions();
    let runtime = diagnostics::RuntimeInput::local(
        stdout_tty,
        stderr_tty,
        color,
        table::terminal_width(),
        self_update::route_name(),
    );
    let input = diagnostics::DiagnosticInput::local(catalog_root, home, None, harnesses, runtime);
    Ok(output::diagnostics(&diagnostics::collect(&input)))
}

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
