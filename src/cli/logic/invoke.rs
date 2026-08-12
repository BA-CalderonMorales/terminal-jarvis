use super::resolve;
use crate::contracts::{Capability, CommandPlan, Harness};
use crate::runtime;

pub fn invocation(
    invocation: resolve::Invocation,
    harnesses: &[Harness],
    narrate: bool,
) -> Result<(i32, String), String> {
    capability(
        harnesses,
        &invocation.harness,
        invocation.capability,
        &invocation.extra,
        narrate,
    )
}

pub fn capability(
    harnesses: &[Harness],
    harness: &str,
    capability: Capability,
    extra: &[String],
    narrate: bool,
) -> Result<(i32, String), String> {
    let selected = find(harnesses, harness)?;
    let plan = selected
        .plan(capability)
        .ok_or_else(|| format!("{harness} lacks {capability}"))?;
    if matches!(capability, Capability::Download | Capability::Update) && narrate {
        eprintln!(
            "{} {harness}: {} ...",
            verb(capability),
            plan.command.render()
        );
    }
    match runtime::run_command(plan, extra) {
        Ok(0) => Ok((0, String::new())),
        Ok(code) => {
            eprintln!("{}", diagnostic(harness, capability, &plan.command, code));
            Ok((code, String::new()))
        }
        Err(error) => {
            let (code, message) = command_error(selected, plan.command.command.as_str(), error);
            eprintln!("{message}");
            Ok((code, String::new()))
        }
    }
}

fn verb(capability: Capability) -> &'static str {
    if capability == Capability::Download {
        "installing"
    } else {
        "updating"
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

fn command_error(harness: &Harness, binary: &str, error: std::io::Error) -> (i32, String) {
    let name = &harness.name;
    let (code, message) = match error.kind() {
        std::io::ErrorKind::NotFound => {
            let advice = if harness.plan(Capability::Download).is_some() {
                format!("; run `terminal-jarvis install {name}` or `terminal-jarvis plan {name} download`")
            } else {
                "; its download plan is undocumented; see `terminal-jarvis plan ".to_string()
                    + name
                    + "`"
            };
            (127, format!("{name} binary '{binary}' was not found on PATH{advice}"))
        }
        std::io::ErrorKind::PermissionDenied => {
            (126, format!("{name} binary '{binary}' is not executable; fix its permissions or reinstall {name}"))
        }
        _ => (3, format!("failed to start {} binary '{binary}': {error}", harness.name)),
    };
    (code, crate::diagnostics::redact_process_text(&message))
}

#[cfg(test)]
#[path = "../tests/invoke_test.rs"]
mod tests;
