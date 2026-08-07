use super::*;

#[test]
fn compose_puts_the_given_hint_below_the_prompt() {
    let previous = crate::cli::style::set(true, true);
    assert_eq!(
        compose(true, "active: alpha | list, home, exit"),
        "[>_] \nactive: alpha | list, home, exit\x1b[1A\r[>_] "
    );
    crate::cli::style::restore(previous);
}

#[test]
fn compose_stays_plain_without_ansi() {
    let previous = crate::cli::style::set(true, true);
    assert_eq!(compose(false, "pick a number"), "[>_] pick a number\n[>_] ");
    crate::cli::style::restore(previous);
}

#[test]
fn retire_clears_the_box_and_keeps_the_committed_line_above() {
    let previous = crate::cli::style::set(true, true);
    assert_eq!(
        retire("use opencode", true, "pick a number"),
        "\x1b[1B\x1b[2K\x1b[1A\n"
    );
    assert_eq!(
        retire("", true, "pick a number"),
        "\r[>_] \npick a number\x1b[1A\r[>_] "
    );
    assert_eq!(retire("use opencode", false, "pick a number"), "\n");
    crate::cli::style::restore(previous);
}
