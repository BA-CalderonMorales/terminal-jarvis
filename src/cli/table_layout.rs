pub fn widths(headers: &[&str], rows: &[Vec<String>]) -> Vec<usize> {
    let mut widths = headers
        .iter()
        .map(|header| width(header))
        .collect::<Vec<_>>();
    for row in rows {
        for (index, value) in row.iter().enumerate().take(widths.len()) {
            widths[index] = widths[index].max(value.lines().map(width).max().unwrap_or(0));
        }
    }
    let budget = terminal_width().saturating_sub(headers.len() * 3 + 1);
    let shrink_by = widths.iter().sum::<usize>().saturating_sub(budget);
    for _ in 0..shrink_by {
        let Some(index) = widest_shrinkable(&widths, headers) else {
            break;
        };
        widths[index] -= 1;
    }
    widths
}
pub fn lines(values: &[String], widths: &[usize]) -> Vec<Vec<String>> {
    let cells = widths
        .iter()
        .enumerate()
        .map(|(index, width)| wrap(values.get(index).map(String::as_str).unwrap_or(""), *width))
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
fn widest_shrinkable(widths: &[usize], headers: &[&str]) -> Option<usize> {
    widths
        .iter()
        .enumerate()
        .filter(|(index, size)| **size > width(headers[*index]))
        .max_by_key(|(_, size)| **size)
        .map(|(index, _)| index)
}
fn wrap(value: &str, limit: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for part in value.split('\n') {
        let start = lines.len();
        let mut line = String::new();
        for word in part.split_whitespace() {
            if width(word) > limit {
                if !line.is_empty() {
                    lines.push(line);
                    line = String::new();
                }
                chunks(word, limit, &mut lines);
            } else if line.is_empty() {
                line = word.to_string();
            } else if width(&line) + 1 + width(word) <= limit {
                line.push(' ');
                line.push_str(word);
            } else {
                lines.push(line);
                line = word.to_string();
            }
        }
        if !line.is_empty() {
            lines.push(line);
        } else if start == lines.len() {
            lines.push(String::new());
        }
    }
    lines
}

fn chunks(word: &str, limit: usize, lines: &mut Vec<String>) {
    let mut chunk = String::new();
    for character in word.chars() {
        chunk.push(character);
        if width(&chunk) == limit {
            lines.push(std::mem::take(&mut chunk));
        }
    }
    if !chunk.is_empty() {
        lines.push(chunk);
    }
}

fn width(value: &str) -> usize {
    value.chars().count()
}

pub(super) fn terminal_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width >= 40)
        .unwrap_or(100)
        .min(120)
}
