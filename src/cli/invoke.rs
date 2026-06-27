use super::resolve;
use crate::contracts::{Capability, Harness};
use crate::runtime;

pub fn invocation(
    invocation: resolve::Invocation,
    harnesses: &[Harness],
) -> Result<(i32, String), String> {
    capability(
        harnesses,
        &invocation.harness,
        invocation.capability,
        &invocation.extra,
    )
}

pub fn capability(
    harnesses: &[Harness],
    harness: &str,
    capability: Capability,
    extra: &[String],
) -> Result<(i32, String), String> {
    let plan = find(harnesses, harness)?
        .plan(capability)
        .ok_or_else(|| format!("{harness} lacks {capability}"))?;
    runtime::run_command(plan, extra)
        .map(|code| (code, String::new()))
        .map_err(|error| command_error(harness, plan.command.command.as_str(), error))
}

fn find<'a>(harnesses: &'a [Harness], name: &str) -> Result<&'a Harness, String> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))
}

fn command_error(harness: &str, binary: &str, error: std::io::Error) -> String {
    if error.kind() == std::io::ErrorKind::NotFound {
        return format!(
            "{harness} binary '{binary}' was not found on PATH; run `terminal-jarvis install {harness}` or `terminal-jarvis plan {harness} download`"
        );
    }
    error.to_string()
}
