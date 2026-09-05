//! Escape: disambiguates a bare Esc from a CSI/Alt introducer without
//! blocking. The continuation byte is read on a detached thread with a
//! short window; a late byte parks in `PENDING` so nothing is lost.

use super::Key;
use std::io::{self, Read};
use std::sync::mpsc;
use std::time::Duration;

static PENDING: std::sync::Mutex<Vec<u8>> = std::sync::Mutex::new(Vec::new());

/// The window a bare Esc waits for its continuation before standing alone.
const WINDOW: Duration = Duration::from_millis(60);

fn park_clear() {
    PENDING.lock().unwrap_or_else(|e| e.into_inner()).clear();
}

/// Parks one raw byte for the next reader; a byte read by a lingering
/// windowed thread lands here so nothing is ever lost.
pub(crate) fn park_byte(byte: u8) {
    park_clear();
    PENDING.lock().unwrap_or_else(|e| e.into_inner()).push(byte);
}

/// The next input byte: a parked late byte first, then the real reader.
pub fn pending() -> Option<u8> {
    let mut parked = PENDING.lock().unwrap_or_else(|e| e.into_inner());
    if parked.is_empty() {
        None
    } else {
        Some(parked.remove(0))
    }
}

/// The real-terminal entry: resolves the Esc dance on the clock and
/// decodes the replay through the same table synthetic readers use.
/// `None` (no continuation within the window) means bare Esc -- the vim
/// mode switch; a late byte parks for the next read, never drops.
pub fn resolve_and_decode(first: u8) -> Option<Key> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let byte = std::io::stdin()
            .lock()
            .bytes()
            .next()
            .map(|b| b.unwrap_or(0));
        let _ = tx.send(byte);
        if let Some(byte) = byte {
            park_byte(byte);
        }
    });
    let continuation = match rx.recv_timeout(WINDOW) {
        Err(_) => return Some(Key::Escape),
        Ok(None) => return Some(Key::Escape),
        Ok(Some(introducer @ (b'[' | b'O'))) => {
            // the final CSI byte reads directly; the introducer parked
            // itself, so clear it before the replay consumes the triple
            let final_byte = std::io::stdin()
                .lock()
                .bytes()
                .next()
                .map(|b| b.unwrap_or(0));
            park_clear();
            Some((introducer, final_byte))
        }
        Ok(Some(parked)) => {
            // an Alt combo: Esc ignored, the combo key replays untouched
            park_clear();
            park_byte(parked);
            Some((parked, None))
        }
    };
    match continuation {
        None => Some(Key::Ignored),
        Some((second, third)) => {
            let mut bytes = vec![first, second];
            if let Some(third) = third {
                bytes.push(third);
            }
            let mut replay = io::Cursor::new(bytes);
            super::keys::decode(&mut replay, first)
        }
    }
}
