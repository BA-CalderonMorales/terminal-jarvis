use super::*;

#[test]
fn restore_reinstates_the_previous_options() {
    let original = set(false, false);
    let previous = set(true, true);
    assert!(plain());
    restore(previous);
    assert!(!plain());
    restore(original);
}

#[test]
fn labels_and_plain_banners_preserve_content() {
    let previous = set(true, true);
    assert_eq!(label("marker"), "marker");
    assert_eq!(banner("Title", "Subtitle"), "Title\nSubtitle\n\n");
    restore(previous);
}

#[test]
fn color_requires_every_enabling_condition() {
    assert!(color_enabled_for(true, false, false, false));
    assert!(!color_enabled_for(false, false, false, false));
    assert!(!color_enabled_for(true, true, false, false));
    assert!(!color_enabled_for(true, false, true, false));
    assert!(!color_enabled_for(true, false, false, true));
}

#[test]
fn diagnostics_decisions_track_terminals_and_env() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|lock| lock.into_inner());
    let previous = set(false, false);
    let term = std::env::var_os("TERM");
    std::env::remove_var("NO_COLOR");
    std::env::set_var("TERM", "xterm");
    let (out_term, err_term, color) = diagnostic_decisions();
    assert_eq!(out_term, std::io::stdout().is_terminal());
    assert_eq!(err_term, std::io::stderr().is_terminal());
    assert_eq!(color, out_term);
    std::env::set_var("TERM", "dumb");
    assert!(!diagnostic_decisions().2);
    if let Some(value) = term {
        std::env::set_var("TERM", value);
    } else {
        std::env::remove_var("TERM");
    }
    restore(previous);
}
