use super::{Code, DiagnosticInput};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Resolution {
    pub code: Code,
    pub path: Option<PathBuf>,
    pub matches: usize,
}

pub fn binary(name: &str, input: &DiagnosticInput) -> Resolution {
    if name.trim().is_empty() {
        return result(Code::Malformed, None, 0);
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
    match found.len() {
        0 if denied => result(Code::PermissionDenied, None, 0),
        0 => result(Code::Missing, None, 0),
        1 => result(Code::Ready, found.pop(), 1),
        count => result(Code::Conflicting, found.into_iter().next(), count),
    }
}

pub fn direct(path: &Path) -> Resolution {
    match executable(path) {
        Ok(true) => result(Code::Ready, Some(path.to_path_buf()), 1),
        Ok(false) => result(Code::PermissionDenied, Some(path.to_path_buf()), 0),
        Err(code) => result(code, Some(path.to_path_buf()), 0),
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
    std::iter::once(name.to_string())
        .chain(
            extensions
                .split(';')
                .filter(|e| !e.is_empty())
                .map(|e| format!("{name}{e}")),
        )
        .collect()
}

fn result(code: Code, path: Option<PathBuf>, matches: usize) -> Resolution {
    Resolution {
        code,
        path,
        matches,
    }
}
