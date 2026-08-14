use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn write_gate(root: &Path, name: &str, binary: &str) {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let body = "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = []\ninstall_hint = \"install\"\n";
    let toml = body.replace("{name}", name).replace("{binary}", binary);
    std::fs::write(directory.join("index.toml"), toml).unwrap();
}

pub fn write_executable(path: &Path, script: &str) {
    std::fs::write(path, script).unwrap();
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

/// A counting gate fixture for preflight: an executable scanner that appends
/// to `counter` on every run, registered in the catalog under `name`.
pub fn counter_gate(catalog: &Path, name: &str, counter: &Path) {
    let directory = catalog.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let binary = directory.join("scan");
    write_executable(
        &binary,
        &format!("#!/bin/sh\necho x >> '{}'\n", counter.display()),
    );
    let body = "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = []\ninstall_hint = \"install\"\n";
    let toml = body
        .replace("{name}", name)
        .replace("{binary}", &binary.to_string_lossy());
    std::fs::write(directory.join("index.toml"), toml).unwrap();
}

/// A runnable gate fixture: an executable scanner under `root` plus the
/// loader shape it needs, ready for `stream::run`.
pub fn scan_gate(root: &Path, name: &str, script: &str) -> crate::gates::logic::loader::Gate {
    std::fs::create_dir_all(root).unwrap();
    let bin = root.join(format!("{name}-scan"));
    write_executable(&bin, script);
    crate::gates::logic::loader::Gate {
        name: name.to_string(),
        display: name.to_string(),
        description: "test".to_string(),
        binary: bin.to_string_lossy().into_owned(),
        args: vec![],
        install_hint: "install".to_string(),
    }
}

pub fn restore_gates_env(previous: Option<std::ffi::OsString>) {
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATES", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATES"),
    }
}

pub fn lock() -> std::sync::MutexGuard<'static, ()> {
    crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner())
}
