//! Finds `flags::enabled("<literal>")` call sites in a source tree.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const CALL: &str = "flags::enabled(\"";

/// Literal flag names asked for under `root`, skipping test trees and the
/// flags domain itself (its docs and tests name flags as examples).
pub fn used(root: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    walk(root, &mut names);
    names
}

fn walk(dir: &Path, names: &mut BTreeSet<String>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if name != "tests" && name != "flags" {
                walk(&path, names);
            }
        } else if name.ends_with(".rs") {
            collect(&fs::read_to_string(&path).unwrap(), names);
        }
    }
}

fn collect(source: &str, names: &mut BTreeSet<String>) {
    for (at, _) in source.match_indices(CALL) {
        let rest = &source[at + CALL.len()..];
        if let Some(end) = rest.find('"') {
            names.insert(rest[..end].to_string());
        }
    }
}

/// Human-readable problems: undeclared uses and dead (unused) declarations.
pub fn problems(declared: &BTreeSet<String>, used: &BTreeSet<String>) -> Vec<String> {
    let mut found = Vec::new();
    for name in used.difference(declared) {
        found.push(format!("'{name}' is used but not declared in flags/"));
    }
    for name in declared.difference(used) {
        found.push(format!("'{name}' is declared but never used; delete it"));
    }
    found
}
