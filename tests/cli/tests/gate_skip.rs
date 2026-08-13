#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use std::io::{IsTerminal, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";
fn self_killing_gate(fixture: &Fixture) {
    let gates = fixture.root.join("gates");
    std::fs::create_dir_all(&gates).unwrap();
    std::fs::write(
        gates.join("acceptance/index.toml"),
        "name = \"acceptance\"\ndisplay = \"Killer gate\"\ndescription = \"self-kills on purpose\"\nbinary = \"fixture-gate-kill\"\nargs = []\ninstall_hint = \"fixture\"\n",
    )
    .unwrap();
    let killer = fixture.root.join("bin").join("fixture-gate-kill");
    std::fs::write(&killer, "#!/bin/sh\nkill -TERM $$\n").unwrap();
    let mut permissions = std::fs::metadata(&killer).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&killer, permissions).unwrap();
}

fn envs(fixture: &Fixture) -> Vec<(&str, String)> {
    vec![
        (
            "PATH",
            format!(
                "{}:{}",
                fixture.root.join("bin").display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        ),
        (
            "TERMINAL_JARVIS_CATALOG",
            fixture.root.join("catalog").display().to_string(),
        ),
        ("TERMINAL_JARVIS_GATE", "acceptance".to_string()),
        (
            "TERMINAL_JARVIS_GATES",
            fixture.root.join("gates").display().to_string(),
        ),
        (
            "TERMINAL_JARVIS_HOME",
            fixture.root.join("home").display().to_string(),
        ),
        (
            "TJ_FIXTURE_MARKER",
            fixture.marker_path().display().to_string(),
        ),
    ]
}

#[test]
fn interrupted_scan_with_no_input_aborts_and_never_downloads() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    self_killing_gate(&fixture);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args([
            "--plain",
            "--no-input",
            "--confirm=download:fixture",
            "install",
            "fixture",
        ])
        .envs(envs(&fixture))
        .output()
        .expect("terminal-jarvis runs");
    assert_eq!(output.status.code(), Some(5), "safety refusal");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("was interrupted (Ctrl+C)"), "{stderr}");
    assert!(
        !fixture.spawned(),
        "the interrupted scan must never download the package"
    );
}

#[test]
fn interrupted_scan_on_a_terminal_can_be_skipped_and_the_install_proceeds() {
    if !std::io::stdin().is_terminal() {
        return;
    }
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    self_killing_gate(&fixture);
    let mut child = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "install", "fixture"])
        .envs(envs(&fixture))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("terminal-jarvis spawns");
    child.stdin.as_mut().unwrap().write_all(b"y\n").unwrap();
    let output = child.wait_with_output().expect("wait");
    assert_eq!(output.status.code(), Some(0), "skipped install succeeds");
    assert!(
        fixture.spawned(),
        "skipping the scan must let the install proceed"
    );
}
