use super::*;

fn temp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("tj-atomic-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn staged_left(dir: &Path) -> usize {
    fs::read_dir(dir)
        .unwrap()
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
        .count()
}

#[test]
fn write_atomic_replaces_an_existing_file_and_leaves_no_staged_file() {
    let dir = temp("replace");
    let path = dir.join("settings.toml");
    write_atomic(&path, "one\n").unwrap();
    write_atomic(&path, "two\n").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "two\n");
    assert_eq!(staged_left(&dir), 0);
}

/// A directory in the way fails the rename without AlreadyExists, so the
/// error path must report it and remove the staged file (a guard that sent
/// this to the windows replace arm would leave the staged file behind).
#[test]
fn write_atomic_onto_a_directory_errors_and_cleans_up_the_staged_file() {
    let dir = temp("directory");
    let path = dir.join("settings.toml");
    fs::create_dir_all(path.join("occupied")).unwrap();
    assert!(write_atomic(&path, "data\n").is_err());
    assert!(path.is_dir());
    assert_eq!(staged_left(&dir), 0);
}
