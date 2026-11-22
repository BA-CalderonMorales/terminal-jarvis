use super::support::temp;
use crate::flags::logic::settings_lock::Lock;
use std::fs;
use std::io::ErrorKind;
use std::time::{Duration, SystemTime};

const NO_WAIT: Duration = Duration::ZERO;
const STALE: Duration = Duration::from_secs(30);

#[test]
fn acquire_creates_the_lock_file_and_drop_releases_it() {
    let file = temp("lock").join("settings.toml");
    let lock_path = file.with_file_name("settings.toml.lock");
    let lock = Lock::acquire_with(&file, NO_WAIT, STALE).unwrap();
    assert!(lock_path.exists());
    drop(lock);
    assert!(!lock_path.exists());
    assert!(Lock::acquire_with(&file, NO_WAIT, STALE).is_ok());
}

#[test]
fn a_held_lock_blocks_a_second_writer_without_waiting() {
    let file = temp("held").join("settings.toml");
    let _held = Lock::acquire_with(&file, NO_WAIT, STALE).unwrap();
    let error = Lock::acquire_with(&file, NO_WAIT, STALE).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::WouldBlock);
    assert!(error.to_string().contains("held by another writer"));
}

#[test]
fn a_stale_lock_is_recovered() {
    let file = temp("stale").join("settings.toml");
    let lock_path = file.with_file_name("settings.toml.lock");
    let abandoned = fs::File::create(&lock_path).unwrap();
    let old = SystemTime::now() - Duration::from_secs(120);
    abandoned.set_modified(old).unwrap();
    drop(abandoned);
    let lock = Lock::acquire_with(&file, NO_WAIT, STALE).unwrap();
    assert!(lock_path.exists());
    drop(lock);
    assert!(!lock_path.exists());
}
