//! PackageAdvisory: pre-install vulnerability guidance for download/update.

use super::{args::Options, error, error::Failure};
use crate::contracts::{Capability, CapabilityPlan, Harness};
use crate::{gates, security};
use std::path::Path;

#[path = "package_prompt.rs"]
mod prompt_impl;
#[path = "package_report.rs"]
mod report;

/// `quiet` routes every announcement through `row` instead of stderr, and
/// turns finding-level prompts into a fail-closed cancel (the streaming
/// surface cannot leak a prompt under the frame).
pub fn check_quiet(
    harness: &Harness,
    plan: &CapabilityPlan,
    options: &Options,
    home: &Path,
    quiet: bool,
    row: &mut dyn FnMut(&str),
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
        return report::warn_ok(&format!(
            "{} {} without a vulnerability check; `terminal-jarvis gate enable trivy` scans installs",
            report::verb(plan.capability),
            harness.name
        ));
    }
    if quiet {
        row(&format!("checking {package} for known vulnerabilities ..."));
    } else if options.narrate {
        eprintln!("checking {package} for known vulnerabilities ...");
    } else {
        eprint!("package check ...");
    }
    match security::package_check(package) {
        None => {
            report::quiet_done(options, "skipped");
            report::warn_ok(&format!(
                "cannot pre-check {package} (npm and trivy must be on PATH); continuing without a package scan"
            ))
        }
        Some(verdict) if verdict.clean => {
            if quiet {
                row(&format!(
                    "package check: clean -- no HIGH/CRITICAL findings for {package}"
                ));
            } else if options.narrate {
                eprintln!("no HIGH/CRITICAL findings for {package}");
            } else {
                report::quiet_done(options, "clean");
            }
            Ok(())
        }
        Some(verdict) => {
            let v = report::verb(plan.capability);
            if quiet {
                return Err(error::Failure::safety(
                    "package_findings",
                    format!(
                        "HIGH/CRITICAL findings for {package} before {v}: {}",
                        verdict.detail
                    ),
                    "review the findings, then run headless with --confirm to proceed",
                ));
            }
            eprintln!(
                "HIGH/CRITICAL findings for {package} before {v}:\n{}",
                verdict.detail
            );
            prompt_impl::continue_prompt(harness, plan, package, options)
        }
    }
}

/// Rewrites the live "package check ..." line with the outcome.
fn uncheckable(harness: &Harness, plan: &CapabilityPlan, gate_on: bool) -> error::Result<()> {
    let tail = if gate_on {
        "uses a custom installer that cannot be pre-scanned; continuing"
    } else {
        "runs without a vulnerability check; `terminal-jarvis gate enable trivy` scans installs"
    };
    report::warn_ok(&format!(
        "{}'s {} {tail}",
        harness.name,
        report::verb(plan.capability)
    ))
}

#[cfg(test)]
#[path = "../tests/package_advisory_test.rs"]
mod tests;
