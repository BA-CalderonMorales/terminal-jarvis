//! The per-harness readiness row. It is the one harness row the concise report
//! always keeps, so it also carries the next step for a harness that is simply
//! not installed yet.

use super::super::{Code, HarnessInput, Record, Severity};
use crate::contracts::SunsetState;

pub(super) fn record(
    base: &str,
    harness: &HarnessInput,
    ready: bool,
    support_ready: bool,
    missing: bool,
) -> Record {
    let code = match (ready, support_ready) {
        (true, _) => Code::Ready,
        (false, false) => Code::Unsupported,
        (false, true) if harness.retirement.is_some() => Code::Unsupported,
        (false, true) => Code::Missing,
    };
    // A passed date asks the reader to act; an announced one is a heads-up
    // about a tool that still runs.
    let (severity, value) = match (ready, harness.retirement) {
        (true, _) => (Severity::Info, "ready"),
        (false, Some(SunsetState::Done)) => (Severity::Warning, "retired"),
        (false, Some(SunsetState::Scheduled)) => (Severity::Info, "retiring"),
        (false, None) => (Severity::Info, "not-ready"),
    };
    let record = Record::new(format!("{base}.readiness"), code, severity, value);
    if missing && support_ready && harness.retirement.is_none() {
        record.action(format!("terminal-jarvis install {}", harness.name))
    } else {
        record
    }
}

#[cfg(test)]
#[path = "../tests/harness_readiness.rs"]
mod tests;
