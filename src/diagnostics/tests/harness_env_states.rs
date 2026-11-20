//! Collect-level env evidence: per-name state records by mode and the
//! summary record's missing-credential action.

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
        retirement: None,
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
