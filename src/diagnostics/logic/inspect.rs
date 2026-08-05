use super::Code;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy)]
pub enum Kind {
    Any,
    Directory,
}

pub fn path(path: &Path, kind: Kind, now: SystemTime, stale_after: Option<Duration>) -> Code {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) => return io_code(&error),
    };
    let shape_ok = match kind {
        Kind::Any => metadata.is_file() || metadata.is_dir(),
        Kind::Directory => metadata.is_dir(),
    };
    if !shape_ok {
        return Code::Malformed;
    }
    if metadata.is_file() && metadata.len() == 0 {
        return Code::Empty;
    }
    if metadata.is_dir() {
        match fs::read_dir(path) {
            Err(error) => return io_code(&error),
            Ok(mut entries) => match entries.next() {
                None => return Code::Empty,
                Some(Err(error)) => return io_code(&error),
                Some(Ok(_)) => {}
            },
        }
    }
    if is_stale(&metadata, now, stale_after) {
        Code::Stale
    } else {
        Code::Ready
    }
}

pub fn writable(target: &Path, kind: Kind, now: SystemTime, stale_after: Option<Duration>) -> Code {
    let code = path(target, kind, now, stale_after);
    if !matches!(code, Code::Ready | Code::Empty | Code::Stale) {
        return code;
    }
    let Ok(metadata) = fs::metadata(target) else {
        return Code::Unknown;
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o222 == 0 {
            return Code::PermissionDenied;
        }
    }
    #[cfg(not(unix))]
    if metadata.permissions().readonly() {
        return Code::PermissionDenied;
    }
    code
}

pub fn io_code(error: &io::Error) -> Code {
    match error.kind() {
        io::ErrorKind::NotFound => Code::Missing,
        io::ErrorKind::PermissionDenied => Code::PermissionDenied,
        io::ErrorKind::InvalidData | io::ErrorKind::InvalidInput => Code::Malformed,
        _ => Code::Unknown,
    }
}

fn is_stale(metadata: &fs::Metadata, now: SystemTime, limit: Option<Duration>) -> bool {
    let (Some(limit), Ok(modified)) = (limit, metadata.modified()) else {
        return false;
    };
    now.duration_since(modified).is_ok_and(|age| age > limit)
}
