//! The user's choices on disk: `settings.toml` in the Terminal Jarvis home.

use super::settings_parse::{parse, render, Choices};
use crate::flags::constants::fixed::SETTINGS_FILE;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

static WARNED: AtomicBool = AtomicBool::new(false);

pub fn path(home: &Path) -> PathBuf {
    home.join(SETTINGS_FILE)
}

/// The saved choices. A missing file is no choices; a malformed one warns on
/// stderr (once per process) and also means no choices, like `session.toml`.
pub fn load(home: &Path) -> Choices {
    let Ok(data) = std::fs::read_to_string(path(home)) else {
        return Choices::new();
    };
    parse(&data).unwrap_or_else(|_| {
        if !WARNED.swap(true, Ordering::Relaxed) {
            eprintln!("warning: settings.toml could not be parsed; using defaults");
        }
        Choices::new()
    })
}

/// Saves one choice (`None` removes it) atomically, keeping the rest.
pub fn save(home: &Path, name: &str, value: Option<bool>) -> io::Result<()> {
    let mut choices = load(home);
    match value {
        Some(value) => choices.insert(name.to_string(), value),
        None => choices.remove(name),
    };
    crate::context::write_atomic(&path(home), &render(&choices))
}
