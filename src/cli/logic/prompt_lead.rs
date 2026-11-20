//! Quiet (tui clean) mode swaps the full plan table for one line that says
//! what the action is, by which mechanism, and where the bytes come from;
//! headless narration keeps the complete plan for review.
use super::args::Options;
use crate::cli::logic::{output, output_plan};
use crate::contracts::{Capability, CapabilityPlan, Harness};

pub fn confirm_lead(
    options: &Options,
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
) -> String {
    confirm_lead_on(options, harness, plan, extra, &crate::catalog::today_utc())
}

/// `today` is a parameter so the retirement verb is pinned by tests.
pub fn confirm_lead_on(
    options: &Options,
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
    today: &str,
) -> String {
    if options.narrate {
        return output::plan_with_extra_on(harness, plan.capability, extra, today);
    }
    // The one line carries the retirement before the user answers.
    let retirement = output_plan::retirement(harness, today)
        .map(|line| format!(" · {line}"))
        .unwrap_or_default();
    format!(
        "{} {} · {}{retirement}\n",
        verb(plan.capability),
        harness.name,
        source(plan)
    )
}

/// The verb the user is actually reaching for.
fn verb(capability: Capability) -> &'static str {
    match capability {
        Capability::Download => "install",
        Capability::Update => "update",
        _ => "run",
    }
}

/// Where the bytes come from, in one shape for every mechanism. The raw argv
/// says neither the mechanism nor the source, and `sh -c 'curl … | sh'` reads
/// as a dare rather than a plan.
fn source(plan: &CapabilityPlan) -> String {
    let args = plan.command.args.join(" ");
    match plan.command.command.as_str() {
        "npm" => match plan.package.as_deref() {
            Some(package) => format!("npm · {package}"),
            None => format!("npm · {}", last_arg(&args)),
        },
        "uv" => format!("uv tool · {}", last_arg(&args)),
        "pip" | "pipx" => format!("pip · {}", last_arg(&args)),
        "sh" | "bash" => format!("remote script · {}", host(&args).unwrap_or("unknown host")),
        other => {
            let args = args.trim();
            if args.is_empty() {
                other.to_string()
            } else {
                format!("{other} · {args}")
            }
        }
    }
}

/// The last whitespace-separated argument: the package a package manager was
/// handed, whatever flags precede it.
fn last_arg(args: &str) -> &str {
    args.split_whitespace().last().unwrap_or("")
}

/// The host a remote installer is fetched from -- the trust-relevant fact.
fn host(args: &str) -> Option<&str> {
    let rest = args.split("https://").nth(1)?;
    Some(rest.split('/').next().unwrap_or(rest))
}
