use crate::contracts::{CapabilityPlan, Harness, SupportState};
use crate::{catalog, platform, security};

pub fn is_harness_ready(harness: &Harness) -> bool {
    security::command_on_path(&harness.binary)
        && security::missing_env(harness).is_empty()
        && harness.capabilities.iter().any(executable_support)
}

fn executable_support(plan: &CapabilityPlan) -> bool {
    matches!(
        plan.support,
        SupportState::Verified | SupportState::Expected
    ) && catalog::freshness_status(plan) == "fresh"
        && platform::id().is_some_and(|id| plan.platforms.iter().any(|candidate| candidate == id))
}

#[cfg(test)]
#[path = "output_readiness_test.rs"]
mod tests;
