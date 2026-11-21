//! Declared <-> used: every flag literal in src/ is declared, every flag
//! declared is used (promoting a flag deletes it).

use crate::logic::sandbox::Sandbox;
use crate::logic::scan;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use terminal_jarvis::flags;

fn embedded() -> Vec<flags::Flag> {
    flags::load(Path::new(terminal_jarvis::context::EMBEDDED_FLAGS)).unwrap()
}

fn src() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn declared_flags_and_used_flags_are_the_same_set() {
    let declared: BTreeSet<_> = embedded().into_iter().map(|flag| flag.name).collect();
    let problems = scan::problems(&declared, &scan::used(&src()));
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn the_scan_catches_undeclared_and_dead_flags() {
    let root = Sandbox::new(None).home.join("tree");
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::create_dir_all(root.join("flags")).unwrap();
    fs::write(
        root.join("a.rs"),
        "if crate::flags::enabled(\"used-one\") {}\nflags::enabled(\"stray\");",
    )
    .unwrap();
    fs::write(root.join("tests/skip.rs"), "flags::enabled(\"in-tests\")").unwrap();
    fs::write(root.join("flags/skip.rs"), "flags::enabled(\"in-domain\")").unwrap();
    let used = scan::used(&root);
    assert_eq!(used.iter().collect::<Vec<_>>(), ["stray", "used-one"]);
    let declared: BTreeSet<_> = ["used-one", "dead-one"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let problems = scan::problems(&declared, &used);
    assert_eq!(problems.len(), 2, "{problems:?}");
    assert!(problems
        .iter()
        .any(|p| p.contains("'stray'") && p.contains("not declared")));
    assert!(problems
        .iter()
        .any(|p| p.contains("'dead-one'") && p.contains("never used")));
}
