use super::{
    args::Options, dispatch_support, error, guard_intent, guard_policy, invoke, output, resolve,
};
use crate::contracts::{Capability, Harness};
use crate::gates;
use std::path::Path;

pub fn run(
    words: &[String],
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
) -> error::Result<(i32, String)> {
    let explicit = explicit_capability(words, harnesses);
    let invocation = resolve::run(words, harnesses, home).map_err(resolve_error)?;
    execute(invocation, options, harnesses, home, explicit)
}

pub fn direct(
    name: &str,
    extra: &[String],
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
) -> error::Result<(i32, String)> {
    let invocation = resolve::direct(name, extra, harnesses).map_err(resolve_error)?;
    execute(invocation, options, harnesses, home, false)
}

pub fn capability(
    harnesses: &[Harness],
    name: &str,
    capability: Capability,
    options: &Options,
    home: &Path,
) -> error::Result<(i32, String)> {
    let invocation = resolve::Invocation {
        harness: name.to_string(),
        capability,
        extra: Vec::new(),
    };
    execute(invocation, options, harnesses, home, true)
}

fn execute(
    invocation: resolve::Invocation,
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
    explicit: bool,
) -> error::Result<(i32, String)> {
    let harness = dispatch_support::find(harnesses, &invocation.harness)?;
    let plan = harness.plan(invocation.capability).ok_or_else(|| {
        error::Failure::state(
            "catalog_incomplete",
            format!("{} lacks {}", harness.name, invocation.capability),
            "repair the harness catalog",
        )
    })?;
    guard_policy::check(harness, plan)?;
    guard_intent::check(harness, plan, &invocation.extra, options, explicit)?;
    if options.dry_run {
        return Ok((
            0,
            output::plan_with_extra(harness, invocation.capability, &invocation.extra),
        ));
    }
    gates::preflight(home).map_err(|message| {
        error::Failure::safety("gate_blocked", message, "run `terminal-jarvis gate status`")
    })?;
    invoke::invocation(invocation, harnesses).map_err(dispatch_support::unavailable_error)
}

fn explicit_capability(words: &[String], harnesses: &[Harness]) -> bool {
    words.len() >= 2
        && harnesses.iter().any(|harness| harness.name == words[0])
        && Capability::parse(&words[1]).is_some()
}

fn resolve_error(message: String) -> error::Failure {
    if message.contains("no active harness") || message.contains("active harness") {
        return error::Failure::state(
            "active_harness_invalid",
            message,
            "run `terminal-jarvis use <harness>` or pass a harness",
        );
    }
    error::Failure::unavailable("harness_unknown", message, "run `terminal-jarvis list`")
}
