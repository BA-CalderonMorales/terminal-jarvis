//! Two bounded pumps: cap both queued output and unterminated lines.
use super::live::Line;
use std::io::{self, BufRead};
use std::sync::mpsc::SyncSender;
pub const CHUNK: u64 = 16 * 1024;

pub fn pump<R: io::Read + Send + 'static>(pipe: R, tag: fn(String) -> Line, tx: SyncSender<Line>) {
    std::thread::spawn(move || {
        let mut reader = io::BufReader::new(pipe);
        loop {
            let mut bytes = Vec::new();
            match io::Read::take(&mut reader, CHUNK).read_until(b'\n', &mut bytes) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    if bytes.last() == Some(&b'\n') {
                        bytes.pop();
                    }
                    if bytes.last() == Some(&b'\r') {
                        bytes.pop();
                    }
                    if tx
                        .send(tag(String::from_utf8_lossy(&bytes).into_owned()))
                        .is_err()
                    {
                        break;
                    }
                }
            }
        }
    });
}
