use super::{
    args::{Action, Options},
    compat, dispatch_compat, dispatch_security, dispatch_support, error, experimental, gate_cmd,
    guard, output,
};
use crate::contracts::{Capability, Harness};
use std::path::Path;

pub fn dispatch(
    action: Action,
    options: &Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    home: &Path,
) -> error::Result<(i32, String)> {
    match action {
        Action::List => Ok((0, output::list(harnesses))),
        Action::Check => Ok((0, output::checks(harnesses))),
        Action::Current => Ok((0, output::current(dispatch_support::session(home)?))),
        Action::Use(name) => {
            dispatch_support::find(harnesses, &name)?;
            crate::context::save(home, &name).map_err(dispatch_support::session_error)?;
            Ok((0, output::selected(&name)))
        }
        Action::Show(name) => Ok((0, output::show(dispatch_support::find(harnesses, &name)?))),
        Action::Plan {
            harness,
            capability,
        } => {
            let selected = dispatch_support::selected_name(harness, home)?;
            Ok((
                0,
                output::plan(dispatch_support::find(harnesses, &selected)?, capability),
            ))
        }
        Action::SelfUpdate { .. } => {
            unreachable!("self-update handled before catalog load in execute()")
        }
        Action::Run(words) => guard::run(&words, options, harnesses, home),
        Action::Direct { harness, extra } => {
            guard::direct(&harness, &extra, options, harnesses, home)
        }
        Action::Install(name) => {
            guard::capability(harnesses, &name, Capability::Download, options, home)
        }
        Action::Update(Some(name)) => {
            guard::capability(harnesses, &name, Capability::Update, options, home)
        }
        Action::Update(None) => Ok((0, compat::update_summary(harnesses))),
        Action::Auth(words) => dispatch_compat::auth(&words, harnesses),
        Action::Config(words) => dispatch_compat::config(&words, catalog_root, home),
        Action::Cache(words) => dispatch_compat::cache(&words),
        Action::Security(words) => dispatch_security::run(&words, harnesses),
        Action::Gate(words) => gate_cmd::handle(&words, home).map_err(|message| {
            error::Failure::safety("gate_blocked", message, "run `terminal-jarvis gate status`")
        }),
        Action::Experimental(words) => experimental::run(&words, harnesses, home)
            .map(|body| (0, body))
            .map_err(dispatch_support::experimental_error),
        Action::Legacy(command) => Err(error::Failure::unavailable(
            "removed_command",
            format!("{command} was removed with the v0.1 catalog rewrite"),
            "use list, show, plan, run, install, update, auth, or security",
        )),
        Action::Help => Ok((0, output::help())),
        Action::CommandHelp(_) => unreachable!("command help handled before catalog load"),
        Action::Version { .. } => unreachable!("version is handled before catalog load"),
    }
}

#[cfg(test)]
#[path = "dispatch_test.rs"]
mod tests;
