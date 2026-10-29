//! Screen face: boots the full-viewport frame when the terminal can hold it,
//! tears it down for child runs, and watches for resizes. Chat mode stays
//! the fallback whenever geometry, termcap, or tty-ness says otherwise --
//! "just works" beats clever.

use super::structs::Size;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);
static BOOTED: AtomicBool = AtomicBool::new(false);

/// Owns the alternate screen for the shell's lifetime; dropping restores.
pub struct Session;

pub fn boot() -> Option<Session> {
    super::theme::boot_from_env();
    let (cols, rows) = crate::tui::term::size()?;
    let size = Size { cols, rows };
    let usable = size.usable() && crate::tui::term::ansi_enabled();
    usable.then(|| {
        BOOTED.store(true, Ordering::Release);
        ACTIVE.store(true, Ordering::Release);
        print!("\x1b[?1049h");
        print!("{}", crate::tui::term::set_title("terminal-jarvis"));
        io::stdout().flush().ok();
        Session
    })
}

pub fn size() -> Size {
    // The REAL geometry, never widened: a clamped frame would paint rows
    // wider than the terminal and the terminal would wrap them, walking the
    // prompt up the screen. `usable()` decides whether the frame runs at all.
    let (cols, rows) = crate::tui::term::size().unwrap_or((80, 24));
    Size {
        cols: cols.max(1),
        rows: rows.max(1),
    }
}

pub fn active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

/// Keeps the frame honest across a resize: a terminal that shrinks below the
/// floor hands the session back to chat mode, and one that grows back
/// reclaims the frame. Compares the REAL geometry, not the clamped frame
/// size -- the clamp is what a frame needs, not what the terminal has.
pub fn ensure_usable() {
    if !BOOTED.load(Ordering::Acquire) {
        return;
    }
    let usable = crate::tui::term::size()
        .map(|(cols, rows)| cols >= Size::MIN_COLS && rows >= Size::MIN_ROWS)
        .unwrap_or(false);
    match (active(), usable) {
        (true, false) => {
            suspend();
        }
        (false, true) => resume(true),
        _ => {}
    }
}

/// Leaves the viewport so a child owns the real terminal; returns whether
/// [`resume`] must restore it.
pub fn suspend() -> bool {
    let was = ACTIVE.swap(false, Ordering::AcqRel);
    if was {
        print!("\x1b[?1049l");
        io::stdout().flush().ok();
    }
    was
}

pub fn resume(was: bool) {
    if was && !ACTIVE.swap(true, Ordering::AcqRel) {
        print!("\x1b[?1049h");
        io::stdout().flush().ok();
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        BOOTED.store(false, Ordering::Release);
        if ACTIVE.swap(false, Ordering::AcqRel) {
            // No newline here: the frame's last row is the prompt, and a stray
            // newline scrolls the screen the terminal restores by one line.
            print!("\x1b[?1049l");
            io::stdout().flush().ok();
        }
    }
}

pub use super::theme::{accent, accent2, active_theme, apply_theme, cycle_theme, dim, theme_names};

#[cfg(test)]
#[path = "../tests/screen_boot.rs"]
mod tests;
