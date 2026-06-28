use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn temp_dir() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "terminal-jarvis-catalog-fallback-{}-{}",
        std::process::id(),
        TEMP_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn tj(args: &[&str], cwd: &PathBuf) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(args)
        .current_dir(cwd)
        .env("TERMINAL_JARVIS_HOME", cwd.join("home"))
        .env_remove("TERMINAL_JARVIS_CATALOG")
        .output()
        .expect("terminal-jarvis runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn list_works_without_filesystem_catalog() {
    let cwd = temp_dir();
    let output = tj(&["list"], &cwd);
    assert!(output.status.success());
    let body = stdout(&output);
    assert_eq!(body.lines().count(), 25);
    assert!(body.contains("codex - OpenAI coding agent CLI"));
}
