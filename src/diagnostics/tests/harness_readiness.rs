use super::*;
use crate::contracts::{EnvMode, SunsetState};

fn harness(retirement: Option<SunsetState>) -> HarnessInput {
    HarnessInput {
        name: "xh".into(),
        binary: "xh".into(),
        env_mode: EnvMode::None,
        env: Vec::new(),
        support: Vec::new(),
        version: None,
        retirement,
    }
}

/// The concise report keeps readiness rows and drops info-level executable
/// rows, so the install hint has to live here to reach a fresh machine.
#[test]
fn an_uninstalled_maintained_harness_carries_the_install_hint() {
    let row = record("harness.xh", &harness(None), false, true, true);
    assert_eq!((row.code, row.severity), (Code::Missing, Severity::Info));
    assert_eq!(row.action.as_deref(), Some("terminal-jarvis install xh"));
}

#[test]
fn no_install_hint_when_installing_would_not_help() {
    let broken = record("harness.xh", &harness(None), false, true, false);
    let unsupported = record("harness.xh", &harness(None), false, false, true);
    let retired = record(
        "harness.xh",
        &harness(Some(SunsetState::Done)),
        false,
        true,
        true,
    );
    assert!(broken.action.is_none() && unsupported.action.is_none());
    assert_eq!(
        (retired.severity, retired.action),
        (Severity::Warning, None)
    );
}

/// An announced date is a heads-up, not a fault: the tool still runs.
#[test]
fn a_retiring_harness_informs_and_a_retired_one_warns() {
    let retiring = record(
        "harness.xh",
        &harness(Some(SunsetState::Scheduled)),
        false,
        true,
        true,
    );
    let retired = record(
        "harness.xh",
        &harness(Some(SunsetState::Done)),
        false,
        true,
        true,
    );
    assert_eq!(
        (retiring.severity, retiring.value.as_str()),
        (Severity::Info, "retiring")
    );
    assert_eq!(
        (retired.severity, retired.value.as_str()),
        (Severity::Warning, "retired")
    );
}
