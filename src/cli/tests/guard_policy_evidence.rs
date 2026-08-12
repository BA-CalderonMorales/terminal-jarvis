use super::check;
use super::guard_policy_support::*;
use crate::contracts::SupportState;

#[test]
fn accepts_verified_support_with_fresh_evidence_and_matching_platform() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Verified,
        "2026-07-17T04:59:27Z",
        vec![platform_str().into()],
    );
    let result = check(&harness, &plan, false);
    assert!(result.is_ok());
}

#[test]
fn accepts_expected_support_with_fresh_evidence_and_matching_platform() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Expected,
        "2026-07-17T04:59:27Z",
        vec![platform_str().into()],
    );
    let result = check(&harness, &plan, false);
    assert!(result.is_ok());
}

#[test]
fn rejects_stale_evidence() {
    let harness = dummy_harness();
    // Use a very old date that will be stale
    let plan = dummy_plan(
        SupportState::Verified,
        "2024-01-01T00:00:00Z",
        vec![platform_str().into()],
    );
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "evidence_stale");
}
