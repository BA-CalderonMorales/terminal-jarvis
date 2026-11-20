//! GuardWords: the phrases a refusal speaks. The host a remote installer
//! fetches from is the only fact a user can weigh before accepting a script
//! they have not read, and the verb names the command they would type -- not
//! the internal capability id.

use crate::contracts::{Capability, CapabilityPlan};

/// The host a remote installer fetches from.
pub(super) fn script_host(plan: &CapabilityPlan) -> String {
    let command = plan.command.render();
    command
        .split("https://")
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .map(str::to_string)
        .unwrap_or_else(|| "its vendor".to_string())
}

/// The command a user would type to reach this plan interactively.
pub(super) fn verb(plan: &CapabilityPlan) -> &'static str {
    match plan.capability {
        Capability::Download => "install",
        Capability::Update => "update",
        _ => "run",
    }
}
