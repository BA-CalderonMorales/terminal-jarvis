use super::*;
use std::io::Cursor;
#[test]
fn chunks_keep_valid_unicode_and_blank_lines() {
    let body = format!("{}漢{}", " ".repeat(16383), "x".repeat(30000));
    let (tx, rx) = std::sync::mpsc::sync_channel(8);
    pump(Cursor::new(body.clone().into_bytes()), Line::Out, tx);
    let mut actual = String::new();
    for line in rx {
        if let Line::Out(text) = line {
            assert!(text.len() <= CHUNK as usize);
            actual.push_str(&text);
        }
    }
    assert_eq!(actual, body);
    let (tx, rx) = std::sync::mpsc::sync_channel(8);
    pump(Cursor::new(b"a\n\nb\r\n"), Line::Out, tx);
    let lines: Vec<_> = rx
        .into_iter()
        .map(|line| match line {
            Line::Out(text) => text,
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(lines, ["a", "", "b"]);
}
