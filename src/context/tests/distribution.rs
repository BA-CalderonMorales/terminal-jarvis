use super::*;

#[test]
fn raw_channels_are_normalized_without_passthrough_claims() {
    for raw in ["env", "source"] {
        assert_eq!(normalize(raw), Some("source"));
    }
    for raw in ["github-release", "github-release-cache", "npm"] {
        assert_eq!(normalize(raw), Some("npm"));
    }
    for raw in ["homebrew", "cargo", "direct"] {
        assert_eq!(normalize(raw), Some(raw));
    }
    assert_eq!(normalize("custom"), None);
}

#[test]
fn source_build_classifies_workspace_binaries_only() {
    let root = std::env::current_dir().unwrap();
    let root = root.to_string_lossy();
    assert!(source_build(&format!(
        "{root}/target/debug/terminal-jarvis"
    )));
    assert!(source_build(&format!("{root}/target/release/tj")));
    assert!(source_build(&format!(
        "{root}/target/llvm-cov-target/debug/tj"
    )));
    assert!(!source_build("/srv/target/debug/deps/lib-test"));
    assert!(!source_build("/srv/target/debug/terminal-jarvis"));
    assert!(!source_build("/home/caldo/.cargo/bin/terminal-jarvis"));
    assert!(!source_build(""));
}

fn with_env<T>(key: &str, value: Option<&str>, test: impl FnOnce() -> T) -> T {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let previous = std::env::var_os(key);
    match value {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
    let result = test();
    match previous {
        Some(value) => std::env::set_var(key, value),
        None => std::env::remove_var(key),
    }
    result
}

#[test]
fn channel_is_npm_only_from_a_non_blank_wrapper_variable() {
    fn clear_companions() {
        for key in [
            "TERMINAL_JARVIS_DISTRIBUTION",
            "TERMINAL_JARVIS_RELEASE_URL",
            "TERMINAL_JARVIS_CACHE",
        ] {
            std::env::remove_var(key);
        }
    }
    with_env(crate::context::constants::env::WRAPPER, None, || {
        clear_companions();
        assert_ne!(channel(), Some("npm"));
    });
    with_env(crate::context::constants::env::WRAPPER, Some("cli"), || {
        clear_companions();
        assert_eq!(channel(), Some("npm"));
    });
    with_env(
        crate::context::constants::env::WRAPPER,
        Some(" \t "),
        || {
            clear_companions();
            assert_ne!(channel(), Some("npm"));
        },
    );
    with_env(
        crate::context::constants::env::DISTRIBUTION,
        Some(" \t "),
        || {
            clear_companions();
            std::env::set_var(crate::context::constants::env::DISTRIBUTION, " \t ");
            assert_ne!(channel(), Some("unknown"));
        },
    )
}
