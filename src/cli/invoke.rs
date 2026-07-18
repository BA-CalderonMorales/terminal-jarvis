use super::resolve;
use crate::contracts::{Capability, CommandPlan, Harness};
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
    match runtime::run_command(plan, extra) {
        Ok(0) => Ok((0, String::new())),
        Ok(code) => {
            eprintln!("{}", diagnostic(harness, capability, &plan.command, code));
            Ok((code, String::new()))
        }
        Err(error) => {
            let (code, message) = command_error(harness, plan.command.command.as_str(), error);
            eprintln!("{message}");
            Ok((code, String::new()))
        }
    }
}

fn diagnostic(harness: &str, capability: Capability, command: &CommandPlan, code: i32) -> String {
    crate::diagnostics::redact_process_text(&format!(
        "harness '{harness}' capability '{capability}' failed with exit {code}\n  command: {}",
        command.render()
    ))
}

fn find<'a>(harnesses: &'a [Harness], name: &str) -> Result<&'a Harness, String> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))
}

fn command_error(harness: &str, binary: &str, error: std::io::Error) -> (i32, String) {
    let (code, message) = match error.kind() {
        std::io::ErrorKind::NotFound => (127, format!("{harness} binary '{binary}' was not found on PATH; run `terminal-jarvis install {harness}` or `terminal-jarvis plan {harness} download`")),
        std::io::ErrorKind::PermissionDenied => {
            (126, format!("{harness} binary '{binary}' is not executable; fix its permissions or reinstall {harness}"))
        }
        _ => (3, format!("failed to start {harness} binary '{binary}': {error}")),
    };
    (code, crate::diagnostics::redact_process_text(&message))
}

#[cfg(test)]
#[path = "invoke_test.rs"]
mod tests;
