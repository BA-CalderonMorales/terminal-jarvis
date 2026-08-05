use std::process::Command;

static BINARY: &str = env!("CARGO_BIN_EXE_terminal-jarvis");

fn known_harnesses() -> Vec<&'static str> {
    vec!["aider", "opencode", "codex", "claude", "code"]
}

fn known_capabilities() -> Vec<&'static str> {
    vec!["download", "headless", "version", "models"]
}

fn run(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(BINARY)
        .args(["--plain"])
        .args(args)
        .env("TERMINAL_JARVIS_HOME", temp_home())
        .output()
        .expect("terminal-jarvis runs");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(1);
    (code, stdout, stderr)
}

fn temp_home() -> String {
    std::env::temp_dir()
        .join(format!("tj-redgreen-{}", std::process::id()))
        .to_string_lossy()
        .to_string()
}
#[test]
fn help_is_green() {
    let (code, stdout, _) = run(&["help"]);
    assert_eq!(code, 0, "help should exit 0");
    assert!(
        stdout.contains("Terminal Jarvis"),
        "help should contain title"
    );
    assert!(stdout.contains("usage:"), "help should show usage");
    assert!(
        stdout.contains("global flags"),
        "help should list global flags"
    );
}
#[test]
fn list_is_green() {
    let (code, stdout, _) = run(&["list"]);
    assert_eq!(code, 0, "list should exit 0");
    for harness in &known_harnesses() {
        assert!(stdout.contains(harness), "list should contain {harness}");
    }
}

#[test]
fn version_is_green() {
    let (code, stdout, _) = run(&["version"]);
    assert_eq!(code, 0, "version should exit 0");
    assert!(
        stdout.contains("terminal-jarvis"),
        "version should contain binary name"
    );
}

#[test]
fn current_is_green() {
    let (code, stdout, _) = run(&["current"]);
    assert_eq!(code, 0, "current should exit 0");
    assert!(
        stdout.contains("active harness"),
        "current should show harness status"
    );
}

#[test]
fn show_known_harness_is_green() {
    for harness in &known_harnesses() {
        let (code, stdout, _) = run(&["show", harness]);
        assert_eq!(code, 0, "show {harness} should exit 0");
        assert!(
            stdout.contains("setup:"),
            "show {harness} should show setup hint"
        );
    }
}

#[test]
fn plan_known_capability_is_green() {
    for harness in &known_harnesses() {
        for capability in &known_capabilities() {
            let (code, stdout, _) = run(&["plan", harness, capability]);
            assert_eq!(code, 0, "plan {harness} {capability} should exit 0");
            assert!(
                stdout.contains(&format!("{}:{}", harness, capability)),
                "plan {harness} {capability} should show {harness}:{capability}"
            );
        }
    }
}
