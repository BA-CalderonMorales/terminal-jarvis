use super::*;
use crate::contracts::EnvMode;
use crate::diagnostics::{Environment, RuntimeInput};
use std::path::Path;

fn harness(env_mode: EnvMode, names: &[&str]) -> HarnessInput {
    HarnessInput {
        name: "xh".into(),
        binary: "xh".into(),
        env_mode,
        env: names.iter().map(|name| name.to_string()).collect(),
        support: Vec::new(),
        version: None,
    }
}
fn input(environment: Environment) -> DiagnosticInput {
    let mut local = DiagnosticInput::local(
        Path::new("/tmp/tj-catalog"),
        Path::new("/tmp/tj-home"),
        None,
        &[],
        RuntimeInput {
            gate: "/tmp/tj-gate".into(),
            stdout_tty: true,
            stderr_tty: true,
            color: false,
            width: 80,
            update_route: "source".into(),
            checksum: "".into(),
            probes: false,
        },
    );
    local.environment = environment;
    local
}

#[test]
fn ready_requires_presence_by_mode() {
    assert!(ready(EnvMode::None, &[]));
    assert!(!ready(EnvMode::None, &[ValueState::Missing]));
    assert!(ready(EnvMode::Optional, &[ValueState::Missing]) && ready(EnvMode::Optional, &[]));
    assert!(!ready(EnvMode::Any, &[ValueState::Missing]));
    assert!(ready(EnvMode::Any, &[ValueState::Present]));
    assert!(!ready(EnvMode::All, &[]));
    assert!(ready(EnvMode::All, &[ValueState::Present; 2]));
    assert!(!ready(
        EnvMode::All,
        &[ValueState::Present, ValueState::Missing]
    ));
}

#[test]
fn aggregate_maps_state_sets() {
    assert_eq!(aggregate(&[]), Code::Malformed);
    assert_eq!(aggregate(&[ValueState::Malformed]), Code::Malformed);
    assert_eq!(aggregate(&[ValueState::Empty]), Code::Empty);
    assert_eq!(aggregate(&[ValueState::Missing]), Code::Missing);
}
#[test]
fn collect_marks_present_states_info_and_ready() {
    let mut environment = Environment::default();
    environment.insert("TOKEN", "token-value");
    let (records, ready) = collect(
        &harness(EnvMode::All, &["TOKEN"]),
        &input(environment),
        "harness.xh",
    );
    let present = records
        .iter()
        .find(|record| record.key == "harness.xh.env.TOKEN")
        .unwrap();
    assert_eq!(present.severity, Severity::Info);
    assert!(ready);
}

#[test]
fn collect_marks_missing_states_with_action() {
    let (records, ready) = collect(
        &harness(EnvMode::All, &["TOKEN"]),
        &input(Environment::default()),
        "harness.xh",
    );
    assert!(!ready);
    let missing = records
        .iter()
        .find(|record| record.key == "harness.xh.env.TOKEN")
        .unwrap();
    assert_eq!(missing.severity, Severity::Warning);
    let summary = records
        .iter()
        .find(|record| record.key == "harness.xh.environment")
        .unwrap();
    assert_eq!(summary.code, Code::Missing);
    assert_eq!(
        summary.action.as_deref(),
        Some("set the required credential environment names")
    );
}
