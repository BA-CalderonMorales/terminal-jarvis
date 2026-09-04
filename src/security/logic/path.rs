//! Path: whether a command resolves to a runnable binary on `PATH`, and
//! which exact name a spawn should target. The Windows `PATHEXT` expansion
//! here is what makes npm-style `.cmd` shims spawnable.

use std::env;
use std::path::Path;

pub fn command_on_path(command: &str) -> bool {
    resolve_on_path(command).is_some()
}

/// Finds the first `PATH` candidate that exists and is runnable, expanding
/// `PATHEXT` extensions on Windows (e.g. resolving `opencode` to
/// `opencode.CMD`). Spawning the bare name directly via `Command::new` does
/// not perform this expansion, so callers that need to `spawn()` a harness
/// binary should invoke this first and spawn the resolved name.
///
/// "Runnable" is platform-dependent: on Unix this checks the execute
/// permission bit; on Windows, which has no such bit, any existing file
/// counts (see `executable_mode` below) — a `.CMD` match is not guaranteed
/// to actually be a runnable script, only to exist under that name.
///
/// Walks `PATH` directories in order, trying every candidate extension
/// within each directory before moving to the next — never the reverse.
/// Searching extension-by-extension-across-all-directories first would let
/// a same-named binary in a *later* PATH directory (e.g. a real system
/// install) outrank a shadowing binary in an *earlier* one (e.g. a
/// project-local override), which breaks the ordinary "earlier PATH entries
/// win" guarantee every shell provides.
pub fn resolve_on_path(command: &str) -> Option<String> {
    if command.contains('/') || command.contains('\\') {
        return executable(Path::new(command)).then(|| command.to_string());
    }
    let path = env::var_os("PATH")?;
    let path_ext = env::var("PATHEXT").unwrap_or_default();
    let names = candidates(command, cfg!(windows), &path_ext);
    env::split_paths(&path).find_map(|dir| {
        names
            .iter()
            .find(|name| executable(&dir.join(name)))
            .cloned()
    })
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

/// Candidate file names to probe for `command`, in preference order.
///
/// On Windows, PATHEXT-suffixed names come *before* the bare name. A bare
/// name is not directly launchable there (Windows has no "this file is
/// runnable" bit the way Unix has the execute permission, so any file counts
/// as "executable" per `executable_mode` below) — and many npm-installed
/// tools ship an extensionless POSIX shebang shim (for Git Bash/WSL) right
/// next to the real `.cmd`/`.exe` entry point. Checking the bare name first
/// would match that unrunnable shim before ever trying `.CMD`/`.EXE`.
pub fn candidates(command: &str, windows: bool, path_ext: &str) -> Vec<String> {
    if !windows || Path::new(command).extension().is_some() {
        return vec![command.to_string()];
    }
    let extensions = if path_ext.is_empty() {
        ".COM;.EXE;.BAT;.CMD"
    } else {
        path_ext
    };
    let mut names: Vec<String> = extensions
        .split(';')
        .filter(|extension| !extension.is_empty())
        .map(|extension| format!("{command}{extension}"))
        .collect();
    names.push(command.to_string());
    names
}

#[cfg(test)]
#[path = "../tests/path.rs"]
mod path_tests;

#[cfg(test)]
#[path = "../tests/props.rs"]
mod props;
