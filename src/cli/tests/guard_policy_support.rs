use crate::contracts::{CapabilityPlan, CommandPlan, Effect, Harness, Interaction, SupportState};

pub fn dummy_harness() -> Harness {
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

pub fn dummy_plan(
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
        package: None,
    }
}

pub fn platform_str() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux-x64-gnu",
        ("linux", "aarch64") => "linux-arm64-gnu",
        ("macos", "x86_64") => "macos-x64",
        ("macos", "aarch64") => "macos-arm64",
        _ => "windows-x64-msvc",
    }
}
