use super::*;
use crate::contracts::EnvMode;

use crate::diagnostics::{Environment, RuntimeInput};
use std::path::Path;

pub(crate) fn harness(env_mode: EnvMode, names: &[&str]) -> HarnessInput {
    HarnessInput {
        name: "xh".into(),
        binary: "xh".into(),
        env_mode,
        env: names.iter().map(|name| name.to_string()).collect(),
        support: Vec::new(),
        version: None,
        sunset: false,
    }
}
pub(crate) fn input(environment: Environment) -> DiagnosticInput {
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

pub(crate) fn env_record<'a>(records: &'a [Record], key: &str) -> &'a Record {
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
fn collect_reports_absent_credentials_once_not_per_name() {
    let (records, ready) = collect(
        &harness(EnvMode::Any, &["A", "B"]),
        &input(Environment::default()),
        "harness.xh",
    );
    assert!(!ready);
    let per_name = records
        .iter()
        .filter(|record| record.key.starts_with("harness.xh.env."))
        .count();
    assert_eq!(
        per_name, 0,
        "unset names are summarized once; flooding the report hides the real signal"
    );
    let summary = env_record(&records, "harness.xh.environment");
    assert_eq!(summary.action.as_deref(), Some("set one of: A, B"));
}
