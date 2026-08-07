#[path = "table_alloc.rs"]
mod alloc;

#[path = "table_wrap.rs"]
mod text;

pub(super) use super::width::terminal_width;
pub const MIN_COLUMN: usize = 10;

pub fn widths(headers: &[&str], rows: &[Vec<String>]) -> Vec<usize> {
    (alloc::widths)(headers, rows)
}

pub fn lines(values: &[String], widths: &[usize]) -> Vec<Vec<String>> {
    let cells = widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            text::wrap(values.get(index).map(String::as_str).unwrap_or(""), *width)
        })
        .collect::<Vec<_>>();
    let height = cells.iter().map(Vec::len).max().unwrap_or(1);
    (0..height)
        .map(|line| {
            cells
                .iter()
                .map(|cell| cell.get(line).cloned().unwrap_or_default())
                .collect()
        })
        .collect()
}

#[cfg(test)]
use super::width::display_width as width;

#[cfg(test)]
#[path = "../tests/table_layout_props.rs"]
mod props;
