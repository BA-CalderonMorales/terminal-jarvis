use super::*;

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
