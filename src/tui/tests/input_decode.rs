//! Decode: byte sequences map to semantic keys; escape soup is swallowed
//! whole and multi-byte glyphs arrive as one Char event.

use crate::tui::input::{decode, Key};

fn decoded(bytes: &[u8]) -> Option<Key> {
    decode(&mut bytes[1..].to_vec().as_slice(), bytes[0])
}

#[test]
fn control_bytes_map_to_editing_keys() {
    assert_eq!(decoded(b"\r"), Some(Key::Enter));
    assert_eq!(decoded(b"\n"), Some(Key::Enter));
    assert_eq!(decoded(b"\x7f"), Some(Key::Backspace));
    assert_eq!(decoded(b"\x08"), Some(Key::Backspace));
    assert_eq!(decoded(b"\x15"), Some(Key::ClearLine));
    assert_eq!(decoded(b"\x04"), None);
}

#[test]
fn printable_range_becomes_chars_and_strays_are_ignored() {
    assert_eq!(decoded(b"h"), Some(Key::Char('h')));
    assert_eq!(decoded(b"~"), Some(Key::Char('~')));
    assert_eq!(decoded(b"\x01"), Some(Key::Ignored));
    assert_eq!(decoded(b"\xff"), Some(Key::Ignored));
}

#[test]
fn csi_sequences_resolve_to_moves_or_ignored() {
    assert_eq!(decoded(b"\x1b[A"), Some(Key::Up));
    assert_eq!(decoded(b"\x1bOB"), Some(Key::Down));
    assert_eq!(decoded(b"\x1b[H"), Some(Key::Home));
    assert_eq!(decoded(b"\x1b[F"), Some(Key::End));
    assert_eq!(decoded(b"\x1b[5~"), Some(Key::PageUp));
    assert_eq!(decoded(b"\x1b[6~"), Some(Key::PageDown));
    assert_eq!(decoded(b"\x1b[Z"), Some(Key::Ignored));
    assert_eq!(decoded(b"\x1bX"), Some(Key::Ignored));
    assert_eq!(decoded(b"\x1b"), Some(Key::Ignored));
}

#[test]
fn utf8_leads_consume_their_continuation_bytes() {
    assert_eq!(decoded(b"\xc3\xa9"), Some(Key::Char('é')));
    assert_eq!(decoded(b"\xe2\x9c\x93"), Some(Key::Char('✓')));
    assert_eq!(decoded(b"\xf0\x9f\x91\x8b"), Some(Key::Char('👋')));
    assert_eq!(decoded(b"\xc3"), Some(Key::Ignored));
    assert_eq!(decoded(b"\xc3\xff"), Some(Key::Ignored));
}
