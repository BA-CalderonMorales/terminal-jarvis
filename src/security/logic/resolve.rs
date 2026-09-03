//! Resolve: spawn support for gated tools. On Windows, `Command::new` + a
//! bare name never expands `PATHEXT` (the same gap `runtime::run_command`
//! hits), so the real candidate is resolved first and spawned; a `.cmd`
//! shim like npm's is found that way. Unix keeps the borrowed command.

use std::borrow::Cow;
use std::path::PathBuf;

#[cfg(windows)]
pub fn resolved(command: &str) -> Cow<'_, str> {
    match super::path::resolve_on_path(command) {
        Some(resolved) => Cow::Owned(resolved),
        None => Cow::Borrowed(command),
    }
}

#[cfg(not(windows))]
pub fn resolved(command: &str) -> Cow<'_, str> {
    Cow::Borrowed(command)
}

/// A throwaway working directory for the package-check lockfile resolve +
/// scan, named so concurrent checks never collide.
pub fn scoped_dir() -> Option<PathBuf> {
    let leaf = format!(
        "terminal-jarvis-package-check-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_nanos()
    );
    let dir = std::env::temp_dir().join(leaf);
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}
