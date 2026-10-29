use super::*;
use crate::contracts::{EnvMode, Sunset};

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

fn retirement() -> Sunset {
    Sunset {
        at: "2026-06-18".into(),
        note: "the vendor retired it".into(),
        replaced_by: Some("successor".into()),
    }
}

#[test]
fn a_retired_harness_reads_as_a_dated_retirement_in_plain_form() {
    let previous = style::set(true, true);
    let text = graveyard(&[harness("retired", Some(retirement()))]);
    style::restore(previous);
    assert!(text.contains("retired retired=2026-06-18"), "{text}");
    assert!(text.contains("replaced_by=successor"), "{text}");
    assert!(text.contains("note=the vendor retired it"), "{text}");
}

#[test]
fn a_maintained_only_catalog_has_no_graveyard_rows() {
    let previous = style::set(true, true);
    let text = graveyard(&[harness("alive", None)]);
    style::restore(previous);
    assert!(text.is_empty(), "{text}");
}

#[test]
fn a_retirement_without_a_successor_says_so() {
    let mut sunset = retirement();
    sunset.replaced_by = None;
    let previous = style::set(true, true);
    let text = graveyard(&[harness("retired", Some(sunset))]);
    style::restore(previous);
    assert!(text.contains("replaced_by=no named successor"), "{text}");
}
