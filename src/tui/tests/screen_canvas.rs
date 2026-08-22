//! Quickcheck properties for the canvas rules: visible width ignores ANSI,
//! clipping respects bounds, and row-fitting keeps recent context.

use super::*;

#[test]
fn canvas_holds_fitting_properties() {
    quickcheck::quickcheck(escape_bytes_are_zero_width as fn(usize, String) -> bool);
    quickcheck::quickcheck(clipped_lines_never_exceed_cols as fn(String, bool, u8) -> bool);
    quickcheck::quickcheck(fit_rows_keep_recent_context as fn(Vec<String>, u8) -> bool);
}

fn escape_bytes_are_zero_width(pad: usize, text: String) -> bool {
    let text: String = text
        .chars()
        .filter(|c| !c.is_control() && *c != '\x1b' && *c != '[')
        .collect();
    let line = format!("\x1b[31m{text}\x1b[0m{}", " ".repeat(pad % 8));
    let text_cells: usize = text.chars().map(crate::cli::char_cells).sum();
    visible_width(&line) == text_cells + (pad % 8)
}

fn clipped_lines_never_exceed_cols(line: String, ansi: bool, cols: u8) -> bool {
    let cols = (cols as usize % 120) + 1;
    let line = line.replace('\n', " ");
    let candidate = if ansi {
        format!("\x1b[32m{line}\x1b[0m")
    } else {
        line
    };
    visible_width(&clip_line(&candidate, cols)) <= cols
}

fn fit_rows_keep_recent_context(lines: Vec<String>, rows: u8) -> bool {
    let rows = (rows as usize % 50) + 1;
    let fitted = fit_rows(&lines, rows);
    if lines.len() <= rows.saturating_sub(1) {
        return fitted == lines;
    }
    fitted.len() == rows
        && fitted[0].contains("more lines above")
        && fitted[1..] == lines[lines.len() + 1 - rows..]
}
