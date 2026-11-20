use super::args::Options;
use super::error;
use crate::contracts::{CapabilityPlan, Harness, SupportState};

#[path = "guard_platform.rs"]
mod platform;
use platform::guarded;
#[path = "guard_words.rs"]
mod words;
use words::{script_host, verb};

/// May this command run right now? A capability the catalog never claimed and
/// a platform that cannot run it block: those mean "this will not work here".
/// An unknown claim (a curl-pipe or custom installer) is "are you sure?": a
/// terminal consents through the intent prompt, a headless run consents with
/// `--confirm`, and a preview (`--dry-run`) needs no consent because nothing
/// executes -- `plan` already shows the same command.
///
/// Evidence age never blocks. The catalog ships inside the binary, so the only
/// fix for stale evidence is a maintainer refresh; `plan` and `show` report it.
pub fn check(
    harness: &Harness,
    plan: &CapabilityPlan,
    options: &Options,
    terminal: bool,
) -> error::Result<()> {
    let consented = terminal || options.dry_run || options.confirm.is_some();
    match plan.support {
        SupportState::Verified | SupportState::Expected => {}
        SupportState::Manual => return guarded(harness, plan, "manual_procedure_required"),
        SupportState::Stub => return guarded(harness, plan, "capability_stub"),
        SupportState::Unsupported => return guarded(harness, plan, "capability_unsupported"),
        SupportState::Disabled => return guarded(harness, plan, "capability_disabled"),
        SupportState::Unknown if consented => {}
        SupportState::Unknown => return Err(unconsented(harness, plan)),
    }
    platform::check(harness, plan)
}

/// A headless run of a script nobody can pre-scan: name the host it comes
/// from and the two ways forward -- review it in a terminal, or confirm it.
/// The token is the one `guard_intent` checks next, so a wrong token still
/// fails there with the exact expected value.
fn unconsented(harness: &Harness, plan: &CapabilityPlan) -> error::Failure {
    error::Failure::safety(
        "capability_unknown",
        format!(
            "{}:{} runs a script from {} that Terminal Jarvis cannot verify",
            harness.name,
            plan.capability,
            script_host(plan)
        ),
        format!(
            "run `terminal-jarvis {} {}` from a terminal to review it, or pass \
             --no-input --confirm={}:{} once you have accepted it",
            verb(plan),
            harness.name,
            plan.capability,
            harness.name
        ),
    )
}

#[cfg(test)]
#[path = "../tests/guard_policy_support.rs"]
pub mod guard_policy_support;

#[cfg(test)]
#[path = "../tests/guard_policy.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/guard_policy_evidence.rs"]
mod tests_evidence;

#[cfg(test)]
#[path = "../tests/guard_policy_platform.rs"]
mod tests_platform;

#[cfg(test)]
#[path = "../tests/guard_policy_consent.rs"]
mod tests_consent;
