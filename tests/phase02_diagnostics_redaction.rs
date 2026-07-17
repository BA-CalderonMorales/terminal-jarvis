#[path = "phase02_diagnostics_support/redaction.rs"]
mod phase02_diagnostics_redaction_support;

use phase02_diagnostics_redaction_support::*;
use std::fs;
use terminal_jarvis::diagnostics::collect;

#[test]
fn report_is_redacted_deterministic_and_read_only() {
    let root = fixture("redaction");
    let private_home = root.join("home/alice-sensitive");
    let home = private_home.join(".config/terminal-jarvis");
    let catalog = root.join("catalog");
    let bin = root.join("bin");
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(&catalog).unwrap();
    fs::create_dir_all(&bin).unwrap();
    fs::write(catalog.join("index.toml"), "catalog-marker").unwrap();
    fs::write(home.join("session.toml"), "active_harness = \"fixture\"\n").unwrap();
    fs::write(root.join("cache"), "cache-marker").unwrap();
    executable(&bin.join("tj"));
    executable(&bin.join("fixture"));
    let before = snapshot(&root);
    let input = input(&root, &private_home, &home, &catalog, &bin);
    let first = collect(&input);
    let second = collect(&input);
    let output = format!("{}\n{}", first.plain(), first.json());
    assert!(first.ok && first.exit_code() == 0);
    assert_eq!(first, second);
    assert_eq!(first.plain(), second.plain());
    assert_eq!(first.json(), second.json());
    assert!(output.contains("SECRET_API_TOKEN") && output.contains("present"));
    assert!(output.contains("EMPTY_TOKEN") && output.contains("empty"));
    assert!(!output.contains("hunter2-super-secret") && !output.contains("alice-sensitive"));
    assert!(!output.contains("acme-sensitive-directory") && output.contains("missing-tool"));
    assert!(output.contains("~/") && output.contains("$TMP/"));
    assert!(first
        .json()
        .starts_with("{\"schema_version\":1,\"command\":\"check\""));
    assert_eq!(before, snapshot(&root));
    fs::remove_dir_all(root).unwrap();
}
