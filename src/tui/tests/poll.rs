use super::*;

#[test]
fn parked_keys_keep_fifo_order() {
    let _ = drained();
    park(Key::Char('a'));
    park(Key::Char('b'));
    assert_eq!(take(), Some(Key::Char('a')));
    assert_eq!(take(), Some(Key::Char('b')));
    assert_eq!(take(), None);
}
