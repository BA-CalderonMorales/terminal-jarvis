use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};

fn plan(command: String, args: Vec<String>, support: SupportState) -> CapabilityPlan {
    CapabilityPlan {
        capability: Capability::Headless,
        summary: String::new(),
        command: CommandPlan::new(command, args),
        support,
        evidence: EvidenceMode::Deterministic,
        effect: Effect::ReadOnly,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms: Vec::new(),
        executable: "x".into(),
        source: "test".into(),
        verified_at: "2026-07-17T04:59:27Z".into(),
        package: None,
    }
}

fn help_fallback(command: &str) -> bool {
    command
        .split_whitespace()
        .any(|word| word.trim_matches(['\'', '"']) == "--help")
}

fn unsafe_pipe(command: &str) -> bool {
    command.contains("curl ") && (command.contains("| sh") || command.contains("| bash"))
}

fn placeholder(command: &str) -> bool {
    command.contains("not configured") && command.contains("exit 1")
}

fn help_fallback_requires_stub(command: String, args: Vec<String>, support: SupportState) -> bool {
    let candidate = plan(command.clone(), args, support);
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    let rendered = candidate.command.render().to_ascii_lowercase();
    let fallback = help_fallback(&rendered);
    let flagged = errors.iter().any(|e| e.contains("help fallback"));
    flagged == (fallback && support != SupportState::Stub)
}

fn unsafe_pipe_blocks_support(command: String, args: Vec<String>) -> bool {
    let candidate = plan(command.clone(), args, SupportState::Verified);
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    let rendered = candidate.command.render().to_ascii_lowercase();
    errors.iter().any(|e| e.contains("curl-pipe")) == unsafe_pipe(&rendered)
}

fn sudo_is_always_rejected(command: String, args: Vec<String>, support: SupportState) -> bool {
    let candidate = plan(command.clone(), args, support);
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    let rendered = candidate.command.render().to_ascii_lowercase();
    let has_sudo = rendered.split_whitespace().any(|word| word == "sudo");
    errors.iter().any(|e| e.contains("sudo")) == has_sudo
}

fn placeholder_requires_disabled(
    command: String,
    args: Vec<String>,
    support: SupportState,
) -> bool {
    let candidate = plan(command.clone(), args, support);
    let mut errors = Vec::new();
    super::validate("h", &candidate, &mut errors);
    let rendered = candidate.command.render().to_ascii_lowercase();
    errors.iter().any(|e| e.contains("placeholder"))
        == (placeholder(&rendered) && support != SupportState::Disabled)
}

#[test]
fn command_truth_properties() {
    quickcheck::quickcheck(
        help_fallback_requires_stub as fn(String, Vec<String>, SupportState) -> bool,
    );
    quickcheck::quickcheck(unsafe_pipe_blocks_support as fn(String, Vec<String>) -> bool);
    quickcheck::quickcheck(
        sudo_is_always_rejected as fn(String, Vec<String>, SupportState) -> bool,
    );
    quickcheck::quickcheck(
        placeholder_requires_disabled as fn(String, Vec<String>, SupportState) -> bool,
    );
}
