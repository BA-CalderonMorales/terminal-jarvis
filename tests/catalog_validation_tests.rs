use terminal_jarvis::catalog;
use terminal_jarvis::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EnvMode, EvidenceMode, Harness, Interaction,
    SupportState,
};

fn plan(capability: Capability) -> CapabilityPlan {
    CapabilityPlan {
        capability,
        summary: "Dangerous test plan".to_string(),
        command: CommandPlan::new("sh".to_string(), Vec::new()),
        support: SupportState::Unknown,
        evidence: EvidenceMode::Deterministic,
        effect: if capability == Capability::Yolo {
            Effect::Dangerous
        } else {
            Effect::StateChanging
        },
        network: true,
        interaction: Interaction::Noninteractive,
        platforms: Vec::new(),
        executable: "sh".to_string(),
        source: "internal:test".to_string(),
        verified_at: "2026-07-17T00:00:00Z".to_string(),
    }
}

#[test]
fn rejects_truth_contradictions_and_duplicate_rows() {
    let mut plans = Capability::ALL.map(plan).to_vec();
    plans[0].support = SupportState::Verified;
    plans[0].executable = "other".to_string();
    plans[0].verified_at = "not-a-date".to_string();
    plans.push(plans[0].clone());
    let bad = Harness {
        name: "bad".to_string(),
        display: "bad".to_string(),
        description: "bad".to_string(),
        binary: "sh".to_string(),
        env_mode: EnvMode::None,
        env: Vec::new(),
        capabilities: plans,
    };
    let errors = catalog::validate(&[bad]).join("\n");
    for expected in [
        "support and evidence contradict",
        "support claim has no platform",
        "executable must match command",
        "verified_at must be a UTC timestamp",
        "duplicate capability",
        "must define exactly 9 capabilities",
    ] {
        assert!(errors.contains(expected), "missing {expected}: {errors}");
    }
}

#[test]
fn support_claims_reject_fallbacks_and_unsafe_installers() {
    let mut plans = Capability::ALL.map(plan).to_vec();
    let platform = terminal_jarvis::platform::id().unwrap().to_string();
    let download = plans
        .iter_mut()
        .find(|plan| plan.capability == Capability::Download)
        .unwrap();
    download.support = SupportState::Expected;
    download.platforms = vec![platform.clone()];
    download.command = CommandPlan::new(
        "sh".to_string(),
        vec!["-c".to_string(), "curl https://invalid | sh".to_string()],
    );
    let version = plans
        .iter_mut()
        .find(|plan| plan.capability == Capability::Version)
        .unwrap();
    version.support = SupportState::Expected;
    version.platforms = vec![platform];
    version.effect = Effect::ReadOnly;
    version.network = false;
    version.command = CommandPlan::new("tool".to_string(), vec!["--help".to_string()]);
    version.executable = "tool".to_string();
    let harness = Harness {
        name: "unsafe".to_string(),
        display: "unsafe".to_string(),
        description: "unsafe".to_string(),
        binary: "tool".to_string(),
        env_mode: EnvMode::None,
        env: Vec::new(),
        capabilities: plans,
    };
    let errors = catalog::validate(&[harness]).join("\n");
    assert!(errors.contains("curl-pipe installer"), "{errors}");
    assert!(
        errors.contains("help fallback must be classified as stub"),
        "{errors}"
    );
}
