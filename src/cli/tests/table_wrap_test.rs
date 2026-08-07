use super::*;

#[test]
fn segments_split_at_breakable_characters() {
    assert_eq!(segments("a/b:c-d"), vec!["a/", "b:", "c-", "d"]);
}

#[test]
fn chunks_flush_on_character_width_overflow() {
    let mut lines = Vec::new();
    chunks("aかな", 2, &mut lines);
    assert_eq!(lines, vec!["a", "か", "な"]);
}
