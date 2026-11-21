//! `settings`: list the optional features, or set/reset one choice. The tui
//! reaches this same function through the shared dispatch.

use super::args::Options;
use super::{error, settings_output as output};
use crate::flags;
use std::path::Path;

pub fn run(words: &[String], options: &Options, home: &Path) -> error::Result<(i32, String)> {
    let all = flags::resolve_all(home).map_err(|message| {
        error::Failure::unavailable(
            "flags_registry_invalid",
            message,
            "fix or unset TERMINAL_JARVIS_FLAGS",
        )
    })?;
    match words {
        [] => Ok((0, output::list(&all))),
        [action] if action == "list" => Ok((0, output::list(&all))),
        [action, name, state] if action == "set" => {
            let on = flags::parse_switch(state).filter(|_| matches!(state.as_str(), "on" | "off"));
            let on =
                on.ok_or_else(|| usage_error("usage: terminal-jarvis settings set <flag> on|off"))?;
            change(&all, name, Some(on), options, home)
        }
        [action, name] if action == "reset" => change(&all, name, None, options, home),
        _ => Err(usage_error(
            "usage: terminal-jarvis settings [list|set <flag> on|off|reset <flag>]",
        )),
    }
}

fn change(
    all: &[flags::Resolved],
    name: &str,
    value: Option<bool>,
    options: &Options,
    home: &Path,
) -> error::Result<(i32, String)> {
    if !all.iter().any(|row| row.flag.name == name) {
        let valid = all
            .iter()
            .map(|row| row.flag.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let valid = if valid.is_empty() {
            "none declared yet".to_string()
        } else {
            valid
        };
        return Err(error::Failure::usage(
            "unknown_flag",
            format!("unknown flag '{name}'; valid flags: {valid}"),
            "run `terminal-jarvis settings` to list them",
        ));
    }
    let path = flags::settings_path(home);
    if !options.dry_run {
        flags::save(home, name, value).map_err(|cause| {
            error::Failure::state(
                "settings_unwritable",
                format!("could not write {}: {cause}", path.display()),
                "check your Terminal Jarvis home directory permissions and disk space, then retry",
            )
        })?;
    }
    let shown = path.display().to_string();
    Ok((0, output::receipt(name, value, &shown, options.dry_run)))
}

fn usage_error(message: &str) -> error::Failure {
    error::Failure::usage("usage", message, "run `terminal-jarvis help settings`")
}

#[cfg(test)]
#[path = "../tests/settings_cmd_test.rs"]
mod tests;
