use std::fs;
use std::process::{Command, Output};
#[rustfmt::skip]
fn se(o: &Output) -> String { String::from_utf8_lossy(&o.stderr).to_string() }
#[rustfmt::skip]
fn home(name: &str) -> std::path::PathBuf { let h = std::env::temp_dir().join(format!("{}-{}", name, std::process::id())); let _ = fs::remove_dir_all(&h); fs::create_dir_all(&h).unwrap(); h }

#[test]
fn valid_session_skips_warning() {
    let h = home("tj-v");
    fs::write(h.join("session.toml"), "active_harness = \"codex\"").unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("current")
        .env("TERMINAL_JARVIS_HOME", &h)
        .output()
        .unwrap();
    assert!(o.status.success() && !se(&o).contains("session.toml could not be parsed"));
}
#[test]
fn garbled_session_warns_on_stderr() {
    let h = home("tj-g");
    fs::write(h.join("session.toml"), "active_harness = \"codex\n").unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("current")
        .env("TERMINAL_JARVIS_HOME", &h)
        .output()
        .unwrap();
    assert!(o.status.success() && se(&o).contains("session.toml could not be parsed"));
}
#[test]
fn corrupt_session_error_reaches_stderr() {
    let h = home("tj-c");
    fs::write(h.join("session.toml"), [0xff_u8]).unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("current")
        .env("TERMINAL_JARVIS_HOME", h)
        .output()
        .unwrap();
    assert_eq!(o.status.code(), Some(3));
    assert!(se(&o).contains("stream did not contain valid UTF-8"));
}
