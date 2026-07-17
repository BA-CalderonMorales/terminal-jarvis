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
        let usable = !ready.is_empty();
        return (
            Record::new("harness.active", Code::Missing, Severity::Info, "none"),
            usable,
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
        if usable {
            Severity::Info
        } else {
            Severity::Error
        },
        segment(&active),
    );
    if usable {
        (record, true)
    } else {
        (record.action("repair active harness readiness"), false)
    }
}
