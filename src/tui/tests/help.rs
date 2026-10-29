#[test]
fn help_advertises_the_keys_the_surface_honours() {
    let text = super::text(120);
    for key in ["tab completes", "q quits", "? help"] {
        assert!(text.contains(key), "help hides how to reach `{key}`");
    }
}
