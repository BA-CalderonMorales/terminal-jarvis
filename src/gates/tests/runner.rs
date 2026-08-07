use super::*;

#[cfg(unix)]
fn write_gate(root: &Path, name: &str, binary: &str) {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        directory.join("index.toml"),
        format!(
            "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = []\ninstall_hint = \"install\"\n"
        ),
    )
    .unwrap();
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
    assert!(preflight(&home).is_ok());
    crate::gates::enable(&home, "block").unwrap();
    let error = preflight(&home).unwrap_err();
    assert!(error.contains("blocked harness execution (exit 1)"));
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_GATES", value);
    } else {
        std::env::remove_var("TERMINAL_JARVIS_GATES");
    }
    let _ = std::fs::remove_dir_all(root);
}
