use crate::contracts::{CapabilityPlan, Harness, SupportState};

pub fn support_summary(harness: &Harness) -> String {
    let states = [
        SupportState::Verified,
        SupportState::Expected,
        SupportState::Manual,
        SupportState::Stub,
        SupportState::Unsupported,
        SupportState::Disabled,
        SupportState::Unknown,
    ];
    states
        .iter()
        .map(|state| {
            let count = harness
                .capabilities
                .iter()
                .filter(|plan| plan.support == *state)
                .count();
            format!("{}={count}", state.as_str())
        })
        .collect::<Vec<_>>()
        .join(",")
}

pub fn capability_row(plan: &CapabilityPlan) -> Vec<String> {
    vec![
        plan.capability.to_string(),
        plan.support.as_str().to_string(),
        plan.evidence.as_str().to_string(),
        effect(plan),
        platforms(plan),
        freshness(plan),
    ]
}

pub fn plain_capability(plan: &CapabilityPlan) -> String {
    format!(
        "capability={} support={} evidence={} effect={} platforms={} executable={} source={} verified_at={} summary={}\n",
        plan.capability,
        plan.support.as_str(),
        plan.evidence.as_str(),
        effect(plan),
        platforms(plan),
        plan.executable,
        plan.source,
        plan.verified_at,
        plan.summary
    )
}

pub fn effect(plan: &CapabilityPlan) -> String {
    format!(
        "{} network={} interaction={}",
        plan.effect.as_str(),
        plan.network,
        plan.interaction.as_str()
    )
}

pub fn platforms(plan: &CapabilityPlan) -> String {
    if plan.platforms.is_empty() {
        "none".to_string()
    } else {
        plan.platforms.join(",")
    }
}

pub fn freshness(plan: &CapabilityPlan) -> String {
    format!(
        "{} ({})",
        plan.verified_at,
        crate::catalog::freshness_status(plan)
    )
}
