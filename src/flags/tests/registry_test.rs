use super::support::{flag_file, registry};
use crate::flags::{build, load, Stage};
use std::path::Path;

fn err(dir: &str, data: &str) -> String {
    build([(dir, data)].into_iter()).unwrap_err()
}

#[test]
fn a_valid_file_parses_every_field() {
    let file = flag_file("fast-scan", false, "preview");
    let flags = build([("fast-scan", file.as_str())].into_iter()).unwrap();
    assert_eq!(flags[0].name, "fast-scan");
    assert_eq!(flags[0].stage, Stage::Preview);
    assert!(!flags[0].default);
    assert_eq!(flags[0].since, "0.1.26");
}

#[test]
fn a_labs_flag_must_default_off() {
    let message = err("x", &flag_file("x", true, "labs"));
    assert!(
        message.contains("labs flag must default to false"),
        "{message}"
    );
    let stable = flag_file("x", true, "stable");
    assert!(build([("x", stable.as_str())].into_iter()).is_ok());
}

#[test]
fn name_must_match_its_directory_and_be_kebab_case() {
    assert!(err("other", &flag_file("x", false, "stable")).contains("must equal its directory"));
    for bad in ["Bad", "a_b", "-a", "a--b", "1a", ""] {
        let message = err(bad, &flag_file(bad, false, "stable"));
        assert!(message.contains("kebab-case"), "{bad}: {message}");
    }
}

#[test]
fn duplicate_names_are_rejected() {
    let file = flag_file("x", false, "stable");
    let message = build([("x", file.as_str()), ("x", file.as_str())].into_iter()).unwrap_err();
    assert!(message.contains("declared twice"), "{message}");
}

#[test]
fn schema_errors_are_clear() {
    let good = flag_file("x", false, "stable");
    let cases = [
        (good.replace("stable", "beta"), "stage 'beta'"),
        (
            good.replace("false", "maybe"),
            "default must be true or false",
        ),
        (
            good.replace("A fixture flag for x.", ""),
            "description must be",
        ),
        (
            good.replace("A fixture flag", &"a".repeat(101)),
            "description must be",
        ),
        (good.replace("0.1.26", "v1"), "since 'v1'"),
        (format!("{good}extra = \"1\"\n"), "unknown field 'extra'"),
        (good.replace("since = \"0.1.26\"\n", ""), "missing 'since'"),
    ];
    for (data, expected) in cases {
        let message = err("x", &data);
        assert!(
            message.starts_with("flag 'x':") && message.contains(expected),
            "{message}"
        );
    }
}

#[test]
fn a_directory_registry_loads_sorted_and_an_empty_one_is_valid() {
    let root = registry(&[("b-flag", false, "labs"), ("a-flag", true, "stable")]);
    let names: Vec<_> = load(&root)
        .unwrap()
        .into_iter()
        .map(|flag| flag.name)
        .collect();
    assert_eq!(names, ["a-flag", "b-flag"]);
    assert!(load(&registry(&[])).unwrap().is_empty());
    let missing = load(Path::new("/no/such/flags")).unwrap_err();
    assert!(missing.contains("registry missing"));
}

#[test]
fn the_embedded_registry_is_valid_for_any_number_of_flags() {
    let flags = load(Path::new(crate::context::EMBEDDED_FLAGS)).unwrap();
    assert!(flags
        .iter()
        .all(|flag| flag.stage != Stage::Labs || !flag.default));
}
