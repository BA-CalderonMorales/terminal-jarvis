use crate::contracts::{Capability, CapabilityPlan, Effect, Interaction, SupportState};

fn plan(
    capability: Capability,
    effect: Effect,
    network: bool,
    interaction: Interaction,
    support: SupportState,
) -> CapabilityPlan {
    CapabilityPlan {
        capability,
        summary: String::new(),
        command: crate::contracts::CommandPlan::new("tj".into(), Vec::new()),
        support,
        evidence: crate::contracts::EvidenceMode::Deterministic,
        effect,
        network,
        interaction,
        platforms: Vec::new(),
        executable: "tj".into(),
        source: "test".into(),
        verified_at: "2026-07-17T04:59:27Z".into(),
        package: None,
    }
}

fn lifecycle_must_be_networked_noninteractive(
    capability: Capability,
    effect: Effect,
    network: bool,
    interaction: Interaction,
) -> bool {
    let candidate = plan(
        capability,
        effect,
        network,
        interaction,
        SupportState::Verified,
    );
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    let lifecycle = matches!(capability, Capability::Download | Capability::Update);
    let violated =
        effect != Effect::StateChanging || !network || interaction != Interaction::Noninteractive;
    let flagged = errors.iter().any(|e| e.contains("lifecycle"));
    flagged == (lifecycle && violated)
}

fn stub_is_local_read_only_guidance(
    capability: Capability,
    effect: Effect,
    network: bool,
    interaction: Interaction,
) -> bool {
    let candidate = plan(capability, effect, network, interaction, SupportState::Stub);
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    let violated =
        effect != Effect::ReadOnly || network || interaction != Interaction::Noninteractive;
    errors.iter().any(|e| e.contains("stub")) == violated
}

fn manual_must_be_interactive(capability: Capability, interaction: Interaction) -> bool {
    let candidate = plan(
        capability,
        Effect::ReadOnly,
        false,
        interaction,
        SupportState::Manual,
    );
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    errors.iter().any(|e| e.contains("manual support")) == (interaction != Interaction::Interactive)
}

#[test]
fn effect_truth_properties() {
    quickcheck::quickcheck(
        lifecycle_must_be_networked_noninteractive
            as fn(Capability, Effect, bool, Interaction) -> bool,
    );
    quickcheck::quickcheck(
        stub_is_local_read_only_guidance as fn(Capability, Effect, bool, Interaction) -> bool,
    );
    quickcheck::quickcheck(manual_must_be_interactive as fn(Capability, Interaction) -> bool);
}
