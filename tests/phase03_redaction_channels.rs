use std::process::{Command, Output};

const SECRET: &str = "phase03-seeded-super-secret";

fn check(mode: &str, root: &std::path::Path) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    if !mode.is_empty() {
        command.arg(mode);
    }
    command
        .arg("check")
        .env("HOME", root.join("home"))
        .env("XDG_CONFIG_HOME", root.join("home/config"))
        .env("TERMINAL_JARVIS_HOME", root.join("home/tj"))
        .env("TERMINAL_JARVIS_CATALOG", root.join("missing-catalog"))
        .output()
        .expect("redaction probe runs")
}

#[test]
fn rich_plain_json_and_stderr_redact_seeded_catalog_paths() {
    let root =
        std::env::temp_dir().join(format!("terminal-jarvis-{SECRET}-{}", std::process::id()));
    std::fs::create_dir_all(root.join("home")).unwrap();
    for mode in ["", "--plain", "--json"] {
        let output = check(mode, &root);
        assert_eq!(output.status.code(), Some(3));
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(!stdout.contains(SECRET), "mode={mode}: {stdout}");
        assert!(!stderr.contains(SECRET), "mode={mode}: {stderr}");
        if mode == "--json" {
            assert!(stderr.is_empty());
            assert!(stdout.contains("$TMP/missing-catalog"));
        } else {
            assert!(stdout.is_empty());
            assert!(stderr.contains("$TMP/missing-catalog"));
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
