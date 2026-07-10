use crate::contracts::{EnvMode, Harness};
use std::env;
use std::path::Path;

pub fn command_on_path(command: &str) -> bool {
    if command.contains('/') || command.contains('\\') {
        return Path::new(command).exists();
    }
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    let path_ext = env::var("PATHEXT").unwrap_or_default();
    candidates(command, cfg!(windows), &path_ext)
        .iter()
        .any(|name| env::split_paths(&path).any(|dir| dir.join(name).exists()))
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
            if harness.env.iter().any(|name| env::var_os(name).is_some()) {
                Vec::new()
            } else {
                harness.env.clone()
            }
        }
        EnvMode::All => harness
            .env
            .iter()
            .filter(|name| env::var_os(name).is_none())
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::candidates;

    #[test]
    fn windows_candidates_include_pathext_extensions() {
        assert_eq!(
            candidates("trivy", true, ".EXE;.CMD"),
            ["trivy", "trivy.EXE", "trivy.CMD"]
        );
    }

    #[test]
    fn executable_extension_is_not_duplicated() {
        assert_eq!(candidates("trivy.exe", true, ".EXE"), ["trivy.exe"]);
    }
}
