use super::redact::segment;
use super::{Code, HarnessInput, Record, Severity};

pub fn collect(harness: &HarnessInput, base: &str) -> (Record, bool) {
    if harness.support.is_empty() {
        return (unavailable(base, "support-unavailable"), false);
    }
    let value = harness
        .support
        .iter()
        .map(|(capability, state, _)| format!("{}:{}", segment(capability), segment(state)))
        .collect::<Vec<_>>()
        .join(",");
    let ready = harness.support.iter().any(|(_, _, executable)| *executable);
    if ready {
        (
            Record::new(
                format!("{base}.support"),
                Code::Ready,
                Severity::Info,
                value,
            ),
            true,
        )
    } else {
        (unavailable(base, value), false)
    }
}

fn unavailable(base: &str, value: impl Into<String>) -> Record {
    Record::new(
        format!("{base}.support"),
        Code::Unsupported,
        Severity::Error,
        value,
    )
    .action("verify at least one executable capability")
}
