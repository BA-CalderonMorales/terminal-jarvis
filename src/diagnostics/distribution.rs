use super::{Code, DiagnosticInput, Record, Severity, ValueState};

pub fn collect(input: &DiagnosticInput) -> (Record, bool) {
    let state = input.environment.state("TERMINAL_JARVIS_DISTRIBUTION");
    if state != ValueState::Present {
        return (
            Record::new(
                "tj.distribution",
                state.code(),
                Severity::Warning,
                state.as_str(),
            ),
            true,
        );
    }
    let value = input
        .environment
        .text("TERMINAL_JARVIS_DISTRIBUTION")
        .unwrap_or_default();
    let channel = crate::distribution::normalize(value);
    let conflict = input.environment.state("TERMINAL_JARVIS_WRAPPER") == ValueState::Present
        && channel.is_some_and(|value| value != "npm");
    match (channel, conflict) {
        (_, true) => (
            Record::new(
                "tj.distribution",
                Code::Conflicting,
                Severity::Error,
                "conflicting",
            )
            .action("remove conflicting wrapper or distribution metadata"),
            false,
        ),
        (Some(channel), false) => (
            Record::new("tj.distribution", Code::Ready, Severity::Info, channel),
            true,
        ),
        (None, false) => (
            Record::new(
                "tj.distribution",
                Code::Unsupported,
                Severity::Error,
                "unrecognized",
            )
            .action("install through a supported distribution route"),
            false,
        ),
    }
}

pub fn wrapper(input: &DiagnosticInput) -> Record {
    let state = input.environment.state("TERMINAL_JARVIS_WRAPPER");
    Record::new("tj.wrapper", state.code(), Severity::Info, state.as_str())
}
