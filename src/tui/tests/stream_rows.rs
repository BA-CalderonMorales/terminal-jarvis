//! Stream row contract: a classified row becomes a labelled, role-tagged
//! line; anything else passes through untouched, so a gate note or an
//! advisory is never dropped or mislabelled.

use super::render;

#[test]
fn a_classified_row_becomes_a_labelled_line() {
    let info = render("INFO  Downloading thing");
    assert!(info.contains("info"), "the level is a label: {info}");
    assert!(info.contains("Downloading thing"), "{info}");

    let warn = render("WARN  deprecated flag");
    assert!(warn.contains("warn"), "{warn}");
    assert!(warn.contains("deprecated flag"), "{warn}");

    let err = render("ERROR it broke");
    assert!(err.contains("error"), "{err}");
    assert!(err.contains("it broke"), "{err}");
}

#[test]
fn an_unclassified_row_passes_through_untouched() {
    let note = "security scan (acceptance): passed";
    assert_eq!(render(note), note, "a gate note keeps its exact text");
    assert_eq!(render(""), "", "a blank row stays blank");
}
