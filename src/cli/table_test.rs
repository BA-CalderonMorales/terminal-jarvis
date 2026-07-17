use super::layout;

fn with_columns<T>(value: &str, test: impl FnOnce() -> T) -> T {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let previous = std::env::var_os("COLUMNS");
    std::env::set_var("COLUMNS", value);
    let result = test();
    if let Some(value) = previous {
        std::env::set_var("COLUMNS", value);
    } else {
        std::env::remove_var("COLUMNS");
    }
    result
}

#[test]
fn widths_respect_exact_budget_and_header_floors() {
    with_columns("48", || {
        let headers = ["AAAAAAAAAAAAAAAAAAAA", "BBBBBBBBBBBBBBBBBBBB"];
        let rows = [vec!["123456789012345678901".to_string(), "x".to_string()]];
        assert_eq!(layout::widths(&headers, &rows), [21, 20]);
    });
    with_columns("40", || {
        let headers = [
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
        ];
        assert_eq!(layout::widths(&headers, &[]), [30, 30]);
    });
}

#[test]
fn lines_wrap_words_chunks_and_empty_cells_exactly() {
    assert_eq!(
        layout::lines(&["one two".to_string()], &[7]),
        [vec!["one two".to_string()]]
    );
    assert_eq!(
        layout::lines(&["one two".to_string()], &[6]),
        [vec!["one".to_string()], vec!["two".to_string()]]
    );
    assert_eq!(
        layout::lines(&["abcdef".to_string()], &[2]),
        [
            vec!["ab".to_string()],
            vec!["cd".to_string()],
            vec!["ef".to_string()]
        ]
    );
    assert_eq!(layout::lines(&[String::new()], &[3]), [vec![String::new()]]);
}

#[test]
fn terminal_width_validates_and_caps_the_environment() {
    with_columns("39", || assert_eq!(layout::terminal_width(), 40));
    with_columns("40", || assert_eq!(layout::terminal_width(), 40));
    with_columns("121", || assert_eq!(layout::terminal_width(), 120));
    with_columns("invalid", || assert_eq!(layout::terminal_width(), 100));
}

#[test]
fn display_cell_width_wraps_and_pads_wide_text() {
    assert_eq!(
        layout::lines(&["界界".to_string()], &[2]),
        [vec!["界".to_string()], vec!["界".to_string()]]
    );
    assert_eq!(super::width::pad("界", 3), "界 ");
}
