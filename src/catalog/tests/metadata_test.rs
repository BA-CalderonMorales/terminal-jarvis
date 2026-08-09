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
fn package_key_is_optional_and_parses() {
    let with_package = format!("{}\npackage = \"@example/tool\"", valid());
    let fields = crate::catalog::parser::parse(&with_package).unwrap();
    let plan = capability(&fields, Capability::Stats).unwrap();
    assert_eq!(plan.package.as_deref(), Some("@example/tool"));

    let plain = crate::catalog::parser::parse(&valid()).unwrap();
    assert_eq!(capability(&plain, Capability::Stats).unwrap().package, None);
}

#[test]
fn extra_and_missing_metadata_are_rejected() {
    let extra = format!("{}\nfuture = \"no\"", valid());
    let fields = crate::catalog::parser::parse(&extra).unwrap();
    let error = capability(&fields, Capability::Stats).unwrap_err();
    assert!(error.contains("unknown") && error.contains("future"));

    let missing = valid()
        .lines()
        .filter(|line| !line.starts_with("source ="))
        .collect::<Vec<_>>()
        .join("\n");
    let fields = crate::catalog::parser::parse(&missing).unwrap();
    let error = capability(&fields, Capability::Stats).unwrap_err();
    assert!(error.contains("missing") && error.contains("source"));
}

#[test]
fn harness_extra_and_missing_keys_are_rejected() {
    let extra = format!("{}\nfuture = \"no\"", harness_fields());
    let fields = crate::catalog::parser::parse(&extra).unwrap();
    assert!(harness(&fields, Vec::new())
        .unwrap_err()
        .contains("metadata keys must be exactly"));

    let missing = harness_fields()
        .lines()
        .filter(|line| !line.starts_with("name ="))
        .collect::<Vec<_>>()
        .join("\n");
    let fields = crate::catalog::parser::parse(&missing).unwrap();
    assert!(harness(&fields, Vec::new())
        .unwrap_err()
        .contains("metadata keys must be exactly"));
}

fn harness_fields() -> String {
    [
        "name = \"tool\"",
        "display = \"Tool\"",
        "description = \"test\"",
        "binary = \"tool\"",
        "env_mode = \"none\"",
        "env = []",
    ]
    .join("\n")
}

#[test]
fn invalid_typed_metadata_is_rejected() {
    let input = valid().replace("network = false", "network = maybe");
    let fields = crate::catalog::parser::parse(&input).unwrap();
    assert!(capability(&fields, Capability::Stats)
        .unwrap_err()
        .contains("must be true or false"));
}
