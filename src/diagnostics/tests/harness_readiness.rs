use super::*;
use crate::contracts::EnvMode;

fn harness(sunset: bool) -> HarnessInput {
    HarnessInput {
        name: "xh".into(),
        binary: "xh".into(),
        env_mode: EnvMode::None,
        env: Vec::new(),
        support: Vec::new(),
        version: None,
        sunset,
    }
}

/// The concise report keeps readiness rows and drops info-level executable
/// rows, so the install hint has to live here to reach a fresh machine.
#[test]
fn an_uninstalled_maintained_harness_carries_the_install_hint() {
    let row = record("harness.xh", &harness(false), false, true, true);
    assert_eq!((row.code, row.severity), (Code::Missing, Severity::Info));
    assert_eq!(row.action.as_deref(), Some("terminal-jarvis install xh"));
}

#[test]
fn no_install_hint_when_installing_would_not_help() {
    let broken = record("harness.xh", &harness(false), false, true, false);
    let unsupported = record("harness.xh", &harness(false), false, false, true);
    let retired = record("harness.xh", &harness(true), false, true, true);
    assert!(broken.action.is_none() && unsupported.action.is_none());
    assert_eq!(
        (retired.severity, retired.action),
        (Severity::Warning, None)
    );
}
