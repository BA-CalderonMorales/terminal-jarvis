use super::*;

#[test]
fn config_selection_tolerates_whitespace_comments_and_malformed() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let previous = std::env::var_os("TERMINAL_JARVIS_GATE");
    std::env::remove_var("TERMINAL_JARVIS_GATE");
    let home = std::env::temp_dir().join(format!("tj-state-file-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    std::fs::write(
        home.join("gate.toml"),
        "# comment\nenabled = \"unclosed\nenabled = \"trivy\" # inline note\n",
    )
    .unwrap();
    let selection = selected(&home).unwrap().unwrap();
    assert_eq!(selection.name, "trivy");
    assert_eq!(selection.source, "config");
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATE", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATE"),
    }
    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn environment_selection_handles_empty_off_and_named_values() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let home = std::env::temp_dir().join(format!("tj-state-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    let previous = std::env::var_os("TERMINAL_JARVIS_GATE");
    std::env::set_var("TERMINAL_JARVIS_GATE", "");
    assert!(selected(&home).unwrap().is_none());
    std::env::set_var("TERMINAL_JARVIS_GATE", "off");
    assert!(selected(&home).unwrap().is_none());
    std::env::set_var("TERMINAL_JARVIS_GATE", "trivy");
    let selection = selected(&home).unwrap().unwrap();
    assert_eq!(selection.name, "trivy");
    assert_eq!(selection.source, "environment");
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_GATE", value);
    } else {
        std::env::remove_var("TERMINAL_JARVIS_GATE");
    }
}
