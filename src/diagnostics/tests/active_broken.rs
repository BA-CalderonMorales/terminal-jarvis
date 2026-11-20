use super::*;

/// A selected harness whose executable exists but cannot run (shadowed,
/// unreadable) is not "not installed yet": it must keep the report unhealthy.
#[test]
fn a_broken_selected_executable_stays_unhealthy() {
    let (record, usable) = run(Some("xh"), None, &[], &[]);
    assert!(!usable);
    assert_eq!(record.severity, Severity::Error);
    assert_eq!(
        record.action.as_deref(),
        Some("repair the selected harness")
    );
}

fn selected(state: crate::contracts::SunsetState, executable: Code) -> (Record, bool) {
    let mut local = input(Some("xh"), &["xh"]);
    local.harnesses[0].retirement = Some(state);
    let executables = std::collections::BTreeMap::from([("xh".to_string(), executable)]);
    collect(&local, None, &BTreeSet::new(), &executables)
}

/// A retiring tool still runs, so selecting it is healthy with a heads-up;
/// a retired one asks the user to pick a maintained harness.
#[test]
fn a_retiring_selection_stays_healthy_and_a_retired_one_does_not() {
    let (retiring, healthy) = selected(crate::contracts::SunsetState::Scheduled, Code::Ready);
    assert!(healthy);
    assert_eq!(retiring.severity, Severity::Info);
    let (retired, healthy) = selected(crate::contracts::SunsetState::Done, Code::Ready);
    assert!(!healthy);
    assert_eq!(retired.severity, Severity::Warning);
}

/// Retirement and executable health are separate questions: a retiring tool
/// whose binary is broken still fails the check.
#[test]
fn a_broken_retiring_selection_still_fails() {
    let state = crate::contracts::SunsetState::Scheduled;
    let (record, healthy) = selected(state, Code::PermissionDenied);
    assert!(!healthy);
    assert_eq!(
        record.action.as_deref(),
        Some("repair the selected harness")
    );
}
