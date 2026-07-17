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
        assert_eq!(plan.evidence, EvidenceMode::Deterministic);
        assert!(!plan.executable.is_empty());
        assert!(!plan.source.is_empty());
        assert!(plan.verified_at.ends_with('Z'));
        if plan.support == SupportState::Unknown {
            assert!(plan.platforms.is_empty());
        }
    }
    assert_eq!(harnesses.len(), 25);
    assert_eq!(rows, 225);
    assert_eq!(states[&SupportState::Stub], 99);
    assert_eq!(states[&SupportState::Disabled], 23);
    assert_eq!(states[&SupportState::Unknown], 103);
    assert_eq!(effects[&Effect::ReadOnly], 123);
    assert_eq!(effects[&Effect::StateChanging], 75);
    assert_eq!(effects[&Effect::Dangerous], 27);
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
    assert_eq!(network, 102);
    assert_eq!(interactive, 27);
}
