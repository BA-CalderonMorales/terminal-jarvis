use crate::contracts::{EnvMode, Harness};
use std::env;
use std::path::Path;

pub fn command_on_path(command: &str) -> bool {
    if command.contains('/') || command.contains('\\') {
        return executable(Path::new(command));
    }
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    let path_ext = env::var("PATHEXT").unwrap_or_default();
    candidates(command, cfg!(windows), &path_ext)
        .iter()
        .any(|name| env::split_paths(&path).any(|dir| executable(&dir.join(name))))
}

fn executable(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    executable_mode(&metadata)
}

#[cfg(unix)]
fn executable_mode(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn executable_mode(_metadata: &std::fs::Metadata) -> bool {
    true
}

fn candidates(command: &str, windows: bool, path_ext: &str) -> Vec<String> {
    if !windows || Path::new(command).extension().is_some() {
        return vec![command.to_string()];
    }
    let extensions = if path_ext.is_empty() {
        ".COM;.EXE;.BAT;.CMD"
    } else {
        path_ext
    };
    let mut names = vec![command.to_string()];
    names.extend(
        extensions
            .split(';')
            .filter(|extension| !extension.is_empty())
            .map(|extension| format!("{command}{extension}")),
    );
    names
}

pub fn missing_env(harness: &Harness) -> Vec<String> {
    match harness.env_mode {
        EnvMode::None => Vec::new(),
        EnvMode::Any => {
            if harness.env.iter().any(|name| nonempty_env(name)) {
                Vec::new()
            } else {
                harness.env.clone()
            }
        }
        EnvMode::All => harness
            .env
            .iter()
            .filter(|name| !nonempty_env(name))
            .cloned()
            .collect(),
    }
}

fn nonempty_env(name: &str) -> bool {
    env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

#[cfg(test)]
#[path = "checks_test.rs"]
mod tests;
