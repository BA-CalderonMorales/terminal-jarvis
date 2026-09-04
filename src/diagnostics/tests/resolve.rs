use super::harness::{assert_ready, dummy_input, temp_executable};
use super::*;
use std::path::Path;

#[test]
fn direct_with_existing_executable_file() {
    let test_file = temp_executable();
    assert_ready(&direct(&test_file));
    let _ = std::fs::remove_file(&test_file);
}

#[test]
fn direct_rejects_missing_paths_and_directories() {
    let missing = direct(Path::new("/nonexistent/path/to/file"));
    assert_eq!(missing.code, Code::Missing);
    assert_eq!(missing.matches, 0);
    let directory = direct(&std::env::temp_dir());
    assert_eq!(directory.code, Code::Malformed);
}

#[test]
fn binary_rejects_empty_and_unknown_names() {
    let empty = binary("", &dummy_input());
    assert_eq!(empty.code, Code::Malformed);
    let missing = binary("nonexistent_command_12345", &dummy_input());
    assert_eq!(missing.code, Code::Missing);
    assert_eq!(missing.matches, 0);
}

#[test]
fn binary_with_path_separators() {
    let test_file = temp_executable();
    let result = binary(&test_file.to_string_lossy(), &dummy_input());
    assert_ready(&result);
    let _ = std::fs::remove_file(&test_file);
}

#[test]
fn pathext_skips_empty_segments() {
    let mut input = dummy_input();
    input.platform.os = "windows".into();
    input.environment.insert("PATHEXT", ".COM;;.EXE;");
    assert_eq!(
        candidates("tool", &input),
        vec!["tool.COM", "tool.EXE", "tool"]
    );
}
