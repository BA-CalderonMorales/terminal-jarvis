use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};
const KNOWN: &str = "linux-x64-gnu linux-arm64-gnu macos-x64 macos-arm64 windows-x64-msvc";

fn plan(
    executable: String,
    source: String,
    support: SupportState,
    evidence: EvidenceMode,
    platforms: Vec<String>,
) -> CapabilityPlan {
    CapabilityPlan {
        capability: Capability::Headless,
        summary: String::new(),
        command: CommandPlan::new(executable.clone(), Vec::new()),
        support,
        evidence,
        effect: Effect::ReadOnly,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms,
        executable,
        source,
        verified_at: "2026-07-17T04:59:27Z".into(),
        package: None,
    }
}

fn errors(candidate: &CapabilityPlan) -> Vec<String> {
    let mut errors = Vec::new();
    super::validate("h", candidate, &mut errors);
    errors
}
fn empty_required_fields_error(executable: String, source: String) -> bool {
    let candidate = plan(
        executable.clone(),
        source.clone(),
        SupportState::Unknown,
        EvidenceMode::Deterministic,
        Vec::new(),
    );
    let count = errors(&candidate)
        .iter()
        .filter(|e| e.contains("has an empty"))
        .count();
    let expected =
        usize::from(executable.trim().is_empty()) + usize::from(source.trim().is_empty());
    count == expected
}

fn fresh_support_has_no_freshness_error(seed: u8) -> bool {
    let candidate = plan(
        "x".into(),
        "y".into(),
        SupportState::Verified,
        EvidenceMode::DisposableReal,
        vec!["linux-x64-gnu".into()],
    );
    let _ = seed;
    errors(&candidate)
        .iter()
        .all(|e| !e.contains("must be fresh"))
}

fn evidence_must_match_support(support: SupportState, evidence: EvidenceMode) -> bool {
    let candidate = plan("x".into(), "y".into(), support, evidence, Vec::new());
    let flagged = errors(&candidate).iter().any(|e| e.contains("contradict"));
    let expected = match support {
        SupportState::Verified => evidence != EvidenceMode::DisposableReal,
        SupportState::Manual => evidence != EvidenceMode::Manual,
        SupportState::Unsupported => evidence != EvidenceMode::Unsupported,
        _ => evidence != EvidenceMode::Deterministic,
    };
    flagged == expected
}

fn platforms_must_be_known(platforms: Vec<String>) -> bool {
    let candidate = plan(
        "x".into(),
        "y".into(),
        SupportState::Unknown,
        EvidenceMode::Deterministic,
        platforms.clone(),
    );
    let flagged = errors(&candidate)
        .iter()
        .any(|e| e.contains("unknown platform"));
    platforms
        .iter()
        .all(|platform| KNOWN.split(' ').any(|known| known == platform) || flagged)
}

#[test]
fn truth_properties() {
    quickcheck::quickcheck(empty_required_fields_error as fn(String, String) -> bool);
    quickcheck::quickcheck(fresh_support_has_no_freshness_error as fn(u8) -> bool);
    quickcheck::quickcheck(evidence_must_match_support as fn(SupportState, EvidenceMode) -> bool);
    quickcheck::quickcheck(platforms_must_be_known as fn(Vec<String>) -> bool);
}
