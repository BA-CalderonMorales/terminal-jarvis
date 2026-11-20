//! The uninstall preview. Removal is the one destructive verb, so its plan has
//! to be readable before the consent token exists -- otherwise the only way to
//! learn what it would run is to authorize running it.

use super::super::value_block;
use super::super::{output_fields as fields, style};
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::Harness;

pub fn uninstall(
    harness: &Harness,
    command: &crate::contracts::CommandPlan,
    token: &str,
) -> String {
    if style::plain() {
        return format!(
            "uninstall:{}\ncommand: {}\neffect: changes files on this machine; works offline; runs unattended\nflags: --no-input --confirm={token}\n",
            harness.name,
            command.render()
        );
    }
    let width = fields::width();
    let groups = [
        Group::new("what runs", vec![Row::new("command", command.render())]),
        Group::new(
            "what it does",
            vec![Row::new(
                "effect",
                "changes files on this machine; works offline; runs unattended",
            )],
        ),
        Group::new(
            "to run it",
            vec![Row::new("flags", format!("--no-input --confirm={token}"))],
        ),
    ];
    let mut lines = vec![format!("Plan: uninstall {}", harness.name)];
    lines.extend(value_block::card(&groups, width));
    lines.join("\n")
}

#[cfg(test)]
#[path = "../tests/output_uninstall_test.rs"]
mod tests;
