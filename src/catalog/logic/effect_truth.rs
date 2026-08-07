use crate::contracts::{Capability, CapabilityPlan, Effect, Interaction, SupportState};

pub fn validate(prefix: &str, plan: &CapabilityPlan, errors: &mut Vec<String>) {
    match plan.capability {
        Capability::Download | Capability::Update
            if plan.effect != Effect::StateChanging
                || !plan.network
                || plan.interaction != Interaction::Noninteractive =>
        {
            errors.push(format!(
                "{prefix} lifecycle must be networked noninteractive state-changing"
            ));
        }
        Capability::Ui
            if plan.effect != Effect::StateChanging
                || !plan.network
                || plan.interaction != Interaction::Interactive =>
        {
            errors.push(format!(
                "{prefix} ui must be networked interactive state-changing"
            ));
        }
        Capability::Yolo if plan.effect != Effect::Dangerous || !plan.network => {
            errors.push(format!("{prefix} yolo must be networked dangerous"));
        }
        _ => {}
    }
    if plan.support == SupportState::Stub
        && (plan.effect != Effect::ReadOnly
            || plan.network
            || plan.interaction != Interaction::Noninteractive)
    {
        errors.push(format!("{prefix} stub must be local read-only guidance"));
    }
    if plan.support == SupportState::Manual && plan.interaction != Interaction::Interactive {
        errors.push(format!("{prefix} manual support must be interactive"));
    }
}

#[cfg(test)]
#[path = "../tests/effect_truth_props.rs"]
mod props;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{Capability as C, Effect as E, Interaction as I};
    use crate::contracts::{CommandPlan, EvidenceMode};

    fn plan(capability: C, effect: E, network: bool, interaction: I) -> CapabilityPlan {
        CapabilityPlan {
            capability,
            summary: String::new(),
            command: CommandPlan::new("tj".into(), Vec::new()),
            support: SupportState::Verified,
            evidence: EvidenceMode::Deterministic,
            effect,
            network,
            interaction,
            platforms: Vec::new(),
            executable: "tj".into(),
            source: "test".into(),
            verified_at: "2026-07-17T04:59:27Z".into(),
        }
    }

    fn flags(candidate: &CapabilityPlan) -> Vec<String> {
        let mut errors = Vec::new();
        validate("h", candidate, &mut errors);
        errors
    }

    #[test]
    fn arm_guards_fire() {
        let cases = [
            (C::Ui, E::StateChanging, false, I::Interactive),
            (C::Ui, E::StateChanging, true, I::Noninteractive),
            (C::Ui, E::ReadOnly, true, I::Interactive),
            (C::Download, E::StateChanging, false, I::Noninteractive),
            (C::Yolo, E::ReadOnly, true, I::Noninteractive),
            (C::Yolo, E::Dangerous, false, I::Noninteractive),
        ];
        for (capability, effect, network, interaction) in cases {
            let errors = flags(&plan(capability, effect, network, interaction));
            assert!(errors.iter().any(|e| e.contains("must be")));
        }
        let clean = flags(&plan(C::Ui, E::StateChanging, true, I::Interactive));
        assert!(!clean.iter().any(|e| e.contains("must be")));
    }
}
