//! Paint properties: every composed frame is exactly `rows` lines, each
//! visible row fits the width, and the cursor parks past the prompt.

use crate::tui::screen::{frame, parked, Draft, Size};

fn draft() -> Draft {
    Draft {
        title: "TERMINAL JARVIS vtest".into(),
        status: "ACTIVE none · CWD ~ · READY 0/0 ready".into(),
        body: vec!["one".into(), "two".into()],
        prompt: "[>_]::[tj:test]::[harness:none]: ".into(),
        hint: "active: none | list, status, help, exit".into(),
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
    let screen = painted.trim_start_matches("\x1b[H\x1b[2J");
    let lines: Vec<&str> = screen.split('\n').collect();
    let bounded = lines
        .iter()
        .all(|l| crate::tui::screen::visible_width(l) <= size.cols);
    let chrome = lines[0].starts_with('╔')
        && lines[1].contains("ACTIVE")
        && lines[lines.len() - 3].starts_with('├')
        && lines[lines.len() - 2].contains("[>_]")
        && lines[lines.len() - 1].starts_with('╰');
    bounded && lines.len() == size.rows && chrome
}

#[test]
fn parking_lands_one_cell_past_the_prompt() {
    quickcheck::quickcheck(parking_is_deterministic as fn(u8, u8) -> bool);
}

fn parking_is_deterministic(cols: u8, rows: u8) -> bool {
    let size = geometry(cols, rows);
    parked(String::new(), size, 12) == format!("\x1b[{};{}H", size.rows - 1, 12)
}

#[test]
fn long_body_is_clipped_with_a_marker_not_lost_silently() {
    let size = Size { cols: 60, rows: 10 };
    let mut d = draft();
    d.body = (0..50).map(|i| format!("line {i}")).collect();
    let painted = frame(size, &d);
    assert!(painted.contains("more lines above"));
    assert!(painted.contains("line 49"), "the newest line must survive");
}
