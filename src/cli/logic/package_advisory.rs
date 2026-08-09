//! PackageAdvisory: pre-install vulnerability guidance for download/update.

use super::{args::Options, error, error::Failure};
use crate::contracts::{Capability, CapabilityPlan, Harness};
use crate::{gates, security};
use std::path::Path;

#[path = "package_prompt.rs"]
mod prompt_impl;

pub fn check(
    harness: &Harness,
    plan: &CapabilityPlan,
    options: &Options,
    home: &Path,
) -> error::Result<()> {
    if !matches!(plan.capability, Capability::Download | Capability::Update) || options.dry_run {
        return Ok(());
    }
    let gate_on = gates::selected(home)
        .map_err(|cause| {
            Failure::state(
                "gate_state_unreadable",
                cause.to_string(),
                "run `terminal-jarvis gate status`",
            )
        })?
        .is_some();
    let Some(package) = plan.package.as_deref() else {
        return uncheckable(harness, plan, gate_on);
    };
    if !gate_on {
        return warn_ok(&format!(
            "{} {} without a vulnerability check; `terminal-jarvis gate enable trivy` scans installs",
            verb(plan.capability),
            harness.name
        ));
    }
    match security::package_check(package) {
        None => warn_ok(&format!(
            "cannot pre-check {package} (npm and trivy must be on PATH); continuing without a package scan"
        )),
        Some(verdict) if verdict.clean => Ok(()),
        Some(verdict) => {
            eprintln!(
                "HIGH/CRITICAL findings for {package} before {}:\n{}",
                verb(plan.capability),
                verdict.detail
            );
            prompt_impl::continue_prompt(harness, plan, package, options)
        }
    }
}

fn uncheckable(harness: &Harness, plan: &CapabilityPlan, gate_on: bool) -> error::Result<()> {
    let (why, tail) = if gate_on {
        (
            "uses a custom installer",
            "that cannot be pre-scanned; continuing",
        )
    } else {
        (
            "runs without a vulnerability check",
            "; `terminal-jarvis gate enable trivy` scans installs",
        )
    };
    warn_ok(&format!(
        "{}'s {} {why}{tail}",
        harness.name,
        verb(plan.capability)
    ))
}

fn verb(capability: Capability) -> &'static str {
    if capability == Capability::Download {
        "installing"
    } else {
        "updating"
    }
}

fn warn_ok(message: &str) -> error::Result<()> {
    eprintln!("warning: {message}");
    Ok(())
}

#[cfg(test)]
#[path = "../tests/package_advisory_test.rs"]
mod tests;
