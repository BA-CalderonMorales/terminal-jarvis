use super::check;
use super::guard_policy_support::*;
use crate::contracts::SupportState;

#[test]
fn empty_platforms_are_unrestricted() {
    let harness = dummy_harness();
    let plan = dummy_plan(
        SupportState::Expected,
        &crate::catalog::today_utc(),
        Vec::new(),
    );
    assert!(check(&harness, &plan, false).is_ok());
}

#[test]
fn rejects_platform_incompatible() {
    let harness = dummy_harness();
    // Use a different platform than the current one
    let other_platform = if platform_str() == "linux-x64-gnu" {
        "macos-x64"
    } else {
        "linux-x64-gnu"
    };
    let plan = dummy_plan(
        SupportState::Verified,
        &crate::catalog::today_utc(),
        vec![other_platform.into()],
    );
    let result = check(&harness, &plan, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "platform_incompatible");
}
