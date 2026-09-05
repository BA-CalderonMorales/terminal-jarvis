//! TextWrap: the ONE word-boundary wrapper both surfaces share -- whole
//! words move to the next line, a word wider than the row hard-splits by
//! display cells (wide glyphs count two), and an embedded newline resets
//! the line. Never a mid-word row end.

/// Wraps `text` so a complete word moves down, never a fragment.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        let mut cells = 0usize;
        for word in paragraph.split(' ').filter(|word| !word.is_empty()) {
            if cells > 0 && cells + 1 + cells_of(word) > width {
                lines.push(std::mem::take(&mut line));
                cells = 0;
            }
            if cells > 0 {
                line.push(' ');
                cells += 1;
            }
            let mut rest = word;
            while cells_of(rest) + cells > width {
                let split_at = split_index(rest, width.saturating_sub(cells));
                line.push_str(&rest[..split_at]);
                lines.push(std::mem::take(&mut line));
                cells = 0;
                rest = &rest[split_at..];
            }
            line.push_str(rest);
            cells += cells_of(rest);
        }
        lines.push(line);
    }
    lines
}

/// Visible cells of `text`, wide glyphs counting two.
fn cells_of(text: &str) -> usize {
    text.chars().map(crate::cli::char_cells).sum()
}

/// The char index where `rest` may split so the head fits `budget` cells.
/// At least one glyph is always taken, so a wide glyph on a narrow row
/// overflows rather than looping forever.
fn split_index(rest: &str, budget: usize) -> usize {
    let mut taken = 0usize;
    for (index, character) in rest.char_indices() {
        let glyph = crate::cli::char_cells(character);
        if taken > 0 && taken + glyph > budget {
            return index;
        }
        taken += glyph;
    }
    rest.len()
}

#[cfg(test)]
#[path = "../tests/text_wrap_test.rs"]
mod tests;
