use super::*;

#[test]
fn an_arrow_replays_without_a_second_escape() {
    assert_eq!(replayed(0x1b, b"[B"), Some(Key::Down));
    assert_eq!(replayed(0x1b, b"[A"), Some(Key::Up));
    assert_eq!(replayed(0x1b, b"[6~"), Some(Key::PageDown));
    assert_eq!(
        replayed(0x1b, b"OB"),
        Some(Key::Down),
        "SS3 arrows decode too"
    );
}

#[test]
fn an_unknown_sequence_is_swallowed_whole() {
    assert_eq!(replayed(0x1b, b"[Z"), Some(Key::Ignored));
}
