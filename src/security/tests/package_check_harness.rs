use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn fake_bin_pair(npm: &str, trivy: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tj-pkgcheck-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    for (name, script) in [("npm", npm), ("trivy", trivy)] {
        let path = dir.join(script_filename(name));
        std::fs::write(&path, script).unwrap();
        make_executable(&path);
    }
    dir
}

pub fn fake_bin_npm_only() -> (std::path::PathBuf, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!(
        "tj-pkgcheck-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let sentinel = std::env::temp_dir().join(format!(
        "tj-sentinel-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let script = touch_and_exit_ok_script(&sentinel);
    let path = dir.join(script_filename("npm"));
    std::fs::write(&path, script).unwrap();
    make_executable(&path);
    (dir, sentinel)
}

#[cfg(unix)]
pub const EXIT_OK_SCRIPT: &str = "#!/bin/sh\nexit 0\n";
#[cfg(not(unix))]
pub const EXIT_OK_SCRIPT: &str = "@echo off\r\nexit /b 0\r\n";

#[cfg(unix)]
pub const EXIT_FAIL_SCRIPT: &str = "#!/bin/sh\nexit 1\n";
#[cfg(not(unix))]
pub const EXIT_FAIL_SCRIPT: &str = "@echo off\r\nexit /b 1\r\n";

pub fn run_with_path(name: &str, script: &str) -> Option<Verdict> {
    run_with_bins(name, script, EXIT_OK_SCRIPT)
}

pub fn run_with_bins(name: &str, npm: &str, trivy: &str) -> Option<Verdict> {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = fake_bin_pair(npm, trivy);
    let previous = std::env::var_os("PATH");
    let path = if name == "npm-only" {
        dir.to_string_lossy().into_owned()
    } else {
        let mut dirs = vec![dir.clone()];
        if let Some(value) = previous.clone() {
            dirs.extend(std::env::split_paths(&value));
        }
        std::env::join_paths(dirs)
            .unwrap()
            .to_string_lossy()
            .into_owned()
    };
    std::env::set_var("PATH", &path);
    let result = check("fixture-package");
    if let Some(value) = previous {
        std::env::set_var("PATH", value);
    } else {
        std::env::remove_var("PATH");
    }
    let _ = std::fs::remove_dir_all(&dir);
    result
}

// Re-exported for the tests module that pulls this file in.
pub use super::{check, Verdict};

#[cfg(unix)]
fn make_executable(path: &std::path::Path) {
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

#[cfg(not(unix))]
fn make_executable(_path: &std::path::Path) {}

/// `check()` spawns `npm`/`trivy` by name (resolved via `resolve_on_path`,
/// which on Windows only matches `PATHEXT`-suffixed files — see
/// `package_check::resolved`), so the fake binaries need a `.cmd` extension
/// there to actually be found and launched.
#[cfg(unix)]
fn script_filename(name: &str) -> String {
    name.to_string()
}

#[cfg(not(unix))]
fn script_filename(name: &str) -> String {
    format!("{name}.cmd")
}

#[cfg(unix)]
fn touch_and_exit_ok_script(sentinel: &std::path::Path) -> String {
    format!("#!/bin/sh\n: > {}\nexit 0\n", sentinel.display())
}

#[cfg(not(unix))]
fn touch_and_exit_ok_script(sentinel: &std::path::Path) -> String {
    format!(
        "@echo off\r\ntype nul > \"{}\"\r\nexit /b 0\r\n",
        sentinel.display()
    )
}
