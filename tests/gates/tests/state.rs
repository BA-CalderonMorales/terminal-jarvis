use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use terminal_jarvis::context;
use terminal_jarvis::gates;

static ID: AtomicUsize = AtomicUsize::new(0);

fn dir() -> PathBuf {
    // process id + counter already guarantee uniqueness within this file; a
    // thread name is not a safe path component (Rust's default test-thread
    // names are the full `::`-separated test path, and `:` is invalid in
    // Windows paths). The "state" tag (coverage.rs has its own "coverage"
    // tag) keeps this file's counter from colliding with coverage.rs's --
    // both start at 0 and run in the same process.
    let path = std::env::temp_dir().join(format!(
        "tj-gatecov-state-{}-{}",
        std::process::id(),
        ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn home() -> PathBuf {
    let path = dir().join("home");
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn selection_lifecycle_round_trips_through_config() {
    let h = home();
    assert!(gates::selected(&h).unwrap().is_none());
    gates::enable(&h, "trivy").unwrap();
    let sel = gates::selected(&h).unwrap().unwrap();
    assert_eq!(sel.name, "trivy");
    assert_eq!(sel.source, "config");
    gates::disable(&h).unwrap();
    assert!(gates::selected(&h).unwrap().is_none());
}

#[test]
fn gates_root_resolves_to_a_gates_directory() {
    let root = context::gates_root();
    assert!(root.ends_with("gates"));
}
