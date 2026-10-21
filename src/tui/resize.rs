//! Resize: the SIGWINCH handler, the flag it sets, and the one clear a size
//! change needs.
//!
//! A terminal resize must repaint the frame without waiting for a keystroke,
//! so the handler is async-signal-safe -- it only stores to an atomic -- and
//! the readers poll the flag on a short tick. Nothing here writes a frame: the
//! loop that owns it paints, and [`settle`] only decides whether that paint
//! has to clear first, so a resize can never corrupt a half-written row.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static RESIZED: AtomicBool = AtomicBool::new(false);

/// The size the last painted frame was composed for.
static PAINTED: Mutex<Option<(usize, usize)>> = Mutex::new(None);

/// Clears the screen once per size change, then passes the frame through.
///
/// A frame pads its rows to `cols - 2` so a full row can never set the
/// terminal's pending-wrap state, which leaves the last two columns holding
/// whatever the previous frame wrote there. That is invisible while the
/// terminal keeps its size, and a stale tail as soon as it shrinks. Erasing
/// every frame would fix it and flicker; clearing only when the size changes
/// leaves steady-state paints overwriting in place.
pub fn settle(painted: String, cols: usize, rows: usize) -> String {
    let mut last = PAINTED.lock().unwrap_or_else(|e| e.into_inner());
    settle_at(painted, &mut last, cols, rows)
}

/// The decision, against state the caller owns, so a test can pin it.
fn settle_at(
    painted: String,
    last: &mut Option<(usize, usize)>,
    cols: usize,
    rows: usize,
) -> String {
    if *last == Some((cols, rows)) {
        return painted;
    }
    *last = Some((cols, rows));
    // Erase only: a frame homes its own cursor, and a second home reads to the
    // frame reader as an extra, empty frame.
    format!("\x1b[2J{painted}")
}
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
