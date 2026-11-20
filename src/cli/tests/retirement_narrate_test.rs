use super::tests::{harness, quiet};
use crate::cli::logic::prompt_lead;

#[test]
fn the_narrated_lead_flips_on_the_supplied_date() {
    let h = harness("jules", Some(("2026-12-31", Some("antigravity"))));
    let plan = &h.capabilities[0];
    let mut options = quiet();
    options.narrate = true;
    let before = crate::cli::logic::style::set(true, true);
    let at = |today: &str| prompt_lead::confirm_lead_on(&options, &h, plan, &[], today);
    assert!(at("2026-12-30").contains("retirement: retires 2026-12-31"));
    assert!(at("2027-01-01").contains("retirement: retired 2026-12-31"));
    assert!(!at("2027-01-01").contains("retires 2026"));
    crate::cli::logic::style::restore(before);
}
