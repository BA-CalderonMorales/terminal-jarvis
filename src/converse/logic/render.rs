//! Render: the transcript as chat bubbles -- the first harness on the left,
//! the reply on the right. Wrapped on whole words (a long word hard-splits
//! rather than wandering), by display cells so wide glyphs stay honest.

use crate::cli::wrap;
use crate::converse::transcript::Transcript;

const BUBBLE_MAX: usize = 60;

/// The chapter header: emitted once when the conversation opens.
pub fn header(transcript: &Transcript) -> Vec<String> {
    vec![
        format!("── converse: {} ⇄ {} ──", transcript.a, transcript.b),
        format!("topic: {}", transcript.topic),
        String::new(),
    ]
}

/// Bubbles for `transcript.turns[from..]` -- the not-yet-rendered delta.
pub fn turns(transcript: &Transcript, from: usize, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for turn in transcript.turns.iter().skip(from) {
        let left = turn.speaker == transcript.a;
        lines.extend(bubble(&turn.speaker, &turn.text, width, left));
        lines.push(String::new());
    }
    lines
}

/// One rounded box: `left` bubbles hug the margin in the theme accent,
/// replies indent right in the second accent, so the two voices read at
/// a glance even mid-scroll.
fn bubble(speaker: &str, text: &str, width: usize, left: bool) -> Vec<String> {
    let bubble_w = width.saturating_sub(6).clamp(24, BUBBLE_MAX);
    let text_w = bubble_w.saturating_sub(4);
    type Tint = fn(&str) -> String;
    let (paint, reply_paint): (Tint, Tint) = if left {
        (crate::tui::screen::accent, crate::tui::screen::accent2)
    } else {
        (crate::tui::screen::accent2, crate::tui::screen::accent)
    };
    let name_row = format!(
        "╭─ {} {}╮",
        paint(speaker),
        "─".repeat(bubble_w.saturating_sub(speaker.chars().count() + 5))
    );
    let mut wrapped = wrap(text, text_w);
    while wrapped
        .last()
        .map(|row| row.trim().is_empty())
        .unwrap_or(false)
    {
        wrapped.pop();
    }
    let mut out = vec![name_row];
    for line in wrapped {
        let cells = line.chars().map(crate::cli::char_cells).sum::<usize>();
        out.push(format!(
            "│ {}{} │",
            paint(&line),
            " ".repeat(text_w.saturating_sub(cells))
        ));
    }
    out.push(format!(
        "╰{}╯",
        reply_paint(&"─".repeat(bubble_w.saturating_sub(2)))
    ));
    let indent = if left {
        0
    } else {
        width.saturating_sub(bubble_w + 1)
    };
    out.iter()
        .map(|line| format!("{}{line}", " ".repeat(indent)))
        .collect()
}
