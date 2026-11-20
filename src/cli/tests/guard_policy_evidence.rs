use super::check;
use super::guard_policy_support::*;
use crate::cli::logic::args::Options;
use crate::contracts::SupportState;

#[test]
fn accepts_verified_support_with_fresh_evidence_and_matching_platform() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Verified,
        &crate::catalog::today_utc(),
        vec![platform_str().into()],
    );
    let result = check(&harness, &plan, &Options::default(), false);
    assert!(result.is_ok());
}

#[test]
fn accepts_expected_support_with_fresh_evidence_and_matching_platform() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Expected,
        &crate::catalog::today_utc(),
        vec![platform_str().into()],
    );
    let result = check(&harness, &plan, &Options::default(), false);
    assert!(result.is_ok());
}

/// Stale evidence is a maintainer signal, not a user-facing lock. The catalog
/// ships with the binary, so "refresh the upstream evidence" is an instruction
/// the user cannot carry out; blocking on it locked installs in 0.1.23 once
/// the catalog rows aged past their refresh window.
#[test]
fn accepts_stale_evidence_and_leaves_the_warning_to_the_caller() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Verified,
        "2024-01-01T00:00:00Z",
        vec![platform_str().into()],
    );
    assert_eq!(
        crate::catalog::freshness_status(&plan),
        "stale",
        "staleness is still reported"
    );
    let result = check(&harness, &plan, &Options::default(), false);
    assert!(
        result.is_ok(),
        "stale evidence must not block execution: {result:?}"
    );
}
