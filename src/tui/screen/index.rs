//! Screen face: boots the full-viewport frame when the terminal can hold it,
//! tears it down for child runs, and watches for resizes. Chat mode stays
//! the fallback whenever geometry, termcap, or tty-ness says otherwise --
//! "just works" beats clever.

use super::structs::Size;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);

/// Owns the alternate screen for the shell's lifetime; dropping restores.
pub struct Session;

pub fn boot() -> Option<Session> {
    super::theme::boot_from_env();
    let (cols, rows) = crate::tui::term::size()?;
    let size = Size { cols, rows };
    let usable = size.usable() && crate::tui::term::ansi_enabled();
    usable.then(|| {
        ACTIVE.store(true, Ordering::Release);
        print!("\x1b[?1049h");
        io::stdout().flush().ok();
        Session
    })
}

pub fn size() -> Size {
    let (cols, rows) = crate::tui::term::size().unwrap_or((80, 24));
    Size {
        cols: cols.max(Size::MIN_COLS),
        rows: rows.max(Size::MIN_ROWS),
    }
}

pub fn active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

/// A terminal that shrank below the floor hands the session back to chat
/// mode rather than paint an impossible frame.
pub fn ensure_usable() {
    if active() && !size().usable() {
        suspend();
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
        if ACTIVE.swap(false, Ordering::AcqRel) {
            println!("\x1b[?1049l");
            io::stdout().flush().ok();
        }
    }
}

pub use super::theme::{apply_theme, theme_names};

#[cfg(test)]
#[path = "../tests/screen_boot.rs"]
mod tests;
