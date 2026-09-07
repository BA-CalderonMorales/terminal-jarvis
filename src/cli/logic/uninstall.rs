//! Uninstall: the explicit removal verb. The uninstaller derives from
//! the harness's download plan (npm install -g p -> npm uninstall -g p),
//! the consent token is its own grammar (`uninstall:<name>`), and the
//! default is NO -- removal is never an accidental Enter.

use super::dispatch_support;
use super::{args::Options, error};
use crate::contracts::{Capability, CapabilityPlan, Harness};
use std::io::IsTerminal;

pub fn run(harnesses: &[Harness], name: &str, options: &Options) -> error::Result<(i32, String)> {
    use crate::cli::logic::guard_ask;
    use std::io::Write;
    let harness = dispatch_support::find(harnesses, name)?;
    let download = harness
        .plan(Capability::Download)
        .ok_or_else(|| uninstall_unavailable(name))?;
    let derived = derive_uninstall(download, harness).ok_or_else(|| uninstall_unavailable(name))?;
    let token = format!("uninstall:{name}");
    if options.confirm.as_deref() == Some(token.as_str()) {
        return finish_uninstall(name, &derived);
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
    finish_uninstall(name, &derived)
}

fn uninstall_unavailable(name: &str) -> error::Failure {
    error::Failure::state(
        "uninstall_unavailable",
        format!("no uninstaller derivable for {name}"),
        "remove it with its own package manager",
    )
}

/// npm install -g p -> npm uninstall -g p; cargo install -> cargo uninstall.
fn derive_uninstall(
    download: &CapabilityPlan,
    harness: &Harness,
) -> Option<crate::contracts::CommandPlan> {
    let command = download.command.command.as_str();
    let package = download.package.as_deref()?;
    if command == "npm" && download.command.args.iter().any(|arg| arg == "install") {
        return Some(crate::contracts::CommandPlan::new(
            "npm".into(),
            vec!["uninstall".into(), "-g".into(), package.into()],
        ));
    }
    if command == "cargo" && download.command.args.first().map(String::as_str) == Some("install") {
        return Some(crate::contracts::CommandPlan::new(
            "cargo".into(),
            vec!["uninstall".into(), harness.binary.clone()],
        ));
    }
    None
}

fn finish_uninstall(
    name: &str,
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
    };
    crate::runtime::run_command_text(&plan, &[]).map_err(|cause| {
        error::Failure::state(
            "uninstall_failed",
            format!("uninstalling {name} failed: {cause}"),
            "remove it with its own package manager",
        )
    })
}
