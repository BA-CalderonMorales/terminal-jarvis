use super::tests::*;
use super::*;
use crate::contracts::EnvMode;
use crate::diagnostics::Environment;

#[test]
fn collect_marks_env_state_by_mode() {
    let mut with_env = Environment::default();
    with_env.insert("TOKEN", "token-value");
    for (mode, environment, ready_expected, severity) in [
        (EnvMode::All, with_env, true, Severity::Info),
        (
            EnvMode::Optional,
            Environment::default(),
            true,
            Severity::Info,
        ),
        (
            EnvMode::All,
            Environment::default(),
            false,
            Severity::Warning,
        ),
    ] {
        let (records, ready) = collect(
            &harness(mode, &["TOKEN"]),
            &input(environment),
            "harness.xh",
        );
        assert_eq!(ready, ready_expected, "mode={mode:?}");
        assert_eq!(
            env_record(&records, "harness.xh.env.TOKEN").severity,
            severity,
            "mode={mode:?}"
        );
    }
    let (records, _) = collect(
        &harness(EnvMode::All, &["TOKEN"]),
        &input(Environment::default()),
        "harness.xh",
    );
    let summary = env_record(&records, "harness.xh.environment");
    assert_eq!(summary.code, Code::Missing);
    assert_eq!(
        summary.action.as_deref(),
        Some("set the required credential environment names")
    );
}
