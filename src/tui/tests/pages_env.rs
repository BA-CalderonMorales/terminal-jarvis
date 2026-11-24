//! Flag-state guard for TUI tests. The `tui-pages` flag resolves from the
//! environment first and then from `settings.toml` under the Terminal Jarvis
//! home. A guard pins the env var, points the home at a fresh empty temp dir
//! (optionally with a settings file), and holds `ENV_LOCK`, so a developer's
//! saved choice never leaks into a test and no reader sees another's value.

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::MutexGuard;

const FLAG: &str = "TJ_FLAG_TUI_PAGES";
const HOME: &str = "TERMINAL_JARVIS_HOME";
static COUNT: AtomicUsize = AtomicUsize::new(0);

/// Restores the previous values and removes the temp home on drop. Declared
/// with the lock last so the environment is put back while still locked.
pub struct FlagGuard {
    flag: Option<OsString>,
    home: Option<OsString>,
    dir: PathBuf,
    _lock: MutexGuard<'static, ()>,
}

/// Pins `TJ_FLAG_TUI_PAGES` to `value` (`None` means off) and an empty home.
pub fn set(value: Option<&str>) -> FlagGuard {
    isolated(value, None)
}

/// As `set`, and writes `settings` as the home's `settings.toml`.
pub fn isolated(value: Option<&str>, settings: Option<&str>) -> FlagGuard {
    let lock = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = std::env::temp_dir().join(format!(
        "tj-tui-pages-{}-{}",
        std::process::id(),
        COUNT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp home");
    if let Some(settings) = settings {
        fs::write(dir.join("settings.toml"), settings).expect("settings file");
    }
    let guard = FlagGuard {
        flag: std::env::var_os(FLAG),
        home: std::env::var_os(HOME),
        dir: dir.clone(),
        _lock: lock,
    };
    std::env::set_var(HOME, &dir);
    // `None` pins the flag OFF with an explicit `0`: the environment outranks
    // settings.toml, so a saved choice can never turn the flag on under test.
    std::env::set_var(FLAG, value.unwrap_or("0"));
    guard
}

impl Drop for FlagGuard {
    fn drop(&mut self) {
        restore(FLAG, self.flag.take());
        restore(HOME, self.home.take());
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn restore(name: &str, previous: Option<OsString>) {
    match previous {
        Some(value) => std::env::set_var(name, value),
        None => std::env::remove_var(name),
    }
}
