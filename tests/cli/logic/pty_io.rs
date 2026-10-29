//! PtyIo: the background byte drain and marker wait shared by the pty
//! probes. Reads the master end until the child closes it.

use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

pub fn drain(mut master: File, bytes: Arc<Mutex<Vec<u8>>>, stopped: Arc<AtomicBool>) {
    let mut buffer = [0u8; 65536];
    loop {
        if !readable(master.as_raw_fd()) {
            if stopped.load(Ordering::Acquire) {
                break;
            }
            continue;
        }
        match master.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => bytes.lock().unwrap().extend_from_slice(&buffer[..size]),
            Err(error) if error.raw_os_error() == Some(5) => break,
            Err(error) => panic!("pseudo-terminal read failed: {error}"),
        }
    }
}

pub fn wait_until(bytes: &Arc<Mutex<Vec<u8>>>, marker: &[u8], deadline: Instant) {
    // Generous on purpose: a slow scanner's heartbeat thread can be starved
    // for seconds under a loaded `cargo test`, and a gate that fails on the
    // host's scheduler is not a gate. A genuine hang still fails, later.
    let deadline = deadline.min(Instant::now() + Duration::from_secs(30));
    while Instant::now() < deadline {
        let seen = bytes.lock().unwrap().clone();
        if seen.windows(marker.len()).any(|window| window == marker) {
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
    let seen = bytes.lock().unwrap();
    let tail = &seen[seen.len().saturating_sub(2000)..];
    panic!(
        "timed out waiting for {:?}; tail: {}",
        String::from_utf8_lossy(marker),
        String::from_utf8_lossy(tail)
    );
}

fn readable(fd: i32) -> bool {
    #[repr(C)]
    struct PollFd {
        fd: i32,
        events: i16,
        revents: i16,
    }
    #[cfg(target_os = "linux")]
    type Nfds = u64;
    #[cfg(not(target_os = "linux"))]
    type Nfds = u32;
    unsafe extern "C" {
        fn poll(fds: *mut PollFd, nfds: Nfds, timeout: i32) -> i32;
    }
    let mut fd = PollFd {
        fd,
        events: 1,
        revents: 0,
    };
    unsafe { poll(&mut fd, 1, 100) > 0 }
}
