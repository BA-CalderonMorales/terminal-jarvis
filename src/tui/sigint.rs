//! Sigint: scoped SIGINT handling. While a guarded action runs, children
//! keep SIG_DFL (an agent dies on Ctrl+C; the shell survives; a stuck gate
//! scanner is SIGKILLed outright). Idle Ctrl+C wipes the echoed `^C` and
//! reprints the prompt prefix -- suppressed in viewport mode, which owns
//! its frame. Std-only libc FFI.
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};

static CHILD_RUNNING: AtomicBool = AtomicBool::new(false);
static ANSI: AtomicBool = AtomicBool::new(false);
static PREFIX: AtomicPtr<u8> = AtomicPtr::new(std::ptr::null_mut());
static PREFIX_LEN: AtomicUsize = AtomicUsize::new(0);

/// Marks the window where a child owns the terminal. Redraw only when false.
pub fn child_running(running: bool) {
    CHILD_RUNNING.store(running, Ordering::Release);
}

pub fn guarded<T>(body: impl FnOnce() -> T) -> T {
    #[cfg(unix)]
    {
        ANSI.store(super::term::ansi_enabled(), Ordering::Relaxed);
        set_sigint(refresh as *const () as usize);
    }
    let result = body();
    #[cfg(unix)]
    {
        set_sigint(SIG_DFL);
    }
    result
}

/// Hands the handler the fully rendered prompt prefix (indicator, colored)
/// so an idle Ctrl+C redraws the exact frame. The string must outlive the
/// process; the shell passes a leaked prefix.
pub fn remember_prefix(prefix: &'static str) {
    PREFIX_LEN.store(prefix.len(), Ordering::Release);
    PREFIX.store(prefix.as_ptr() as *mut u8, Ordering::Release);
}

pub fn should_redraw(child_running: bool, ansi: bool) -> bool {
    !child_running && ansi
}

#[cfg(unix)]
extern "C" fn refresh(_signum: i32) {
    if CHILD_RUNNING.load(Ordering::Acquire) {
        if let Some(pid) = system_gate_pid() {
            let _ = unsafe { raw_kill(pid, 9) };
        }
        return;
    }
    if !should_redraw(false, ANSI.load(Ordering::Relaxed)) {
        return;
    }
    if crate::tui::screen::active() {
        // The viewport owns its frame; erasing rows here would corrupt it.
        return;
    }
    let _ = unsafe { raw_write(1, CORE_FRAME.as_ptr(), CORE_FRAME.len()) };
    let len = PREFIX_LEN.load(Ordering::Acquire);
    let prefix = PREFIX.load(Ordering::Acquire);
    if len > 0 && !prefix.is_null() {
        let _ = unsafe { raw_write(1, prefix as *const u8, len) };
    }
}

/// The current gate scan's pid, or None when nothing is scanning. Killing
/// the scanner outright beats trivy's multi-minute graceful shutdown.
fn system_gate_pid() -> Option<i32> {
    let pid = crate::gates::active_child_pid();
    (pid > 0).then_some(pid)
}

/// Erase the echoed control character, then reprint the stored prefix.
const CORE_FRAME: &[u8] = b"\r\x1b[2K";

#[cfg(unix)]
extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
    #[link_name = "write"]
    fn raw_write(fd: i32, buf: *const u8, count: usize) -> isize;
    #[link_name = "kill"]
    fn raw_kill(pid: i32, signal: i32) -> i32;
}

#[cfg(unix)]
const SIGINT: i32 = 2;
#[cfg(unix)]
const SIG_DFL: usize = 0;

#[cfg(unix)]
fn set_sigint(handler: usize) {
    unsafe {
        signal(SIGINT, handler);
    }
}

#[cfg(test)]
#[path = "tests/sigint_test.rs"]
mod sigint_tests;
