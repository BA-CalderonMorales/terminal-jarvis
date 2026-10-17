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
    let (command, args) = crate::cli::logic::test_support::succeed();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let (code, out) = run_cmd(&command, &args).expect("fixture should run");
    assert_eq!(code, 0);
    assert!(
        out.contains("Terminal Jarvis updated"),
        "success message was: {out:?}"
    );
}

#[test]
fn run_cmd_reports_failure() {
    let (command, args) = crate::cli::logic::test_support::fail(1);
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    assert!(run_cmd(&command, &args).is_err(), "fixture should fail");
}

#[test]
fn route_names_follow_the_distribution_channel() {
    let _g = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("TERMINAL_JARVIS_WRAPPER");
    for (channel, expected) in [
        ("homebrew", "homebrew"),
        ("cargo", "cargo"),
        ("npm", "npm"),
        ("direct", "direct"),
        ("edge", "unknown"),
    ] {
        std::env::set_var("TERMINAL_JARVIS_DISTRIBUTION", channel);
        assert_eq!(route_name(), expected);
        assert_eq!(route::name(), expected);
    }
    std::env::remove_var("TERMINAL_JARVIS_DISTRIBUTION");
}

#[test]
fn homebrew_paths_cover_both_install_layouts() {
    assert!(homebrew_path("/opt/homebrew/bin/terminal-jarvis"));
    assert!(homebrew_path(
        "/usr/local/Cellar/terminal-jarvis/0.1/bin/tj"
    ));
    assert!(!homebrew_path("/usr/local/bin/terminal-jarvis"));
}
