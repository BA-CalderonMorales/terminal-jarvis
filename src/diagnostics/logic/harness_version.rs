use super::{Code, HarnessInput, Record, Severity};

pub(super) fn record(
    base: &str,
    harness: &HarnessInput,
    resolution: &super::resolve::Resolution,
    probes: bool,
) -> Record {
    let key = format!("{base}.version");
    if resolution.code != Code::Ready || !probes {
        return Record::new(key, Code::Unknown, Severity::Info, "unknown:not-probed");
    }
    let Some((command, args)) = &harness.version else {
        return Record::new(key, Code::Unknown, Severity::Info, "unknown:not-probed");
    };
    match super::probe::version(command, args) {
        Some(value) => Record::new(key, Code::Ready, Severity::Info, value),
        None => Record::new(key, Code::Unknown, Severity::Info, "unknown:probe-failed"),
    }
}

#[cfg(test)]
#[path = "../tests/harness_version.rs"]
mod tests;
