#![cfg(unix)]

#[path = "phase02_diagnostics_support/redaction.rs"]
mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use terminal_jarvis::diagnostics::{collect, Code};

#[test]
fn read_only_home_is_reported_without_a_probe_write() {
    let root = support::fixture("readonly");
    let private_home = root.join("home/alice-sensitive");
    let home = private_home.join(".config/terminal-jarvis");
    let catalog = root.join("catalog");
    let bin = root.join("bin");
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(&catalog).unwrap();
    fs::create_dir_all(&bin).unwrap();
    fs::write(catalog.join("index.toml"), "catalog").unwrap();
    fs::write(home.join("session.toml"), "active_harness = \"fixture\"\n").unwrap();
    fs::write(root.join("cache"), "cache").unwrap();
    support::executable(&bin.join("tj"));
    support::executable(&bin.join("fixture"));
    let before = support::snapshot(&root);
    fs::set_permissions(&home, fs::Permissions::from_mode(0o500)).unwrap();
    let report = collect(&support::input(&root, &private_home, &home, &catalog, &bin));
    let state = report
        .records
        .iter()
        .find(|record| record.key == "state.home")
        .unwrap();
    assert_eq!(state.code, Code::PermissionDenied);
    assert!(!report.ok);
    fs::set_permissions(&home, fs::Permissions::from_mode(0o700)).unwrap();
    assert_eq!(before, support::snapshot(&root));
    fs::remove_dir_all(root).unwrap();
}
