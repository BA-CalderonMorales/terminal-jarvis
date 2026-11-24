//! Single-source guard: every `flag = "..."` in the bundled gate data names a
//! flag the registry declares, and the scanner-choice pin reads the same flag.

use crate::catalog::parser;
use crate::gates::logic::flag_gate;
use crate::gates::scanner_choice_tests::with_flag;
use std::path::PathBuf;

fn bundled_gate_files() -> Vec<(PathBuf, String)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("gates");
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&root).unwrap().flatten() {
        let dir = entry.path();
        if dir.is_dir() {
            let data = std::fs::read_to_string(dir.join("index.toml")).unwrap();
            files.push((dir, data));
        }
    }
    files
}

#[test]
fn every_gate_flag_in_bundled_data_names_a_declared_flag() {
    with_flag(None, || {
        let mut named = Vec::new();
        for (dir, data) in bundled_gate_files() {
            let fields = parser::parse(&data).unwrap();
            if let Some(name) = parser::optional_string(&fields, "flag").unwrap() {
                assert!(
                    crate::flags::is_declared(&name),
                    "{} names undeclared flag '{name}'",
                    dir.display()
                );
                named.push(name);
            }
        }
        assert_eq!(
            named,
            ["scanner-choice", "scanner-choice", "scanner-choice"]
        );
    });
}

#[test]
fn the_scanner_choice_pin_reads_the_flag_gate_loading_reads() {
    with_flag(Some("1"), || assert!(flag_gate::scanner_choice_on()));
    with_flag(Some("0"), || assert!(!flag_gate::scanner_choice_on()));
}
