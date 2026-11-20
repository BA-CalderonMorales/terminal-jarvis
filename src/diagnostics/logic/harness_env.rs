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
        // An unset credential is a setup step, not a defect: it is reported
        // once, in the summary row below, not as one warning per name. A name
        // that is set but empty or malformed is the case that actually needs
        // a look, so it keeps its own row and warns.
        if state != ValueState::Missing {
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
        // A missing credential is a setup step the user has not taken yet,
        // not a broken installation. `Any` mode means any one of the listed
        // names works, so it is a hint; `All` mode is the only case where a
        // missing name actually blocks the harness.
        if ready {
            Severity::Info
        } else if harness.env_mode == EnvMode::All {
            Severity::Warning
        } else {
            Severity::Info
        },
        if ready { "ready" } else { code.as_str() },
    );
    if !ready {
        let which = if harness.env_mode == EnvMode::All {
            "all"
        } else {
            "one"
        };
        summary.action = Some(format!("set {which} of: {}", harness.env.join(", ")));
    }
    records.push(summary);
    (records, ready)
}

fn ready(mode: EnvMode, states: &[ValueState]) -> bool {
    match mode {
        EnvMode::None => states.is_empty(),
        // Optional credentials never gate readiness: the harness works with
        // its own login flow (auth.json), and the env names are advisory.
        EnvMode::Optional => true,
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

#[cfg(test)]
#[path = "../tests/harness_env_states.rs"]
mod state_tests;
#[cfg(test)]
#[path = "../tests/harness_env.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/harness_env_state.rs"]
mod tests_state;
