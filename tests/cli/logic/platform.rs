//! Platform shims for CLI fixtures: the execute-marker, fixture filename,
//! gate marker script, and PATH joining rules that differ per platform.

use std::path::Path;

#[cfg(unix)]
pub fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

/// Windows has no execute-bit; a `.cmd` file is runnable purely by extension
/// (see `resolve_on_path` in `security::path`), so there is nothing to
/// mark executable.
#[cfg(not(unix))]
pub fn make_executable(_path: &Path) {}

/// A `#!/bin/sh` fixture script on Unix is written under its bare name; on
/// Windows the same logical fixture is a `.cmd` file, since that is what
/// `resolve_on_path`'s `PATHEXT` search actually finds.
#[cfg(unix)]
pub fn script_filename(name: &str) -> String {
    name.to_string()
}

#[cfg(not(unix))]
pub fn script_filename(name: &str) -> String {
    format!("{name}.cmd")
}

#[cfg(unix)]
pub const GATE_MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_GATE_MARKER\"\n";
#[cfg(not(unix))]
pub const GATE_MARKER_SCRIPT: &str = "@echo off\r\ntype nul > \"%TJ_FIXTURE_GATE_MARKER%\"\r\n";

/// Prepends `dir` onto the current `PATH` using the platform's own list
/// separator (`:` on Unix, `;` on Windows) via `env::join_paths`, rather
/// than a hardcoded `:` that silently breaks PATH parsing on Windows.
pub fn prepend_to_path(dir: &Path) -> std::ffi::OsString {
    let existing = std::env::var_os("PATH").unwrap_or_default();
    let mut dirs = vec![dir.to_path_buf()];
    dirs.extend(std::env::split_paths(&existing));
    std::env::join_paths(dirs).expect("PATH entries join")
}
