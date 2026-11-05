//! Uninstall: the explicit removal verb. The removal plan comes from the
//! harness's download entry (its own uninstaller, or the inverse of npm/uv/
//! cargo), the consent token is its own grammar (`uninstall:<name>`), and the
//! default is NO -- removal is never an accidental Enter.

use super::dispatch_support;
use super::uninstall_plan;
use super::{args::Options, error};
use crate::contracts::{Capability, Harness};
use std::io::IsTerminal;

pub fn run(harnesses: &[Harness], name: &str, options: &Options) -> error::Result<(i32, String)> {
    use crate::cli::logic::guard_ask;
    use std::io::Write;
    let harness = dispatch_support::find(harnesses, name)?;
    let download = harness
        .plan(Capability::Download)
        .ok_or_else(|| uninstall_plan::unavailable(name))?;
    let derived = uninstall_plan::derive(download, harness)
        .ok_or_else(|| uninstall_plan::unavailable(name))?;
    let token = format!("uninstall:{name}");
    if options.confirm.as_deref() == Some(token.as_str()) {
        return finish_uninstall(name, harness, &derived);
    }
    let terminal = std::io::stdin().is_terminal();
    if options.no_input || !terminal {
        return Err(error::Failure::safety(
            "confirmation_required",
            format!("noninteractive uninstall requires --no-input --confirm={token}"),
            format!("review the plan, then pass --no-input --confirm={token}"),
        ));
    }
    let plan = format!("{} {}", derived.command, derived.args.join(" "));
    eprint!("Uninstall plan: {plan}\nContinue with uninstall:{name}? [y/N] ");
    std::io::stderr().flush().ok();
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .map_err(|cause| error::Failure::state("prompt_failed", cause.to_string(), "retry"))?;
    guard_ask::consent(answer.trim())?;
    finish_uninstall(name, harness, &derived)
}

fn finish_uninstall(
    name: &str,
    harness: &Harness,
    derived: &crate::contracts::CommandPlan,
) -> error::Result<(i32, String)> {
    use crate::contracts::{CapabilityPlan, EvidenceMode, Interaction, SupportState};
    let plan = CapabilityPlan {
        capability: Capability::Download,
        summary: format!("uninstall {name}"),
        command: derived.clone(),
        support: SupportState::Verified,
        evidence: EvidenceMode::Deterministic,
        effect: crate::contracts::Effect::StateChanging,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms: vec![],
        executable: derived.command.clone(),
        source: "internal:uninstall".into(),
        verified_at: String::new(),
        package: None,
        uninstall: None,
    };
    let (code, text) = crate::runtime::run_command_text(&plan, &[]).map_err(|cause| {
        error::Failure::state(
            "uninstall_failed",
            format!("uninstalling {name} failed: {cause}"),
            "remove it with its own package manager",
        )
    })?;
    // A package manager can report success while another channel still owns
    // the binary -- a script install, a brew copy, a shim. Never call that a
    // clean uninstall: name the survivor and the channel that can remove it.
    if code == 0 && crate::security::command_on_path(&harness.binary) {
        return Err(error::Failure::state(
            "uninstall_incomplete",
            format!(
                "{name} left the package manager but `{}` is still on PATH",
                harness.binary
            ),
            format!("it was installed by another channel; remove it with that channel (`show {name}` names the install source)"),
        ));
    }
    Ok((code, text))
}

#[cfg(test)]
#[path = "../tests/uninstall_test.rs"]
mod tests;
