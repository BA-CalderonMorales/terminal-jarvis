//! StatusRows: the fleet-name paragraphs -- word-wrapped to ~60 cells
//! with a two-space indent, so any fleet size reads as tidy text.

/// Word-wrapped name rows for one readiness group.
pub fn wrap(names: &[&str]) -> Vec<String> {
    let mut rows = vec![String::from("  ")];
    for name in names {
        let candidate = format!("{}, ", name);
        let width: usize = rows
            .last()
            .unwrap()
            .chars()
            .map(crate::cli::char_cells)
            .sum::<usize>()
            + candidate.chars().map(crate::cli::char_cells).sum::<usize>();
        if width > 62 {
            rows.push(String::from("  "));
        }
        rows.last_mut().unwrap().push_str(&candidate);
    }
    rows.into_iter()
        .map(|row| row.trim_end().trim_end_matches(',').to_string())
        .collect()
}
