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

fn env_record<'a>(records: &'a [Record], key: &str) -> &'a Record {
    records.iter().find(|record| record.key == key).unwrap()
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
fn collect_marks_env_state_by_mode() {
    let mut with_env = Environment::default();
    with_env.insert("TOKEN", "token-value");
    for (mode, environment, ready_expected, severity) in [
        (EnvMode::All, with_env, true, Severity::Info),
        (
            EnvMode::Optional,
            Environment::default(),
            true,
            Severity::Info,
        ),
        (
            EnvMode::All,
            Environment::default(),
            false,
            Severity::Warning,
        ),
    ] {
        let (records, ready) = collect(
            &harness(mode, &["TOKEN"]),
            &input(environment),
            "harness.xh",
        );
        assert_eq!(ready, ready_expected, "mode={mode:?}");
        assert_eq!(
            env_record(&records, "harness.xh.env.TOKEN").severity,
            severity,
            "mode={mode:?}"
        );
    }
    let (records, _) = collect(
        &harness(EnvMode::All, &["TOKEN"]),
        &input(Environment::default()),
        "harness.xh",
    );
    let summary = env_record(&records, "harness.xh.environment");
    assert_eq!(summary.code, Code::Missing);
    assert_eq!(
        summary.action.as_deref(),
        Some("set the required credential environment names")
    );
}
