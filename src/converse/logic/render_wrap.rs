//! RenderWrap: word-boundary wrapping by display cells -- whole words
//! move to the next line; a word wider than the row hard-splits.

/// Wraps `text` so a complete word moves down, never a fragment.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        let mut cells = 0usize;
        for word in paragraph.split(' ').filter(|word| !word.is_empty()) {
            let word_cells: usize = word.chars().map(crate::cli::char_cells).sum();
            if cells > 0 && cells + 1 + word_cells > width {
                lines.push(std::mem::take(&mut line));
                cells = 0;
            }
            if cells > 0 {
                line.push(' ');
                cells += 1;
            }
            let mut rest = word;
            while rest.chars().map(crate::cli::char_cells).sum::<usize>() + cells > width {
                let mut taken = 0usize;
                let mut split_at = rest.len();
                for (index, character) in rest.char_indices() {
                    let glyph = crate::cli::char_cells(character);
                    if cells + taken + glyph > width {
                        split_at = index;
                        break;
                    }
                    taken += glyph;
                }
                line.push_str(&rest[..split_at]);
                lines.push(std::mem::take(&mut line));
                cells = 0;
                rest = &rest[split_at..];
                if rest.is_empty() {
                    break;
                }
            }
            line.push_str(rest);
            cells += rest.chars().map(crate::cli::char_cells).sum::<usize>();
        }
        lines.push(line);
    }
    lines
}
