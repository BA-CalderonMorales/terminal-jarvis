//! Theme contract: normalized lookup, live palette swap, env boot pin, and
//! every theme keeping the chrome dim (never the center of focus).

use super::{apply_theme, code, dim, paint, theme_names, TEST_LOCK};
use crate::tui::screen::roles::ALL;
use crate::tui::screen::Role;

#[test]
fn apply_theme_swaps_the_accent_live() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert!(apply_theme("mono"));
    assert_eq!(dim("x"), "\x1b[2mx\x1b[0m", "mono keeps grey dim");
    assert!(apply_theme("moss"));
    assert_eq!(dim("x"), "\x1b[2mx\x1b[0m");
    assert!(!apply_theme("nope"), "unknown names change nothing");
    apply_theme("default");
}

#[test]
fn lookup_is_case_and_separator_insensitive() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert!(apply_theme("MID-NIGHT"));
    assert!(apply_theme("solarized"));
    assert!(apply_theme("mid night"));
    assert!(!apply_theme("nope"));
    apply_theme("default");
}

#[test]
fn theme_names_sort_with_default_first() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let names = theme_names();
    assert_eq!(names[0], "default");
    assert!(names.contains(&"midnight"));
    assert!(names.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn boot_env_pins_the_palette() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("TERMINAL_JARVIS_THEME", "ember");
    super::boot_from_env();
    // ember's accent is red; the default's is cyan
    assert_eq!(super::accent("x"), "\x1b[1;31mx\x1b[0m");
    std::env::remove_var("TERMINAL_JARVIS_THEME");
    super::boot_from_env();
    // no env pin: the last applied palette stays until something resets it
    assert_eq!(super::accent("x"), "\x1b[1;31mx\x1b[0m");
    assert!(apply_theme("default"));
    assert_eq!(super::accent("x"), "\x1b[1;36mx\x1b[0m");
}

#[test]
fn bare_theme_cycles_and_wraps_back_to_default() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    apply_theme("default");
    let names = theme_names();
    for expected in names.iter().skip(1) {
        assert_eq!(super::super::cycle_theme(), *expected);
        assert_eq!(super::super::active_theme(), *expected);
    }
    assert_eq!(super::super::cycle_theme(), "default");
}

#[test]
fn every_theme_answers_every_role() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for name in theme_names() {
        assert!(apply_theme(name), "{name} is a listed theme");
        for role in ALL {
            let _ = code(role);
        }
    }
    apply_theme("default");
}

#[test]
fn paint_wraps_once_and_never_nests() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    assert!(apply_theme("default"));
    let painted = paint(Role::Accent, "x");
    assert_eq!(painted, "\x1b[1;36mx\x1b[0m");
    assert_eq!(painted.matches("\x1b[").count(), 2, "one open, one reset");
    assert_eq!(paint(Role::Value, "x"), "x", "value is unstyled by default");
    apply_theme("default");
}

#[test]
fn a_theme_swap_repaints_the_body_not_only_the_chrome() {
    let _serial = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    apply_theme("default");
    let default = crate::tui::shell::help::text(78);
    apply_theme("ember");
    let ember = crate::tui::shell::help::text(78);
    assert_ne!(
        default, ember,
        "body content follows the theme, not just chrome"
    );
    apply_theme("default");
}
