//! Previews always work; consent is asked only when something would run.
use std::process::Command;

static BINARY: &str = env!("CARGO_BIN_EXE_terminal-jarvis");

fn run(args: &[&str]) -> (i32, String, String) {
    let home = std::env::temp_dir().join(format!("tj-previews-{}", std::process::id()));
    let output = Command::new(BINARY)
        .arg("--plain")
        .args(args)
        .env(
            "TERMINAL_JARVIS_CATALOG",
            crate::structs::fresh_catalog::root(),
        )
        .env("TERMINAL_JARVIS_HOME", &home)
        .output()
        .expect("terminal-jarvis runs");
    let text = |bytes: &[u8]| String::from_utf8_lossy(bytes).to_string();
    let code = output.status.code().unwrap_or(1);
    (code, text(&output.stdout), text(&output.stderr))
}

#[test]
fn an_unknown_installer_previews_without_consent() {
    let (code, stdout, stderr) = run(&["install", "claude", "--dry-run"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("https://claude.ai/install.sh"), "{stdout}");
}

#[test]
fn a_headless_unknown_installer_names_the_host_and_the_token() {
    let (code, stdout, stderr) = run(&["--no-input", "install", "claude"]);
    assert_eq!(code, 5, "{stderr}");
    assert!(stdout.is_empty());
    assert!(stderr.contains("claude.ai"), "{stderr}");
    assert!(stderr.contains("--confirm=download:claude"), "{stderr}");
}

#[test]
fn uninstall_dry_run_previews_the_removal_and_its_token() {
    let (code, stdout, stderr) = run(&["uninstall", "codex", "--dry-run"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(
        stdout.contains("npm uninstall -g @openai/codex"),
        "{stdout}"
    );
    assert!(stdout.contains("--confirm=uninstall:codex"), "{stdout}");
}

#[test]
fn a_preview_never_unlocks_a_stub() {
    let (code, stdout, stderr) = run(&["run", "claude", "models", "--dry-run"]);
    assert_eq!(code, 4, "{stdout}");
    assert!(stderr.contains(" is stub"), "{stderr}");
}

#[cfg(unix)]
#[test]
fn uninstall_dry_run_wins_over_a_matching_confirm_token() {
    let script = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";
    let fixture = crate::logic::cli_driver::Fixture::new("expected", "expected", script);
    let row = fixture.root.join("catalog/fixture/download/index.toml");
    let mut body = std::fs::read_to_string(&row).unwrap();
    body.push_str("uninstall_command = \"fixture-child\"\nuninstall_args = [\"uninstall\"]\n");
    std::fs::write(&row, body).unwrap();
    let args = ["--plain", "--no-input", "uninstall", "fixture"];
    let token = "--confirm=uninstall:fixture";
    let preview = fixture.run(&[&args[..], &[token, "--dry-run"]].concat());
    assert_eq!(preview.status.code(), Some(0), "{preview:?}");
    assert!(String::from_utf8_lossy(&preview.stdout).contains("fixture-child uninstall"));
    assert!(!fixture.spawned(), "a preview must never run the removal");
    // Control: the same token without --dry-run does reach the remover.
    fixture.run(&[&args[..], &[token]].concat());
    assert!(fixture.spawned(), "the fixture remover is observable");
}
