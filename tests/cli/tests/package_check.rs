use crate::structs::catalog;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn root() -> String {
    std::env::temp_dir()
        .join(format!(
            "terminal-jarvis-pkgcheck-{}-{}",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ))
        .to_string_lossy()
        .to_string()
}

fn tj(args: &[&str], home: &str, catalog_root: &std::path::Path, path: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--plain")
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .env("TERMINAL_JARVIS_CATALOG", catalog_root)
        .env("PATH", path)
        .output()
        .expect("terminal-jarvis runs")
}

fn write_executable(dir: &std::path::Path, name: &str, body: &str) {
    let entry = dir.join(name);
    std::fs::write(&entry, body).unwrap();
    let mut permissions = std::fs::metadata(&entry).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    std::fs::set_permissions(&entry, permissions).unwrap();
}

fn tool_bin(root_dir: &std::path::Path, trivy_exit: i32) -> std::path::PathBuf {
    let dir = root_dir.join(format!("bin-{trivy_exit}"));
    std::fs::create_dir_all(&dir).unwrap();
    write_executable(
        &dir,
        "npm",
        "#!/bin/sh\nprintf '{\"name\":\"fixture\"}' > package-lock.json\nexit 0\n",
    );
    write_executable(
        &dir,
        "trivy",
        &format!(
            "#!/bin/sh\nif [ -f package-lock.json ]; then printf 'CRITICAL minimist CVE-2021-44906\\n' >&2; exit {trivy_exit}; fi\nexit 0\n"
        ),
    );
    write_executable(&dir, "fixture-child", "#!/bin/sh\nexit 0\n");
    dir
}

#[test]
fn gate_off_install_warns_and_continues() {
    let root_dir = std::path::PathBuf::from(root());
    let home = root_dir.join("home");
    let catalog_root = root_dir.join("catalog");
    let bin = tool_bin(&root_dir, 0);
    catalog::write_with_package(
        &catalog_root,
        "expected",
        "expected",
        Some("fixture-package"),
    );
    let output = tj(
        &[
            "install",
            "fixture",
            "--no-input",
            "--confirm=download:fixture",
        ],
        &home.to_string_lossy(),
        &catalog_root,
        &bin.to_string_lossy(),
    );
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("without a vulnerability check"), "{stderr}");
}

#[test]
fn gate_on_findings_fail_closed_noninteractive() {
    let root_dir = std::path::PathBuf::from(root());
    let home = root_dir.join("home");
    let catalog_root = root_dir.join("catalog");
    let bin = tool_bin(&root_dir, 1);
    catalog::write_with_package(
        &catalog_root,
        "expected",
        "expected",
        Some("fixture-package"),
    );
    assert!(tj(
        &["gate", "enable", "trivy"],
        &home.to_string_lossy(),
        &catalog_root,
        &bin.to_string_lossy()
    )
    .status
    .success());
    let output = tj(
        &[
            "install",
            "fixture",
            "--no-input",
            "--confirm=download:fixture",
        ],
        &home.to_string_lossy(),
        &catalog_root,
        &bin.to_string_lossy(),
    );
    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("HIGH/CRITICAL findings"), "{stderr}");
}

#[test]
fn gate_on_clean_proceeds_without_warning() {
    let root_dir = std::path::PathBuf::from(root());
    let home = root_dir.join("home");
    let catalog_root = root_dir.join("catalog");
    let bin = tool_bin(&root_dir, 0);
    catalog::write_with_package(
        &catalog_root,
        "expected",
        "expected",
        Some("fixture-package"),
    );
    assert!(tj(
        &["gate", "enable", "trivy"],
        &home.to_string_lossy(),
        &catalog_root,
        &bin.to_string_lossy()
    )
    .status
    .success());
    let output = tj(
        &[
            "install",
            "fixture",
            "--no-input",
            "--confirm=download:fixture",
        ],
        &home.to_string_lossy(),
        &catalog_root,
        &bin.to_string_lossy(),
    );
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("warning:"), "{stderr}");
}
