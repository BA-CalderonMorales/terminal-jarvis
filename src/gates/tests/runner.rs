use super::*;

#[cfg(unix)]
fn write_gate(root: &Path, name: &str, binary: &str) {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let body = "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = []\ninstall_hint = \"install\"\n";
    std::fs::write(
        directory.join("index.toml"),
        body.replace("{name}", name).replace("{binary}", binary),
    )
    .unwrap();
}
#[cfg(unix)]
fn restore_gates_env(previous: Option<std::ffi::OsString>) {
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATES", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATES"),
    }
}
#[cfg(unix)]
#[test]
fn block_summary_drops_info_lines_and_keeps_the_signal_tail() {
    let verbatim = "2026 INFO [vuln] enabled\n2026 WARN spam\n2026 INFO secret\n2026 FATAL fs scan error: stat file: no such file\n";
    assert_eq!(
        block_summary(verbatim),
        "2026 WARN spam\n2026 FATAL fs scan error: stat file: no such file"
    );
    assert_eq!(
        block_summary("2026 INFO [vuln] enabled\n"),
        "2026 INFO [vuln] enabled"
    );
}
#[cfg(unix)]
#[test]
fn preflight_accepts_success_and_reports_blocking_exit() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let root = std::env::temp_dir().join(format!("tj-preflight-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "pass", "true");
    write_gate(&catalog, "block", "false");
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "pass").unwrap();
    assert!(preflight(&home, true).is_ok());
    crate::gates::enable(&home, "block").unwrap();
    let error = preflight(&home, true).unwrap_err();
    assert!(error.contains("blocked harness execution (exit 1)"));
    restore_gates_env(previous);
    let _ = std::fs::remove_dir_all(root);
}
#[cfg(unix)]
#[test]
fn run_streams_gate_output_and_captures_it() {
    let script = "#!/bin/sh\nprintf 'streamed-1\\n'\nprintf 'streamed-2\\n' >&2\n";
    let dir = std::env::temp_dir().join(format!("tj-gate-stream-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let bin = dir.join("scan");
    std::fs::write(&bin, script).unwrap();
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(&bin).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&bin, permissions).unwrap();
    let gate = Gate {
        name: "scan".to_string(),
        display: "Scan".to_string(),
        description: "test".to_string(),
        binary: bin.to_string_lossy().into_owned(),
        args: vec![],
        install_hint: "install".to_string(),
    };
    let (code, body) = run(&gate, true).unwrap();
    assert_eq!(code, 0);
    assert!(body.contains("streamed-1"));
    assert!(body.contains("streamed-2"));
    let _ = std::fs::remove_dir_all(&dir);
}
#[cfg(unix)]
#[test]
fn preflight_warns_and_continues_when_binary_is_missing() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let root = std::env::temp_dir().join(format!("tj-preflight-missing-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "phantom", "definitely-not-a-real-binary-xyz");
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "phantom").unwrap();
    assert!(preflight(&home, true).is_ok());
    restore_gates_env(previous);
    let _ = std::fs::remove_dir_all(root);
}
