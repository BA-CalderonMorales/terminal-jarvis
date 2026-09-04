//! Sanitize properties: data lines keep color but never carry terminal
//! directives -- OSC, cursor movement, queries, and control bytes die here.

use super::*;

#[test]
fn sanitize_holds_frame_safety_properties() {
    quickcheck::quickcheck(sanitized_lines_carry_no_directives as fn(String) -> bool);
    quickcheck::quickcheck(plain_lines_pass_through_untouched as fn(String) -> bool);
}

fn directives(line: &str) -> bool {
    line.chars()
        .any(|c| c == '\x1b' || c.is_control() || ('\u{80}'..='\u{9f}').contains(&c))
}

fn sanitized_lines_carry_no_directives(line: String) -> bool {
    !directives(&keep_color(&line)) && is_plain(&keep_color(&line))
}

fn plain_lines_pass_through_untouched(line: String) -> bool {
    let plain: String = line
        .chars()
        .filter(|c| !c.is_control() || *c == '\n')
        .collect();
    if directives(&plain) {
        return true;
    }
    keep_color(&plain) == plain
}

#[test]
fn sgr_color_survives_but_cursor_motion_and_osc_die() {
    assert_eq!(keep_color("\x1b[32mgreen\x1b[0m"), "\x1b[32mgreen\x1b[0m");
    assert_eq!(keep_color("\x1b]52;c;YWJj\x07steal"), "steal");
    assert_eq!(keep_color("\x1b[2;3Hjump"), "jump");
    assert_eq!(keep_color("\x1b[6nquery"), "query");
    assert_eq!(keep_color("alt\x1b?screen"), "altscreen");
    assert_eq!(keep_color("dangling\x1b[38;5"), "dangling");
}
