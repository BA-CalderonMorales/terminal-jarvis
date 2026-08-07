use super::*;
use std::sync::{Mutex, OnceLock};

static ENV: OnceLock<Mutex<()>> = OnceLock::new();

fn without_color_env<T>(test: impl FnOnce() -> T) -> T {
    let _guard = ENV.get_or_init(|| Mutex::new(())).lock().unwrap();
    let term = std::env::var_os("TERM");
    let no_color = std::env::var_os("NO_COLOR");
    std::env::remove_var("TERM");
    std::env::remove_var("NO_COLOR");
    let result = test();
    if let Some(value) = term {
        std::env::set_var("TERM", value);
    }
    if let Some(value) = no_color {
        std::env::set_var("NO_COLOR", value);
    }
    result
}

#[test]
fn ansi_is_gated_on_a_color_capable_terminal() {
    without_color_env(|| {
        assert!(ansi_enabled_for(true));
        assert!(!ansi_enabled_for(false));
    });
}

#[test]
fn ansi_refuses_dumb_and_no_color_environments() {
    without_color_env(|| {
        std::env::set_var("NO_COLOR", "1");
        assert!(!ansi_enabled_for(true));
        std::env::remove_var("NO_COLOR");
        std::env::set_var("TERM", "dumb");
        assert!(!ansi_enabled_for(true));
    });
}

#[test]
fn sequences_are_exact() {
    assert_eq!(cursor_left(18), "\x1b[18D");
    assert_eq!(erase_line(), "\x1b[2K");
    assert_eq!(carriage_return(), "\r");
    assert_eq!(clear_screen(), "\x1b[2J\x1b[H");
}
