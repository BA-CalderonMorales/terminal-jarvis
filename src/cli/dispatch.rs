use super::{args::Action, compat, invoke, output, resolve};
use crate::context;
use crate::contracts::{Capability, Harness};
use std::path::Path;

pub fn dispatch(
    action: Action,
    harnesses: &[Harness],
    catalog_root: &Path,
    home: &Path,
) -> Result<(i32, String), String> {
    match action {
        Action::List => Ok((0, output::list(harnesses))),
        Action::Check => Ok((0, output::checks(harnesses))),
        Action::Current => Ok((0, output::current(context::load(home).map_err(err)?))),
        Action::Use(name) => {
            find(harnesses, &name)?;
            context::save(home, &name).map_err(err)?;
            Ok((0, format!("active harness = {name}\n")))
        }
        Action::Show(name) => Ok((0, output::show(find(harnesses, &name)?))),
        Action::Plan {
            harness,
            capability,
        } => {
            let selected = selected_name(harness, home)?;
            Ok((0, output::plan(find(harnesses, &selected)?, capability)))
        }
        Action::SelfUpdate => unreachable!("self-update handled before catalog load in execute()"),
        Action::Run(words) => invoke::invocation(resolve::run(&words, harnesses, home)?, harnesses),
        Action::Direct { harness, extra } => {
            invoke::invocation(resolve::direct(&harness, &extra, harnesses)?, harnesses)
        }
        Action::Install(name) => invoke::capability(harnesses, &name, Capability::Download, &[]),
        Action::Update(Some(name)) => invoke::capability(harnesses, &name, Capability::Update, &[]),
        Action::Update(None) => Ok((0, compat::update_summary(harnesses))),
        Action::Auth(words) => compat::auth(&words, harnesses).map(|body| (0, body)),
        Action::Config(words) => compat::config(
            &words,
            catalog_root,
            home,
            context::load(home).map_err(err)?,
        )
        .map(|body| (0, body)),
        Action::Cache(words) => compat::cache(&words).map(|body| (0, body)),
        Action::Security(words) => security(&words, harnesses),
        Action::Legacy(command) => Ok((0, compat::legacy(&command))),
        Action::Help => Ok((0, output::help().to_string())),
        Action::Version { .. } => unreachable!("version is handled before catalog load"),
    }
}

fn security(words: &[String], harnesses: &[Harness]) -> Result<(i32, String), String> {
    match words {
        [] => Ok((0, output::status(harnesses))),
        [action] if action == "status" => Ok((0, output::status(harnesses))),
        [action] if action == "audit" => Ok((0, output::audit(harnesses))),
        [name] => Ok((
            0,
            output::plan(
                find(harnesses, name)
                    .map_err(|_| "usage: terminal-jarvis security [status|audit|harness]")?,
                Capability::Security,
            ),
        )),
        _ => Err("usage: terminal-jarvis security [status|audit|harness]".to_string()),
    }
}

fn selected_name(explicit: Option<String>, home: &Path) -> Result<String, String> {
    explicit.map_or_else(
        || {
            context::load(home)
                .map_err(err)?
                .map(|session| session.active_harness)
                .ok_or_else(|| "no active harness; run `terminal-jarvis use <harness>`".to_string())
        },
        Ok,
    )
}

fn find<'a>(harnesses: &'a [Harness], name: &str) -> Result<&'a Harness, String> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))
}

fn err(error: impl std::fmt::Display) -> String {
    error.to_string()
}

#[cfg(test)]
#[path = "dispatch_test.rs"]
mod tests;
