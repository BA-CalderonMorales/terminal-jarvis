use super::*;

fn display_width_is_never_more_than_double_char_count(value: String) -> bool {
    let chars = value.chars().count();
    display_width(&value) <= chars * 2
}

fn pad_roundtrips_width(value: String, target: usize) -> bool {
    let target = target % 120;
    let padded = pad(&value, target);
    display_width(&padded) == target.max(display_width(&value))
}

fn character_width_ascii(character: char) -> bool {
    if character.is_ascii() && !character.is_control() {
        character_width(character) == 1
    } else {
        character_width(character) <= 2
    }
}

#[test]
fn wide_glyphs_measure_two_cells() {
    // Through the public wrapper (table::char_cells): if it collapses to 1
    // the framed layouts drift on wide output.
    let cells = crate::cli::logic::table::char_cells;
    assert_eq!(cells('一'), 2);
    assert_eq!(cells('Ａ'), 2);
    assert_eq!(cells('a'), 1);
    assert_eq!(cells('·'), 1);
}

fn control_chars_have_zero_width(character: char) -> bool {
    !character.is_control() || character_width(character) == 0
}

#[test]
fn exact_widths_for_combining_and_wide_chars() {
    assert_eq!(character_width('\u{02ff}'), 1);
    assert_eq!(character_width('\u{0300}'), 0);
    assert_eq!(character_width('\u{036f}'), 0);
    assert_eq!(character_width('\u{04ff}'), 1);
    assert_eq!(character_width('\u{4e00}'), 2);
    assert_eq!(display_width("e\u{0301}"), 1);
}

#[test]
fn terminal_width_clamps_and_defaults() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|lock| lock.into_inner());
    std::env::remove_var("COLUMNS");
    assert_eq!(terminal_width(), 100);
    for (raw, expected) in [("60", 60), ("10", 40), ("999", 120)] {
        std::env::set_var("COLUMNS", raw);
        assert_eq!(terminal_width(), expected);
    }
    std::env::remove_var("COLUMNS");
}

#[test]
fn width_properties() {
    quickcheck::quickcheck(
        display_width_is_never_more_than_double_char_count as fn(String) -> bool,
    );
    quickcheck::quickcheck(pad_roundtrips_width as fn(String, usize) -> bool);
    quickcheck::quickcheck(character_width_ascii as fn(char) -> bool);
    quickcheck::quickcheck(control_chars_have_zero_width as fn(char) -> bool);
}
