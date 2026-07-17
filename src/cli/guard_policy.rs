use super::error;
use crate::contracts::{CapabilityPlan, Harness, SupportState};

pub fn check(harness: &Harness, plan: &CapabilityPlan) -> error::Result<()> {
    match plan.support {
        SupportState::Verified | SupportState::Expected => {}
        SupportState::Manual => return guarded(harness, plan, "manual_procedure_required"),
        SupportState::Stub => return guarded(harness, plan, "capability_stub"),
        SupportState::Unsupported => return guarded(harness, plan, "capability_unsupported"),
        SupportState::Disabled => return guarded(harness, plan, "capability_disabled"),
        SupportState::Unknown => return guarded(harness, plan, "capability_unknown"),
    }
    if crate::catalog::freshness_status(plan) != "fresh" {
        return Err(error::Failure::unavailable(
            "evidence_stale",
            format!(
                "{}:{} evidence from {} is stale",
                harness.name, plan.capability, plan.verified_at
            ),
            "refresh the upstream evidence before execution",
        ));
    }
    let Some(platform) = crate::platform::id() else {
        return Err(error::Failure::unavailable(
            "platform_unsupported",
            format!(
                "{}:{} is not claimed on {}-{} ({})",
                harness.name,
                plan.capability,
                std::env::consts::OS,
                std::env::consts::ARCH,
                crate::platform::libc()
            ),
            "use a claimed native target or follow the upstream manual procedure",
        ));
    };
    if !plan.platforms.iter().any(|candidate| candidate == platform) {
        return Err(error::Failure::unavailable(
            "platform_incompatible",
            format!(
                "{}:{} does not support platform {platform}",
                harness.name, plan.capability
            ),
            format!(
                "run `terminal-jarvis plan {} {}`",
                harness.name, plan.capability
            ),
        ));
    }
    Ok(())
}

fn guarded(harness: &Harness, plan: &CapabilityPlan, code: &'static str) -> error::Result<()> {
    Err(error::Failure::unavailable(
        code,
        format!(
            "{}:{} is {}; {}",
            harness.name,
            plan.capability,
            plan.support.as_str(),
            plan.summary
        ),
        format!(
            "run `terminal-jarvis plan {} {}`",
            harness.name, plan.capability
        ),
    ))
}
