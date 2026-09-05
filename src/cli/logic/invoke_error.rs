//! InvokeError: the failure mapping for one-shot harness invocations --
//! diagnostics redaction and the exit-code -> advice table.

use crate::contracts::{Capability, CommandPlan, Harness};

pub(super) fn verb(capability: Capability) -> &'static str {
    if capability == Capability::Download {
        "installing"
    } else {
        "updating"
    }
}

pub(super) fn diagnostic(
    harness: &str,
    capability: Capability,
    command: &CommandPlan,
    code: i32,
) -> String {
    crate::diagnostics::redact_process_text(&format!(
        "harness '{harness}' capability '{capability}' failed with exit {code}\n  command: {}",
        command.render()
    ))
}

pub(super) fn command_error(
    harness: &Harness,
    binary: &str,
    error: std::io::Error,
) -> (i32, String) {
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
