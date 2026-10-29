//! Golden frames: the tui's screens, pinned as text.
//!
//! A screen's layout is a design decision, so it gets a committed frame the way
//! the site gets a screenshot: the diff shows exactly what moved, and a change
//! to the frame has to be deliberate. Re-record with `TJ_UPDATE_GOLDEN=1`.
#![cfg(unix)]

use std::path::PathBuf;

fn frame_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/cli/golden")
        .join(format!("{name}.txt"))
}

/// Replace what changes between machines or runs: the repository path, the
/// fixture's scratch directory, the home directory, and the version.
pub fn normalize(rows: &[String], fixture_root: &str) -> Vec<String> {
    let home = std::env::var("HOME").unwrap_or_default();
    let version = env!("CARGO_PKG_VERSION");
    rows.iter()
        .map(|row| {
            let row = row
                .trim_end()
                .replace(&terminal_jarvis::catalog::today_utc(), "<verified-at>");
            let row = row.replace(env!("CARGO_MANIFEST_DIR"), "<repo>");
            let row = row.replace(fixture_root, "<fixture>");
            let row = if home.is_empty() {
                row
            } else {
                row.replace(&home, "$HOME")
            };
            row.replace(version, "<version>")
        })
        .collect()
}

/// Compare a screen with its committed frame, or re-record it.
pub fn check(name: &str, rows: &[String], fixture_root: &str) {
    let mut actual = normalize(rows, fixture_root).join("\n");
    actual.push('\n');
    let file = frame_path(name);
    if std::env::var("TJ_UPDATE_GOLDEN").is_ok() {
        std::fs::create_dir_all(file.parent().expect("a golden directory")).unwrap();
        std::fs::write(&file, actual).expect("the golden frame writes");
        return;
    }
    let expected = std::fs::read_to_string(&file).unwrap_or_else(|error| {
        panic!("no golden frame for {name}: {error}; record it with TJ_UPDATE_GOLDEN=1")
    });
    assert_eq!(
        expected, actual,
        "the {name} screen changed; re-record it if that is the plan"
    );
}
