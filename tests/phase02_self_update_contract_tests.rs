use std::process::Command;
use std::time::{Duration, Instant};

fn tj(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(args)
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
        .env("TERMINAL_JARVIS_CATALOG", "/missing/catalog")
        .output()
        .expect("terminal-jarvis runs")
}

#[test]
fn noninteractive_self_update_requires_exact_bound_intent_without_hanging() {
    let started = Instant::now();
    let missing = tj(&["--update"]);
    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(missing.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&missing.stderr)
        .contains("--no-input --confirm=self-update:terminal-jarvis"));

    let wrong = tj(&["--update", "--no-input", "--confirm=update:terminal-jarvis"]);
    assert_eq!(wrong.status.code(), Some(5));
}

#[test]
fn self_update_dry_run_is_local_and_side_effect_free() {
    let output = tj(&["--update", "--dry-run", "--plain"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan: cargo install terminal-jarvis"));
    assert!(output.stderr.is_empty());
}

#[test]
fn dangerous_opt_in_is_rejected_for_self_update() {
    let output = tj(&["--update", "--allow-dangerous", "--dry-run"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("not valid for self-update"));
}
