use self::harness::{mock_binary_on_path, mock_harness, tmpdir};
use super::*;
use crate::contracts::EnvMode;

#[cfg(test)]
#[path = "output_test_harness.rs"]
mod harness;

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

#[test]
fn env_status_names_missing_vars_when_required() {
    let h = mock_harness("mock-harness", EnvMode::All, vec!["TJ_MISSING_TEST".into()]);
    let rendered = env_status(&h, &["TJ_MISSING_TEST".to_string()]);
    assert!(rendered.contains("missing TJ_MISSING_TEST"));
}
