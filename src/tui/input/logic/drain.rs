//! Drain: the answer-tail eater. After an in-frame consent reads its
//! one keystroke, leftover typed bytes ("es" of a typed "yes") must
//! never reach the prompt buffer. Every read is windowed: buffered
//! bytes arrive instantly, silence breaks the drain, and a byte that
//! arrives late parks for the next reader -- never a hang, never a loss.

use super::poll::{wait_for, watcher_active};
use super::Key;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Eats the tail of a typed answer so leftover keystrokes ("es" of a
/// typed "yes") never reach the prompt buffer. Reads through a windowed
/// byte reader: buffered bytes arrive instantly, silence breaks the
/// drain, and a byte that arrives late parks for the next reader --
/// never a hang, never a loss.
pub fn drain_answer(limit: Duration) -> usize {
    let deadline = Instant::now() + limit;
    let mut eaten = 0;
    while Instant::now() < deadline {
        match tail_byte(limit) {
            Some(b'\r' | b'\n') | None => break,
            Some(_) => eaten += 1,
        }
    }
    eaten
}

/// The next tail byte within `window`: a parked escape-leftover first,
/// then -- when the watcher does not own stdin -- stdin through a
/// windowed reader. `None` ends the tail; a late byte parks.
pub(crate) fn tail_byte(window: Duration) -> Option<u8> {
    if let Some(byte) = super::escape::pending() {
        return match byte {
            b'\r' | b'\n' => None,
            other => Some(other),
        };
    }
    if watcher_active() {
        // the tail surfaces as parked keys; anything that is not the
        // Enter is tail junk -- counted, discarded
        match wait_for(window) {
            Some(Key::Enter) | None => None,
            Some(_) => Some(b'x'),
        }
    } else {
        windowed_byte(window)
    }
}

/// Reads one raw byte on a detached thread; a byte that outlives the
/// window parks in the escape queue for the next reader.
fn windowed_byte(window: Duration) -> Option<u8> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        use std::io::Read;
        let mut one = [0u8; 1];
        let byte = std::io::stdin()
            .lock()
            .read(&mut one)
            .ok()
            .filter(|read| *read > 0)
            .map(|_| one[0]);
        if tx.send(byte).is_err() {
            if let Some(byte) = byte {
                super::escape::park_byte(byte);
            }
        }
    });
    rx.recv_timeout(window).ok().flatten()
}
