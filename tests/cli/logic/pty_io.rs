//! PtyIo: the background byte drain and marker wait shared by the pty
//! probes. Reads the master end until the child closes it.

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn drain(mut master: File, bytes: Arc<Mutex<Vec<u8>>>) {
    let mut buffer = [0u8; 65536];
    loop {
        match master.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => bytes.lock().unwrap().extend_from_slice(&buffer[..size]),
            Err(error) if error.raw_os_error() == Some(5) => break,
            Err(error) => panic!("pseudo-terminal read failed: {error}"),
        }
    }
}

pub fn wait_until(bytes: &Arc<Mutex<Vec<u8>>>, marker: &[u8]) {
    for _ in 0..=500 {
        let seen = bytes.lock().unwrap().clone();
        if seen.windows(marker.len()).any(|window| window == marker) {
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
    panic!("timed out waiting for {marker:?} in: {bytes:?}");
}
