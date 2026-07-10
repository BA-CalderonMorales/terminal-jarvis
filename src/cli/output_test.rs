use std::path::{Path, PathBuf};

use super::*;
use crate::contracts::EnvMode;

fn tmpdir() -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("tjharnesstest_{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn mock_binary_on_path(tmpdir: &Path) -> String {
    let bin = tmpdir.join("mock-harness");
    std::fs::write(&bin, "#!/bin/sh\necho ok").unwrap();
    std::fs::set_permissions(&bin, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    let old = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}:{}", tmpdir.display(), &old));
    old
}

fn mock_harness(binary: &str, env_mode: EnvMode, env: Vec<String>) -> Harness {
    Harness {
        name: "x".into(),
        display: "X".into(),
        description: "".into(),
        binary: binary.into(),
        env_mode,
        env,
        capabilities: vec![],
    }
}

#[test]
fn is_harness_ready_false_when_binary_missing() {
    let h = mock_harness("does-not-exist-hopefully", EnvMode::None, vec![]);
    assert!(!is_harness_ready(&h));
}

#[test]
fn is_harness_ready_false_when_env_var_missing() {
    let dir = tmpdir();
    let _old = mock_binary_on_path(&dir);
    let h = mock_harness(
        "mock-harness",
        EnvMode::All,
        vec!["SOME_MISSING_VAR".into()],
    );
    assert!(!is_harness_ready(&h));
}

#[test]
fn is_harness_ready_true_when_binary_on_path_and_no_env_required() {
    let dir = tmpdir();
    let _old = mock_binary_on_path(&dir);
    let h = mock_harness("mock-harness", EnvMode::None, vec![]);
    assert!(is_harness_ready(&h));
}

#[test]
fn is_harness_ready_true_when_binary_on_path_and_env_var_set() {
    let dir = tmpdir();
    let _old = mock_binary_on_path(&dir);
    std::env::set_var("TJHARNESS_TEST_VAR", "1");
    let h = mock_harness(
        "mock-harness",
        EnvMode::All,
        vec!["TJHARNESS_TEST_VAR".into()],
    );
    assert!(is_harness_ready(&h));
    std::env::remove_var("TJHARNESS_TEST_VAR");
}

#[test]
fn status_adds_readiness_summary_absent_from_checks() {
    let dir = tmpdir();
    let _old = mock_binary_on_path(&dir);
    let h = mock_harness("mock-harness", EnvMode::None, vec![]);
    let harnesses = vec![h];
    let checks = checks(&harnesses);
    let status = status(&harnesses);
    assert!(!checks.contains("harnesses ready"));
    assert!(status.contains("Security Status") && status.contains("1/1 harnesses"));
}
