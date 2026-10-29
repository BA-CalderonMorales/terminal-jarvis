use super::*;
use crate::catalog::structs::sunset;

fn facade(extra: &str) -> Fields {
    let base = [
        "name = \"tool\"",
        "display = \"Tool\"",
        "description = \"test\"",
        "binary = \"tool\"",
        "env_mode = \"none\"",
        "env = []",
    ]
    .join("\n");
    crate::catalog::parser::parse(&format!("{base}\n{extra}")).expect("valid metadata")
}

#[test]
fn a_retired_tool_carries_the_date_the_story_and_its_successor() {
    let fields = facade(
        "sunset_at = \"2026-06-18\"\nsunset_note = \"vendor retired it\"\nsunset_replaced_by = \"successor\"",
    );
    let sunset = sunset::parse(&fields).unwrap().expect("retired");
    assert_eq!(sunset.at, "2026-06-18");
    assert_eq!(sunset.note, "vendor retired it");
    assert_eq!(sunset.successor(), "successor");
}

#[test]
fn a_maintained_tool_has_no_retirement_record() {
    assert_eq!(sunset::parse(&facade("")).unwrap(), None);
}

#[test]
fn a_successor_without_a_retirement_is_an_incomplete_story() {
    let error = sunset::parse(&facade("sunset_replaced_by = \"successor\"")).unwrap_err();
    assert!(error.contains("must appear together"), "{error}");
}

#[test]
fn a_retirement_date_must_be_a_calendar_day() {
    for bad in ["2026-06", "2026-13-01", "yesterday"] {
        let fields = facade(&format!(
            "sunset_at = \"{bad}\"\nsunset_note = \"vendor retired it\""
        ));
        let error = sunset::parse(&fields).unwrap_err();
        assert!(error.contains("YYYY-MM-DD"), "{bad}: {error}");
    }
}

#[test]
fn a_retirement_must_say_what_happened() {
    let fields = facade("sunset_at = \"2026-06-18\"\nsunset_note = \"   \"");
    let error = sunset::parse(&fields).unwrap_err();
    assert!(error.contains("say what happened"), "{error}");
}
