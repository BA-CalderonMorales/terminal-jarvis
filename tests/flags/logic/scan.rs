//! Finds `flags::enabled("<literal>")` call sites in a source tree. Comments
//! never count; whitespace (a rustfmt line break) after `(` does.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const CALL: &str = "flags::enabled(";

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
            collect(&strip_comments(&fs::read_to_string(&path).unwrap()), names);
        }
    }
}

fn collect(source: &str, names: &mut BTreeSet<String>) {
    for (at, _) in source.match_indices(CALL) {
        let rest = source[at + CALL.len()..].trim_start();
        if let Some(literal) = rest.strip_prefix('"') {
            if let Some(end) = literal.find('"') {
                names.insert(literal[..end].to_string());
            }
        }
    }
}

/// Drops `//` and `/* */` comments, leaving string literals (and the `'"'`
/// char literal) intact.
pub fn strip_comments(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let (mut out, mut at) = (String::new(), 0);
    let mut quoted = false;
    while at < chars.len() {
        let (c, next) = (chars[at], chars.get(at + 1).copied());
        if quoted {
            let end = if c == '\\' { at + 2 } else { at + 1 };
            out.extend(&chars[at..end.min(chars.len())]);
            quoted = c != '"';
            at = end;
        } else if c == '/' && next == Some('/') {
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
        } else if c == '/' && next == Some('*') {
            at += 2;
            while at < chars.len() && !(chars[at] == '*' && chars.get(at + 1) == Some(&'/')) {
                at += 1;
            }
            at += 2;
        } else if c == '\''
            && (chars[at..].starts_with(&['\'', '"', '\''])
                || chars[at..].starts_with(&['\'', '\\', '"', '\'']))
        {
            let len = if next == Some('"') { 3 } else { 4 };
            out.extend(&chars[at..at + len]);
            at += len;
        } else {
            quoted = c == '"';
            out.push(c);
            at += 1;
        }
    }
    out
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
