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

#[rustfmt::skip] fn stdout(output: &Output) -> String { String::from_utf8_lossy(&output.stdout).to_string() }
#[rustfmt::skip] fn stderr(output: &Output) -> String { String::from_utf8_lossy(&output.stderr).to_string() }
#[test]
fn list_outputs_catalog_truth_with_live_claims() {
    let body = stdout(&tj(&["list"]));
    assert!(body.lines().count() == 25);
    assert!(body.contains("codex support=") && body.contains("vibe support="));
    assert!(body.contains("verified=1"));
}

#[test]
fn show_includes_setup_and_capability_truth() {
    let body = stdout(&tj(&["show", "aider"]));
    assert!(
        body.contains("setup: set one of:")
            && body.contains("capability=download support=expected")
    );
    assert!(body.contains("summary=Install Aider"));
    assert!(body.contains("capability=ui support=expected"));
}

#[test]
fn help_announces_the_tui() {
    let body = stdout(&tj(&["--plain", "help"]));
    assert!(body.contains("Command center") && body.contains("terminal-jarvis tui"));
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
    assert!(current.status.success() && stdout(&current).contains("active harness = codex"));
}

#[test]
fn check_reports_setup_readiness() {
    let output = tj(&["check", "--verbose"]);
    assert_eq!(output.status.code(), Some(4));
    let body = stdout(&output);
    assert!(
        body.contains("harness.jules.readiness\tmissing\terror")
            && body.contains("harness.aider.executable")
    );
}

#[test]
fn unknown_harness_fails_with_message() {
    let output = tj(&["show", "missing"]);
    assert!(output.status.code() == Some(4) && stderr(&output).contains("unknown harness"));
}

#[test]
fn disabled_yolo_fails_before_placeholder_spawn() {
    let output = tj(&["run", "aider", "yolo"]);
    assert!(output.status.code() == Some(4) && stdout(&output).is_empty());
    assert!(stderr(&output).contains("aider:yolo is disabled"));
}
