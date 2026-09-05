//! Render: the transcript as chat bubbles -- the first harness on the left,
//! the reply on the right. Wrapped by display cells (wide glyphs count two)
//! so the frame's width math stays honest.

use crate::converse::transcript::Transcript;

const BUBBLE_MAX: usize = 60;

/// The full tab body: chapter header, topic, then one bubble per turn.
pub fn bubbles(transcript: &Transcript, width: usize) -> Vec<String> {
    let mut lines = vec![
        format!("── converse: {} ⇄ {} ──", transcript.a, transcript.b),
        format!("topic: {}", transcript.topic),
        String::new(),
    ];
    for turn in &transcript.turns {
        let left = turn.speaker == transcript.a;
        lines.extend(bubble(&turn.speaker, &turn.text, width, left));
        lines.push(String::new());
    }
    lines
}

/// One rounded box: `left` bubbles hug the margin, replies indent right.
fn bubble(speaker: &str, text: &str, width: usize, left: bool) -> Vec<String> {
    let bubble_w = width.saturating_sub(6).clamp(24, BUBBLE_MAX);
    let text_w = bubble_w.saturating_sub(4);
    let mut out = vec![format!(
        "╭─ {speaker} {}╮",
        "─".repeat(bubble_w.saturating_sub(speaker.chars().count() + 5))
    )];
    for line in wrap(text, text_w) {
        let cells = line.chars().map(crate::cli::char_cells).sum::<usize>();
        out.push(format!(
            "│ {line}{} │",
            " ".repeat(text_w.saturating_sub(cells))
        ));
    }
    out.push(format!("╰{}╯", "─".repeat(bubble_w.saturating_sub(2))));
    let indent = if left {
        0
    } else {
        width.saturating_sub(bubble_w + 1)
    };
    out.iter()
        .map(|line| format!("{}{line}", " ".repeat(indent)))
        .collect()
}

/// Cell-aware wrapping with a hard reset on embedded newlines, so a reply
/// that carries its own paragraphs never skews the box.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        let mut cells = 0;
        for character in paragraph.chars() {
            let glyph = crate::cli::char_cells(character);
            if cells + glyph > width && !line.is_empty() {
                lines.push(std::mem::take(&mut line));
                cells = 0;
            }
            line.push(character);
            cells += glyph;
        }
        lines.push(line);
    }
    lines
}
