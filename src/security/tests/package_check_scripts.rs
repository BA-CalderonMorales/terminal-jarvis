//! Platform script fixtures: the same logical fixture script per platform
//! (`#!/bin/sh` heredocs on Unix, `@echo off` batches on Windows) plus the
//! executable-marker and filename rules the fakes must follow.

use std::path::Path;

#[cfg(unix)]
pub const EXIT_OK_SCRIPT: &str = "#!/bin/sh\nexit 0\n";
#[cfg(not(unix))]
pub const EXIT_OK_SCRIPT: &str = "@echo off\r\nexit /b 0\r\n";

#[cfg(unix)]
pub const EXIT_FAIL_SCRIPT: &str = "#!/bin/sh\nexit 1\n";
#[cfg(not(unix))]
pub const EXIT_FAIL_SCRIPT: &str = "@echo off\r\nexit /b 1\r\n";

#[cfg(unix)]
pub fn make_executable(path: &Path) {
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

#[cfg(not(unix))]
pub fn make_executable(_path: &Path) {}

/// `check()` spawns `npm`/`trivy` by name (resolved via `resolve_on_path`,
/// which on Windows only matches `PATHEXT`-suffixed files — see
/// `security::resolve`), so the fake binaries need a `.cmd` extension
/// there to actually be found and launched.
#[cfg(unix)]
pub fn script_filename(name: &str) -> String {
    name.to_string()
}

#[cfg(not(unix))]
pub fn script_filename(name: &str) -> String {
    format!("{name}.cmd")
}

#[cfg(unix)]
pub fn touch_and_exit_ok_script(sentinel: &Path) -> String {
    format!("#!/bin/sh\n: > {}\nexit 0\n", sentinel.display())
}

#[cfg(not(unix))]
pub fn touch_and_exit_ok_script(sentinel: &Path) -> String {
    format!(
        "@echo off\r\ntype nul > \"{}\"\r\nexit /b 0\r\n",
        sentinel.display()
    )
}
