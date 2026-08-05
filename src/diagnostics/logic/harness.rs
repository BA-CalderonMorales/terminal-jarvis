use super::redact::{segment, Redactor};
use super::{Code, DiagnosticInput, Record, Severity};
use std::collections::BTreeSet;
use std::path::Path;

pub struct HarnessResult {
    pub records: Vec<Record>,
    pub ready: BTreeSet<String>,
}

pub fn collect(input: &DiagnosticInput, redact: &Redactor<'_>) -> HarnessResult {
    let mut harnesses = input.harnesses.iter().collect::<Vec<_>>();
    harnesses.sort_by(|left, right| left.name.cmp(&right.name));
    let mut records = Vec::new();
    let mut ready = BTreeSet::new();
    for harness in harnesses {
        let base = format!("harness.{}", segment(&harness.name));
        let (support, support_ready) = super::harness_support::collect(harness, &base);
        records.push(support);
        records.push(Record::new(
            format!("{base}.version"),
            Code::Unknown,
            Severity::Info,
            "unknown:not-probed",
        ));
        let resolution = super::resolve::binary(&harness.binary, input);
        let value = if matches!(resolution.code, Code::Ready | Code::Conflicting) {
            resolution
                .path
                .as_deref()
                .map(|path| redact.full(path))
                .unwrap_or_else(|| redact.minimal(Path::new(&harness.binary)))
        } else {
            redact.minimal(Path::new(&harness.binary))
        };
        let mut executable = Record::new(
            format!("{base}.executable"),
            resolution.code,
            if resolution.code == Code::Ready {
                Severity::Info
            } else {
                Severity::Error
            },
            value,
        );
        if resolution.code != Code::Ready {
            executable.action = Some("install or repair the harness executable".into());
        }
        records.push(executable);
        let (mut environment, environment_ready) =
            super::harness_env::collect(harness, input, &base);
        records.append(&mut environment);
        let harness_ready = resolution.code == Code::Ready && environment_ready && support_ready;
        let readiness_code = if harness_ready {
            Code::Ready
        } else if !support_ready {
            Code::Unsupported
        } else {
            Code::Missing
        };
        records.push(Record::new(
            format!("{base}.readiness"),
            readiness_code,
            if harness_ready {
                Severity::Info
            } else {
                Severity::Error
            },
            if harness_ready { "ready" } else { "not-ready" },
        ));
        if harness_ready {
            ready.insert(harness.name.clone());
        }
    }
    HarnessResult { records, ready }
}
