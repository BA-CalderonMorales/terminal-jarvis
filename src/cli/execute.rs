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
    let harnesses =
        catalog::load(catalog_root).map_err(|cause| catalog_error(catalog_root, cause))?;
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

fn catalog_error(path: &Path, cause: std::io::Error) -> error::Failure {
    let (code, message) = match cause.kind() {
        std::io::ErrorKind::NotFound => (
            "catalog_missing",
            format!("harness catalog is missing at {}", path.display()),
        ),
        std::io::ErrorKind::PermissionDenied => (
            "catalog_permission_denied",
            format!("harness catalog is not readable at {}", path.display()),
        ),
        std::io::ErrorKind::InvalidData => (
            "catalog_invalid",
            format!("harness catalog is invalid: {cause}"),
        ),
        _ => (
            "catalog_unreadable",
            format!(
                "failed to load harness catalog at {}: {cause}",
                path.display()
            ),
        ),
    };
    error::Failure::state(
        code,
        message,
        "reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG to a valid catalog",
    )
}
