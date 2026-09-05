//! GuardIntent: the consent layer. Safety gates (dry-run, interactive
//! requirements, dangerous opt-in, confirm tokens) always apply; the final
//! question is a strategy -- stderr+stdin in a terminal, in-frame rows and
//! one raw key inside the tui's streaming surface.

use super::guard_ask::{ask_in_terminal, confirm_error, reject_irrelevant};
use super::{args::Options, error};
use crate::cli::logic::prompt_lead;
use crate::contracts::{CapabilityPlan, Effect, Harness, Interaction};
use std::io::IsTerminal;

pub fn check(
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
    options: &Options,
    explicit: bool,
) -> error::Result<()> {
    check_with(
        harness,
        plan,
        extra,
        options,
        explicit,
        &mut ask_in_terminal,
    )
}

/// The confirm step is a strategy: `ask(lead, token)` presents the plan
/// and resolves to Ok on consent.
pub fn check_with(
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
    options: &Options,
    explicit: bool,
    ask: &mut dyn FnMut(&str, &str) -> error::Result<()>,
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
    ask(
        &prompt_lead::confirm_lead(options, harness, plan, extra),
        &token,
    )
}

#[cfg(test)]
#[path = "../tests/guard_intent.rs"]
mod guard_intent_tests;
