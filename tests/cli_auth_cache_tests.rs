use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);
const RELEASE: &str = "https://example.invalid/release.tgz";

fn tj(args: &[&str]) -> (Output, PathBuf) {
    let cache = temp_path("cache");
    let output = base_command()
        .args(args)
        .env("TERMINAL_JARVIS_CACHE", &cache)
        .env("TERMINAL_JARVIS_DISTRIBUTION", "github-release-cache")
        .env("TERMINAL_JARVIS_RELEASE_URL", RELEASE)
        .env_remove("OPENCODE_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("terminal-jarvis runs");
    (output, cache)
}

fn base_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    command
        .arg("--plain")
        .env("TERMINAL_JARVIS_HOME", temp_path("home"));
    command
}

fn temp_path(label: &str) -> PathBuf {
    let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "terminal-jarvis-auth-cache-{label}-{}-{id}",
        std::process::id()
    ))
}

#[rustfmt::skip]
fn stdout(output: &Output) -> String { String::from_utf8_lossy(&output.stdout).to_string() }
#[rustfmt::skip]
fn stderr(output: &Output) -> String { String::from_utf8_lossy(&output.stderr).to_string() }

#[test]
fn cache_status_reports_wrapper_cache_metadata() {
    let (output, cache) = tj(&["cache", "status"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains(&format!("cache: {}", cache.display())));
    assert!(body.contains("distribution: npm"));
    assert!(body.contains(&format!("release: {RELEASE}")));
}

#[test]
fn cache_without_subcommand_reports_status() {
    let (output, _) = tj(&["cache"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("distribution: npm"));
}

#[test]
fn cache_rejects_unknown_subcommands() {
    let (output, _) = tj(&["cache", "unknown"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("usage: terminal-jarvis cache"));
}

#[test]
fn cache_status_suppresses_empty_release_url() {
    let output = base_command()
        .args(["cache", "status"])
        .env("TERMINAL_JARVIS_CACHE", temp_path("cache"))
        .env("TERMINAL_JARVIS_DISTRIBUTION", "github-release-cache")
        .env("TERMINAL_JARVIS_RELEASE_URL", "")
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    assert!(!stdout(&output).contains("release:"));
}

#[test]
fn cache_status_treats_empty_distribution_as_unknown() {
    let output = base_command()
        .args(["cache", "status"])
        .env("TERMINAL_JARVIS_CACHE", temp_path("cache"))
        .env("TERMINAL_JARVIS_DISTRIBUTION", "")
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    assert!(stdout(&output).contains("distribution: unknown"));
}

#[test]
fn auth_reports_missing_environment_status() {
    let (output, _) = tj(&["auth", "opencode"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains("setup: set one of: OPENCODE_API_KEY, OPENAI_API_KEY"));
    assert!(body.contains("status: missing one of: OPENCODE_API_KEY, OPENAI_API_KEY"));
}
