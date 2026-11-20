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
    let index = super::dir_index::DirIndex::from_paths(&input.environment.paths());
    let mut records = Vec::new();
    let mut ready = BTreeSet::new();
    for harness in harnesses {
        let base = format!("harness.{}", segment(&harness.name));
        let (support, support_ready) = super::harness_support::collect(harness, &base);
        records.push(support);
        let resolution = super::resolve::binary_with(&harness.binary, input, &index);
        records.push(super::harness_version::record(
            &base,
            harness,
            &resolution,
            input.runtime.probes,
        ));
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
            // Not installed is the first-run state for most of the catalog, so
            // it is information; only a broken installation needs the user.
            match resolution.code {
                Code::Ready | Code::Missing => Severity::Info,
                _ => Severity::Warning,
            },
            value,
        );
        if resolution.code == Code::Missing {
            executable.action = Some(format!("terminal-jarvis install {}", harness.name));
        } else if resolution.code != Code::Ready {
            executable.action = Some("repair the harness executable".into());
        }
        records.push(executable);
        let (mut environment, _) = super::harness_env::collect(harness, input, &base);
        records.append(&mut environment);

        // Available = installed, runnable, and maintained; creds never gate
        // the count. A retired tool is reported as such instead of being
        // recommended, even when its binary still resolves.
        let harness_ready = !harness.sunset
            && matches!(resolution.code, Code::Ready | Code::Conflicting)
            && support_ready;
        let readiness_code = match (harness_ready, support_ready) {
            (true, _) => Code::Ready,
            (false, false) => Code::Unsupported,
            (false, true) if harness.sunset => Code::Unsupported,
            (false, true) => Code::Missing,
        };
        records.push(Record::new(
            format!("{base}.readiness"),
            readiness_code,
            if harness.sunset && !harness_ready {
                Severity::Warning
            } else {
                Severity::Info
            },
            if harness_ready {
                "ready"
            } else if harness.sunset {
                "retired"
            } else {
                "not-ready"
            },
        ));
        if harness_ready {
            ready.insert(harness.name.clone());
        }
    }
    HarnessResult { records, ready }
}

#[cfg(test)]
#[path = "../tests/harness.rs"]
mod tests;
