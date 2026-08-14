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
/// the tui's install-then-run flow stops scanning twice. The memo is keyed by
/// the workspace that was scanned (the gate scans the current directory), so
/// switching projects always rescans.
use std::sync::Mutex;

static MEMO: Mutex<Option<(String, String)>> = Mutex::new(None);

pub fn memo_hit(gate: &str, workspace: &str) -> bool {
    *MEMO.lock().unwrap() == Some((gate.to_string(), workspace.to_string()))
}

pub fn memo_set(gate: &str, workspace: &str) {
    *MEMO.lock().unwrap() = Some((gate.to_string(), workspace.to_string()));
}

pub fn memo_clear() {
    *MEMO.lock().unwrap() = None;
}

/// Joins gate reader threads: a normal finish drains every pipe fully (the
/// scanner reaped itself, so EOF is guaranteed). Only a killed scanner needs
/// the bound — a pipe-holding descendant would otherwise block the join
/// forever — so the grace period applies solely to the timeout path.
pub fn bounded_join(handles: Vec<std::thread::JoinHandle<String>>, timed_out: bool) -> Vec<String> {
    if !timed_out {
        return handles
            .into_iter()
            .filter_map(|handle| handle.join().ok())
            .collect();
    }
    let grace = std::time::Instant::now() + std::time::Duration::from_millis(500);
    while !handles.iter().all(std::thread::JoinHandle::is_finished)
        && std::time::Instant::now() < grace
    {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    handles
        .into_iter()
        .filter_map(|handle| handle.is_finished().then(|| handle.join().ok()).flatten())
        .collect()
}
