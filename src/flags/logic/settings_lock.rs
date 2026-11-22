//! A std-only cross-process lock: `<file>.lock` created with `create_new`,
//! a bounded retry, stale-lock recovery, and release on drop.

use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

const WAIT: Duration = Duration::from_secs(2);
const STALE: Duration = Duration::from_secs(30);
const POLL: Duration = Duration::from_millis(20);

#[derive(Debug)]
pub struct Lock {
    path: PathBuf,
}

impl Lock {
    /// Locks `file` for a read-modify-write (waits up to 2s; a lock older
    /// than 30s is presumed abandoned by a crashed writer and taken over).
    pub fn acquire(file: &Path) -> io::Result<Self> {
        Self::acquire_with(file, WAIT, STALE)
    }

    pub fn acquire_with(file: &Path, wait: Duration, stale: Duration) -> io::Result<Self> {
        let mut name = file.as_os_str().to_owned();
        name.push(".lock");
        let path = PathBuf::from(name);
        let deadline = Instant::now() + wait;
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(_) => return Ok(Self { path }),
                Err(error) if error.kind() != io::ErrorKind::AlreadyExists => return Err(error),
                Err(_) if is_stale(&path, stale) => {
                    let _ = fs::remove_file(&path);
                    continue;
                }
                Err(_) if Instant::now() >= deadline => {
                    return Err(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        format!("{} is held by another writer", path.display()),
                    ))
                }
                Err(_) => std::thread::sleep(POLL),
            }
        }
    }
}

fn is_stale(path: &Path, stale: Duration) -> bool {
    fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|at| SystemTime::now().duration_since(at).ok())
        .is_some_and(|age| age > stale)
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
