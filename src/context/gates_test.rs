use super::*;

#[test]
fn candidates_include_real_gate_locations() {
    let candidates = candidates();
    assert!(!candidates.is_empty());
    assert!(candidates.iter().all(|path| path.ends_with("gates")));
}

#[test]
fn configured_root_ignores_empty_values() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", "");
    assert!(!gates_root().as_os_str().is_empty());
    std::env::set_var("TERMINAL_JARVIS_GATES", "/custom/gates");
    assert_eq!(gates_root(), PathBuf::from("/custom/gates"));
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_GATES", value);
    } else {
        std::env::remove_var("TERMINAL_JARVIS_GATES");
    }
}
