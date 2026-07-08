use std::fs;
use std::process::{Command, Output};

#[rustfmt::skip]
fn cmd(args: &[&str]) -> Output { Command::new(env!("CARGO_BIN_EXE_terminal-jarvis")).args(args).output().unwrap() }
#[rustfmt::skip]
fn so(o: &Output) -> String { String::from_utf8_lossy(&o.stdout).to_string() }
#[rustfmt::skip]
fn se(o: &Output) -> String { String::from_utf8_lossy(&o.stderr).to_string() }

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
    assert!(so(&output).starts_with("terminal-jarvis "));
}

#[test]
fn verbose_version_reports_provenance_paths() {
    let o = cmd(&["version", "--verbose"]);
    assert!(o.status.success());
    let body = so(&o);
    assert!(body.contains("binary:"));
    assert!(body.contains("release:"));
    assert!(body.contains("catalog:"));
}

#[test]
fn verbose_version_ignores_empty_wrapper_env_values() {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["version", "--verbose"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "")
        .env("TERMINAL_JARVIS_RELEASE_URL", "")
        .env("TERMINAL_JARVIS_CACHE", "")
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    let body = so(&output);
    assert!(body.contains("distribution: unknown"));
    assert!(body.contains("release: https://github.com/BA-CalderonMorales"));
    assert!(body.contains("cache: unavailable"));
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
    let error = se(&output);
    assert!(error.contains("harness catalog is missing at"));
    assert!(error.contains("TERMINAL_JARVIS_CATALOG"));
}

#[test]
#[rustfmt::skip]
fn garbled_session_warns_on_stderr() {
    let home = std::env::temp_dir().join(format!("tj-garbled-{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).unwrap();
    fs::write(home.join("session.toml"), "active_harness = \"codex\n").unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis")).arg("current").env("TERMINAL_JARVIS_HOME", &home).output().unwrap();
    assert!(o.status.success());
    assert!(se(&o).contains("warning"));
    assert!(se(&o).contains("session.toml"));
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
    assert!(se(&output).contains("stream did not contain valid UTF-8"));
}
