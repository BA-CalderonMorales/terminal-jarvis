//! Heartbeat: while a slow gate scan runs, the clean view redraws a
//! fixed-width progress line so the user knows the scan is alive and why;
//! drawing is pure, property-tested.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub const TICK: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(250);
const TAIL: &str = " · scanning workspace; this can take a minute or more";

/// The live line for an elapsed scan time: CR, fixed-width seconds, padding.
pub fn live_line(prefix: &str, secs: u64) -> String {
    let body = format!("{prefix} {secs:>4}s{TAIL}");
    format!(
        "\r{body}{}",
        " ".repeat(live_width(prefix).saturating_sub(body.len()))
    )
}

/// The stable width every live line is padded to, so ticks leave no residue.
pub fn live_width(prefix: &str) -> usize {
    prefix.len() + 1 + 4 + 1 + TAIL.len()
}

pub fn should_tick(elapsed_secs: u64) -> bool {
    elapsed_secs >= TICK.as_secs() && elapsed_secs % TICK.as_secs() == 0
}

/// A finished scan: outcome, captured output, heartbeat visibility.
#[derive(Debug)]
pub struct Scan {
    pub code: i32,
    pub output: String,
    pub heartbeat: bool,
    /// Wall-clock duration of the whole scan, so timing tests can tell a
    /// real spurious tick from scheduler starvation under parallel load.
    pub elapsed: Duration,
}
/// The background redraw loop for one scan; stop it before printing anything
pub struct Heartbeat {
    stop: Arc<AtomicBool>,
    fired: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Heartbeat {
    pub fn start(prefix: &str) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let fired = Arc::new(AtomicBool::new(false));
        let (stop_loop, fired_flag) = (stop.clone(), fired.clone());
        let line = prefix.to_string();
        let handle = thread::spawn(move || {
            let started = Instant::now();
            let mut drawn = 0;
            loop {
                thread::sleep(POLL);
                if stop_loop.load(Ordering::Relaxed) {
                    break;
                }
                let secs = started.elapsed().as_secs();
                if should_tick(secs) && secs != drawn {
                    drawn = secs;
                    fired_flag.store(true, Ordering::Relaxed);
                    eprint!("{}", live_line(&line, secs));
                    let _ = std::io::stderr().flush();
                }
            }
        });
        Self {
            stop,
            fired,
            handle: Some(handle),
        }
    }

    pub fn fired(&self) -> bool {
        self.fired.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub fn stopped(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
#[path = "../tests/heartbeat_probe.rs"]
mod heartbeat_probe_tests;
#[cfg(test)]
#[path = "../tests/heartbeat.rs"]
mod heartbeat_tests;
