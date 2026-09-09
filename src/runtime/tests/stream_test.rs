use super::*;
use crate::contracts::{Capability, CommandPlan, Effect, EvidenceMode, Interaction, SupportState};

fn plan(script: &str) -> CapabilityPlan {
    CapabilityPlan {
        command: CommandPlan::new("sh".into(), vec!["-c".into(), script.into()]),
        capability: Capability::Headless,
        summary: String::new(),
        support: SupportState::Unknown,
        evidence: EvidenceMode::Deterministic,
        effect: Effect::ReadOnly,
        network: false,
        interaction: Interaction::Noninteractive,
        platforms: vec![],
        executable: String::new(),
        source: String::new(),
        verified_at: String::new(),
        package: None,
    }
}

#[test]
fn run_drains_both_streams_after_a_lull_and_preserves_exit_code() {
    let mut lines = vec![];
    let code = run(
        &plan("printf early; sleep 0.2; printf '\\nlate-out\\n'; printf late-err >&2; exit 17"),
        &[],
        &mut |line| lines.push(line.to_string()),
    )
    .unwrap();
    assert_eq!(code, 17);
    assert_eq!(lines.len(), 3);
    assert!(lines.contains(&"early".to_string()));
    assert!(lines.contains(&"late-out".to_string()));
    assert!(lines.contains(&"late-err".to_string()));
}

#[test]
fn classify_labels_failures_warnings_and_normal_lines() {
    assert!(classify("failed marker").contains(" ERROR failed marker"));
    assert!(classify("deprecated marker").contains(" WARN  deprecated marker"));
    assert!(classify("plain marker").contains(" INFO  plain marker"));
}

#[test]
fn stamp_is_a_utc_clock_value() {
    let stamp = stamp();
    assert_eq!(stamp.len(), 8);
    assert_eq!(&stamp[2..3], ":");
    assert_eq!(&stamp[5..6], ":");
    assert!(stamp
        .chars()
        .enumerate()
        .all(|(i, c)| matches!(i, 2 | 5) || c.is_ascii_digit()));
}
