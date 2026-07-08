use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub active_harness: String,
}

pub fn default_home() -> PathBuf {
    if let Some(value) = env::var_os("TERMINAL_JARVIS_HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(value);
    }
    config_home().join("terminal-jarvis")
}

fn config_home() -> PathBuf {
    if let Some(value) = env::var_os("XDG_CONFIG_HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(value);
    }
    if let Some(value) = env::var_os("HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(value).join(".config");
    }
    PathBuf::from(".config")
}

pub fn catalog_root() -> PathBuf {
    if let Some(path) = env::var_os("TERMINAL_JARVIS_CATALOG").filter(|path| !path.is_empty()) {
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

pub fn save(home: &Path, harness: &str) -> io::Result<()> {
    fs::create_dir_all(home)?;
    fs::write(
        home.join("session.toml"),
        format!("active_harness = \"{harness}\"\n"),
    )
}

pub fn load(home: &Path) -> io::Result<Option<Session>> {
    let path = home.join("session.toml");
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(path)?;
    let result = parse_active(&data).map(|active_harness| Session { active_harness });
    if result.is_none() && !data.trim().is_empty() {
        eprintln!("warning: session.toml could not be parsed; using defaults");
    }
    Ok(result)
}

fn parse_active(data: &str) -> Option<String> {
    data.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        if key.trim() != "active_harness" {
            return None;
        }
        value
            .trim()
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .map(str::to_string)
    })
}
