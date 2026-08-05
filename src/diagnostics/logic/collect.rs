use super::redact::Redactor;
use super::{DiagnosticInput, Report};

pub fn collect(input: &DiagnosticInput) -> Report {
    let redact = Redactor::new(input.home_prefix.as_ref(), input.temp_prefix.as_ref());
    let (mut records, system_ok) = super::system::collect(input, &redact);
    let (mut runtime, runtime_ok) = super::runtime_records::collect(input, &redact);
    records.append(&mut runtime);
    let (mut storage, config, storage_ok) = super::storage::collect(input, &redact);
    records.append(&mut storage);
    let harnesses = super::harness::collect(input, &redact);
    let (active, selection_ok) = super::active::collect(input, config.active, &harnesses.ready);
    records.push(active);
    records.extend(harnesses.records);
    Report {
        records,
        ready_harnesses: harnesses.ready.len(),
        ok: system_ok && runtime_ok && storage_ok && selection_ok,
    }
}
