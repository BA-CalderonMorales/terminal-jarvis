use super::super::{experimental, presentation_args};

#[test]
fn presentation_flags_are_removed_and_accumulated() {
    let (args, plain, no_color) = presentation_args(["tj", "--plain", "--no-color", "list"]);
    assert_eq!(args, ["tj", "list"]);
    assert!(plain);
    assert!(no_color);
    let (_, plain, no_color) = presentation_args(["tj", "--plain", "list"]);
    assert!(plain);
    assert!(!no_color);
    let (_, plain, no_color) = presentation_args(["tj", "--no-color", "list"]);
    assert!(!plain);
    assert!(no_color);
}

#[test]
fn experimental_rejects_unknown_actions() {
    let words = ["unknown".to_string()];
    let error = experimental::run(&words, &[], std::path::Path::new("/missing")).unwrap_err();
    assert_eq!(error, "usage: terminal-jarvis experimental dashboard");
}
