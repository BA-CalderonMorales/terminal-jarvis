//! The pid of the gate scan currently running, if any. The tui's SIGINT
//! handler reads this to kill a stuck scanner outright (trivy's graceful
//! shutdown can idle for minutes) instead of begging it once more.

use std::sync::atomic::{AtomicI32, Ordering};

static ACTIVE_CHILD: AtomicI32 = AtomicI32::new(0);

pub fn track(pid: i32) {
    ACTIVE_CHILD.store(pid, Ordering::Release);
}

pub fn active_pid() -> i32 {
    ACTIVE_CHILD.load(Ordering::Acquire)
}

/// Session memo of the passed gate scan: a workspace scan is expensive (TIO
/// GWINSZ-grade waits on slow mounts), and its verdict covers every action in
/// this session, so a passed scan is not repeated. Critical: the memo is
/// process-global, which means headless one-shot invocations never hit it and
/// the tui's install-then-run flow stops scanning twice.
use std::sync::Mutex;

static MEMO: Mutex<Option<String>> = Mutex::new(None);

pub fn memo_hit(gate: &str) -> bool {
    MEMO.lock().unwrap().as_deref() == Some(gate)
}

pub fn memo_set(gate: &str) {
    *MEMO.lock().unwrap() = Some(gate.to_string());
}

pub fn memo_clear() {
    *MEMO.lock().unwrap() = None;
}
