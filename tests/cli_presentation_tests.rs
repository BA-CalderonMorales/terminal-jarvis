use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn home() -> String {
    std::env::temp_dir()
        .join(format!(
            "terminal-jarvis-presentation-{}-{}",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ))
        .to_string_lossy()
        .to_string()
}

fn tj(args: &[&str], home: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .env_remove("COLUMNS")
        .output()
        .expect("terminal-jarvis runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn assert_table(output: &Output, title: &str) {
    assert!(output.status.success(), "{output:?}");
    let body = stdout(output);
    assert!(body.contains(title), "{body}");
    assert!(body.contains('+') && body.contains('|'), "{body}");
}

#[test]
fn default_output_is_structured_across_the_core_read_only_surface() {
    let home = home();
    for (args, title) in [
        (&["--help"][..], "Commands"),
        (&["list"], "Available Harnesses"),
        (&["check"], "Harness Readiness"),
        (&["show", "codex"], "Capabilities"),
        (&["plan", "codex", "headless"], "Plan: codex headless"),
        (&["version", "--verbose"], "Terminal Jarvis"),
        (&["update"], "Harness Updates"),
        (&["auth", "help", "codex"], "Authentication"),
        (&["config", "path"], "Configuration Paths"),
        (&["cache", "status"], "Cache Status"),
        (&["security", "audit"], "Security Audit"),
        (&["gate", "status"], "Security Gate"),
        (&["templates"], "Legacy Command"),
    ] {
        assert_table(&tj(args, &home), title);
    }
}

#[test]
fn rich_mode_handles_selection_and_plain_mode_stays_script_friendly() {
    let home = home();
    assert_table(&tj(&["use", "codex"], &home), "Active Harness");
    assert_table(&tj(&["current"], &home), "Active Harness");
    let plain = tj(&["--plain", "list"], &home);
    assert!(plain.status.success());
    assert_eq!(stdout(&plain).lines().count(), 25);
    assert!(!stdout(&plain).contains("Available Harnesses"));
    let no_color = tj(&["--no-color", "list"], &home);
    assert_table(&no_color, "Available Harnesses");
    assert!(!stdout(&no_color).contains('\x1b'));
}

#[test]
fn headless_tables_wrap_without_losing_their_frame() {
    let output = tj(&["security", "audit"], &home());
    assert_table(&output, "Security Audit");
    assert!(stdout(&output)
        .lines()
        .all(|line| line.chars().count() <= 100));
}
