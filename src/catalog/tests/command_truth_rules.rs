//! Deterministic companion to the command-truth properties: the stand-in
//! reproduces the shape the shape-gate used to miss -- a `verified` version
//! probe that claims to run anywhere (`platforms` empty) yet routes through a
//! POSIX shell, which a clean Windows `PATH` does not provide.

use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};

fn plan(executable: &str, support: SupportState, platforms: Vec<String>) -> CapabilityPlan {
    CapabilityPlan {
        capability: Capability::Version,
        summary: String::new(),
        command: CommandPlan::new(executable.into(), vec!["--version".into()]),
        support,
        evidence: EvidenceMode::Deterministic,
        effect: Effect::ReadOnly,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms,
        executable: executable.into(),
        source: "test".into(),
        verified_at: "2026-07-17T04:59:27Z".into(),
        package: None,
        uninstall: None,
    }
}

fn flagged(candidate: &CapabilityPlan) -> bool {
    let mut errors = Vec::new();
    super::validate("fixture:version", candidate, &mut errors);
    errors.iter().any(|error| error.contains("POSIX shell"))
}

#[test]
fn platform_independent_execution_cannot_require_a_posix_shell() {
    assert!(flagged(&plan("sh", SupportState::Verified, Vec::new())));
    assert!(flagged(&plan("bash", SupportState::Expected, Vec::new())));
    // Narrowing to a POSIX platform is honest; so is a shell-free command.
    assert!(!flagged(&plan(
        "sh",
        SupportState::Verified,
        vec!["linux-x64-gnu".into()]
    )));
    assert!(!flagged(&plan("sh", SupportState::Disabled, Vec::new())));
    assert!(!flagged(&plan("aider", SupportState::Verified, Vec::new())));
}
