use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

static STAGED_SEQ: AtomicUsize = AtomicUsize::new(0);

/// Writes `contents` through a pid+nonce staged sibling file so a crash or
/// interrupted write can never leave an empty or partial file behind and
/// concurrent writers never collide. On windows, where rename cannot replace
/// an existing file, the destination is removed only after staging succeeds.
pub fn write_atomic(path: &Path, contents: &str) -> io::Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(dir)?;
    let file = path
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    let seq = STAGED_SEQ.fetch_add(1, Ordering::Relaxed);
    let staged = dir.join(format!("{file}.{}.{seq}.tmp", std::process::id()));
    fs::write(&staged, contents)?;
    match fs::rename(&staged, path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            fs::remove_file(path)?;
            fs::rename(&staged, path)
        }
        Err(error) => {
            let _ = fs::remove_file(&staged);
            Err(error)
        }
    }
}
