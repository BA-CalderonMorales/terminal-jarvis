pub mod args;
mod output;

use crate::{catalog, context, runtime};
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
    match action {
        Action::List => Ok((0, output::list(&harnesses))),
        Action::Check => Ok((0, output::checks(&harnesses))),
        Action::Current => Ok((
            0,
            output::current(context::load(home).map_err(|e| e.to_string())?),
        )),
        Action::Use(name) => {
            find(&harnesses, &name)?;
            context::save(home, &name).map_err(|error| error.to_string())?;
            Ok((0, format!("active harness = {name}\n")))
        }
        Action::Show(name) => Ok((0, output::show(find(&harnesses, &name)?))),
        Action::Plan {
            harness,
            capability,
        } => {
            let selected = selected_name(harness, home)?;
            Ok((0, output::plan(find(&harnesses, &selected)?, capability)))
        }
        Action::Run {
            harness,
            capability,
            extra,
        } => {
            let plan = find(&harnesses, &harness)?
                .plan(capability)
                .ok_or_else(|| format!("{harness} lacks {capability}"))?;
            runtime::run_command(plan, &extra)
                .map(|code| (code, String::new()))
                .map_err(|error| error.to_string())
        }
        Action::Help => Ok((0, output::help().to_string())),
    }
}

fn selected_name(explicit: Option<String>, home: &Path) -> Result<String, String> {
    if let Some(name) = explicit {
        return Ok(name);
    }
    context::load(home)
        .map_err(|error| error.to_string())?
        .map(|session| session.active_harness)
        .ok_or_else(|| "no active harness; run `terminal-jarvis use <harness>`".to_string())
}

fn find<'a>(
    harnesses: &'a [crate::contracts::Harness],
    name: &str,
) -> Result<&'a crate::contracts::Harness, String> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))
}
