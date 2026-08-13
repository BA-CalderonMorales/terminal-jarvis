//! Heartbeat: while a slow gate scan runs, the clean view redraws a
//! fixed-width progress line every few seconds so the user knows the scan is
//! alive and why it takes time, instead of staring at one static line. The
//! line drawing is a pure function so the redraw contract is
//! property-tested; the thread is a small racy-free wrapper around it.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub const TICK: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(250);
const TAIL: &str = " · scanning workspace; this can take a minute or more";

/// The live line for an elapsed scan time: CR, fixed-width seconds, the
/// patience note, then padding so every tick is the same width.
pub fn live_line(prefix: &str, secs: u64) -> String {
    let body = format!("{prefix} {secs:>4}s{TAIL}");
    format!(
        "\r{body}{}",
        " ".repeat(live_width(prefix).saturating_sub(body.len()))
    )
}

/// The stable width every live line is padded to, so ticks never leave
/// residue and the final outcome rewrite can overpad past them.
pub fn live_width(prefix: &str) -> usize {
    prefix.len() + 1 + 4 + 1 + TAIL.len()
}

/// A finished scan: its outcome, its captured output, and whether the user
/// saw live heartbeat ticks while it ran (the outcome line overpads past
/// those ticks so no residue survives).
#[derive(Debug)]
pub struct Scan {
    pub code: i32,
    pub output: String,
    pub heartbeat: bool,
}

/// The background redraw loop for one scan. Stop it before printing anything
/// after the scan so a late tick cannot race the outcome line.
pub struct Heartbeat {
    stop: Arc<AtomicBool>,
    fired: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

impl Heartbeat {
    pub fn start(prefix: &str) -> Self {
        let (stop, fired) = (
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(false)),
        );
        let (stop_loop, fired_flag) = (stop.clone(), fired.clone());
        let line = prefix.to_string();
        let handle = thread::spawn(move || {
            let started = Instant::now();
            loop {
                if stop_loop.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(POLL);
                if stop_loop.load(Ordering::Relaxed) {
                    break;
                }
                let secs = started.elapsed().as_secs();
                if secs >= TICK.as_secs() && secs % TICK.as_secs() == 0 {
                    fired_flag.store(true, Ordering::Relaxed);
                    eprint!("{}", live_line(&line, secs));
                    let _ = std::io::stderr().flush();
                }
            }
        });
        Self {
            stop,
            fired,
            handle,
        }
    }

    pub fn fired(&self) -> bool {
        self.fired.load(Ordering::Relaxed)
    }

    pub fn stop(self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.handle.join();
    }
}

#[cfg(test)]
#[path = "../tests/heartbeat.rs"]
mod heartbeat_tests;
