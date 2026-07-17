use super::redact::segment;
use super::{Code, DiagnosticInput, HarnessInput, Record, Severity, ValueState};
use crate::contracts::EnvMode;

pub fn collect(harness: &HarnessInput, input: &DiagnosticInput, base: &str) -> (Vec<Record>, bool) {
    let mut names = harness.env.iter().collect::<Vec<_>>();
    names.sort();
    let mut records = Vec::new();
    let mut states = Vec::new();
    for name in names {
        let state = input.environment.state(name);
        states.push(state);
        records.push(Record::new(
            format!("{base}.env.{}", segment(name)),
            state.code(),
            if state == ValueState::Present {
                Severity::Info
            } else {
                Severity::Warning
            },
            state.as_str(),
        ));
    }
    let ready = ready(harness.env_mode, &states);
    let code = if ready {
        Code::Ready
    } else {
        aggregate(&states)
    };
    let mut summary = Record::new(
        format!("{base}.environment"),
        code,
        if ready {
            Severity::Info
        } else {
            Severity::Error
        },
        if ready { "ready" } else { code.as_str() },
    );
    if !ready {
        summary.action = Some("set the required credential environment names".into());
    }
    records.push(summary);
    (records, ready)
}

fn ready(mode: EnvMode, states: &[ValueState]) -> bool {
    match mode {
        EnvMode::None => states.is_empty(),
        EnvMode::Any => !states.is_empty() && states.contains(&ValueState::Present),
        EnvMode::All => {
            !states.is_empty() && states.iter().all(|state| *state == ValueState::Present)
        }
    }
}

fn aggregate(states: &[ValueState]) -> Code {
    if states.is_empty() || states.contains(&ValueState::Malformed) {
        Code::Malformed
    } else if states.contains(&ValueState::Empty) {
        Code::Empty
    } else {
        Code::Missing
    }
}
