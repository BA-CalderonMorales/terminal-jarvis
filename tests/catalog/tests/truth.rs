use std::collections::BTreeMap;
use std::path::Path;
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::{Effect, EvidenceMode, Interaction, SupportState};

#[test]
fn all_catalog_rows_have_honest_truth_metadata() {
    let harnesses = catalog::load(Path::new("harnesses")).expect("catalog loads");
    let mut states = BTreeMap::new();
    let mut effects = BTreeMap::new();
    let mut rows = 0;
    for plan in harnesses.iter().flat_map(|harness| &harness.capabilities) {
        rows += 1;
        *states.entry(plan.support).or_insert(0) += 1;
        *effects.entry(plan.effect).or_insert(0) += 1;
        let expected_evidence = if plan.support == SupportState::Verified {
            EvidenceMode::DisposableReal
        } else {
            EvidenceMode::Deterministic
        };
        assert_eq!(plan.evidence, expected_evidence);
        assert!(!plan.executable.is_empty());
        assert!(!plan.source.is_empty());
        assert!(plan.verified_at.ends_with('Z'));
        if plan.support == SupportState::Unknown {
            assert!(plan.platforms.is_empty());
        }
    }
    assert_eq!(harnesses.len(), 25);
    assert_eq!(rows, 225);
    assert_eq!(states[&SupportState::Stub], 98);
    assert_eq!(states[&SupportState::Disabled], 25);
    assert_eq!(states[&SupportState::Unknown], 8);
    assert_eq!(states[&SupportState::Expected], 94);
    assert!(!states.contains_key(&SupportState::Verified));
    assert_eq!(effects[&Effect::ReadOnly], 122);
    assert_eq!(effects[&Effect::StateChanging], 75);
    assert_eq!(effects[&Effect::Dangerous], 28);
}

#[test]
fn interaction_and_network_counts_match_the_frozen_audit() {
    let harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let plans = harnesses.iter().flat_map(|harness| &harness.capabilities);
    let (mut network, mut interactive) = (0, 0);
    for plan in plans {
        network += usize::from(plan.network);
        interactive += usize::from(plan.interaction == Interaction::Interactive);
    }
    assert_eq!(network, 103);
    assert_eq!(interactive, 27);
}

/// Regression witness for the shape-gate rule in
/// `catalog::logic::command_truth`: an executable claim that also declares
/// itself platform-independent (`platforms` empty) must not route through a
/// POSIX shell, which a clean Windows `PATH` lacks. This is the check that
/// would have caught the 23 version probes that used to `sh -c`.
#[test]
fn no_row_claims_platform_independence_through_a_posix_shell() {
    let harnesses = catalog::load(Path::new("harnesses")).expect("catalog loads");
    for harness in &harnesses {
        for plan in &harness.capabilities {
            let executable_claim = matches!(
                plan.support,
                SupportState::Verified | SupportState::Expected
            );
            assert!(
                !(executable_claim
                    && plan.platforms.is_empty()
                    && matches!(plan.executable.as_str(), "sh" | "bash")),
                "{}:{} claims to run anywhere but requires the POSIX shell '{}'",
                harness.name,
                plan.capability,
                plan.executable
            );
        }
    }
}
