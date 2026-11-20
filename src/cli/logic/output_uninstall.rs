//! The uninstall preview. Removal is the one destructive verb, so its plan has
//! to be readable before the consent token exists -- otherwise the only way to
//! learn what it would run is to authorize running it.

use super::super::value_block;
use super::super::{output_fields as fields, style};
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::{CommandPlan, Harness};

pub fn uninstall(harness: &Harness, command: &CommandPlan, token: &str) -> String {
    let effect = effect(command);
    if style::plain() {
        return format!(
            "uninstall:{}\ncommand: {}\neffect: {effect}\nflags: --no-input --confirm={token}\n",
            harness.name,
            command.render()
        );
    }
    let width = fields::width();
    let groups = [
        Group::new("what runs", vec![Row::new("command", command.render())]),
        Group::new("what it does", vec![Row::new("effect", effect)]),
        Group::new(
            "to run it",
            vec![Row::new("flags", format!("--no-input --confirm={token}"))],
        ),
    ];
    let mut lines = vec![format!("Plan: uninstall {}", harness.name)];
    lines.extend(value_block::card(&groups, width));
    lines.join("\n")
}

/// Only a local package-manager removal is known to work offline; a tool's
/// own uninstaller may reach the network, so nothing is claimed for it.
fn effect(command: &CommandPlan) -> &'static str {
    let local = matches!(
        command.command.as_str(),
        "npm" | "uv" | "cargo" | "pip" | "pip3" | "pipx"
    ) && command.args.iter().any(|arg| arg == "uninstall");
    if local {
        "changes files on this machine; works offline; runs unattended"
    } else {
        "changes files on this machine; runs unattended"
    }
}

#[cfg(test)]
#[path = "../tests/output_uninstall_test.rs"]
mod tests;
