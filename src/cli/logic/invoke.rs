use super::resolve;
use crate::contracts::{Capability, Harness};
use crate::runtime;

fn find<'a>(harnesses: &'a [Harness], name: &str) -> Result<&'a Harness, String> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))
}

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
    if matches!(capability, Capability::Download | Capability::Update) {
        if narrate {
            eprintln!(
                "{} {harness}: {} ...",
                errors::verb(capability),
                plan.command.render()
            );
        } else {
            eprintln!("{} {harness} ...", errors::verb(capability));
        }
    }
    match runtime::run_command_text(plan, extra) {
        Ok((0, captured)) => Ok((0, errors::installed_note(selected, capability, captured))),
        Ok((code, captured)) => {
            eprintln!(
                "{}",
                errors::diagnostic(harness, capability, &plan.command, code)
            );
            Ok((code, captured))
        }
        Err(error) => {
            let (code, message) =
                errors::command_error(selected, plan.command.command.as_str(), error);
            eprintln!("{message}");
            Ok((code, String::new()))
        }
    }
}

/// One-shot headless run for the converse loop: policy-checked, then
/// executed; a nonzero exit becomes the failure string the session shows.
pub fn headless_one_shot(
    harnesses: &[Harness],
    name: &str,
    prompt: &str,
) -> Result<String, String> {
    let selected = find(harnesses, name)?;
    let plan = selected
        .plan(Capability::Headless)
        .ok_or_else(|| format!("{name} lacks headless"))?;
    if let Err(failure) = super::guard_policy::check(selected, plan, true) {
        return Err(format!("{}: {}", failure.code, failure.message));
    }
    let (code, text) = capability(
        harnesses,
        name,
        Capability::Headless,
        &[prompt.to_string()],
        false,
    )?;
    if code == 0 {
        Ok(text)
    } else {
        Err(format!("exit {code}"))
    }
}

#[path = "invoke_error.rs"]
mod errors;

#[cfg(test)]
#[path = "../tests/invoke_test.rs"]
mod tests;
