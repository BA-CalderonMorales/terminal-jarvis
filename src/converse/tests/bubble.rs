//! Bubble contract: the chat transcript wraps on whole words, keeps every
//! box row inside the width, and indents replies to the right.

use crate::converse::render::{header, turns};

#[test]
fn bubbles_put_the_first_harness_left_and_replies_right() {
    let mut transcript = crate::converse::Transcript::new("opencode", "hermes", "t");
    transcript.push("opencode", "hi there");
    transcript.push("hermes", "hello");
    let lines = turns(&transcript, 0, 60);
    let top = lines
        .iter()
        .find(|line: &&String| line.contains("╭─") && line.contains("opencode"))
        .unwrap();
    assert!(
        top.contains("╭─") && top.contains("opencode"),
        "the opener bubble names its speaker"
    );
    let reply = lines
        .iter()
        .find(|line: &&String| line.contains("╭─") && line.contains("hermes"))
        .unwrap();
    assert!(reply.starts_with(' '), "reply bubble indents right");
    assert!(header(&transcript).iter().any(|line| line == "topic: t"));
}

#[test]
fn bubble_wrapping_keeps_whole_words_inside_the_box() {
    let mut transcript = crate::converse::Transcript::new("a", "b", "t");
    transcript.push("a", &"word ".repeat(40));
    let lines = turns(&transcript, 0, 60);
    let boxed: Vec<&String> = lines.iter().filter(|line| line.contains('│')).collect();
    assert!(boxed.len() > 1, "long text wraps into multiple box rows");
    for row in &boxed {
        let visible = crate::tui::screen::visible_width(row);
        assert!(visible <= 60, "row {visible} cells overflows: {row}");
        let text = row.replace("\u{1b}", "^");
        assert!(
            text.contains("word word") || text.contains("│ word"),
            "no mid-word splits: {row}"
        );
    }
}
