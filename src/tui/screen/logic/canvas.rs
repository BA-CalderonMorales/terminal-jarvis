//! Canvas: pure width rules for the body zone. Lines are measured by
//! visible cells (ANSI escape bytes are zero-width) so styled and plain
//! text obey the same bounds. Row windowing lives in `scroll`; nothing
//! here ever drops a line.

/// Visible cell width of a line, skipping ANSI CSI sequences.
pub fn visible_width(line: &str) -> usize {
    let mut width = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for p in chars.by_ref() {
                if p.is_ascii_alphabetic() {
                    break;
                }
            }
        } else if !c.is_control() {
            width += crate::cli::char_cells(c);
        }
    }
    width
}

/// Clips one line into `cols` visible cells, ellipsizing the tail. Wide
/// glyphs are never split: the clip stops before a glyph that would cross.
pub fn clip_line(line: &str, cols: usize) -> String {
    if visible_width(line) <= cols {
        return line.to_string();
    }
    let keep = cols.saturating_sub(1);
    let mut clipped = String::new();
    let mut width = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            pass_escape(&mut clipped, &mut chars);
            continue;
        }
        let cells = crate::cli::char_cells(c);
        if width + cells > keep {
            break;
        }
        width += cells;
        clipped.push(c);
    }
    clipped.push('…');
    clipped
}

fn pass_escape<I: Iterator<Item = char>>(out: &mut String, chars: &mut std::iter::Peekable<I>) {
    out.push('\x1b');
    if let Some(&bracket) = chars.peek() {
        out.push(bracket);
        chars.next();
    }
    for p in chars.by_ref() {
        out.push(p);
        if p.is_ascii_alphabetic() {
            break;
        }
    }
}

/// Hard character-budget clip for border labels (no ellipsis: rules fill).
pub fn hint_clip(hint: &str, budget: usize) -> String {
    hint.chars().take(budget).collect()
}

#[cfg(test)]
#[path = "../../tests/screen_canvas.rs"]
mod tests;
