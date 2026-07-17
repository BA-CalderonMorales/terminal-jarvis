use std::path::Path;

pub fn write(bin: &Path, command: &str) {
    assert_eq!(
        Path::new(command)
            .file_name()
            .and_then(|name| name.to_str()),
        Some(command)
    );
    let path = bin.join(command);
    let script = "#!/bin/sh\nprintf '%s\\n' \"$0\" >> \"$TJ_PHASE03_SPAWN_LOG\"\nexit 97\n";
    std::fs::write(&path, script).unwrap();
    make_executable(&path);
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}
