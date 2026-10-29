//! Two bounded pumps preserve UTF-8 across chunk boundaries.
use super::live::Line;
use std::io::{self, BufRead};
use std::sync::mpsc::SyncSender;
pub const CHUNK: u64 = 16 * 1024;

pub fn pump<R: io::Read + Send + 'static>(pipe: R, tag: fn(String) -> Line, tx: SyncSender<Line>) {
    std::thread::spawn(move || {
        let mut reader = io::BufReader::new(pipe);
        let mut pending = Vec::new();
        loop {
            let mut bytes = std::mem::take(&mut pending);
            let limit = CHUNK - bytes.len() as u64;
            match io::Read::take(&mut reader, limit).read_until(b'\n', &mut bytes) {
                Ok(0) | Err(_) => {
                    if !bytes.is_empty() {
                        let _ = tx.send(tag(String::from_utf8_lossy(&bytes).into_owned()));
                    }
                    break;
                }
                Ok(_) => {
                    let newline = bytes.last() == Some(&b'\n');
                    if newline {
                        bytes.pop();
                        if bytes.last() == Some(&b'\r') {
                            bytes.pop();
                        }
                    } else if let Err(error) = std::str::from_utf8(&bytes) {
                        if error.error_len().is_none() {
                            pending = bytes.split_off(error.valid_up_to());
                        }
                    }
                    if bytes.is_empty() && !newline {
                        continue;
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

#[cfg(test)]
#[path = "../tests/pump.rs"]
mod tests;
