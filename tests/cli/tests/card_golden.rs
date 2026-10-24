//! The headless cards, pinned as text.
//!
//! The tui frame and the terminal read the same card, so both are pinned:
//! a change to one that does not reach the other is a change nobody made.

use crate::logic::cli_driver::Fixture;
use crate::logic::golden;
use crate::logic::tui::MARKER_SCRIPT;

/// The card a headless command painted, at a pinned terminal width.
fn card(fixture: &Fixture, args: &[&str]) -> Vec<String> {
    let output = fixture.run_with(args, &[("COLUMNS", "80")]);
    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

#[test]
fn the_headless_cards_are_pinned() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let root = fixture.root.display().to_string();

    golden::check("card-show", &card(&fixture, &["show", "fixture"]), &root);
    golden::check(
        "card-plan",
        &card(&fixture, &["plan", "fixture", "headless"]),
        &root,
    );
}
