use super::redact::segment;
use super::{Code, DiagnosticInput, Record, Severity};
use std::collections::BTreeSet;

pub fn collect(
    input: &DiagnosticInput,
    config: Option<String>,
    ready: &BTreeSet<String>,
) -> (Record, bool) {
    let active = input.active_harness.as_ref().cloned().or(config);
    let Some(active) = active else {
        // No selection is a valid first-run state, not a fault. Whether the
        // fleet is installed is reported separately, so it must not make
        // Terminal Jarvis itself look broken.
        return (
            Record::new("harness.active", Code::Missing, Severity::Info, "none"),
            true,
        );
    };
    if !input.harnesses.iter().any(|harness| harness.name == active) {
        return (
            Record::new(
                "harness.active",
                Code::Malformed,
                Severity::Error,
                "unknown",
            )
            .action("select a harness present in the catalog"),
            false,
        );
    }
    let usable = ready.contains(&active);
    let record = Record::new(
        "harness.active",
        if usable { Code::Ready } else { Code::Missing },
        // Selecting a harness you have not installed yet is a legitimate
        // first step: `use codex` then `install`. It is only worth flagging
        // when the harness is known to be retired.
        if usable || !retired(input, &active) {
            Severity::Info
        } else {
            Severity::Warning
        },
        segment(&active),
    );
    if usable {
        (record, true)
    } else if retired(input, &active) {
        (
            record.action("this harness is retired; select a maintained one"),
            false,
        )
    } else {
        (record.action("install it, then run again"), true)
    }
}

fn retired(input: &DiagnosticInput, name: &str) -> bool {
    input
        .harnesses
        .iter()
        .any(|harness| harness.name == name && harness.sunset)
}

#[cfg(test)]
#[path = "../tests/active.rs"]
mod tests;
