//! Poll: the shared decoded-key queue. Once a conversation spawns the
//! watcher thread, it is the SOLE stdin reader -- every key it decodes
//! parks here, and consumers (prompt, consent, the live turn loop) take
//! keys from the queue instead of racing stdin. `drain_answer` eats the
//! tail of a typed answer so "yes<Enter>" never leaks "es" into the prompt.

use super::Key;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static WATCHER: AtomicBool = AtomicBool::new(false);
static PARKED: Mutex<Vec<Key>> = Mutex::new(Vec::new());

fn park(key: Key) {
    PARKED.lock().unwrap_or_else(|e| e.into_inner()).push(key);
}

/// The next parked key, if any.
pub fn take() -> Option<Key> {
    let mut parked = PARKED.lock().unwrap_or_else(|e| e.into_inner());
    if parked.is_empty() {
        None
    } else {
        Some(parked.remove(0))
    }
}

/// Every key already parked, without blocking.
pub fn drained() -> Vec<Key> {
    std::mem::take(&mut PARKED.lock().unwrap_or_else(|e| e.into_inner()))
}

/// True while the watcher thread owns stdin; consumers poll the queue
/// instead of reading the terminal directly.
pub fn watcher_active() -> bool {
    WATCHER.load(Ordering::Acquire)
}

/// Spawns the sole stdin reader: it decodes keys and parks them until the
/// process ends. Idempotent -- the first spawn wins, later turns reuse it.
pub fn spawn_watcher() {
    if WATCHER.swap(true, Ordering::AcqRel) {
        return;
    }
    std::thread::spawn(|| {
        while let Some(key) = super::keys::read_stdin_key() {
            park(key);
        }
        park(Key::Dead);
    });
}

/// Blocks until a key is parked -- the consumer's read while the watcher
/// owns stdin. The parked `Dead` (EOF) flows through unchanged.
pub fn wait() -> Option<Key> {
    loop {
        if let Some(key) = take() {
            return Some(key);
        }
        std::thread::sleep(Duration::from_millis(15));
    }
}

/// Eats the tail of a typed answer so leftover keystrokes ("es" of a
/// typed "yes") never reach the prompt buffer: parked keys up to the
/// Enter when the watcher owns stdin, buffered bytes otherwise.
pub fn drain_answer(limit: Duration) -> usize {
    let deadline = Instant::now() + limit;
    let mut eaten = 0;
    if watcher_active() {
        while Instant::now() < deadline {
            match take() {
                Some(Key::Enter) | None => break,
                Some(_) => eaten += 1,
            }
        }
    } else {
        let mut sin = std::io::stdin().lock();
        while Instant::now() < deadline {
            match next_tail_byte(&mut sin) {
                Some(b'\r' | b'\n') | None => break,
                Some(_) => eaten += 1,
            }
        }
    }
    eaten
}

/// The next tail byte: an escape-resolver leftover first, then stdin;
/// `None` ends the tail (EOF or a closed pipe).
fn next_tail_byte(sin: &mut std::io::StdinLock<'_>) -> Option<u8> {
    use std::io::Read;
    super::escape::pending().or_else(|| {
        let mut one = [0u8; 1];
        sin.read(&mut one).ok().filter(|n| *n > 0).map(|_| one[0])
    })
}
