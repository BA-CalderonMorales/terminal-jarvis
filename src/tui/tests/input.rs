use super::*;

const V: &str = env!("CARGO_PKG_VERSION");

fn fixture() -> Indicator {
    Indicator {
        active: "codex".into(),
        debug: false,
    }
}

#[test]
fn compose_puts_the_given_hint_below_the_prompt_with_the_indicator() {
    let previous = crate::cli::style::set(true, true);
    assert_eq!(
        compose(true, &fixture(), "active: codex | list, home, exit"),
        format!("[>_]::[tj:{V}]::[harness:codex]: \nactive: codex | list, home, exit\x1b[1A\r[>_]::[tj:{V}]::[harness:codex]: ")
    );
    crate::cli::style::restore(previous);
}

#[test]
fn compose_stays_plain_without_ansi() {
    let previous = crate::cli::style::set(true, true);
    assert_eq!(
        compose(false, &fixture(), "pick a number"),
        format!("[>_]::[tj:{V}]::[harness:codex]: pick a number\n[>_]::[tj:{V}]::[harness:codex]: ")
    );
    crate::cli::style::restore(previous);
}

#[test]
fn indicator_marks_harness_and_debug_and_survives_roundtrips() {
    let previous = crate::cli::style::set(true, true);
    let plain = Indicator {
        active: "none".into(),
        debug: true,
    };
    assert!(Indicator {
        active: "codex".into(),
        debug: false
    }
    .raw()
    .contains("[harness:codex]"));
    assert_eq!(plain.raw(), format!("[>_]::[tj:{V}]::[harness:none]::[debug]:"));
    assert_eq!(
        plain.render(false),
        format!("[>_]::[tj:{V}]::[harness:none]::[debug]: ")
    );
    crate::cli::style::restore(previous);
}

#[test]
fn retire_clears_the_box_and_keeps_the_committed_line_above() {
    let previous = crate::cli::style::set(true, true);
    assert_eq!(
        retire("use opencode", true, "pick a number", &fixture()),
        "\x1b[1B\x1b[2K\x1b[1A\n"
    );
    assert_eq!(
        retire("", true, "pick a number", &fixture()),
        format!("\r\x1b[2K[>_]::[tj:{V}]::[harness:codex]: ")
    );
    assert_eq!(
        retire("use opencode", false, "pick a number", &fixture()),
        "\n"
    );
    crate::cli::style::restore(previous);
}
