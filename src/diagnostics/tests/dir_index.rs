use super::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_dir_with_files(names: &[&str]) -> PathBuf {
    let tag = COUNTER.fetch_add(1, Ordering::SeqCst);
    let directory = std::env::temp_dir().join(format!("dir_index_{}_{}", std::process::id(), tag));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    for name in names {
        fs::write(directory.join(name), "").unwrap();
    }
    directory
}

#[test]
fn locate_finds_present_candidates_across_dirs() {
    let first = temp_dir_with_files(&["alpha", "beta"]);
    let second = temp_dir_with_files(&["gamma"]);
    let index = DirIndex::from_paths(&[first.clone(), second.clone()]);
    assert_eq!(index.locate(&["alpha".to_string()]).len(), 1);
    assert_eq!(index.locate(&["missing".to_string()]).len(), 0);
    assert_eq!(
        index
            .locate(&["beta".to_string(), "gamma".to_string()])
            .len(),
        2
    );
    let _ = fs::remove_dir_all(&first);
    let _ = fs::remove_dir_all(&second);
}

#[test]
fn duplicate_directories_are_indexed_once() {
    let directory = temp_dir_with_files(&["alpha"]);
    let index = DirIndex::from_paths(&[directory.clone(), directory.clone()]);
    assert_eq!(index.locate(&["alpha".to_string()]).len(), 1);
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn missing_directories_are_skipped() {
    let oracle = temp_dir_with_files(&["alpha"]);
    let index = DirIndex::from_paths(&[PathBuf::from("/nonexistent/dir"), oracle.clone()]);
    assert_eq!(index.locate(&["alpha".to_string()]).len(), 1);
    let _ = fs::remove_dir_all(&oracle);
}
