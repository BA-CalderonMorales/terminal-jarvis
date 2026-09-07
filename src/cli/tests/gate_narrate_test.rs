use super::*;

#[test]
fn gate_scans_narrate_only_outside_the_viewport() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    crate::tui::screen::resume(true);
    assert!(!narrate());
    crate::tui::screen::suspend();
    assert!(narrate());
}
