use super::presentation::{assert_table, home, stdout, tj};

#[test]
fn headless_tables_wrap_without_losing_their_frame() {
    let output = tj(&["security", "audit"], &home());
    assert_table(&output, "Security Audit");
    assert!(stdout(&output)
        .lines()
        .all(|line| line.chars().count() <= 100));
}
