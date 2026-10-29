use super::*;
use crate::contracts::EnvMode;

fn probing_harness() -> HarnessInput {
    HarnessInput {
        name: "xh".into(),
        binary: "xh".into(),
        env_mode: EnvMode::None,
        env: Vec::new(),
        support: vec![("run".into(), "ok".into(), true)],
        version: Some(crate::cli::logic::test_support::version_probe()),
        sunset: false,
    }
}

fn resolution(code: Code) -> super::super::resolve::Resolution {
    super::super::resolve::Resolution {
        code,
        path: None,
        paths: Vec::new(),
        matches: 0,
    }
}

fn probe(code: Code, probes: bool) -> (Code, String) {
    let record = record("harness.xh", &probing_harness(), &resolution(code), probes);
    (record.code, record.value)
}

#[test]
fn version_record_gates_probing() {
    assert_eq!(probe(Code::Ready, true), (Code::Ready, "v1.2.3".into()));
    assert_eq!(
        probe(Code::Missing, true),
        (Code::Unknown, "unknown:not-probed".into())
    );
    assert_eq!(
        probe(Code::Ready, false),
        (Code::Unknown, "unknown:not-probed".into())
    );
}
