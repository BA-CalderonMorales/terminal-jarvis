use super::*;
use crate::contracts::{CapabilityPlan, EnvMode, Harness};

fn fake_harness() -> Vec<Harness> {
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "t".into(),
        binary: "sh".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![CapabilityPlan {
            capability: Capability::Download,
            summary: "d".into(),
            command: CommandPlan::new("sh".into(), vec!["-c".into(), "exit 3".into()]),
        }],
    }]
}

#[test]
fn failing_command_diagnoses_harness_capability_and_exit() {
    let (code, body) = capability(&fake_harness(), "vibe", Capability::Download, &[]).unwrap();
    assert_eq!(code, 3);
    assert!(body.contains("vibe"), "harness: {body}");
    assert!(body.contains("download"), "capability: {body}");
    assert!(body.contains("exit 3"), "command: {body}");
    assert!(body.contains('3'), "exit code: {body}");
}

fn pipefail_harness() -> Vec<Harness> {
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "t".into(),
        binary: "sh".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![CapabilityPlan {
            capability: Capability::Download,
            summary: "d".into(),
            command: CommandPlan::new(
                "sh".into(),
                vec!["-c".into(), "echo pipefail >&2; exit 3".into()],
            ),
        }],
    }]
}

#[test]
fn failing_command_appends_pipefail_hint() {
    let (code, body) = capability(&pipefail_harness(), "vibe", Capability::Download, &[]).unwrap();
    assert_eq!(code, 3);
    assert!(body.contains("pipefail"), "stderr not surfaced: {body}");
    assert!(
        body.contains("bash -c"),
        "pipefail hint not appended: {body}"
    );
}
