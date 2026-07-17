use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "phase02_fixture/catalog.rs"]
mod catalog;

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn home() -> String {
    std::env::temp_dir()
        .join(format!(
            "terminal-jarvis-gate-{}-{}",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ))
        .to_string_lossy()
        .to_string()
}

fn tj(args: &[&str], home: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--plain")
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .output()
        .expect("terminal-jarvis runs")
}

#[test]
fn gate_is_disabled_by_default_and_can_be_enabled() {
    let home = home();
    let status = tj(&["gate", "status"], &home);
    assert!(status.status.success());
    assert!(String::from_utf8_lossy(&status.stdout).contains("gate: disabled"));
    assert!(tj(&["gate", "enable", "trivy"], &home).status.success());
    let enabled = tj(&["gate", "status"], &home);
    assert!(String::from_utf8_lossy(&enabled.stdout).contains("gate: trivy (config)"));
}

#[test]
fn enabled_missing_trivy_blocks_harness_execution_with_guidance() {
    let root = home();
    let home = format!("{root}/home");
    let catalog_root = std::path::Path::new(&root).join("catalog");
    catalog::write(&catalog_root, "expected", "expected");
    assert!(tj(&["gate", "enable", "trivy"], &home).status.success());
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "run", "fixture", "headless"])
        .env("TERMINAL_JARVIS_HOME", home)
        .env("TERMINAL_JARVIS_CATALOG", catalog_root)
        .env("PATH", "")
        .output()
        .expect("terminal-jarvis runs");
    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("optional gate 'trivy' is enabled"));
    assert!(stderr.contains("trivy.dev/docs/latest/getting-started/installation"));
}
