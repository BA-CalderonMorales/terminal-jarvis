use super::*;
use std::fs;

#[test]
fn wrapper_path_requires_package_json() {
    let _g = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let base = std::env::temp_dir().join(format!("tjwrap-{}", std::process::id()));
    let bin = base.join("bin");
    fs::create_dir_all(&bin).unwrap();

    std::env::remove_var("TERMINAL_JARVIS_WRAPPER");
    assert!(wrapper_path().is_none());

    std::env::set_var("TERMINAL_JARVIS_WRAPPER", bin.join("terminal-jarvis"));
    assert!(wrapper_path().is_none());

    fs::write(base.join("package.json"), "{}").unwrap();
    assert!(wrapper_path().is_some());

    std::env::remove_var("TERMINAL_JARVIS_WRAPPER");
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn run_cmd_reports_success_output() {
    let (code, out) = run_cmd("echo", &["mutation-marker"]).expect("echo should run");
    assert_eq!(code, 0);
    assert!(
        out.contains("Terminal Jarvis updated"),
        "success message was: {out:?}"
    );
}

#[test]
fn run_cmd_reports_failure() {
    assert!(run_cmd("false", &[]).is_err(), "false should fail");
}

#[test]
fn homebrew_paths_cover_both_install_layouts() {
    assert!(homebrew_path("/opt/homebrew/bin/terminal-jarvis"));
    assert!(homebrew_path(
        "/usr/local/Cellar/terminal-jarvis/0.1/bin/tj"
    ));
    assert!(!homebrew_path("/usr/local/bin/terminal-jarvis"));
}
