#[cfg(unix)]
mod unix {
    use std::fs;
    use std::process::{Command, Output};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

    fn tj(args: &[&str], home: &str, path: Option<&str>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
        command.args(args).env("TERMINAL_JARVIS_HOME", home);
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

    fn fake_bin(name: &str) -> (String, String) {
        use std::os::unix::fs::PermissionsExt;

        let dir = std::path::PathBuf::from(temp_home()).join("bin");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(name);
        fs::write(&path, "#!/usr/bin/env sh\nprintf '%s\\n' \"$*\"\n").unwrap();
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).unwrap();
        let old_path = std::env::var("PATH").unwrap_or_default();
        (dir.to_string_lossy().to_string(), old_path)
    }

    fn stdout(output: &Output) -> String {
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    #[test]
    fn compat_info_auth_config_cache_security_and_legacy_are_helpful() {
        let home = temp_home();
        let update = stdout(&tj(&["update"], &home, None));
        assert!(update.contains("updates are per harness"));
        assert!(update.contains("opencode: npm update -g opencode-ai"));

        assert!(stdout(&tj(&["info", "opencode"], &home, None)).contains("OpenCode"));
        assert!(stdout(&tj(&["auth"], &home, None)).contains("credential manager"));
        assert!(stdout(&tj(&["auth", "help", "opencode"], &home, None)).contains("OPENCODE"));
        assert!(stdout(&tj(&["auth", "opencode"], &home, None)).contains("OPENCODE"));
        assert!(stdout(&tj(&["config", "show"], &home, None)).contains("active harness"));
        assert!(stdout(&tj(&["config", "path"], &home, None)).contains("catalog:"));
        assert!(stdout(&tj(&["config", "reset"], &home, None)).contains("not automatic"));
        assert!(stdout(&tj(&["cache", "status"], &home, None)).contains("cache:"));
        assert!(stdout(&tj(&["cache", "clear"], &home, None)).contains("cache clear:"));
        assert!(stdout(&tj(&["cache", "refresh"], &home, None)).contains("cache refresh:"));
        assert!(stdout(&tj(&["security"], &home, None)).contains("jules binary="));
        assert!(stdout(&tj(&["security", "status"], &home, None)).contains("jules binary="));
        assert!(stdout(&tj(&["security", "opencode"], &home, None)).contains("opencode:security"));
        assert!(stdout(&tj(&["security", "audit"], &home, None)).contains("audit summary:"));
        assert!(stdout(&tj(&["templates"], &home, None)).contains("removed"));
    }

    #[test]
    fn install_update_and_active_run_forms_use_catalog_commands() {
        let home = temp_home();
        let (bin, old_path) = fake_bin("npm");
        let (agent_bin, _) = fake_bin("opencode");
        let path = format!("{agent_bin}:{bin}:{old_path}");

        assert_eq!(
            stdout(&tj(&["install", "opencode"], &home, Some(&path))),
            "install -g opencode-ai@latest\n"
        );
        assert_eq!(
            stdout(&tj(&["update", "opencode"], &home, Some(&path))),
            "update -g opencode-ai\n"
        );
        assert!(tj(&["use", "opencode"], &home, Some(&path))
            .status
            .success());
        assert_eq!(stdout(&tj(&["run"], &home, Some(&path))), "\n");
        assert_eq!(
            stdout(&tj(&["run", "headless", "yo"], &home, Some(&path))),
            "run yo\n"
        );
        assert_eq!(stdout(&tj(&["run", "yo"], &home, Some(&path))), "run yo\n");
    }
}
