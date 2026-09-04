use super::*;
use crate::contracts::{Capability, CommandPlan, Effect, EvidenceMode, Interaction, SupportState};

fn code_of(script: &str) -> i32 {
    status_code(
        std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .status()
            .unwrap(),
    )
}

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
fn maps_exit_codes_and_signal_terms() {
    assert_eq!(code_of("exit 0"), 0);
    assert_eq!(code_of("exit 7"), 7);
    assert_eq!(code_of("kill -TERM $$"), 143);
}

#[test]
fn child_gets_default_sigint_when_parent_suppresses() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe {
        signal(SIGINT, SIG_IGN);
    }
    let code = run_command(&plan("kill -INT $$"), &[]).unwrap();
    unsafe {
        signal(SIGINT, SIG_DFL);
    }
    assert_eq!(code, 130);
}

#[test]
fn headless_runs_inherit_the_terminal_and_capture_nothing() {
    // Outside the tui the child owns the real stdio: the captured text must
    // stay empty no matter what the child prints. Deleting either the
    // capture-branch guard or its negation flips this into a capture.
    let (code, text) = run_command_text(&plan("echo headless-marker"), &[]).unwrap();
    assert_eq!(code, 0);
    assert!(text.is_empty(), "headless must not capture: {text:?}");
}

#[test]
fn headless_stderr_is_never_folded_into_the_body_text() {
    let (_code, text) =
        run_command_text(&plan("echo out-marker; echo err-marker >&2"), &[]).unwrap();
    assert!(text.is_empty(), "headless must not capture: {text:?}");
}
