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
    (records, distribution_ok && executable_ok && platform_ok)
}
