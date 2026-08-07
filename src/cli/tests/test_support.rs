use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};

pub fn plan(capability: Capability, command: &str, args: Vec<String>) -> CapabilityPlan {
    CapabilityPlan {
        capability,
        summary: capability.as_str().to_string(),
        command: CommandPlan::new(command.to_string(), args),
        support: SupportState::Unknown,
        evidence: EvidenceMode::Deterministic,
        effect: match capability {
            Capability::Yolo => Effect::Dangerous,
            Capability::Download | Capability::Update | Capability::Ui => Effect::StateChanging,
            _ => Effect::ReadOnly,
        },
        network: matches!(
            capability,
            Capability::Download | Capability::Update | Capability::Ui | Capability::Yolo
        ),
        interaction: if capability == Capability::Ui {
            Interaction::Interactive
        } else {
            Interaction::Noninteractive
        },
        platforms: Vec::new(),
        executable: command.to_string(),
        source: "internal:test-fixture".to_string(),
        verified_at: "2026-07-17T04:59:27Z".to_string(),
    }
}
