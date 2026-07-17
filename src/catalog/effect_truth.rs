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
