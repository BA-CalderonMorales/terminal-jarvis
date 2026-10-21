//! Support fixtures for the diagnostics state tests, which are unix-only
//! (they build an `OsString` from raw bytes and a fake HOME layout), so the
//! helpers are scoped to unix rather than compiled dead on Windows.
#![cfg(unix)]

use std::fs;
use std::path::PathBuf;
use terminal_jarvis::diagnostics::{Code, Report};

pub fn assert_code(report: &Report, key: &str, code: Code) {
    assert_eq!(
        report
            .records
            .iter()
            .find(|record| record.key == key)
            .unwrap()
            .code,
        code
    );
}

pub fn fixture() -> PathBuf {
    let path = std::env::temp_dir().join(format!("tj-diagnostics-states-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }
    path
}
