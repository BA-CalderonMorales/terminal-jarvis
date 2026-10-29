use std::path::Path;
use terminal_jarvis::{catalog, contracts::SupportState};

#[test]
fn stale_evidence_remains_inspectable_but_is_reported_stale() {
    let mut harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let plan = &mut harnesses[0].capabilities[0];
    plan.support = SupportState::Expected;
    plan.verified_at = "2000-01-01T00:00:00Z".into();
    assert_eq!(catalog::freshness_status(plan), "stale");
    assert!(catalog::validate(&harnesses).is_empty());
}

#[test]
fn freshness_expiration_and_future_dates_use_an_injected_reference_date() {
    let harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let mut plan = harnesses[0].capabilities[0].clone();
    plan.verified_at = "2026-01-01T00:00:00Z".into();
    for (support, last, expired) in [
        (
            SupportState::Verified,
            "2026-01-31T00:00:00Z",
            "2026-02-01T00:00:00Z",
        ),
        (
            SupportState::Expected,
            "2026-04-01T00:00:00Z",
            "2026-04-02T00:00:00Z",
        ),
    ] {
        plan.support = support;
        assert_eq!(catalog::freshness_at(&plan, last), "fresh");
        assert_eq!(catalog::freshness_at(&plan, expired), "stale");
        assert_eq!(
            catalog::freshness_at(&plan, "2025-12-30T00:00:00Z"),
            "stale"
        );
    }
    plan.verified_at = "2025-02-29T00:00:00Z".into();
    assert_eq!(
        catalog::freshness_at(&plan, "2026-01-01T00:00:00Z"),
        "invalid"
    );
}
