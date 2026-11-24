//! The user's choices on disk: `settings.toml` in the Terminal Jarvis home.

use super::settings_lock::Lock;
use super::settings_parse::{parse, render, Choices};
use crate::flags::constants::fixed::SETTINGS_FILE;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

static WARNED: AtomicBool = AtomicBool::new(false);

pub fn path(home: &Path) -> PathBuf {
    home.join(SETTINGS_FILE)
}

/// The saved choices for runtime reads. A missing file is no choices; an
/// unreadable or malformed one warns on stderr (once per process) and also
/// means no choices, like `session.toml`.
pub fn load(home: &Path) -> Choices {
    read_strict(home).unwrap_or_else(|_| {
        if !WARNED.swap(true, Ordering::Relaxed) {
            eprintln!("warning: settings.toml could not be parsed; using defaults");
        }
        Choices::new()
    })
}

/// The saved choices for writers: only a missing file is empty; any other
/// read or parse failure is an error, so a write never builds on a guess.
pub fn read_strict(home: &Path) -> io::Result<Choices> {
    let file = path(home);
    let data = match std::fs::read_to_string(&file) {
        Ok(data) => data,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Choices::new()),
        Err(error) => return Err(error),
    };
    parse(&data).map_err(|message| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} could not be parsed ({message})", file.display()),
        )
    })
}

/// Saves one choice (`None` removes it) atomically, keeping the rest. The
/// read-modify-write runs under a cross-process lock; a corrupt file is
/// reported and left untouched.
pub fn save(home: &Path, name: &str, value: Option<bool>) -> io::Result<()> {
    std::fs::create_dir_all(home)?;
    let _lock = Lock::acquire(&path(home))?;
    let mut choices = read_strict(home)?;
    match value {
        Some(value) => choices.insert(name.to_string(), value),
        None => choices.remove(name),
    };
    crate::context::write_atomic(&path(home), &render(&choices))
}
