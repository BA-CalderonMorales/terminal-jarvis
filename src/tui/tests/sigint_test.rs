use super::*;

#[test]
fn redraw_happens_only_idle_and_ansi() {
    assert!(should_redraw(false, true));
    assert!(!should_redraw(true, true));
    assert!(!should_redraw(false, false));
    assert!(!should_redraw(true, false));
}

#[test]
fn core_frame_erases_the_line_before_any_redraw() {
    let frame = std::str::from_utf8(CORE_FRAME).unwrap();
    assert_eq!(frame, "\r\x1b[2K");
}
