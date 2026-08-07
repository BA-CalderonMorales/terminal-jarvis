use super::redact::Redactor;
use super::{Code, DiagnosticInput, Record, Severity};

pub fn collect(input: &DiagnosticInput, redact: &Redactor<'_>) -> (Vec<Record>, bool) {
    let (distribution, distribution_ok) = super::distribution::collect(input);
    let (executable, path, executable_ok) = super::program::collect(input, redact);
    let (mut platform, platform_ok) = super::platform_records::collect(input);
    let mut records = vec![
        Record::new(
            "tj.version",
            Code::Ready,
            Severity::Info,
            super::redact::clean(&input.version),
        ),
        distribution,
        super::distribution::wrapper(input),
        executable,
        path,
    ];
    records.append(&mut platform);
    (records, all_ok(distribution_ok, executable_ok, platform_ok))
}

fn all_ok(distribution: bool, executable: bool, platform: bool) -> bool {
    distribution && executable && platform
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_report_must_be_ok() {
        assert!(all_ok(true, true, true));
        assert!(!all_ok(false, true, true));
        assert!(!all_ok(true, false, true));
        assert!(!all_ok(true, true, false));
    }
}
