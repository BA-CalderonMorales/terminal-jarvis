use super::*;
use crate::contracts::EnvMode;
use crate::diagnostics::{Environment, HarnessInput, PlatformInput, RuntimeInput};
use std::collections::BTreeSet;
use std::time::{Duration, SystemTime};

fn input(active: Option<&str>, names: &[&str]) -> DiagnosticInput {
    DiagnosticInput {
        version: String::new(),
        executable: None,
        catalog: Default::default(),
        home: Default::default(),
        config: Default::default(),
        home_prefix: None,
        temp_prefix: None,
        active_harness: active.map(str::to_string),
        harnesses: names
            .iter()
            .map(|name| HarnessInput {
                name: (*name).into(),
                binary: String::new(),
                env_mode: EnvMode::None,
                env: Vec::new(),
                support: Vec::new(),
                version: None,
            })
            .collect(),
        platform: PlatformInput {
            os: String::new(),
            arch: String::new(),
            libc: String::new(),
            wsl: String::new(),
        },
        environment: Environment::default(),
        runtime: RuntimeInput {
            gate: Default::default(),
            stdout_tty: false,
            stderr_tty: false,
            color: false,
            width: 0,
            update_route: String::new(),
            checksum: String::new(),
            probes: false,
        },
        now: SystemTime::UNIX_EPOCH,
        stale_after: Duration::ZERO,
    }
}

#[test]
fn known_harness_is_ready() {
    let ready = BTreeSet::from(["xh".to_string()]);
    let (record, usable) = collect(&input(Some("xh"), &["xh"]), None, &ready);
    assert_eq!(usable, true);
    assert_eq!(record.code, Code::Ready);
    assert_eq!(record.severity, Severity::Info);
}

#[test]
fn unknown_harness_is_malformed() {
    let (record, usable) = collect(&input(Some("nope"), &["xh"]), None, &BTreeSet::new());
    assert_eq!(record.code, Code::Malformed);
    assert_eq!(record.severity, Severity::Error);
    assert_eq!(usable, false);
}

#[test]
fn active_from_config_when_unset_in_input() {
    let ready = BTreeSet::from(["xh".to_string()]);
    let (record, usable) = collect(&input(None, &["xh"]), Some("xh".into()), &ready);
    assert_eq!(record.code, Code::Ready);
    assert_eq!(usable, true);
}

#[test]
fn no_active_harness_reports_missing() {
    let (record, usable) = collect(&input(None, &["xh"]), None, &BTreeSet::new());
    assert_eq!(record.code, Code::Missing);
    assert_eq!(record.value, "none");
    assert_eq!(usable, false);
}

#[test]
fn known_but_not_ready_harness_gets_action() {
    let (record, usable) = collect(&input(Some("xh"), &["xh"]), None, &BTreeSet::new());
    assert_eq!(record.code, Code::Missing);
    assert_eq!(
        record.action.as_deref(),
        Some("repair active harness readiness")
    );
    assert_eq!(usable, false);
}
