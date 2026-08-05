#[cfg(unix)]
mod unix {
    use std::fs;
    use std::process::{Command, Output};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

    fn tj(args: &[&str], home: &str, path: &str) -> Output {
        Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
            .arg("--plain")
            .args(args)
            .env("TERMINAL_JARVIS_HOME", home)
            .env("PATH", path)
            .output()
            .expect("terminal-jarvis runs")
    }

    fn temp_home() -> String {
        std::env::temp_dir()
            .join(format!(
                "terminal-jarvis-run-{}-{}",
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

    fn stderr(output: &Output) -> String {
        String::from_utf8_lossy(&output.stderr).to_string()
    }

    #[test]
    fn unknown_run_rows_do_not_launch_fake_tools() {
        let home = temp_home();
        let (bin, old_path) = fake_bin("opencode");
        let path = format!("{bin}:{old_path}");

        let launch = tj(&["run", "opencode"], &home, &path);
        assert_eq!(launch.status.code(), Some(4));
        assert!(stdout(&launch).is_empty());
        assert!(stderr(&launch).contains("unknown"));

        let prompt = tj(&["run", "opencode", "yo!", "fix", "this"], &home, &path);
        assert_eq!(prompt.status.code(), Some(4));
        assert!(stdout(&prompt).is_empty());
    }

    #[test]
    fn direct_tool_help_is_parsed_but_unknown_support_blocks_launch() {
        let home = temp_home();
        let (bin, old_path) = fake_bin("opencode");
        let path = format!("{bin}:{old_path}");

        let output = tj(&["opencode", "--help"], &home, &path);
        assert_eq!(output.status.code(), Some(4));
        assert!(stdout(&output).is_empty());
        assert!(stderr(&output).contains("opencode:ui is unknown"));
    }

    #[test]
    fn support_guard_precedes_missing_binary_diagnosis() {
        let home = temp_home();
        let empty = std::path::PathBuf::from(temp_home()).join("bin");
        fs::create_dir_all(&empty).unwrap();

        let output = tj(&["run", "opencode"], &home, &empty.to_string_lossy());

        assert_eq!(output.status.code(), Some(4));
        assert!(stderr(&output).contains("opencode:ui is unknown"));
    }
}
