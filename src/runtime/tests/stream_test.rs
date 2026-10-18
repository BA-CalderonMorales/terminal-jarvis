use super::*;
use crate::contracts::{Capability, CommandPlan, Effect, EvidenceMode, Interaction, SupportState};

fn plan(command: &str, args: Vec<String>) -> CapabilityPlan {
    CapabilityPlan {
        command: CommandPlan::new(command.into(), args),
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
    let (command, args) = crate::cli::logic::test_support::shell(
        "printf early; sleep 0.2; printf '\\nlate-out\\n'; printf late-err >&2; exit 17",
        "set /p =early<nul& ping -n 2 127.0.0.1 >nul& echo.& echo late-out& echo late-err>&2& exit 17",
    );
    let code = run(&plan(&command, args), &[], &mut |line| {
        lines.push(line.to_string())
    })
    .unwrap();
    assert_eq!(code, 17);
    assert_eq!(lines.len(), 3);
    assert!(lines.contains(&"early".to_string()));
    assert!(lines.contains(&"late-out".to_string()));
    assert!(lines.contains(&"late-err".to_string()));
}

#[test]
fn classify_labels_failures_warnings_and_normal_lines() {
    let failed = classify("failed marker");
    assert!(failed.starts_with("ERROR "), "{failed}");
    assert!(failed.ends_with("failed marker"), "{failed}");
    let warned = classify("deprecated marker");
    assert!(warned.starts_with("WARN "), "{warned}");
    assert!(warned.ends_with("deprecated marker"), "{warned}");
    let plain = classify("plain marker");
    assert!(plain.starts_with("INFO "), "{plain}");
    assert!(plain.ends_with("plain marker"), "{plain}");
}

#[test]
fn classify_keeps_a_declared_level_instead_of_stacking_one() {
    assert_eq!(
        classify("INFO Downloading thing"),
        "INFO  Downloading thing"
    );
    assert_eq!(classify("error: boom"), "ERROR boom");
    assert_eq!(classify("warn deprecated"), "WARN  deprecated");
    // No wall-clock stamp is prefixed: the row starts with the level.
    assert!(!classify("plain").contains(':'));
}
