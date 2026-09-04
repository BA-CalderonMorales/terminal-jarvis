//! Package-check fixture: a scratch catalog, home, and tool bin wired so
//! `install` exercises the gate and the vulnerability check end-to-end.

use super::package_check_scripts::{noop_script, npm_script, trivy_script, write_executable};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub const INSTALL: &[&str] = &[
    "install",
    "fixture",
    "--no-input",
    "--confirm=download:fixture",
];

pub fn tj(args: &[&str], home: &Path, catalog_root: &Path, path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--plain")
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .env("TERMINAL_JARVIS_CATALOG", catalog_root)
        .env("PATH", path)
        .output()
        .expect("terminal-jarvis runs")
}

fn tool_bin(root_dir: &Path, trivy_exit: i32) -> PathBuf {
    let dir = root_dir.join(format!("bin-{trivy_exit}"));
    std::fs::create_dir_all(&dir).unwrap();
    write_executable(&dir, "npm", &npm_script());
    write_executable(&dir, "trivy", &trivy_script(trivy_exit));
    write_executable(&dir, "fixture-child", noop_script());
    dir
}

pub fn fixture(trivy_exit: i32) -> (PathBuf, PathBuf, PathBuf) {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root_dir = std::env::temp_dir().join(format!(
        "terminal-jarvis-pkgcheck-{}-{nanos}",
        std::process::id()
    ));
    let home = root_dir.join("home");
    let catalog_root = root_dir.join("catalog");
    let bin = tool_bin(&root_dir, trivy_exit);
    crate::structs::catalog::write_with_package(
        &catalog_root,
        "expected",
        "expected",
        Some("fixture-package"),
    );
    (home, catalog_root, bin)
}

pub fn gate_on(home: &Path, catalog_root: &Path, bin: &Path) {
    assert!(tj(&["gate", "enable", "trivy"], home, catalog_root, bin)
        .status
        .success());
}
