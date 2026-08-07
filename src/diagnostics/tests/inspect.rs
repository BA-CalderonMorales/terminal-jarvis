use super::{io_code, path, Code, Kind};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn temp_file(tag: &str) -> PathBuf {
    let file = std::env::temp_dir().join(format!("tj_inspect_{}_{}", std::process::id(), tag));
    let _ = fs::remove_file(&file);
    fs::write(&file, "data").unwrap();
    file
}

#[test]
fn io_code_maps_every_handled_kind() {
    assert_eq!(
        io_code(&io::Error::new(io::ErrorKind::NotFound, "")),
        Code::Missing
    );
    assert_eq!(
        io_code(&io::Error::new(io::ErrorKind::PermissionDenied, "")),
        Code::PermissionDenied
    );
    assert_eq!(
        io_code(&io::Error::new(io::ErrorKind::InvalidData, "")),
        Code::Malformed
    );
    assert_eq!(
        io_code(&io::Error::new(io::ErrorKind::InvalidInput, "")),
        Code::Malformed
    );
    assert_eq!(
        io_code(&io::Error::new(io::ErrorKind::UnexpectedEof, "")),
        Code::Unknown
    );
}

#[test]
fn exactly_fresh_metadata_is_not_stale() {
    let file = temp_file("fresh");
    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    assert_eq!(
        path(&file, Kind::Any, modified, Some(Duration::ZERO)),
        Code::Ready
    );
    assert_eq!(
        path(
            &file,
            Kind::Any,
            modified + Duration::from_secs(1),
            Some(Duration::from_secs(1))
        ),
        Code::Ready
    );
    let _ = fs::remove_file(&file);
}

#[test]
fn strictly_older_metadata_is_stale() {
    let file = temp_file("old");
    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    assert_eq!(
        path(
            &file,
            Kind::Any,
            modified + Duration::from_secs(2),
            Some(Duration::from_secs(1))
        ),
        Code::Stale
    );
    let _ = fs::remove_file(&file);
}

#[test]
fn missing_path_and_empty_file_map_to_codes() {
    assert_eq!(
        path(
            &PathBuf::from("/nonexistent/tj-xyz"),
            Kind::Any,
            SystemTime::UNIX_EPOCH,
            None
        ),
        Code::Missing
    );
    let file = temp_file("empty");
    fs::write(&file, "").unwrap();
    assert_eq!(
        path(&file, Kind::Any, SystemTime::UNIX_EPOCH, None),
        Code::Empty
    );
    let _ = fs::remove_file(&file);
}
