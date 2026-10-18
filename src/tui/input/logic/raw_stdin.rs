//! RawStdin: the tui's one unbuffered stdin reader.
//!
//! `std::io::stdin()` buffers. A single 1-byte read fills its 8 KiB buffer,
//! and the bytes left there are invisible to `poll` -- so a typed-ahead line
//! stalls until the next byte reaches the fd, and a resize can never wake a
//! reader that is waiting on a buffer nobody will fill. The tui reads fd 0
//! directly instead, so `poll` and `read` always agree on the same buffer.

use std::io::{self, Read};
use std::sync::Mutex;

/// Serializes reads: the escape resolver's thread and the prompt must never
/// split a byte sequence between them.
static LOCK: Mutex<()> = Mutex::new(());

/// One read from stdin, unbuffered. `Ok(0)` is EOF.
pub fn read(buf: &mut [u8]) -> io::Result<usize> {
    let _serial = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(unix)]
    {
        use std::os::fd::FromRawFd;
        // fd 0 belongs to the process, not to this handle: ManuallyDrop keeps
        // the temporary from closing it on drop.
        let file = std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(0) });
        (&*file).read(buf)
    }
    #[cfg(not(unix))]
    {
        io::stdin().read(buf)
    }
}

/// One byte, or None at EOF. Unbuffered, so a caller's `poll` agrees.
pub fn read_byte() -> Option<u8> {
    let mut one = [0u8; 1];
    match read(&mut one) {
        Ok(1) => Some(one[0]),
        _ => None,
    }
}

/// The unbuffered stdin as a `Read`, for the decoders that take one.
pub struct Stdin;

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        read(buf)
    }
}
