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
