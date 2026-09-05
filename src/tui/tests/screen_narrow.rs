//! NarrowFrame pins: the chrome degrades by segment priority on small
//! terminals -- the readiness verdict and the hint's primary clause are
//! the last things standing, never a cut mid-segment.

use crate::tui::screen::{frame, Size};

#[test]
fn header_keeps_the_verdict_and_drops_the_cwd_when_narrow() {
    let painted = frame(Size { cols: 60, rows: 20 }, &super::tests::draft());
    let first = painted.split('\n').next().unwrap();
    assert!(first.contains("Terminal Jarvis"), "{first}");
    assert!(first.contains("READY 1/1 ready"), "{first}");
    assert!(!first.contains("terminal-jarvis"), "cwd dies first");
}

#[test]
fn at_forty_columns_whole_clauses_die_and_the_verdict_survives() {
    let painted = frame(Size { cols: 40, rows: 20 }, &super::tests::draft());
    let first = painted.split('\n').next().unwrap();
    assert!(
        first.contains("READY 1/1 ready"),
        "verdict survives: {first}"
    );
    assert!(
        !first.contains("Terminal Jarvis"),
        "the title dies first: {first}"
    );
    assert!(
        !first.contains(".../working"),
        "cwd dies before the verdict: {first}"
    );
    assert!(
        !first.contains("context command center"),
        "tagline dies too: {first}"
    );
    assert!(!first.contains('…'), "no mid-segment cut: {first}");
}

#[test]
fn at_sixty_columns_the_modeline_hint_degrades_to_its_primary_clause() {
    let painted = frame(Size { cols: 60, rows: 20 }, &super::tests::draft());
    let last = painted.split('\n').next_back().unwrap();
    assert!(
        last.contains("active: fixture"),
        "shortest useful form: {last}"
    );
    assert!(
        !last.contains("list, status"),
        "the secondary clause dies at 60: {last}"
    );
}
