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
    let path = std::env::temp_dir().join(format!("tj-phase02-states-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }
    path
}
