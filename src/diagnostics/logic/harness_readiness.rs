//! The per-harness readiness row. It is the one harness row the concise report
//! always keeps, so it also carries the next step for a harness that is simply
//! not installed yet.

use super::super::{Code, HarnessInput, Record, Severity};

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
        (false, true) if harness.sunset => Code::Unsupported,
        (false, true) => Code::Missing,
    };
    let (severity, value) = if ready {
        (Severity::Info, "ready")
    } else if harness.sunset {
        (Severity::Warning, "retired")
    } else {
        (Severity::Info, "not-ready")
    };
    let record = Record::new(format!("{base}.readiness"), code, severity, value);
    if missing && support_ready && !harness.sunset {
        record.action(format!("terminal-jarvis install {}", harness.name))
    } else {
        record
    }
}

#[cfg(test)]
#[path = "../tests/harness_readiness.rs"]
mod tests;
