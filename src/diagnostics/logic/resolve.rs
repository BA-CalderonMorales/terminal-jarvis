use super::{Code, DiagnosticInput};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Resolution {
    pub code: Code,
    pub path: Option<PathBuf>,
    pub paths: Vec<PathBuf>,
    pub matches: usize,
}
pub fn binary(name: &str, input: &DiagnosticInput) -> Resolution {
    if name.trim().is_empty() {
        return result(Code::Malformed, None, Vec::new(), 0);
    }
    if name.contains('/') || name.contains('\\') {
        return direct(Path::new(name));
    }
    let mut found = Vec::new();
    let mut denied = false;
    for directory in input.environment.paths() {
        for candidate in candidates(name, input) {
            let path = directory.join(candidate);
            match executable(&path) {
                Ok(true) => found.push(path),
                Ok(false) => denied = true,
                Err(Code::PermissionDenied) => denied = true,
                Err(_) => {}
            }
        }
    }
    let mut unique = BTreeSet::new();
    found.retain(|path| unique.insert(fs::canonicalize(path).unwrap_or_else(|_| path.clone())));
    let paths = found.clone();
    match paths.len() {
        0 if denied => result(Code::PermissionDenied, None, paths, 0),
        0 => result(Code::Missing, None, paths, 0),
        1 => result(Code::Ready, paths.first().cloned(), paths, 1),
        count => result(Code::Conflicting, paths.first().cloned(), paths, count),
    }
}

pub fn direct(path: &Path) -> Resolution {
    let owned = path.to_path_buf();
    match executable(path) {
        Ok(true) => result(Code::Ready, Some(owned.clone()), vec![owned], 1),
        Ok(false) => result(Code::PermissionDenied, Some(owned), Vec::new(), 0),
        Err(code) => result(code, Some(owned), Vec::new(), 0),
    }
}

fn executable(path: &Path) -> Result<bool, Code> {
    let metadata = fs::metadata(path).map_err(|error| super::inspect::io_code(&error))?;
    if !metadata.is_file() {
        return Err(Code::Malformed);
    }
    Ok(executable_mode(&metadata))
}

#[cfg(unix)]
fn executable_mode(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn executable_mode(_: &fs::Metadata) -> bool {
    true
}

fn candidates(name: &str, input: &DiagnosticInput) -> Vec<String> {
    if input.platform.os != "windows" || Path::new(name).extension().is_some() {
        return vec![name.to_string()];
    }
    let extensions = input
        .environment
        .text("PATHEXT")
        .unwrap_or(".COM;.EXE;.BAT;.CMD");
    extensions
        .split(';')
        .filter(|ext| !ext.is_empty())
        .map(|ext| format!("{name}{ext}"))
        .collect()
}
fn result(code: Code, path: Option<PathBuf>, paths: Vec<PathBuf>, matches: usize) -> Resolution {
    Resolution {
        code,
        path,
        paths,
        matches,
    }
}

#[cfg(test)]
#[path = "../tests/resolve.rs"]
mod tests;
