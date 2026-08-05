use std::process::{Command, Output};

fn tj(args: &[&str], home: &std::path::Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .output()
        .expect("terminal-jarvis runs")
}

#[test]
fn check_is_canonical_in_plain_rich_and_json_modes() {
    let home = std::env::temp_dir().join(format!("tj-check-cli-{}", std::process::id()));
    let plain = tj(&["--plain", "check"], &home);
    assert_eq!(plain.status.code(), Some(4));
    assert!(plain.stderr.is_empty());
    let text = String::from_utf8(plain.stdout).unwrap();
    assert!(text.contains("tj.version\tready\tinfo"));
    assert!(text.contains("harness.codex.support\tunsupported\terror"));

    let rich = tj(&["--no-color", "check"], &home);
    assert_eq!(rich.status.code(), Some(4));
    assert!(String::from_utf8(rich.stdout)
        .unwrap()
        .contains("Terminal Jarvis Diagnostics"));

    let json = tj(&["--json", "check"], &home);
    assert_eq!(json.status.code(), Some(4));
    assert!(json.stderr.is_empty());
    let document = String::from_utf8(json.stdout).unwrap();
    assert_eq!(document.lines().count(), 1);
    assert!(document
        .starts_with("{\"schema_version\":1,\"command\":\"check\",\"ok\":false,\"exit_code\":4"));
    assert!(document.contains("\"diagnostics\":["));
    assert!(!document.contains("\"text\":"));
}

#[test]
fn verbose_check_expands_per_harness_details() {
    let home = std::env::temp_dir().join(format!("tj-check-verbose-{}", std::process::id()));
    let concise = tj(&["--plain", "check"], &home);
    let verbose = tj(&["--plain", "check", "--verbose"], &home);
    assert_eq!(concise.status.code(), Some(4));
    assert_eq!(verbose.status.code(), Some(4));
    assert!(verbose.stdout.len() > concise.stdout.len());
    assert!(!String::from_utf8(concise.stdout)
        .unwrap()
        .contains("harness.codex.version"));
    assert!(String::from_utf8(verbose.stdout)
        .unwrap()
        .contains("harness.codex.version"));
}
