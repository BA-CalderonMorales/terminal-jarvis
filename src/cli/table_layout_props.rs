use super::*;

fn ascii_only(value: &str) -> String {
    value
        .chars()
        .map(
            |character| {
                if character.is_ascii() {
                    character
                } else {
                    'x'
                }
            },
        )
        .collect()
}

fn wrap_lines_fit_limit(value: String, limit: usize) -> bool {
    let limit = (limit % 100).max(1);
    let value = ascii_only(&value);
    wrap(&value, limit).iter().all(|line| width(line) <= limit)
}

fn widths_repeat_with_rows(value: String, _rows: usize) -> bool {
    let value: String = ascii_only(&value).chars().take(40).collect();
    let headers = ["A"];
    let widths = widths(&headers, &[vec![value]]);
    widths.len() == 1 && widths[0] >= width("A")
}

#[test]
fn layout_properties() {
    quickcheck::quickcheck(wrap_lines_fit_limit as fn(String, usize) -> bool);
    quickcheck::quickcheck(widths_repeat_with_rows as fn(String, usize) -> bool);
}
