use super::redact::segment;
use super::{Code, DiagnosticInput, Record, Severity};
use crate::contracts::SunsetState;
use std::collections::{BTreeMap, BTreeSet};

pub fn collect(
    input: &DiagnosticInput,
    config: Option<String>,
    ready: &BTreeSet<String>,
    executables: &BTreeMap<String, Code>,
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
    let retirement = retirement(input, &active);
    let executable = executables.get(&active).copied();
    let missing = executable == Some(Code::Missing);
    let runs_or_absent = missing || matches!(executable, Some(Code::Ready | Code::Conflicting));
    // Selecting a harness you have not installed yet is a legitimate first
    // step: `use codex` then `install`. A retiring tool still runs, so it stays
    // healthy with a heads-up -- unless its executable is broken. A retired
    // selection or a broken executable never passes.
    let healthy = usable
        || (retirement == Some(SunsetState::Scheduled) && runs_or_absent)
        || (retirement.is_none() && missing);
    let severity = match (healthy, retirement) {
        (true, _) => Severity::Info,
        (false, Some(SunsetState::Done)) => Severity::Warning,
        (false, _) => Severity::Error,
    };
    let code = if usable { Code::Ready } else { Code::Missing };
    let record = Record::new("harness.active", code, severity, segment(&active));
    let action = match retirement {
        _ if usable => return (record, true),
        Some(SunsetState::Done) => "this harness is retired; select a maintained one",
        Some(SunsetState::Scheduled) if runs_or_absent => {
            "it is retiring; `terminal-jarvis graveyard` names its successor"
        }
        None if missing => "install it, then run again",
        _ => "repair the selected harness",
    };
    (record.action(action), healthy)
}

fn retirement(input: &DiagnosticInput, name: &str) -> Option<SunsetState> {
    let harness = input
        .harnesses
        .iter()
        .find(|harness| harness.name == name)?;
    harness.retirement
}

#[cfg(test)]
#[path = "../tests/active.rs"]
mod tests;
