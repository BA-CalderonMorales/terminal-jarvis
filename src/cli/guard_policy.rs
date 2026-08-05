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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{
        CapabilityPlan, CommandPlan, Effect, Harness, Interaction, SupportState,
    };

    fn dummy_harness() -> Harness {
        Harness {
            name: "test".into(),
            display: "Test".into(),
            description: "test".into(),
            binary: "test".into(),
            env_mode: crate::contracts::EnvMode::None,
            env: vec![],
            capabilities: vec![],
        }
    }

    fn dummy_plan(
        support: SupportState,
        verified_at: &str,
        platforms: Vec<String>,
    ) -> CapabilityPlan {
        CapabilityPlan {
            capability: crate::contracts::Capability::Headless,
            summary: "test".into(),
            command: CommandPlan::new("test".into(), vec![]),
            support,
            evidence: crate::contracts::EvidenceMode::Deterministic,
            effect: Effect::ReadOnly,
            network: false,
            interaction: Interaction::Noninteractive,
            platforms,
            executable: "test".into(),
            source: "test".into(),
            verified_at: verified_at.into(),
        }
    }

    #[test]
    fn rejects_manual_support() {
        let harness = dummy_harness();
        let plan = dummy_plan(
            SupportState::Manual,
            "2026-07-17T04:59:27Z",
            vec![platform_str().into()],
        );
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "manual_procedure_required");
        assert_eq!(err.exit_code, 4);
    }

    #[test]
    fn rejects_stub_support() {
        let harness = dummy_harness();
        let plan = dummy_plan(SupportState::Stub, "2026-07-17T04:59:27Z", vec![]);
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "capability_stub");
    }

    #[test]
    fn rejects_unsupported_support() {
        let harness = dummy_harness();
        let plan = dummy_plan(SupportState::Unsupported, "2026-07-17T04:59:27Z", vec![]);
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "capability_unsupported");
    }

    #[test]
    fn rejects_disabled_support() {
        let harness = dummy_harness();
        let plan = dummy_plan(SupportState::Disabled, "2026-07-17T04:59:27Z", vec![]);
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "capability_disabled");
    }

    #[test]
    fn rejects_unknown_support() {
        let harness = dummy_harness();
        let plan = dummy_plan(SupportState::Unknown, "2026-07-17T04:59:27Z", vec![]);
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "capability_unknown");
    }

    #[test]
    fn accepts_verified_support_with_fresh_evidence_and_matching_platform() {
        let harness = dummy_harness();
        let plan = dummy_plan(
            SupportState::Verified,
            "2026-07-17T04:59:27Z",
            vec![platform_str().into()],
        );
        let result = check(&harness, &plan);
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_expected_support_with_fresh_evidence_and_matching_platform() {
        let harness = dummy_harness();
        let plan = dummy_plan(
            SupportState::Expected,
            "2026-07-17T04:59:27Z",
            vec![platform_str().into()],
        );
        let result = check(&harness, &plan);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_stale_evidence() {
        let harness = dummy_harness();
        // Use a very old date that will be stale
        let plan = dummy_plan(
            SupportState::Verified,
            "2024-01-01T00:00:00Z",
            vec![platform_str().into()],
        );
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "evidence_stale");
    }

    #[test]
    fn rejects_platform_incompatible() {
        let harness = dummy_harness();
        // Use a different platform than the current one
        let other_platform = if platform_str() == "linux-x64-gnu" {
            "macos-x64"
        } else {
            "linux-x64-gnu"
        };
        let plan = dummy_plan(
            SupportState::Verified,
            "2026-07-17T04:59:27Z",
            vec![other_platform.into()],
        );
        let result = check(&harness, &plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "platform_incompatible");
    }

    fn platform_str() -> &'static str {
        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("linux", "x86_64") => "linux-x64-gnu",
            ("linux", "aarch64") => "linux-arm64-gnu",
            ("macos", "x86_64") => "macos-x64",
            ("macos", "aarch64") => "macos-arm64",
            _ => "windows-x64-msvc",
        }
    }
}
