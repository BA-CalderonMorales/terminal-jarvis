use std::process::Command;

static BINARY: &str = env!("CARGO_BIN_EXE_terminal-jarvis");

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
fn install_known_harness_requires_explicit_intent() {
    for harness in ["aider", "opencode", "codex", "code"] {
        let (code, _, stderr) = run(&["install", harness]);
        assert_eq!(code, 5, "install {harness} must be intent-gated");
        assert!(
            stderr.contains("confirm=download:"),
            "install {harness} should demand the download intent token"
        );
    }
}

#[test]
fn install_unguarded_harness_is_red() {
    for harness in ["ollama", "vibe"] {
        let (code, _, stderr) = run(&["install", harness]);
        assert_eq!(code, 4, "install {harness} should exit 4");
        assert!(
            stderr.contains("is unknown; Install"),
            "install {harness} should say capability is unknown"
        );
    }
}

#[test]
fn tui_requires_an_interactive_terminal() {
    let (code, _, stderr) = run(&["tui"]);
    assert_eq!(code, 4, "noninteractive tui must fail closed");
    assert!(stderr.contains("interactive terminal"));
}

#[test]
fn tui_with_lifecycle_flags_fails_closed() {
    let (code, _, _) = run(&["tui", "--dry-run"]);
    assert_eq!(code, 2, "lifecycle options are not valid for the tui");
}

#[test]
fn install_unknown_harness_is_red() {
    for unknown in ["nonexistent", "xyz", "foo-bar"] {
        let (code, _, stderr) = run(&["install", unknown]);
        assert_eq!(code, 4, "install {unknown} should exit 4");
        assert!(
            stderr.contains("unknown harness"),
            "install {unknown} should say harness is unknown"
        );
        assert!(
            stderr.contains("terminal-jarvis list"),
            "install {unknown} should suggest list"
        );
    }
}

#[test]
fn unknown_flag_is_red() {
    let (code, _, _) = run(&["--bogus"]);
    assert_eq!(code, 2, "unknown flag should exit 2");
}

#[test]
fn use_without_args_is_red() {
    let (code, _, _) = run(&["use"]);
    assert_eq!(code, 2, "use without args should exit 2");
}
