//! Quickcheck properties for guarded launches inside the tui: any slash
//! invocation whose capability the guard refuses (stub support) must return
//! the switcher to its prompt -- never exit, never panic, never fall through
//! to execution. Witnessed by the vhs guarded-flow audit of release/0.1.16.

use super::*;
use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};

fn stub_plan(capability: Capability) -> CapabilityPlan {
    let mut plan = CapabilityPlan {
        capability,
        summary: "stub probe".into(),
        command: CommandPlan::new("sh".into(), vec!["-c".into(), "exit 0".into()]),
        support: SupportState::Verified,
        evidence: EvidenceMode::Deterministic,
        effect: Effect::ReadOnly,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms: vec![],
        executable: "sh".into(),
        source: "internal:test-fixture".into(),
        verified_at: "2026-08-06T00:00:00Z".into(),
        package: None,
    };
    plan.support = SupportState::Stub;
    plan
}

fn stub_harness(name: &str, capability: Capability) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![stub_plan(capability)],
    }
}

fn options() -> args::Options {
    args::Options::default()
}

fn guard_refusal_returns_to_the_prompt(name: String, pick: usize) -> bool {
    if name.is_empty()
        || !name.chars().all(|c| c.is_ascii_alphanumeric())
        || name.chars().all(|c| c.is_ascii_digit())
    {
        return true;
    }
    let capability = Capability::ALL[pick % Capability::ALL.len()];
    let harnesses = [stub_harness(&name, capability)];
    let input = format!("/{name} {}", capability.as_str());
    let (catalog_root, state_home) = (std::env::temp_dir(), std::env::temp_dir());
    let previous = crate::cli::style::set(true, true);
    let next = handle(&harnesses, &catalog_root, &state_home, &options(), &input);
    crate::cli::style::restore(previous);
    matches!(next, Next::Again { .. })
}

#[test]
fn guarded_launches_hold_survival_properties() {
    quickcheck::quickcheck(guard_refusal_returns_to_the_prompt as fn(String, usize) -> bool);
}
