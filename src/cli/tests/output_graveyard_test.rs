use super::*;
use crate::contracts::{EnvMode, Harness};
fn harness(name: &str, sunset: Option<Sunset>) -> Harness {
    Harness {
        name: name.into(),
        display: "Sunset Tool".into(),
        description: "probe".into(),
        binary: "tj-sunset-probe-missing".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![],
        sunset,
    }
}

fn retirement(at: &str) -> Sunset {
    Sunset {
        at: at.into(),
        note: "the vendor retired it".into(),
        replaced_by: Some("successor".into()),
    }
}

/// A day well after every date these tests use, so "already happened" is
/// unambiguous and the assertion does not drift with the calendar.
const TODAY: &str = "2030-01-01";

#[test]
fn a_retired_harness_reads_as_a_dated_retirement_in_plain_form() {
    let shown = [harness("retired", Some(retirement("2026-06-18")))];
    let refs = shown.iter().collect::<Vec<_>>();
    let text = plain(&refs, TODAY);
    assert!(text.contains("state=retired"), "{text}");
    assert!(text.contains("retired=2026-06-18"), "{text}");
    assert!(text.contains("replaced_by=successor"), "{text}");
    assert!(text.contains("note=the vendor retired it"), "{text}");
}

#[test]
fn an_announced_retirement_is_scheduled_not_retired() {
    // A vendor names a shutdown date before it happens, and the tool keeps
    // working until then. Claiming it is already retired makes the reader
    // distrust every other count on the screen.
    let shown = [harness("leaving", Some(retirement("2026-12-31")))];
    let refs = shown.iter().collect::<Vec<_>>();
    let text = plain(&refs, "2026-11-15");
    assert!(text.contains("state=scheduled"), "{text}");
    assert!(!text.contains("state=retired"), "{text}");
}

#[test]
fn a_maintained_only_catalog_has_no_graveyard_rows() {
    let shown = [harness("alive", None)];
    let refs = shown.iter().collect::<Vec<_>>();
    assert!(plain(&refs, TODAY).is_empty());
}

#[test]
fn a_retirement_without_a_successor_says_so() {
    let mut sunset = retirement("2026-06-18");
    sunset.replaced_by = None;
    let shown = [harness("retired", Some(sunset))];
    let refs = shown.iter().collect::<Vec<_>>();
    assert!(plain(&refs, TODAY).contains("replaced_by=no named successor"));
}

#[test]
fn the_heading_counts_retired_and_retiring_apart() {
    let shown = [
        harness("gone", Some(retirement("2026-06-18"))),
        harness("leaving", Some(retirement("2026-12-31"))),
    ];
    let refs = shown.iter().collect::<Vec<_>>();
    let heading = headline(&refs, 25, "2026-11-15");
    assert!(heading.contains("1 retired"), "{heading}");
    assert!(heading.contains("1 retiring"), "{heading}");
    assert!(heading.contains("of 25"), "{heading}");
}

#[test]
fn the_card_label_changes_tense_with_the_calendar() {
    let record = retirement("2026-12-31");
    assert_eq!(date_label(&record, "2026-11-15"), "retires");
    assert_eq!(date_label(&record, "2027-01-01"), "retired");
    assert_eq!(record.state("2026-11-15"), SunsetState::Scheduled);
    assert_eq!(record.state("2026-12-31"), SunsetState::Done);
}

#[test]
fn a_sunset_never_reads_as_ready() {
    // The readiness rule and the wording rule have to agree: a tool upstream
    // has left is not "ready" even when its binary still resolves.
    let found = harness("gone", Some(retirement("2026-06-18")));
    assert_eq!(found.retirement_adjective(TODAY), Some("retired"));
    assert_eq!(harness("alive", None).retirement_adjective(TODAY), None);
}
