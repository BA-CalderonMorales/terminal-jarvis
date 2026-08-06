#[path = "catalog_load.rs"]
mod catalog_load;

use super::{
    args, dispatch, error, help_command, output, response::Response, self_update,
    self_update_intent, style, table, version,
};
use crate::{catalog, cli::args::Action, diagnostics};
use std::path::Path;

pub fn run(parsed: args::Parsed, catalog_root: &Path, home: &Path) -> error::Result<Response> {
    let args::Parsed { action, options } = parsed;
    if action == Action::Help {
        return Ok((0, output::help().to_string()).into());
    }
    if let Action::CommandHelp(ref command) = action {
        return Ok((0, help_command::text(command)).into());
    }
    if let Action::Version { verbose } = action {
        return Ok((0, version::text(verbose, catalog_root, home)).into());
    }
    if let Action::SelfUpdate { dry_run } = action {
        let preview = self_update::preview();
        self_update_intent::check(&options, &preview)?;
        if dry_run {
            return Ok((0, preview).into());
        }
        return self_update::run(dry_run)
            .map(Response::from)
            .map_err(|message| {
                error::Failure::unavailable(
                    "update_route_unavailable",
                    message,
                    "run `terminal-jarvis --update --dry-run` and update manually",
                )
            });
    }
    let harnesses = catalog::load(catalog_root)
        .map_err(|cause| catalog_load::catalog_error(catalog_root, cause))?;
    if action == Action::Tui {
        let plain = options.output != args::OutputMode::Rich;
        return crate::tui::run(catalog_root, home, plain, &harnesses)
            .map(Response::from)
            .map_err(|message| {
                error::Failure::unavailable(
                    "tui_unavailable",
                    message,
                    "run headless commands instead",
                )
            });
    }
    if action == Action::Check {
        let (stdout_tty, stderr_tty, color) = style::diagnostic_decisions();
        let runtime = diagnostics::RuntimeInput::local(
            stdout_tty,
            stderr_tty,
            color,
            table::terminal_width(),
            self_update::route_name(),
        );
        let input =
            diagnostics::DiagnosticInput::local(catalog_root, home, None, &harnesses, runtime);
        let report = diagnostics::collect(&input);
        let display = if options.verbose {
            report.clone()
        } else {
            report.concise()
        };
        let document = format!("{}\n", display.json());
        return Ok(Response::document(
            report.exit_code(),
            output::diagnostics(&display),
            document,
        ));
    }
    dispatch::dispatch(action, &options, &harnesses, catalog_root, home).map(Response::from)
}
