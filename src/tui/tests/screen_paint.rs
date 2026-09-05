//! Paint properties: every composed frame is exactly `rows` lines, each
//! visible row fits the width, the repaint is surgical (no erase), and the
//! header keeps its verdict on narrow terminals.

use crate::tui::screen::{frame, Draft, Size};

pub(crate) fn draft() -> Draft {
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
fn rows_carry_carriage_returns_for_raw_mode() {
    // Raw mode disables OPOST: a bare \n stair-steps every row after the
    // first. Every line break in a painted frame must be an explicit \r\n.
    let painted = frame(Size { cols: 80, rows: 24 }, &draft());
    let bare_newlines = painted
        .char_indices()
        .filter(|(index, c)| *c == '\n' && (*index == 0 || painted.as_bytes()[*index - 1] != b'\r'))
        .count();
    assert_eq!(bare_newlines, 0, "no bare newlines");
    assert!(painted.contains("\r\n"));
}
#[test]
fn no_box_chrome_anywhere() {
    let painted = frame(Size { cols: 80, rows: 24 }, &draft());
    for chrome in ["╔", "╚", "╠", "╣", "├", "┤"] {
        assert!(!painted.contains(chrome), "{chrome} in frame");
    }
}

pub(crate) fn harness_prompt() -> &'static str {
    "[>_]::[tj:test]::[harness:fixture]: "
}
