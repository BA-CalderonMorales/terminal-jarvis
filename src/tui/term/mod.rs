//! Term: minimal, dependency-free terminal control. Emits the smallest set of
//! ANSI sequences a line-oriented TUI needs -- cursor retreat, line erasure,
//! carriage return -- and gates them behind the same environment checks the
//! rest of the tool applies, so automation pipes never receive control bytes.
//! This is the intentional std-only stand-in for crossterm/termion: a tiny,
//! fully unit-tested surface instead of a crate.

use std::io::IsTerminal;

pub fn ansi_enabled_for(stdout_terminal: bool) -> bool {
    stdout_terminal
        && std::env::var_os("NO_COLOR").is_none()
        && std::env::var("TERM")
            .map(|term| term != "dumb")
            .unwrap_or(true)
}

pub fn ansi_enabled() -> bool {
    ansi_enabled_for(std::io::stdout().is_terminal())
}

pub fn cursor_left(count: usize) -> String {
    format!("\x1b[{count}D")
}

pub fn erase_line() -> String {
    "\x1b[2K".to_string()
}

pub fn carriage_return() -> String {
    "\r".to_string()
}

pub fn clear_screen() -> String {
    "\x1b[2J\x1b[H".to_string()
}

#[cfg(test)]
#[path = "../tests/term.rs"]
mod tests;
