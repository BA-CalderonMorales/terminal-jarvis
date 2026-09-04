//! Package check acceptance: `install` warns without a gate, fails closed
//! on HIGH/CRITICAL findings, and proceeds silently on a clean scan.

use super::package_check_fixture::{fixture, gate_on, tj, INSTALL};
use std::process::Output;

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

#[test]
fn gate_off_install_warns_and_continues() {
    let (home, catalog_root, bin) = fixture(0);
    let output = tj(INSTALL, &home, &catalog_root, &bin);
    assert_eq!(output.status.code(), Some(0));
    assert!(stderr(&output).contains("without a vulnerability check"));
}

#[test]
fn gate_on_findings_fail_closed_noninteractive() {
    let (home, catalog_root, bin) = fixture(1);
    gate_on(&home, &catalog_root, &bin);
    let output = tj(INSTALL, &home, &catalog_root, &bin);
    assert_eq!(output.status.code(), Some(5));
    assert!(stderr(&output).contains("HIGH/CRITICAL findings"));
}

#[test]
fn gate_on_clean_proceeds_without_warning() {
    let (home, catalog_root, bin) = fixture(0);
    gate_on(&home, &catalog_root, &bin);
    let output = tj(INSTALL, &home, &catalog_root, &bin);
    assert_eq!(output.status.code(), Some(0));
    assert!(
        !stderr(&output).contains("warning:"),
        "{:?}",
        stderr(&output)
    );
}
