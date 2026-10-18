//! Resize: the SIGWINCH handler and the flag it sets.
//!
//! A terminal resize must repaint the frame without waiting for a keystroke,
//! so the handler is async-signal-safe -- it only stores to an atomic -- and
//! the readers poll the flag on a short tick. Nothing here touches the frame:
//! the loop that owns the frame repaints when it sees the event, so a resize
//! can never corrupt a half-written row.

use std::sync::atomic::{AtomicBool, Ordering};

static RESIZED: AtomicBool = AtomicBool::new(false);

/// Consumes one resize event: true at most once per signal.
pub fn take() -> bool {
    RESIZED.swap(false, Ordering::AcqRel)
}

/// True without consuming, for a reader that only needs to know it happened.
pub fn pending() -> bool {
    RESIZED.load(Ordering::Acquire)
}

/// Marks a resize from inside the handler; tests call it directly.
pub fn note() {
    RESIZED.store(true, Ordering::Release);
}

/// Installs the resize handler for the session and restores the default on
/// drop. Off unix this is inert: a console resize arrives through the input
/// record, not a signal.
pub fn watch() -> Guard {
    #[cfg(unix)]
    unsafe {
        signal(SIGWINCH, on_winch as *const () as usize);
    }
    Guard
}

pub struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            signal(SIGWINCH, SIG_DFL);
        }
    }
}

#[cfg(unix)]
extern "C" fn on_winch(_signum: i32) {
    note();
}

#[cfg(unix)]
unsafe extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

#[cfg(unix)]
const SIGWINCH: i32 = 28;
#[cfg(unix)]
const SIG_DFL: usize = 0;

#[cfg(test)]
#[path = "tests/resize.rs"]
mod tests;
