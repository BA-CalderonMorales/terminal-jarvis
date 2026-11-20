//! GuardPlatform: the platform half of the guard. A capability the catalog
//! never claimed for this target cannot run here, and that is an "unavailable"
//! answer -- distinct from "are you sure?", which is the consent path.

use super::error;
use crate::contracts::{CapabilityPlan, Harness};

/// Reject a plan the catalog does not claim for this platform.
pub(super) fn check(harness: &Harness, plan: &CapabilityPlan) -> error::Result<()> {
    let Some(platform) = crate::context::platform::id() else {
        return Err(error::Failure::unavailable(
            "platform_unsupported",
            format!(
                "{}:{} is not claimed on {}-{} ({})",
                harness.name,
                plan.capability,
                std::env::consts::OS,
                std::env::consts::ARCH,
                crate::context::platform::libc()
            ),
            "use a claimed native target or follow the upstream manual procedure",
        ));
    };
    if !plan.platforms.is_empty() && !plan.platforms.iter().any(|candidate| candidate == platform) {
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

pub(super) fn guarded(
    harness: &Harness,
    plan: &CapabilityPlan,
    code: &'static str,
) -> error::Result<()> {
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
