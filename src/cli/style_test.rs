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
    assert!(term_is_dumb(Some("dumb")));
    assert!(!term_is_dumb(Some("xterm")));
    assert!(!term_is_dumb(None));
}
