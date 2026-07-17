#[path = "table_layout.rs"]
mod layout;
#[path = "table_width.rs"]
mod width;

use super::style;

pub fn render(title: &str, headers: &[&str], rows: &[Vec<String>]) -> String {
    let widths = layout::widths(headers, rows);
    let header = headers
        .iter()
        .map(|value| (*value).to_string())
        .collect::<Vec<_>>();
    let mut out = format!("{}\n", style::heading(title));
    out.push_str(&border(&widths));
    out.push_str(&style::label(&row(&header, &widths)));
    out.push('\n');
    out.push_str(&border(&widths));
    for values in rows {
        for line in layout::lines(values, &widths) {
            out.push_str(&row(&line, &widths));
            out.push('\n');
        }
    }
    out.push_str(&border(&widths));
    out
}

pub fn fields(title: &str, values: &[(&str, String)]) -> String {
    if style::plain() {
        return values
            .iter()
            .map(|(key, value)| format!("{}: {value}\n", key.to_ascii_lowercase()))
            .collect();
    }
    let rows = values
        .iter()
        .map(|(key, value)| vec![(*key).to_string(), value.clone()])
        .collect::<Vec<_>>();
    render(title, &["FIELD", "VALUE"], &rows)
}

pub fn terminal_width() -> usize {
    width::terminal_width()
}

fn border(widths: &[usize]) -> String {
    format!(
        "+{}+\n",
        widths
            .iter()
            .map(|width| "-".repeat(width + 2))
            .collect::<Vec<_>>()
            .join("+")
    )
}

fn row<T: AsRef<str>>(values: &[T], widths: &[usize]) -> String {
    let cells = widths
        .iter()
        .enumerate()
        .map(|(index, target)| format!(" {} ", width::pad(values[index].as_ref(), *target)))
        .collect::<Vec<_>>();
    format!("|{}|", cells.join("|"))
}

#[cfg(test)]
#[path = "table_test.rs"]
mod tests;
