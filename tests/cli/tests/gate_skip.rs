#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use crate::logic::pty;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";
const INSTALL: [&str; 3] = ["--plain", "install", "fixture"];
const NO_INPUT: [&str; 5] = [
    "--plain",
    "--no-input",
    "--confirm=download:fixture",
    "install",
    "fixture",
];
fn killed_fixture() -> Fixture {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let gates = fixture.root.join("gates");
    std::fs::create_dir_all(gates.join("acceptance")).unwrap();
    std::fs::write(
        gates.join("acceptance/index.toml"),
        "name = \"acceptance\"\ndisplay = \"Killer gate\"\ndescription = \"self-kills on purpose\"\nbinary = \"fixture-gate-kill\"\nargs = []\ninstall_hint = \"fixture\"\n",
    )
    .unwrap();
    let killer = fixture.root.join("bin").join("fixture-gate-kill");
    std::fs::write(&killer, "#!/bin/sh\nkill -TERM $$\n").unwrap();
    std::fs::set_permissions(&killer, std::fs::Permissions::from_mode(0o755)).unwrap();
    fixture
}

fn envs(fixture: &Fixture) -> Vec<(&'static str, String)> {
    let root = |path: std::path::PathBuf| path.display().to_string();
    let path = format!(
        "{}:{}",
        root(fixture.root.join("bin")),
        std::env::var("PATH").unwrap_or_default()
    );
    vec![
        ("PATH", path),
        (
            "TERMINAL_JARVIS_CATALOG",
            root(fixture.root.join("catalog")),
        ),
        ("TERMINAL_JARVIS_GATE", "acceptance".to_string()),
        ("TERMINAL_JARVIS_GATES", root(fixture.root.join("gates"))),
        ("TERMINAL_JARVIS_HOME", root(fixture.root.join("home"))),
        ("TJ_FIXTURE_MARKER", root(fixture.marker_path().into())),
    ]
}

#[test]
fn interrupted_scan_with_no_input_aborts_and_never_downloads() {
    let fixture = killed_fixture();
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(NO_INPUT)
        .envs(envs(&fixture))
        .output()
        .expect("terminal-jarvis runs");
    assert_eq!(output.status.code(), Some(5), "safety refusal");
    assert!(String::from_utf8_lossy(&output.stderr).contains("was interrupted (Ctrl+C)"));
    assert!(!fixture.spawned(), "must not download");
}

#[test]
fn no_input_on_a_terminal_never_opens_the_skip_prompt() {
    let fixture = killed_fixture();
    let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    command.args(NO_INPUT).envs(envs(&fixture));
    let (status, bytes) = pty::run_pty_input(command, b"y\n");
    let screen = String::from_utf8_lossy(&bytes);
    assert_eq!(status.code(), Some(5), "no-input still refuses");
    assert!(!screen.contains("Skip the scan"));
    assert!(!fixture.spawned(), "nothing downloads");
}

#[test]
fn an_interrupted_scan_can_be_skipped_and_the_install_proceeds() {
    let fixture = killed_fixture();
    let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    command.args(INSTALL).envs(envs(&fixture));
    let (status, _) = pty::run_pty_probe(
        command,
        &[
            (
                b"",
                Some(b"Continue with download:fixture? [y/N]".as_slice()),
            ),
            (b"y\n", None),
            (
                b"",
                Some(b"Skip the scan and continue with download:fixture? [y/N]".as_slice()),
            ),
            (b"y\n", None),
        ],
    );
    assert_eq!(status.code(), Some(0), "skipped install succeeds");
    assert!(fixture.spawned());
}
