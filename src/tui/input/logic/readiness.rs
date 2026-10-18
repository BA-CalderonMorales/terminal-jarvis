//! Readiness: is stdin readable yet? A `poll(2)` tick so a reader wakes on a
//! resize even when no key arrives, and never blocks past the tick. Off unix
//! the check is a plain yes: the read blocks, and a console resize arrives
//! through the input record rather than a signal.

use std::time::Duration;

/// The tick a reader waits between resize checks.
pub const TICK: Duration = Duration::from_millis(60);

/// Blocks until stdin has a byte or a resize lands. `false` means a resize:
/// the caller bails out of its read so the frame loop can repaint.
pub fn wait_for_input() -> bool {
    while !super::escape::has_pending() && !stdin_ready(TICK) {
        if crate::tui::resize::take() {
            return false;
        }
    }
    true
}

/// True when a byte is ready within `timeout`. A signal (the resize) makes
/// `poll` return early with an error, which reads as "not ready".
#[cfg(unix)]
pub fn stdin_ready(timeout: Duration) -> bool {
    #[repr(C)]
    struct PollFd {
        fd: i32,
        events: i16,
        revents: i16,
    }
    // `nfds_t` is `unsigned long` on Linux and `unsigned int` elsewhere;
    // passing the wrong width would let the callee read a garbage count.
    #[cfg(target_os = "linux")]
    type Nfds = u64;
    #[cfg(not(target_os = "linux"))]
    type Nfds = u32;
    unsafe extern "C" {
        fn poll(fds: *mut PollFd, nfds: Nfds, timeout: i32) -> i32;
    }
    const POLLIN: i16 = 0x001;
    let mut fd = PollFd {
        fd: 0,
        events: POLLIN,
        revents: 0,
    };
    let ms = timeout.as_millis().min(i32::MAX as u128) as i32;
    unsafe { poll(&mut fd, 1, ms) > 0 }
}

#[cfg(not(unix))]
pub fn stdin_ready(_timeout: Duration) -> bool {
    true
}
