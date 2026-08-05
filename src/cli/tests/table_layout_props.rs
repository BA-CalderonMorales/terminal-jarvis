use super::text::wrap;
use super::*;

fn ascii_only(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii() { c } else { 'x' })
        .collect()
}

fn wrap_lines_fit_limit(value: String, limit: usize) -> bool {
    let limit = (limit % 100).max(1);
    let value = ascii_only(&value);
    wrap(&value, limit).iter().all(|line| width(line) <= limit)
}

fn glued_breaks_have_separators(tokens_raw: Vec<String>, limit_spec: usize) -> bool {
    let limit = (limit_spec % 60).max(1);
    let tokens: Vec<String> = tokens_raw
        .into_iter()
        .map(|token| ascii_only(&token))
        .map(|token| token.chars().filter(|c| !c.is_whitespace()).collect())
        .filter(|token: &String| !token.is_empty())
        .collect();
    let value = tokens.join(" ");
    let lines = wrap(&value, limit);
    let mut cursor = tokens.iter().cloned();
    let mut current = String::new();
    let mut pos = 0usize;
    for line in &lines {
        for character in line.chars() {
            if character == ' ' {
                continue;
            }
            if pos >= current.len() {
                match cursor.next() {
                    Some(next) => current = next,
                    None => return false,
                }
                pos = 0;
            }
            if character as usize != current.as_bytes()[pos] as usize {
                return false;
            }
            pos += 1;
        }
        if pos >= current.len() {
            continue;
        }
        let remainder = &current.as_bytes()[pos..];
        let end = line.chars().last().unwrap_or_default();
        let unavoidable = remainder.iter().all(|byte| !breakable_char(*byte as char));
        if !breakable_char(end) && width(line) != limit && !unavoidable {
            return false;
        }
    }
    true
}

fn breakable_char(character: char) -> bool {
    matches!(
        character,
        '/' | '.' | ',' | ':' | ';' | '-' | '_' | '=' | '\\' | '@' | '+'
    )
}

fn widths_repeat_with_rows(value: String, _rows: usize) -> bool {
    let value: String = ascii_only(&value).chars().take(40).collect();
    let headers = ["A"];
    let widths = widths(&headers, &[vec![value]]);
    widths.len() == 1 && widths[0] >= width("A")
}

fn table_width_bounded(headers_raw: Vec<String>, value: String) -> bool {
    let names: Vec<&str> = headers_raw
        .iter()
        .map(|header| header.as_str())
        .take(6)
        .collect();
    if names.is_empty() {
        return true;
    }
    let rows = [vec![ascii_only(&value)]];
    let widths = widths(&names, &rows);
    let budget = terminal_width().saturating_sub(names.len() * 3 + 1);
    let sum = widths.iter().sum::<usize>();
    sum <= budget || widths.iter().all(|width| *width == MIN_COLUMN)
}

#[test]
fn layout_properties() {
    quickcheck::quickcheck(wrap_lines_fit_limit as fn(String, usize) -> bool);
    quickcheck::quickcheck(glued_breaks_have_separators as fn(Vec<String>, usize) -> bool);
    quickcheck::quickcheck(widths_repeat_with_rows as fn(String, usize) -> bool);
    quickcheck::quickcheck(table_width_bounded as fn(Vec<String>, String) -> bool);
}
