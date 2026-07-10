use super::*;

#[test]
fn configured_root_distinguishes_unset_empty_and_nonempty() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::remove_var("TERMINAL_JARVIS_GATES");
    assert!(!configured_root());
    std::env::set_var("TERMINAL_JARVIS_GATES", "");
    assert!(!configured_root());
    std::env::set_var("TERMINAL_JARVIS_GATES", "/missing/gates");
    assert!(configured_root());
    assert_eq!(
        load(Path::new("/definitely/missing/gates"))
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_GATES", value);
    } else {
        std::env::remove_var("TERMINAL_JARVIS_GATES");
    }
}
