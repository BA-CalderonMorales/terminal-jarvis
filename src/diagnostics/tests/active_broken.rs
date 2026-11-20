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
