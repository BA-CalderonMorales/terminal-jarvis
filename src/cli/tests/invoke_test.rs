use super::errors::{diagnostic, verb};
use super::*;
use crate::contracts::CommandPlan;
use crate::contracts::{EnvMode, Harness};

fn harness(command: &str, args: Vec<String>) -> Vec<Harness> {
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "test fixture".into(),
        binary: command.into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![crate::cli::logic::test_support::plan(
            Capability::Download,
            command,
            args,
        )],
    }]
}

#[test]
fn failing_command_preserves_exit_without_crossing_streams() {
    let plans = harness("sh", vec!["-c".into(), "exit 3".into()]);
    let (code, body) = capability(&plans, "vibe", Capability::Download, &[], true).unwrap();
    assert_eq!(code, 3);
    assert!(body.is_empty());

    // redact_process_text (see diagnostics::logic::redact_process) checks
    // both HOME and USERPROFILE; only HOME is set by default on Unix, only
    // USERPROFILE on Windows, so this mirrors that same fallback.
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap();
    let command = CommandPlan::new(format!("{home}/private-child/fixture"), vec![]);
    let rendered = diagnostic("vibe", Capability::Download, &command, 3);
    assert!(!rendered.contains(&home));
    assert!(rendered.contains("~/private-child/fixture"));
}

#[test]
fn missing_binary_maps_to_shell_not_found_exit() {
    let plans = harness("terminal-jarvis-definitely-missing", vec![]);
    let (code, body) = capability(&plans, "vibe", Capability::Download, &[], true).unwrap();
    assert_eq!(code, 127);
    assert!(body.is_empty());
}

#[test]
fn narration_verb_matches_the_capability() {
    assert_eq!(verb(Capability::Download), "installing");
    assert_eq!(verb(Capability::Update), "updating");
    assert_eq!(verb(Capability::Headless), "updating");
    assert_eq!(verb(Capability::Yolo), "updating");
}
