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

fn selected(state: crate::contracts::SunsetState) -> (Record, bool) {
    let mut local = input(Some("xh"), &["xh"]);
    local.harnesses[0].retirement = Some(state);
    collect(&local, None, &BTreeSet::new(), &BTreeSet::new())
}

/// A retiring tool still runs, so selecting it is healthy with a heads-up;
/// a retired one asks the user to pick a maintained harness.
#[test]
fn a_retiring_selection_stays_healthy_and_a_retired_one_does_not() {
    let (retiring, healthy) = selected(crate::contracts::SunsetState::Scheduled);
    assert!(healthy);
    assert_eq!(retiring.severity, Severity::Info);
    let (retired, healthy) = selected(crate::contracts::SunsetState::Done);
    assert!(!healthy);
    assert_eq!(retired.severity, Severity::Warning);
}
