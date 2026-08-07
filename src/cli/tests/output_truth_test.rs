use super::*;
use crate::cli::logic::test_support;
use crate::contracts::{Capability, CapabilityPlan, EnvMode, Harness, SupportState};

fn harness(plans: Vec<CapabilityPlan>) -> Harness {
    Harness {
        name: "opencode".into(),
        display: "opencode".into(),
        description: String::new(),
        binary: "sh".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: plans,
    }
}

#[test]
fn support_summary_counts_only_present_states() {
    let base = test_support::plan(Capability::Download, "sh", vec![]);
    let verified = CapabilityPlan {
        support: SupportState::Verified,
        ..base.clone()
    };
    let unsupported = CapabilityPlan {
        support: SupportState::Unsupported,
        ..base
    };
    assert_eq!(
        support_summary(&harness(vec![verified, unsupported])),
        "verified=1,unsupported=1"
    );
    assert_eq!(support_summary(&harness(vec![])), "unclaimed");
}

#[test]
fn capability_row_pins_all_six_cells() {
    let mut plan = test_support::plan(Capability::Yolo, "sh", vec!["-x".into()]);
    plan.platforms = vec!["linux".into(), "macos".into()];
    assert_eq!(
        capability_row(&plan),
        vec![
            "yolo".to_string(),
            "unknown".to_string(),
            "deterministic".to_string(),
            "dangerous network=true interaction=noninteractive".to_string(),
            "linux,macos".to_string(),
            "2026-07-17T04:59:27Z (policy-reviewed)".to_string(),
        ]
    );
}

#[test]
fn effect_platforms_freshness_render_exactly() {
    let plan = test_support::plan(Capability::Security, "sh", vec![]);
    assert_eq!(
        effect(&plan),
        "read-only network=false interaction=noninteractive"
    );
    assert_eq!(platforms(&plan), "none");
    assert_eq!(freshness(&plan), "2026-07-17T04:59:27Z (policy-reviewed)");
    let mut narrowed = plan.clone();
    narrowed.platforms = vec!["linux".into()];
    assert_eq!(platforms(&narrowed), "linux");
}
