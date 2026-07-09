use super::*;
use std::path::Path;

fn with_env<F: FnOnce() -> T, T>(f: F) -> T {
    let _g = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    f()
}

fn clear() {
    std::env::remove_var("TERMINAL_JARVIS_DISTRIBUTION");
    std::env::remove_var("TERMINAL_JARVIS_WRAPPER");
    std::env::remove_var("TERMINAL_JARVIS_RELEASE_URL");
    std::env::remove_var("TERMINAL_JARVIS_CACHE");
}

#[test]
fn plain_version_no_channel() {
    with_env(|| {
        clear();
        let out = text(false, Path::new("/cat"), Path::new("/home"));
        assert!(out.starts_with("terminal-jarvis "));
        assert!(!out.contains('('));
    });
}
#[test]
fn source_channel_suffix() {
    with_env(|| {
        clear();
        std::env::set_var("TERMINAL_JARVIS_DISTRIBUTION", "source");
        let out = text(false, Path::new("/cat"), Path::new("/home"));
        assert!(out.contains(" (source)"));
    });
}
#[test]
fn npm_channel_from_github_release() {
    with_env(|| {
        clear();
        std::env::set_var("TERMINAL_JARVIS_DISTRIBUTION", "github-release-cache");
        let out = text(false, Path::new("/cat"), Path::new("/home"));
        assert!(out.contains(" (npm)"));
    });
}
#[test]
fn passthrough_channel_suffix() {
    with_env(|| {
        clear();
        std::env::set_var("TERMINAL_JARVIS_DISTRIBUTION", "custom");
        let out = text(false, Path::new("/cat"), Path::new("/home"));
        assert!(out.contains(" (custom)"));
    });
}
#[test]
fn verbose_text_reports_fields() {
    with_env(|| {
        clear();
        std::env::set_var("TERMINAL_JARVIS_DISTRIBUTION", "source");
        std::env::set_var("TERMINAL_JARVIS_RELEASE_URL", "https://example/release");
        std::env::set_var("TERMINAL_JARVIS_CACHE", "/my/cache");
        let out = text(true, Path::new("/cat"), Path::new("/home"));
        assert!(out.contains("binary:"));
        assert!(out.contains("distribution: source"));
        assert!(out.contains("release: https://example/release"));
        assert!(out.contains("cache: /my/cache"));
        assert!(out.contains("catalog: /cat"));
        assert!(out.contains("home: /home"));
    });
}
#[test]
fn homebrew_path_detection() {
    assert_eq!(
        homebrew_path("/opt/homebrew/bin/tj"),
        Some("homebrew".to_string())
    );
    assert_eq!(
        homebrew_path("/usr/local/Cellar/tj"),
        Some("homebrew".to_string())
    );
    assert_eq!(homebrew_path("/usr/local/bin/tj"), None);
}

#[test]
fn verbose_text_defaults() {
    with_env(|| {
        clear();
        let out = text(true, Path::new("/cat"), Path::new("/home"));
        assert!(out.contains("distribution: unknown"));
        assert!(out.contains("cache: unavailable"));
        assert!(!out.contains("wrapper:"));
    });
}
