use super::{
    args::{Action, Options},
    compat, dispatch_compat, dispatch_security, dispatch_support, error, gate_cmd, guard, output,
    uninstall,
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
        Action::Check => unreachable!("check handled by the canonical diagnostics route"),
        Action::Tui => unreachable!("tui handled before catalog dispatch"),
        Action::Current => Ok((0, output::current(dispatch_support::session(home)?))),
        Action::Use(name) => {
            dispatch_support::find(harnesses, &name)?;
            crate::context::save(home, &name).map_err(dispatch_support::session_write_error)?;
            Ok((0, output::selected(&name)))
        }
        Action::Show(name) => {
            let selected = dispatch_support::selected_name(name, home)?;
            Ok((
                0,
                output::show(dispatch_support::find(harnesses, &selected)?),
            ))
        }
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
            let selected = dispatch_support::selected_name(name, home)?;
            guard::capability(harnesses, &selected, Capability::Download, options, home)
        }
        Action::Uninstall(Some(name)) => uninstall::run(harnesses, &name, options),
        Action::Uninstall(None) => {
            let selected = dispatch_support::selected_name(None, home)?;
            uninstall::run(harnesses, &selected, options)
        }
        Action::Update(Some(name)) => {
            guard::capability(harnesses, &name, Capability::Update, options, home)
        }
        Action::Update(None) if options.dry_run => Ok((0, compat::update_summary(harnesses))),
        Action::Update(None) => {
            let selected = dispatch_support::selected_name(None, home)?;
            guard::capability(harnesses, &selected, Capability::Update, options, home)
        }
        Action::Auth(words) => dispatch_compat::auth(&words, harnesses),
        Action::Config(words) => dispatch_compat::config(&words, catalog_root, home),
        Action::Cache(words) => dispatch_compat::cache(&words),
        Action::Security(words) => dispatch_security::run(&words, harnesses),
        Action::Gate(words) => gate_cmd::handle(&words, home).map_err(|message| {
            error::Failure::safety("gate_blocked", message, "run `terminal-jarvis gate status`")
        }),
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
#[path = "../tests/dispatch_test.rs"]
mod tests;
