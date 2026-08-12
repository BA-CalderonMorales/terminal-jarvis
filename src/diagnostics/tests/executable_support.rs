use super::super::harness_input::executable_support;
use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};

fn plan(support: SupportState, platforms: Vec<String>) -> CapabilityPlan {
    CapabilityPlan {
        capability: Capability::Version,
        summary: "probe".into(),
        command: CommandPlan::new("probe".into(), vec![]),
        support,
        evidence: EvidenceMode::Deterministic,
        effect: Effect::ReadOnly,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms,
        executable: "probe".into(),
        source: "probe".into(),
        verified_at: "2026-08-05T00:00:00Z".into(),
        package: None,
    }
}

fn current() -> String {
    crate::context::platform::id()
        .expect("platform is known")
        .to_string()
}

#[test]
fn empty_platform_claims_are_unrestricted() {
    assert!(executable_support(&plan(SupportState::Expected, vec![])));
    assert!(executable_support(&plan(SupportState::Verified, vec![])));
}

#[test]
fn explicit_platform_claims_are_enforced() {
    assert!(executable_support(&plan(
        SupportState::Expected,
        vec![current()]
    )));
    assert!(!executable_support(&plan(
        SupportState::Expected,
        vec!["other".into()]
    )));
}

#[test]
fn guarded_states_are_never_executable() {
    for state in [
        SupportState::Manual,
        SupportState::Stub,
        SupportState::Unsupported,
        SupportState::Disabled,
        SupportState::Unknown,
    ] {
        assert!(!executable_support(&plan(state, vec![])));
    }
}

#[test]
fn stale_evidence_is_never_executable() {
    let mut stale = plan(SupportState::Verified, vec![current()]);
    stale.verified_at = "2000-01-01T00:00:00Z".into();
    assert!(!executable_support(&stale));
}
