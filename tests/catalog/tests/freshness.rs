use std::path::Path;
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::SupportState;

#[test]
fn catalog_rows_report_freshness_matching_their_support() {
    let harnesses = catalog::load(Path::new("harnesses")).unwrap();
    for plan in harnesses.iter().flat_map(|harness| &harness.capabilities) {
        let supported = matches!(
            plan.support,
            SupportState::Verified | SupportState::Expected | SupportState::Manual
        );
        let status = catalog::freshness_status(plan);
        if supported {
            assert_eq!(
                status, "fresh",
                "{}:{:?} revision must be fresh",
                plan.capability, plan.support
            );
        } else {
            assert_eq!(status, "policy-reviewed");
        }
    }
}

#[test]
fn executable_support_claims_expose_stale_evidence() {
    let harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let mut plan = harnesses[0].capabilities[0].clone();
    plan.support = SupportState::Expected;
    plan.verified_at = "2000-01-01T00:00:00Z".to_string();
    assert_eq!(catalog::freshness_status(&plan), "stale");
}

#[test]
fn stale_executable_support_claims_fail_catalog_validation() {
    let mut harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let plan = &mut harnesses[0].capabilities[0];
    plan.support = SupportState::Expected;
    plan.platforms = vec![terminal_jarvis::context::platform::id().unwrap().into()];
    plan.verified_at = "2000-01-01T00:00:00Z".into();
    let errors = catalog::validate(&harnesses).join("\n");
    assert!(errors.contains("evidence must be fresh"), "{errors}");
}
