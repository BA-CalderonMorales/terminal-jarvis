use std::process::{Command, Output};

fn tj(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(args)
        .env(
            "TERMINAL_JARVIS_HOME",
            "/tmp/terminal-jarvis-auth-cache-test",
        )
        .env("TERMINAL_JARVIS_CACHE", "/tmp/terminal-jarvis-cache")
        .env("TERMINAL_JARVIS_DISTRIBUTION", "github-release-cache")
        .env(
            "TERMINAL_JARVIS_RELEASE_URL",
            "https://example.invalid/release.tgz",
        )
        .env_remove("OPENCODE_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("terminal-jarvis runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

#[test]
fn cache_status_reports_wrapper_cache_metadata() {
    let output = tj(&["cache", "status"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains("cache: /tmp/terminal-jarvis-cache"));
    assert!(body.contains("distribution: github-release-cache"));
    assert!(body.contains("release: https://example.invalid/release.tgz"));
}

#[test]
fn cache_without_subcommand_reports_status() {
    let output = tj(&["cache"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("distribution: github-release-cache"));
}

#[test]
fn cache_rejects_unknown_subcommands() {
    let output = tj(&["cache", "unknown"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("usage: terminal-jarvis cache"));
}

#[test]
fn cache_status_suppresses_empty_release_url() {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["cache", "status"])
        .env(
            "TERMINAL_JARVIS_HOME",
            "/tmp/terminal-jarvis-auth-cache-test",
        )
        .env("TERMINAL_JARVIS_CACHE", "/tmp/terminal-jarvis-cache")
        .env("TERMINAL_JARVIS_DISTRIBUTION", "github-release-cache")
        .env("TERMINAL_JARVIS_RELEASE_URL", "")
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    assert!(!stdout(&output).contains("release:"));
}

#[test]
fn auth_reports_missing_environment_status() {
    let output = tj(&["auth", "opencode"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains("setup: set one of: OPENCODE_API_KEY, OPENAI_API_KEY"));
    assert!(body.contains("status: missing one of: OPENCODE_API_KEY, OPENAI_API_KEY"));
}
