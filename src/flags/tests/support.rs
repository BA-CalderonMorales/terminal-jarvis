//! Fixtures: flag files as strings, fixture registries on disk, an env guard.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::MutexGuard;

static ID: AtomicUsize = AtomicUsize::new(0);

pub fn flag_file(name: &str, default: bool, stage: &str) -> String {
    format!(
        "name = \"{name}\"\ndefault = {default}\nstage = \"{stage}\"\n\
         description = \"A fixture flag for {name}.\"\nsince = \"0.1.26\"\n"
    )
}

pub fn temp(tag: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "tj-flags-{tag}-{}-{}",
        std::process::id(),
        ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

pub fn registry(files: &[(&str, bool, &str)]) -> PathBuf {
    let root = temp("registry");
    for (name, default, stage) in files {
        fs::create_dir_all(root.join(name)).unwrap();
        let data = flag_file(name, *default, stage);
        fs::write(root.join(name).join("index.toml"), data).unwrap();
    }
    root
}

/// Holds `ENV_LOCK` and restores the named variables on drop, panic or not.
pub struct Env {
    _lock: MutexGuard<'static, ()>,
    saved: Vec<(String, Option<std::ffi::OsString>)>,
}

impl Env {
    pub fn new() -> Self {
        let lock = crate::ENV_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Self {
            _lock: lock,
            saved: Vec::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: impl AsRef<Path>) {
        self.saved.push((key.to_string(), std::env::var_os(key)));
        std::env::set_var(key, value.as_ref());
    }

    pub fn remove(&mut self, key: &str) {
        self.saved.push((key.to_string(), std::env::var_os(key)));
        std::env::remove_var(key);
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..).rev() {
            match value {
                Some(value) => std::env::set_var(&key, value),
                None => std::env::remove_var(&key),
            }
        }
    }
}
