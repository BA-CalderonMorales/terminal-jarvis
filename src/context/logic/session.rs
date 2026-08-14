use crate::context::constants::env as env_const;
use crate::context::logic::session_parse::{parse, ParseError};
use crate::context::structs::session::Session;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn default_home() -> PathBuf {
    if let Some(value) = env::var_os(env_const::HOME).filter(|value| !value.is_empty()) {
        return PathBuf::from(value);
    }
    config_home().join("terminal-jarvis")
}

fn config_home() -> PathBuf {
    if let Some(value) = env::var_os(env_const::XDG_CONFIG_HOME).filter(|value| !value.is_empty()) {
        return PathBuf::from(value);
    }
    if let Some(value) = env::var_os("HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(value).join(".config");
    }
    PathBuf::from(".config")
}

pub fn catalog_root() -> PathBuf {
    if let Some(path) = env::var_os(env_const::CATALOG).filter(|path| !path.is_empty()) {
        return PathBuf::from(path);
    }
    catalog_candidates()
        .into_iter()
        .find(|path| path.is_dir())
        .unwrap_or_else(|| PathBuf::from("harnesses"))
}

fn catalog_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(cwd) = env::current_dir() {
        candidates.push(cwd.join("harnesses"));
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(bin) = exe.parent() {
            candidates.push(bin.join("harnesses"));
            if let Some(root) = bin.parent() {
                candidates.push(root.join("harnesses"));
                candidates.push(root.join("share/terminal-jarvis/harnesses"));
            }
        }
    }
    candidates
}

/// Writes the session through a pid+nonce staged sibling file so a crash or
/// interrupted write can never leave an empty or partial session behind and
/// concurrent writers never collide. On windows, where rename cannot replace
/// an existing file, the destination is removed only after staging succeeds.
pub fn save(home: &Path, harness: &str) -> io::Result<()> {
    if harness.contains(['"', '\n']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "harness name must be a plain word",
        ));
    }
    fs::create_dir_all(home)?;
    let path = home.join("session.toml");
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let staged = home.join(format!("session.toml.{}.{nonce}.tmp", std::process::id()));
    fs::write(&staged, format!("active_harness = \"{harness}\"\n"))?;
    match fs::rename(&staged, &path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            fs::remove_file(&path)?;
            fs::rename(&staged, &path)
        }
        Err(error) => {
            let _ = fs::remove_file(&staged);
            Err(error)
        }
    }
}

pub fn load(home: &Path) -> io::Result<Option<Session>> {
    let path = home.join("session.toml");
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(path)?;
    match parse(&data) {
        Ok(Some(active_harness)) => Ok(Some(Session { active_harness })),
        Ok(None) => Ok(None),
        Err(_) => {
            eprintln!("warning: session.toml could not be parsed; using defaults");
            Ok(None)
        }
    }
}

#[cfg(test)]
#[path = "../tests/session.rs"]
mod tests;
