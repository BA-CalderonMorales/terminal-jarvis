//! Raw: flips the controlling tty into per-byte, no-echo mode so arrow
//! keys arrive as they happen; the guard restores cooked mode on drop, so
//! child runs and unwinds always get a sane terminal. `stty` does the
//! termios work -- boring and portable, zero unsafe.

use std::fs::File;
use std::process::{Command, Stdio};

/// Owns cooked-mode restoration for one prompt lifetime.
pub struct Guard {
    saved: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        stty(&[&self.saved]);
    }
}

/// Enters raw mode; `None` keeps the caller on its legacy read path.
pub fn enable() -> Option<Guard> {
    let saved = stty(&["-g"])?.trim().to_string();
    if saved.is_empty() || stty(&["raw", "-echo"]).is_none() {
        return None;
    }
    Some(Guard { saved })
}

fn stty(args: &[&str]) -> Option<String> {
    let tty = File::open("/dev/tty").ok()?;
    let out = Command::new("stty")
        .args(args)
        .stdin(Stdio::from(tty))
        .stderr(Stdio::null())
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).to_string())
}
