use super::HarnessInput;
use crate::contracts::Harness;

impl From<&Harness> for HarnessInput {
    fn from(harness: &Harness) -> Self {
        let support = harness
            .capabilities
            .iter()
            .filter(|plan| !plan.executable.trim().is_empty())
            .map(|plan| {
                (
                    plan.capability.as_str().to_string(),
                    plan.support.as_str().to_string(),
                    executable_support(plan),
                )
            })
            .collect();
        let version = harness
            .plan(crate::contracts::Capability::Version)
            .filter(|plan| {
                matches!(
                    plan.support,
                    crate::contracts::SupportState::Verified
                        | crate::contracts::SupportState::Expected
                )
            })
            .map(|plan| (plan.command.command.clone(), plan.command.args.clone()));
        Self {
            name: harness.name.clone(),
            binary: harness.binary.clone(),
            env_mode: harness.env_mode,
            env: harness.env.clone(),
            support,
            version,
        }
    }
}

pub(super) fn executable_support(plan: &crate::contracts::CapabilityPlan) -> bool {
    matches!(
        plan.support,
        crate::contracts::SupportState::Verified | crate::contracts::SupportState::Expected
    ) && crate::catalog::freshness_status(plan) == "fresh"
        && crate::context::platform::id().is_none_or(|platform| {
            plan.platforms.is_empty()
                || plan.platforms.iter().any(|candidate| candidate == platform)
        })
}

#[cfg(test)]
#[path = "../tests/executable_support.rs"]
mod tests;
