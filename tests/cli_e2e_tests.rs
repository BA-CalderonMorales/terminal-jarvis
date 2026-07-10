use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn tj(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain"])
        .args(args)
        .env("TERMINAL_JARVIS_HOME", temp_home())
        .output()
        .expect("terminal-jarvis runs")
}

fn tj_with_home(args: &[&str], home: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain"])
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .output()
        .expect("terminal-jarvis runs")
}

fn temp_home() -> String {
    std::env::temp_dir()
        .join(format!(
            "terminal-jarvis-e2e-{}-{}",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ))
        .to_string_lossy()
        .to_string()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}
fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
#[test]
fn list_outputs_promoted_harnesses() {
    let output = tj(&["list"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert_eq!(body.lines().count(), 25);
    assert!(body.contains("codex - OpenAI coding agent CLI"));
    assert!(body.contains("vibe - Minimal CLI coding agent by Mistral AI"));
}

#[test]
fn show_includes_setup_and_agent_loop() {
    let output = tj(&["show", "aider"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains("setup: set one of:"));
    assert!(body.contains("download: Install Aider"));
    assert!(body.contains("ui: Open the interactive Aider interface."));
}

#[test]
fn plan_renders_copyable_shell_command() {
    let output = tj(&["plan", "claude", "download"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("sh -c 'curl -fsSL https://claude.ai/install.sh | bash'"));
}

#[test]
fn use_and_current_round_trip_active_harness() {
    let home = temp_home();
    assert!(tj_with_home(&["use", "codex"], &home).status.success());
    let current = tj_with_home(&["current"], &home);
    assert!(current.status.success());
    assert!(stdout(&current).contains("active harness = codex"));
}

#[test]
fn check_reports_setup_readiness() {
    let output = tj(&["check"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains("jules binary=missing env=ready"));
    assert!(body
        .lines()
        .any(|line| line.starts_with("aider binary=") && line.contains(" env=")));
}

#[test]
fn unknown_harness_fails_with_message() {
    let output = tj(&["show", "missing"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("unknown harness 'missing'"));
}

#[test]
fn yolo_placeholder_runs_and_fails_closed() {
    let output = tj(&["run", "aider", "yolo"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stdout(&output).contains("danger yolo mode is not configured for aider"));
}
