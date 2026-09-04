//! Parked contract: the cursor lands on the last row, after the typed
//! tail, and the prompt row carries its tiered right side.

use super::tests::harness_prompt;
use crate::tui::screen::{frame, parked, visible_width, Draft, Size};

fn draft() -> Draft {
    Draft {
        header: "Terminal Jarvis · ACTIVE fixture · READY 1/1 ready".into(),
        cwd: ".../working/terminal-jarvis".into(),
        tagline: "context command center · active [fixture] · fleet readiness 1/1".into(),
        body: vec!["one".into(), "two".into()],
        prompt: "[>_]::[tj:test]::[harness:fixture]: ".into(),
        offset: 0,
        hint: "active: fixture | list, status, help, exit".into(),
    }
}

#[test]
fn hint_sits_on_the_prompt_row_with_the_scroll_badge() {
    let size = Size { cols: 80, rows: 24 };
    let mut d = draft();
    d.body = (0..50).map(|i| format!("line {i}")).collect();
    let painted = frame(size, &d);
    let prompt_row = painted.split('\n').nth(size.rows - 1).unwrap();
    assert!(
        prompt_row.contains("\u{2191} 28"),
        "scroll badge: {prompt_row}"
    );
    // at 80 cols the badge wins the row and the long hint yields
    assert!(
        !prompt_row.contains("active: fixture"),
        "hint yields: {prompt_row}"
    );
}

#[test]
fn cursor_parks_on_the_prompt_row_after_the_tail() {
    let size = Size { cols: 80, rows: 24 };
    let prompt_str = harness_prompt();
    let prompt_cells = visible_width(prompt_str) + 1;
    let painted = parked(frame(size, &draft()), size, prompt_cells);
    let expected = format!("\x1b[{};{}H", size.rows, prompt_cells);
    assert!(
        painted.ends_with(&expected),
        "cursor must sit on the last row, after the prompt: {expected}"
    );
}
