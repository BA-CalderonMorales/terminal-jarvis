use super::*;
use crate::contracts::{EnvMode, Harness};

fn harness(command: &str, args: Vec<String>) -> Vec<Harness> {
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "test fixture".into(),
        binary: command.into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![crate::cli::test_support::plan(
            Capability::Download,
            command,
            args,
        )],
    }]
}

#[test]
fn failing_command_preserves_exit_without_crossing_streams() {
    let plans = harness("sh", vec!["-c".into(), "exit 3".into()]);
    let (code, body) = capability(&plans, "vibe", Capability::Download, &[]).unwrap();
    assert_eq!(code, 3);
    assert!(body.is_empty());
}

#[test]
fn missing_binary_maps_to_shell_not_found_exit() {
    let plans = harness("terminal-jarvis-definitely-missing", vec![]);
    let (code, body) = capability(&plans, "vibe", Capability::Download, &[]).unwrap();
    assert_eq!(code, 127);
    assert!(body.is_empty());
}
