#[cfg(unix)]
mod unix {
    use std::process::{Command, Output};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

    fn tj(args: &[&str], home: &str, path: Option<&str>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
        command
            .arg("--plain")
            .args(args)
            .env("TERMINAL_JARVIS_HOME", home);
        if let Some(path) = path {
            command.env("PATH", path);
        }
        command.output().expect("terminal-jarvis runs")
    }

    fn temp_home() -> String {
        std::env::temp_dir()
            .join(format!(
                "terminal-jarvis-compat-{}-{}",
                std::process::id(),
                TEMP_ID.fetch_add(1, Ordering::Relaxed)
            ))
            .to_string_lossy()
            .to_string()
    }

    fn stdout(output: &Output) -> String {
        String::from_utf8_lossy(&output.stdout).to_string()
    }
    fn stderr(output: &Output) -> String {
        String::from_utf8_lossy(&output.stderr).to_string()
    }

    #[test]
    fn compat_info_auth_config_cache_security_and_legacy_are_helpful() {
        let home = temp_home();
        let update = stdout(&tj(&["update"], &home, None));
        assert!(update.contains("updates are per harness"));
        assert!(update.contains("opencode: support=unknown"));
        assert!(!update.contains("npm update -g opencode-ai"));

        assert!(stdout(&tj(&["info", "opencode"], &home, None)).contains("OpenCode"));
        assert!(stdout(&tj(&["auth"], &home, None)).contains("credential manager"));
        assert!(stdout(&tj(&["auth", "help", "opencode"], &home, None)).contains("OPENCODE"));
        assert!(stdout(&tj(&["auth", "opencode"], &home, None)).contains("OPENCODE"));
        assert!(stdout(&tj(&["config", "show"], &home, None)).contains("active harness"));
        assert!(stdout(&tj(&["config", "path"], &home, None)).contains("catalog:"));
        let reset = tj(&["config", "reset"], &home, None);
        assert_eq!(reset.status.code(), Some(4));
        assert!(stderr(&reset).contains("guidance-only"));
        assert!(stdout(&tj(&["cache", "status"], &home, None)).contains("cache:"));
        for action in ["clear", "refresh"] {
            let cache = tj(&["cache", action], &home, None);
            assert_eq!(cache.status.code(), Some(4));
            assert!(stderr(&cache).contains("guidance-only"));
        }
        assert!(stdout(&tj(&["security"], &home, None)).contains("jules binary="));
        assert!(stdout(&tj(&["security", "status"], &home, None)).contains("jules binary="));
        assert!(stdout(&tj(&["security", "opencode"], &home, None)).contains("opencode:security"));
        assert!(stdout(&tj(&["security", "audit"], &home, None)).contains("audit summary:"));
        let removed = tj(&["templates"], &home, None);
        assert_eq!(removed.status.code(), Some(4));
        assert!(stderr(&removed).contains("removed"));
    }

    #[test]
    fn unverified_catalog_rows_fail_closed_before_catalog_commands() {
        let home = temp_home();
        for args in [
            &["install", "opencode"][..],
            &["update", "opencode"],
            &["run", "opencode"],
            &["run", "opencode", "headless", "yo"],
        ] {
            let output = tj(args, &home, None);
            assert_eq!(output.status.code(), Some(4));
            assert!(stdout(&output).is_empty());
            assert!(stderr(&output).contains("unknown"));
        }
    }
}
