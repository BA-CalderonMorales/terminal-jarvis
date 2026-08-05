use super::*;
use crate::contracts::Capability;

fn valid() -> String {
    [
        "summary = \"test\"",
        "command = \"test\"",
        "args = []",
        "support = \"unknown\"",
        "evidence = \"deterministic\"",
        "effect = \"read-only\"",
        "network = false",
        "interaction = \"noninteractive\"",
        "platforms = []",
        "executable = \"test\"",
        "source = \"internal:test\"",
        "verified_at = \"2026-07-17T00:00:00Z\"",
    ]
    .join("\n")
}

#[test]
fn duplicate_keys_are_rejected() {
    let input = format!("{}\nsupport = \"stub\"", valid());
    let error = crate::catalog::parser::parse(&input).unwrap_err();
    assert!(error.contains("duplicate key 'support'"));
}

#[test]
fn extra_and_missing_metadata_are_rejected() {
    let extra = format!("{}\nfuture = \"no\"", valid());
    let fields = crate::catalog::parser::parse(&extra).unwrap();
    assert!(capability(&fields, Capability::Stats)
        .unwrap_err()
        .contains("metadata keys must be exactly"));

    let missing = valid()
        .lines()
        .filter(|line| !line.starts_with("source ="))
        .collect::<Vec<_>>()
        .join("\n");
    let fields = crate::catalog::parser::parse(&missing).unwrap();
    assert!(capability(&fields, Capability::Stats)
        .unwrap_err()
        .contains("metadata keys must be exactly"));
}

#[test]
fn invalid_typed_metadata_is_rejected() {
    let input = valid().replace("network = false", "network = maybe");
    let fields = crate::catalog::parser::parse(&input).unwrap();
    assert!(capability(&fields, Capability::Stats)
        .unwrap_err()
        .contains("must be true or false"));
}
