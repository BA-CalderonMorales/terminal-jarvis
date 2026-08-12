use super::check;
use super::guard_policy_support::*;
use crate::contracts::SupportState;

#[test]
fn rejects_manual_support() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Manual,
        "2026-07-17T04:59:27Z",
        vec![platform_str().into()],
    );
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "manual_procedure_required");
    assert_eq!(err.exit_code, 4);
}

#[test]
fn rejects_stub_support() {
    let harness = dummy_harness();
    let plan = dummy_plan(SupportState::Stub, "2026-07-17T04:59:27Z", vec![]);
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "capability_stub");
}

#[test]
fn rejects_unsupported_support() {
    let harness = dummy_harness();
    let plan = dummy_plan(SupportState::Unsupported, "2026-07-17T04:59:27Z", vec![]);
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "capability_unsupported");
}

#[test]
fn rejects_disabled_support() {
    let harness = dummy_harness();
    let plan = dummy_plan(SupportState::Disabled, "2026-07-17T04:59:27Z", vec![]);
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "capability_disabled");
}

#[test]
fn rejects_unknown_support_headless_but_consents_interactively() {
    let harness = dummy_harness();
    let plan = dummy_plan(SupportState::Unknown, "2026-08-06T00:00:00Z", vec![]);
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "capability_unknown");
    let interactive = check(&harness, &plan, true);
    assert!(
        interactive.is_ok(),
        "interactive consent failed: {interactive:?}"
    );
}
