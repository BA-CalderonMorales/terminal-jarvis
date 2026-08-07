use super::*;
use crate::diagnostics::{Record, Severity};

fn report(records: Vec<Record>) -> super::Report {
    super::Report {
        records,
        ready_harnesses: 0,
        ok: true,
    }
}

fn record(key: &str, severity: Severity) -> Record {
    Record::new(key, crate::diagnostics::Code::Ready, severity, key)
}

#[test]
fn concise_drops_plain_harness_records_and_keeps_active_readiness_and_errors() {
    let base = super::Report {
        records: vec![
            record("harness.alpha", Severity::Info),
            record("harness.beta", Severity::Info),
            record("harness.active", Severity::Info),
            record("harness.gamma.readiness", Severity::Info),
            record("harness.delta", Severity::Error),
            record("state.catalog", Severity::Info),
        ],
        ready_harnesses: 0,
        ok: true,
    };
    let out = base.concise();
    let keys = out
        .records
        .iter()
        .map(|r| r.key.as_str())
        .collect::<Vec<_>>();
    assert!(!keys.contains(&"harness.alpha"));
    assert!(keys.contains(&"harness.active"));
    assert!(keys.contains(&"harness.gamma.readiness"));
    assert!(keys.contains(&"harness.delta"));
    assert!(keys.contains(&"state.catalog"));
    assert!(!keys.contains(&"harness.beta"));
}

#[test]
fn concise_is_idempotent_over_empty_reports() {
    let out = report(Vec::new()).concise();
    assert!(out.records.is_empty());
}
