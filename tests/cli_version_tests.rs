use std::process::{Command, Output};
#[rustfmt::skip]
fn tj(args: &[&str]) -> Output { Command::new(env!("CARGO_BIN_EXE_terminal-jarvis")).args(args).output().expect("tj runs") }
#[rustfmt::skip]
fn se(o: &Output) -> String { String::from_utf8_lossy(&o.stderr).to_string() }
#[rustfmt::skip]
fn nocat() -> String { let p = std::env::temp_dir().join(format!("tj-nocat-{}", std::process::id())); p.to_string_lossy().to_string() }

#[test]
fn version_flags_do_not_require_catalog() {
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--version")
        .env("TERMINAL_JARVIS_CATALOG", nocat())
        .output()
        .unwrap();
    assert!(
        o.status.success() && String::from_utf8_lossy(&o.stdout).starts_with("terminal-jarvis ")
    );
}
#[test]
fn plain_version_omits_channel_without_distribution_env() {
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--version")
        .env_clear()
        .output()
        .unwrap();
    assert!(o.status.success());
    let b = String::from_utf8_lossy(&o.stdout);
    assert!(b.starts_with("terminal-jarvis ") && !b.contains('('));
}
#[test]
fn plain_version_reports_source_channel() {
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--version")
        .env_clear()
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
        .output()
        .unwrap();
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains("(source)"));
}
#[test]
fn plain_version_reports_npm_channel_for_wrapped_install() {
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--version")
        .env_clear()
        .env(
            "TERMINAL_JARVIS_WRAPPER",
            "/opt/node_modules/terminal-jarvis/bin/tj",
        )
        .output()
        .unwrap();
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains("(npm)"));
}
#[test]
fn verbose_version_reports_provenance_paths() {
    let o = tj(&["version", "--verbose"]);
    assert!(o.status.success());
    let b = String::from_utf8_lossy(&o.stdout);
    assert!(b.contains("binary:") && b.contains("release:") && b.contains("catalog:"));
}
#[test]
fn verbose_version_ignores_empty_wrapper_env_values() {
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["version", "--verbose"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "")
        .env("TERMINAL_JARVIS_RELEASE_URL", "")
        .env("TERMINAL_JARVIS_CACHE", "")
        .output()
        .unwrap();
    assert!(o.status.success());
    let b = String::from_utf8_lossy(&o.stdout);
    assert!(
        b.contains("distribution: unknown")
            && b.contains("release: https://github.com")
            && b.contains("cache: unavailable")
    );
}
#[test]
fn missing_catalog_error_names_catalog_path() {
    let o = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("list")
        .env("TERMINAL_JARVIS_CATALOG", nocat())
        .output()
        .unwrap();
    assert_eq!(o.status.code(), Some(2));
    assert!(
        se(&o).contains("harness catalog is missing at")
            && se(&o).contains("TERMINAL_JARVIS_CATALOG")
    );
}
