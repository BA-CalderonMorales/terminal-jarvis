//! Paint properties: every composed frame is exactly `rows` lines, each
//! visible row fits the width, the repaint is surgical (no erase), and the
//! header keeps its verdict on narrow terminals.

use crate::tui::screen::{frame, Draft, Size};

fn draft() -> Draft {
    Draft {
        header: "Terminal Jarvis · ACTIVE fixture · READY 1/1 ready".into(),
        cwd: ".../working/terminal-jarvis".into(),
        body: vec!["one".into(), "two".into()],
        prompt: "[>_]::[tj:test]::[harness:fixture]: ".into(),
        offset: 0,
        hint: "active: fixture | list, status, help, exit".into(),
    }
}

fn geometry(cols: u8, rows: u8) -> Size {
    Size {
        cols: (cols as usize % 160) + Size::MIN_COLS,
        rows: (rows as usize % 60) + Size::MIN_ROWS,
    }
}

#[test]
fn frames_hold_geometry_properties() {
    quickcheck::quickcheck(frames_fit_their_size as fn(u8, u8) -> bool);
}

fn frames_fit_their_size(cols: u8, rows: u8) -> bool {
    let size = geometry(cols, rows);
    let painted = frame(size, &draft());
    let lines: Vec<&str> = painted.split('\n').collect();
    lines.len() == size.rows
        && lines
            .iter()
            .all(|line| crate::tui::screen::visible_width(line) <= size.cols)
        && !painted.contains("\x1b[2J")
}

#[test]
fn repaint_is_surgical_cursor_home_without_erase() {
    let painted = frame(Size { cols: 80, rows: 24 }, &draft());
    assert!(painted.starts_with("\x1b[H"));
    assert!(!painted.contains("\x1b[2J"), "no full-erase flicker");
}

#[test]
fn header_keeps_the_verdict_and_drops_the_cwd_when_narrow() {
    let painted = frame(Size { cols: 60, rows: 20 }, &draft());
    let first = painted.split('\n').next().unwrap();
    assert!(first.contains("Terminal Jarvis"), "{first}");
    assert!(first.contains("READY 1/1 ready"), "{first}");
    assert!(!first.contains("terminal-jarvis"), "cwd dies first");
}

#[test]
fn hint_sits_on_the_prompt_row_with_the_scroll_badge() {
    let size = Size { cols: 80, rows: 24 };
    let mut d = draft();
    d.body = (0..50).map(|i| format!("line {i}")).collect();
    let painted = frame(size, &d);
    let prompt_row = painted.split('\n').nth(size.rows - 1).unwrap();
    assert!(prompt_row.contains("↑ 29"), "scroll badge: {prompt_row}");
    // at 80 cols the badge wins the row and the long hint yields
    assert!(
        !prompt_row.contains("active: fixture"),
        "hint yields: {prompt_row}"
    );
    assert!(
        !painted.contains("more lines above"),
        "history is scrolled, not dropped"
    );
}

#[test]
fn long_body_windows_to_the_newest_lines() {
    let size = Size { cols: 80, rows: 24 };
    let mut d = draft();
    d.body = (0..50).map(|i| format!("line {i}")).collect();
    let painted = frame(size, &d);
    assert!(painted.contains("line 49"), "the newest line must survive");
    assert!(
        !painted.contains("line 0"),
        "older lines yield to the window"
    );
}

#[test]
fn no_box_chrome_anywhere() {
    let painted = frame(Size { cols: 80, rows: 24 }, &draft());
    for chrome in ["╔", "╚", "╠", "╣", "├", "┤"] {
        assert!(!painted.contains(chrome), "{chrome} in frame");
    }
}
