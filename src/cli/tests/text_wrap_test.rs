//! TextWrap contract: one wrapper for every surface. No row ever ends
//! mid-word, an embedded newline resets the line, and wide glyphs (CJK,
//! emoji) count two cells so rows never overflow their budget.

use crate::cli::wrap;
use crate::tui::screen::visible_width;

#[test]
fn no_row_ends_mid_word() {
    let rows = wrap("alpha bravo charlie delta", 11);
    assert_eq!(rows, vec!["alpha bravo", "charlie", "delta"]);
    for row in &rows {
        assert!(visible_width(row) <= 11, "overflow: {row}");
    }
}

#[test]
fn a_word_wider_than_the_row_hard_splits_by_cells() {
    assert_eq!(wrap("abcdefgh", 3), vec!["abc", "def", "gh"]);
    assert_eq!(wrap("ab cd", 3), vec!["ab", "cd"]);
}

#[test]
fn embedded_newlines_reset_the_line() {
    assert_eq!(wrap("ab\ncd", 10), vec!["ab", "cd"]);
    assert_eq!(wrap("a\n\nb", 10), vec!["a", "", "b"]);
    assert_eq!(wrap("\n", 10), vec!["", ""]);
}

#[test]
fn cjk_and_emoji_count_two_cells() {
    // each glyph is two cells: a 3-cell row holds only one
    assert_eq!(wrap("你好", 3), vec!["你", "好"]);
    assert_eq!(wrap("👍👍", 3), vec!["👍", "👍"]);
    // a 5-cell row holds two glyphs
    assert_eq!(wrap("你好", 5), vec!["你好"]);
}

#[test]
fn rows_never_exceed_the_width_for_arbitrary_text() {
    fn holds(text: String, width: u8) -> bool {
        let width = (width % 24) as usize + 4;
        wrap(&text, width)
            .iter()
            .all(|row| visible_width(row) <= width)
    }
    quickcheck::quickcheck(holds as fn(String, u8) -> bool);
}
