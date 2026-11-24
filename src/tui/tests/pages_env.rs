//! Flag-state guard for TUI tests. The `tui-pages` flag is read from the
//! process environment, so every test that reads a page that the flag can
//! change takes `ENV_LOCK` and pins the variable; a test that flips it holds
//! the same lock, so no reader ever sees another test's value.

use std::ffi::OsString;
use std::sync::MutexGuard;

const VAR: &str = "TJ_FLAG_TUI_PAGES";

/// Restores the previous value on drop; declared before the lock so the
/// environment is put back while the lock is still held.
pub struct FlagGuard {
    previous: Option<OsString>,
    _lock: MutexGuard<'static, ()>,
}

/// Pins `TJ_FLAG_TUI_PAGES` to `value` (`None` removes it) until dropped.
pub fn set(value: Option<&str>) -> FlagGuard {
    let lock = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let previous = std::env::var_os(VAR);
    match value {
        Some(value) => std::env::set_var(VAR, value),
        None => std::env::remove_var(VAR),
    }
    FlagGuard {
        previous,
        _lock: lock,
    }
}

impl Drop for FlagGuard {
    fn drop(&mut self) {
        match self.previous.take() {
            Some(value) => std::env::set_var(VAR, value),
            None => std::env::remove_var(VAR),
        }
    }
}
