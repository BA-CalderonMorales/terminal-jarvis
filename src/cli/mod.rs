mod action;
pub mod args;
mod cache;
mod compat;
mod compat_support;
mod dispatch;
mod experimental;
mod gate_cmd;
mod guard;
mod help;
mod invoke;
mod output;
mod resolve;
mod self_update;
mod style;
mod table;
mod version;
use crate::catalog;
use args::Action;
use std::path::Path;

pub fn run<I>(args: I, catalog_root: &Path, home: &Path) -> i32
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let (args, plain, no_color) = presentation_args(args);
    let previous = style::set(plain, no_color);
    let result = execute(args, catalog_root, home);
    let code = match result {
        Ok((code, body)) => {
            if !body.is_empty() {
                print!("{body}");
            }
            code
        }
        Err(error) => {
            eprint!("{}", style::error(&error));
            2
        }
    };
    style::restore(previous);
    code
}

fn presentation_args<I>(args: I) -> (Vec<String>, bool, bool)
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let mut all = args.into_iter().map(Into::into).collect::<Vec<_>>();
    let mut plain = false;
    let mut no_color = false;
    while all
        .get(1)
        .is_some_and(|word| word == "--plain" || word == "--no-color")
    {
        let flag = all.remove(1);
        plain |= flag == "--plain";
        no_color |= flag == "--no-color";
    }
    (all, plain, no_color)
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
    if let Action::SelfUpdate { dry_run } = action {
        return self_update::run(dry_run);
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
