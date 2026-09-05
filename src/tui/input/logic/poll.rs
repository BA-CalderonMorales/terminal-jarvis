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
/// On EOF the watcher retires and hands stdin back to direct reads, so a
/// dead watcher can never leave consumers polling an empty queue.
pub fn spawn_watcher() {
    if WATCHER.swap(true, Ordering::AcqRel) {
        return;
    }
    std::thread::spawn(|| {
        while let Some(key) = super::keys::read_stdin_key() {
            park(key);
        }
        WATCHER.store(false, Ordering::Release);
        park(Key::Dead);
    });
}

/// Blocks until a key is parked -- the consumer's read while the watcher
/// owns stdin. If the watcher retired (EOF), the read falls back to
/// stdin directly; the parked `Dead` flows through unchanged.
pub fn wait() -> Option<Key> {
    loop {
        if let Some(key) = take() {
            return Some(key);
        }
        if !watcher_active() {
            return super::keys::read_stdin_key();
        }
        std::thread::sleep(Duration::from_millis(15));
    }
}

/// Bounded `wait`: `None` when silence outlasts `timeout` -- a drain may
/// never block past its window on a tail that never arrives.
pub(crate) fn wait_for(timeout: Duration) -> Option<Key> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(key) = take() {
            return Some(key);
        }
        if !watcher_active() || Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(Duration::from_millis(15));
    }
}
