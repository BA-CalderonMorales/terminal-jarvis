use std::process::Command;

#[test]
fn dry_run_reports_source_update_without_loading_the_catalog() {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "--update", "--dry-run"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
        .env("TERMINAL_JARVIS_CATALOG", "/missing/catalog")
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "terminal-jarvis update plan: cargo install terminal-jarvis\n"
    );
}

#[test]
fn dry_run_reports_npm_update_for_wrapped_distributions() {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "--update", "--dry-run"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "github-release-cache")
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("npm install -g terminal-jarvis@latest")
    );
}
