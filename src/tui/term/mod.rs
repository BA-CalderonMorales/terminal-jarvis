//! Term: minimal, dependency-free terminal control. Emits the smallest set of
//! ANSI sequences a line-oriented TUI needs -- cursor retreat, line erasure,
//! carriage return -- and gates them behind the same environment checks the
//! rest of the tool applies, so automation pipes never receive control bytes.
//! This is the intentional std-only stand-in for crossterm/termion: a tiny,
//! fully unit-tested surface instead of a crate.

use std::io::IsTerminal;

#[path = "logic/raw.rs"]
mod raw;

/// Raw per-byte, no-echo reads for the viewport prompt; `None` keeps the
/// caller on the classic line reader (pipes, missing tty, non-unix).
pub fn enable_raw() -> Option<raw::Guard> {
    raw::enable()
}

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

/// Terminal columns: real geometry from the tty when available (TIOCGWINSZ
/// via the same raw-FFI pattern as the pty harnesses), then the COLUMNS env
/// fallback, then 100. Std-only -- no libc crate.
pub fn columns() -> usize {
    if let Some((width, _)) = size() {
        return width;
    }
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .map(|width| width.clamp(40, 160))
        .unwrap_or(100)
}

/// (cols, rows) from the tty, or None when stdout is not a sized terminal.
pub fn size() -> Option<(usize, usize)> {
    geometry()
}

#[cfg(unix)]
fn geometry() -> Option<(usize, usize)> {
    #[repr(C)]
    struct Winsize {
        row: u16,
        col: u16,
        xpixel: u16,
        ypixel: u16,
    }
    unsafe extern "C" {
        fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
    #[cfg(target_os = "linux")]
    const TIOCGWINSZ: u64 = 0x5413;
    #[cfg(not(target_os = "linux"))]
    const TIOCGWINSZ: u64 = 0x4008_7468;
    let mut size = Winsize {
        row: 0,
        col: 0,
        xpixel: 0,
        ypixel: 0,
    };
    let ok = unsafe { ioctl(1, TIOCGWINSZ, &mut size) } == 0;
    (ok && size.col > 0 && size.row > 0).then_some((size.col as usize, size.row as usize))
}

#[cfg(not(unix))]
fn geometry() -> Option<(usize, usize)> {
    None
}

#[cfg(test)]
#[path = "../tests/term.rs"]
mod tests;
