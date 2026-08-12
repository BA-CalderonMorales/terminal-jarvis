//! ContinuePrompt: the interactive abort/continue decision for a package
//! that carries HIGH/CRITICAL findings; pulled by package_advisory.rs.

use crate::cli::logic::{args::Options, error, error::Failure};
use crate::contracts::{Capability, CapabilityPlan, Harness};
use std::io::{IsTerminal, Write};

pub fn continue_prompt(
    harness: &Harness,
    plan: &CapabilityPlan,
    package: &str,
    options: &Options,
) -> error::Result<()> {
    let token = format!("package-{}:{}", plan.capability, harness.name);
    if let Some(actual) = options.confirm.as_deref() {
        return (actual == token)
            .then_some(())
            .ok_or_else(|| confirm_error(&token));
    }
    if options.no_input || !std::io::stdin().is_terminal() {
        return Err(confirm_error(&token));
    }
    let verb = if plan.capability == Capability::Download {
        "installing"
    } else {
        "updating"
    };
    eprint!("Continue {verb} {package} anyway? [y/N] ");
    std::io::stderr().flush().map_err(prompt_error)?;
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .map_err(prompt_error)?;
    if matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
        Ok(())
    } else {
        Err(Failure::safety(
            "package_check_declined",
            format!("download of {package} aborted by the vulnerability check"),
            "review the findings, then retry with --confirm",
        ))
    }
}

fn prompt_error(cause: std::io::Error) -> Failure {
    Failure::state(
        "prompt_failed",
        cause.to_string(),
        "retry with --no-input and --confirm",
    )
}

fn confirm_error(token: &str) -> Failure {
    Failure::safety(
        "package_check_requires_confirm",
        format!("noninteractive install needs --no-input --confirm={token}"),
        format!("review the findings, then pass --no-input --confirm={token}"),
    )
}
