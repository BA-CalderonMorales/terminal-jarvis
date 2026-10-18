//! Escape: disambiguates a bare Esc from a CSI/Alt introducer without
//! blocking. The continuation byte is read on a detached thread with a
//! short window; a late byte parks in `PENDING` so nothing is lost.

use super::Key;
use std::io;
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

/// Whether a late byte is parked, without consuming it -- the readiness
/// wait must not sleep past a byte the escape dance already captured.
pub fn has_pending() -> bool {
    !PENDING.lock().unwrap_or_else(|e| e.into_inner()).is_empty()
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
        // Wait for the continuation WITHOUT holding the stdin lock past the
        // window. A read that blocks here would deadlock the next keypress:
        // the reader takes the same lock before it can look at the parked byte.
        let byte = super::readiness::stdin_ready(WINDOW)
            .then(super::raw_stdin::read_byte)
            .flatten();
        let _ = tx.send(byte);
        if let Some(byte) = byte {
            park_byte(byte);
        }
    });
    let second = match rx.recv_timeout(WINDOW) {
        // no continuation: a bare Esc -- the vim mode switch
        Err(_) | Ok(None) => return Some(Key::Escape),
        Ok(Some(second)) => second,
    };
    if second == 0x1b {
        // A second Esc is its own keypress, not an Alt combo: pressing Esc
        // twice in quick succession must not swallow both.
        park_clear();
        return Some(Key::Escape);
    }
    if second != b'[' && second != b'O' {
        // An Alt combo: the Esc is ignored and the combo key stands alone.
        // Replaying the pair would decode to `Ignored` and eat the key too.
        park_clear();
        return super::keys::decode(&mut io::Cursor::new(vec![second]), second);
    }
    // A CSI/SS3 introducer: the final byte reads directly; the introducer
    // parked itself, so clear it before the replay consumes the triple.
    let final_byte = super::raw_stdin::read_byte();
    park_clear();
    let mut bytes = vec![first, second];
    if let Some(third) = final_byte {
        bytes.push(third);
    }
    let mut replay = io::Cursor::new(bytes);
    super::keys::decode(&mut replay, first)
}
