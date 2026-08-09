use crate::structs::catalog;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const INSTALL: &[&str] = &[
    "install",
    "fixture",
    "--no-input",
    "--confirm=download:fixture",
];

fn tj(args: &[&str], home: &Path, catalog_root: &Path, path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--plain")
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .env("TERMINAL_JARVIS_CATALOG", catalog_root)
        .env("PATH", path)
        .output()
        .expect("terminal-jarvis runs")
}

fn write_executable(dir: &Path, name: &str, body: &str) {
    let entry = dir.join(name);
    std::fs::write(&entry, body).unwrap();
    let mut permissions = std::fs::metadata(&entry).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    std::fs::set_permissions(&entry, permissions).unwrap();
}

fn tool_bin(root_dir: &Path, trivy_exit: i32) -> PathBuf {
    let dir = root_dir.join(format!("bin-{trivy_exit}"));
    std::fs::create_dir_all(&dir).unwrap();
    let npm = "#!/bin/sh\nprintf '{\"name\":\"fixture\"}' > package-lock.json\nexit 0\n";
    write_executable(&dir, "npm", npm);
    let trivy = format!(
        "#!/bin/sh\nif [ -f package-lock.json ]; then printf 'CRITICAL minimist CVE-2021-44906\\n' >&2; exit {trivy_exit}; fi\nexit 0\n"
    );
    write_executable(&dir, "trivy", &trivy);
    write_executable(&dir, "fixture-child", "#!/bin/sh\nexit 0\n");
    dir
}

fn fixture(trivy_exit: i32) -> (PathBuf, PathBuf, PathBuf) {
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
    catalog::write_with_package(
        &catalog_root,
        "expected",
        "expected",
        Some("fixture-package"),
    );
    (home, catalog_root, bin)
}

fn gate_on(home: &Path, catalog_root: &Path, bin: &Path) {
    assert!(tj(&["gate", "enable", "trivy"], home, catalog_root, bin)
        .status
        .success());
}

#[test]
fn gate_off_install_warns_and_continues() {
    let (home, catalog_root, bin) = fixture(0);
    let output = tj(INSTALL, &home, &catalog_root, &bin);
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("without a vulnerability check"), "{stderr}");
}

#[test]
fn gate_on_findings_fail_closed_noninteractive() {
    let (home, catalog_root, bin) = fixture(1);
    gate_on(&home, &catalog_root, &bin);
    let output = tj(INSTALL, &home, &catalog_root, &bin);
    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("HIGH/CRITICAL findings"), "{stderr}");
}

#[test]
fn gate_on_clean_proceeds_without_warning() {
    let (home, catalog_root, bin) = fixture(0);
    gate_on(&home, &catalog_root, &bin);
    let output = tj(INSTALL, &home, &catalog_root, &bin);
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("warning:"), "{stderr}");
}
