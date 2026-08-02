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

fn control_chars_have_zero_width(character: char) -> bool {
    !character.is_control() || character_width(character) == 0
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
