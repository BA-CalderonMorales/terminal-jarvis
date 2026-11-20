use super::tests::*;
use super::*;
use crate::contracts::EnvMode;
use crate::diagnostics::Environment;

/// A present credential gets its own row. An absent one does not: it is
/// summarized once in the environment row, because a report that lists every
/// unset variable on the machine buries the one thing the user can act on.
#[test]
fn collect_marks_env_state_by_mode() {
    let mut with_env = Environment::default();
    with_env.insert("TOKEN", "token-value");
    for (mode, environment, ready_expected) in [
        (EnvMode::All, with_env, true),
        (EnvMode::Optional, Environment::default(), true),
        (EnvMode::All, Environment::default(), false),
    ] {
        let (records, ready) = collect(
            &harness(mode, &["TOKEN"]),
            &input(environment),
            "harness.xh",
        );
        assert_eq!(ready, ready_expected, "mode={mode:?}");
        if ready && mode == EnvMode::All {
            assert_eq!(
                env_record(&records, "harness.xh.env.TOKEN").severity,
                Severity::Info,
                "mode={mode:?}"
            );
        }
    }
    let (records, _) = collect(
        &harness(EnvMode::All, &["TOKEN"]),
        &input(Environment::default()),
        "harness.xh",
    );
    let summary = env_record(&records, "harness.xh.environment");
    assert_eq!(summary.code, Code::Missing);
    assert_eq!(summary.action.as_deref(), Some("set one of: TOKEN"));
}
