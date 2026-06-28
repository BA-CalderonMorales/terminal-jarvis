use std::fs;
use std::process::{Command, Output};

fn tj(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(args)
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
fn version_flags_do_not_require_catalog() {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--version")
        .env(
            "TERMINAL_JARVIS_CATALOG",
            "/tmp/terminal-jarvis-missing-catalog",
        )
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    assert!(stdout(&output).starts_with("terminal-jarvis "));
}

#[test]
fn verbose_version_reports_provenance_paths() {
    let output = tj(&["version", "--verbose"]);
    assert!(output.status.success());
    let body = stdout(&output);
    assert!(body.contains("binary:"));
    assert!(body.contains("release:"));
    assert!(body.contains("catalog:"));
}

#[test]
fn missing_catalog_error_names_catalog_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("list")
        .env(
            "TERMINAL_JARVIS_CATALOG",
            "/tmp/terminal-jarvis-missing-catalog",
        )
        .output()
        .expect("terminal-jarvis runs");
    assert_eq!(output.status.code(), Some(2));
    let error = stderr(&output);
    assert!(error.contains("harness catalog is missing at"));
    assert!(error.contains("TERMINAL_JARVIS_CATALOG"));
}

#[test]
fn corrupt_session_error_reaches_stderr() {
    let home = std::env::temp_dir().join(format!(
        "terminal-jarvis-corrupt-session-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).unwrap();
    fs::write(home.join("session.toml"), [0xff_u8]).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("current")
        .env("TERMINAL_JARVIS_HOME", home)
        .output()
        .expect("terminal-jarvis runs");

    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).contains("stream did not contain valid UTF-8"));
}
