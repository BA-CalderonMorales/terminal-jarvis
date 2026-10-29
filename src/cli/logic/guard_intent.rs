//! GuardIntent: the consent layer. Safety gates (dry-run, interactive
//! requirements, dangerous opt-in, confirm tokens) always apply; the final
//! question is a strategy -- stderr+stdin in a terminal, in-frame rows and
//! one raw key inside the tui's streaming surface.

use super::guard_ask::{confirm_error, default_yes, reject_irrelevant};
use super::{args::Options, error};
use crate::cli::logic::prompt_lead;
use crate::contracts::{CapabilityPlan, Effect, Harness, Interaction};
use std::io::IsTerminal;

/// The confirm step is a strategy: `ask(lead, token, default_yes)` presents
/// the plan and resolves to Ok on consent.
pub fn check_with(
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
    options: &Options,
    explicit: bool,
    ask: &mut dyn FnMut(&str, &str, bool) -> error::Result<()>,
) -> error::Result<()> {
    if plan.effect == Effect::ReadOnly {
        return reject_irrelevant(options);
    }
    if options.dry_run {
        return Ok(());
    }
    let terminal = std::io::stdin().is_terminal();
    if plan.interaction == Interaction::Interactive && !terminal {
        return Err(error::Failure::safety(
            "interactive_terminal_required",
            "interactive capability requires a terminal and cannot run in noninteractive automation",
            format!(
                "review `terminal-jarvis plan {} {}` and run it from a terminal",
                harness.name, plan.capability
            ),
        ));
    }
    if plan.effect == Effect::Dangerous && (!explicit || !options.allow_dangerous) {
        return Err(error::Failure::safety(
            "dangerous_opt_in_required",
            "dangerous execution requires an explicit harness/capability and --allow-dangerous",
            format!(
                "review `terminal-jarvis plan {} {}`",
                harness.name, plan.capability
            ),
        ));
    }
    // An interactive launch hands the terminal to the harness, which asks its
    // own questions: a consent keystroke in front of it is friction with
    // nothing to review. State-changing headless runs still confirm, and a
    // dangerous capability was already stopped above.
    if plan.interaction == Interaction::Interactive {
        return Ok(());
    }
    let token = format!("{}:{}", plan.capability, harness.name);
    if let Some(actual) = options.confirm.as_deref() {
        if actual == token && (terminal || options.no_input) {
            return Ok(());
        }
        return Err(confirm_error(&token));
    }
    if options.no_input || !terminal {
        return Err(confirm_error(&token));
    }
    let default_yes = default_yes(&token);
    ask(
        &prompt_lead::confirm_lead(options, harness, plan, extra),
        &token,
        default_yes,
    )
}

#[cfg(test)]
#[path = "../tests/guard_intent.rs"]
mod guard_intent_tests;
