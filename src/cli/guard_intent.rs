use super::{args::Options, error, output};
use crate::contracts::{CapabilityPlan, Effect, Harness, Interaction};
use std::io::{IsTerminal, Write};

pub fn check(
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
    options: &Options,
    explicit: bool,
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
    eprint!(
        "{}Continue with {token}? [y/N] ",
        output::plan_with_extra(harness, plan.capability, extra)
    );
    std::io::stderr().flush().map_err(|cause| {
        error::Failure::state(
            "prompt_failed",
            cause.to_string(),
            "retry with --no-input and --confirm",
        )
    })?;
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).map_err(|cause| {
        error::Failure::state(
            "prompt_failed",
            cause.to_string(),
            "retry with --no-input and --confirm",
        )
    })?;
    if matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
        Ok(())
    } else {
        Err(error::Failure::safety(
            "confirmation_declined",
            "operation was not confirmed",
            "review the plan and retry when ready",
        ))
    }
}

fn reject_irrelevant(options: &Options) -> error::Result<()> {
    if options.dry_run || options.no_input || options.confirm.is_some() || options.allow_dangerous {
        return Err(error::Failure::usage(
            "option_not_applicable",
            "lifecycle options are not valid for a read-only capability",
            "remove the lifecycle option",
        ));
    }
    Ok(())
}

fn confirm_error(token: &str) -> error::Failure {
    error::Failure::safety(
        "explicit_intent_required",
        format!("noninteractive execution requires --no-input --confirm={token}"),
        format!("review the plan, then pass --no-input --confirm={token}"),
    )
}
