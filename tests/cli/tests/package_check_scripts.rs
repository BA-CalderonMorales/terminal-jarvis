//! Package-check script fixtures: per-platform fake `npm`/`trivy` bodies
//! and the executable-marker/filename rules they must follow.

use std::path::Path;

#[cfg(unix)]
pub fn script_filename(name: &str) -> String {
    name.to_string()
}

#[cfg(not(unix))]
pub fn script_filename(name: &str) -> String {
    format!("{name}.cmd")
}

#[cfg(unix)]
pub fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

#[cfg(not(unix))]
pub fn make_executable(_path: &Path) {}

#[cfg(unix)]
pub fn npm_script() -> String {
    "#!/bin/sh\nprintf '{\"name\":\"fixture\"}' > package-lock.json\nexit 0\n".to_string()
}

#[cfg(not(unix))]
pub fn npm_script() -> String {
    "@echo off\r\necho {\"name\":\"fixture\"}> package-lock.json\r\nexit /b 0\r\n".to_string()
}

#[cfg(unix)]
pub fn trivy_script(trivy_exit: i32) -> String {
    format!(
        "#!/bin/sh\nif [ -f package-lock.json ]; then printf 'CRITICAL minimist CVE-2021-44906\\n' >&2; exit {trivy_exit}; fi\nexit 0\n"
    )
}

#[cfg(not(unix))]
pub fn trivy_script(trivy_exit: i32) -> String {
    format!(
        "@echo off\r\nif exist package-lock.json (\r\n  echo CRITICAL minimist CVE-2021-44906 1>&2\r\n  exit /b {trivy_exit}\r\n) else (\r\n  exit /b 0\r\n)\r\n"
    )
}

#[cfg(unix)]
pub fn noop_script() -> &'static str {
    "#!/bin/sh\nexit 0\n"
}

#[cfg(not(unix))]
pub fn noop_script() -> &'static str {
    "@echo off\r\nexit /b 0\r\n"
}

pub fn write_executable(dir: &Path, name: &str, body: &str) {
    let entry = dir.join(script_filename(name));
    std::fs::write(&entry, body).unwrap();
    make_executable(&entry);
}
