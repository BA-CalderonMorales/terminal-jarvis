use super::*;
use std::collections::BTreeSet;

#[path = "active_input.rs"]
mod fixture;
use fixture::input;

fn set(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| name.to_string()).collect()
}

fn run(
    active: Option<&str>,
    config: Option<&str>,
    ready: &[&str],
    missing: &[&str],
) -> (Record, bool) {
    let input = input(active, &["xh"]);
    collect(
        &input,
        config.map(str::to_string),
        &set(ready),
        &set(missing),
    )
}

#[test]
fn known_harness_is_ready() {
    let (record, usable) = run(Some("xh"), None, &["xh"], &[]);
    assert!(usable);
    assert_eq!(record.code, Code::Ready);
    assert_eq!(record.severity, Severity::Info);
}

#[test]
fn unknown_harness_is_malformed() {
    let (record, usable) = run(Some("nope"), None, &[], &[]);
    assert_eq!(record.code, Code::Malformed);
    assert_eq!(record.severity, Severity::Error);
    assert!(!usable);
}

#[test]
fn active_from_config_when_unset_in_input() {
    let (record, usable) = run(None, Some("xh"), &["xh"], &[]);
    assert_eq!(record.code, Code::Ready);
    assert!(usable);
}

#[test]
fn no_active_harness_reports_missing() {
    let (record, usable) = run(None, None, &[], &[]);
    assert_eq!(record.code, Code::Missing);
    assert_eq!(record.value, "none");
    assert!(
        usable,
        "no selection is a valid first-run state; the install state is a separate signal"
    );
}

#[test]
fn known_but_not_ready_harness_gets_action() {
    let (record, usable) = run(Some("xh"), None, &[], &["xh"]);
    assert_eq!(record.code, Code::Missing);
    assert_eq!(record.action.as_deref(), Some("install it, then run again"));
    assert!(
        usable,
        "selecting an uninstalled harness is still a usable state"
    );
}

#[path = "active_broken.rs"]
mod broken;
