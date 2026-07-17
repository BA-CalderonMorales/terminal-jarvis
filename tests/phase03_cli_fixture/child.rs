use std::path::{Path, PathBuf};

pub fn write(bin: &Path, executable: bool) -> PathBuf {
    let path = bin.join("fixture-child");
    std::fs::write(&path, "#!/bin/sh\nexit 0\n").unwrap();
    set_executable(&path, executable);
    path
}

#[cfg(unix)]
fn set_executable(path: &Path, executable: bool) {
    use std::os::unix::fs::PermissionsExt;
    let mode = if executable { 0o755 } else { 0o644 };
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(mode);
    std::fs::set_permissions(path, permissions).unwrap();
}

#[cfg(not(unix))]
fn set_executable(_path: &Path, _executable: bool) {}
